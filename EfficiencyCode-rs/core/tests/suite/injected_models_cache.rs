use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use EfficiencyCode_http_client::HttpClientFactory;
use EfficiencyCode_login::AuthManager;
use EfficiencyCode_login::EfficiencyCodeAuth;
use EfficiencyCode_models_manager::cache::ModelsCache;
use EfficiencyCode_models_manager::cache::ModelsCacheEntry;
use EfficiencyCode_models_manager::cache::ModelsCacheError;
use EfficiencyCode_models_manager::cache::ModelsCacheFuture;
use EfficiencyCode_models_manager::manager::ModelsEndpointClient;
use EfficiencyCode_models_manager::manager::ModelsEndpointFuture;
use EfficiencyCode_models_manager::manager::OpenAiModelsManager;
use EfficiencyCode_models_manager::manager::RefreshStrategy;
use EfficiencyCode_models_manager::manager::SharedModelsManager;
use EfficiencyCode_models_manager::model_info::model_info_from_slug;
use EfficiencyCode_protocol::error::Result as CoreResult;
use EfficiencyCode_protocol::openai_models::ModelInfo;
use EfficiencyCode_protocol::openai_models::ModelVisibility;
use EfficiencyCode_protocol::protocol::EventMsg;
use EfficiencyCode_protocol::protocol::Op;
use EfficiencyCode_protocol::protocol::ThreadSettingsOverrides;
use EfficiencyCode_protocol::user_input::UserInput;
use anyhow::Result;
use chrono::Utc;
use core_test_support::responses;
use core_test_support::responses::ev_assistant_message;
use core_test_support::responses::ev_completed;
use core_test_support::responses::ev_response_created;
use core_test_support::responses::mount_sse_once;
use core_test_support::responses::sse;
use core_test_support::submit_thread_settings;
use core_test_support::test_EfficiencyCode::test_EfficiencyCode;
use core_test_support::wait_for_event;
use pretty_assertions::assert_eq;

#[derive(Debug)]
struct TestModelsCache {
    entry: Option<ModelsCacheEntry>,
    load_error: bool,
    stored_entries: Mutex<Vec<ModelsCacheEntry>>,
}

impl ModelsCache for TestModelsCache {
    fn load<'a>(
        &'a self,
        _client_version: &'a str,
    ) -> ModelsCacheFuture<'a, Result<Option<ModelsCacheEntry>, ModelsCacheError>> {
        Box::pin(async move {
            if self.load_error {
                Err(ModelsCacheError::new("test load failure"))
            } else {
                Ok(self.entry.clone())
            }
        })
    }

    fn store<'a>(
        &'a self,
        entry: &'a ModelsCacheEntry,
    ) -> ModelsCacheFuture<'a, Result<(), ModelsCacheError>> {
        Box::pin(async move {
            self.stored_entries
                .lock()
                .expect("stored entries lock should not be poisoned")
                .push(entry.clone());
            Ok(())
        })
    }

    fn refresh_ttl<'a>(
        &'a self,
        _client_version: &'a str,
    ) -> ModelsCacheFuture<'a, Result<(), ModelsCacheError>> {
        Box::pin(async move {
            let mut entry = self
                .entry
                .clone()
                .ok_or_else(|| ModelsCacheError::new("cache not found"))?;
            entry.fetched_at = Utc::now();
            self.stored_entries
                .lock()
                .expect("stored entries lock should not be poisoned")
                .push(entry);
            Ok(())
        })
    }
}

#[derive(Debug)]
struct TestModelsEndpoint {
    models: Vec<ModelInfo>,
    fetch_count: AtomicUsize,
}

impl TestModelsEndpoint {
    fn new(models: Vec<ModelInfo>) -> Arc<Self> {
        Arc::new(Self {
            models,
            fetch_count: AtomicUsize::new(0),
        })
    }
}

impl ModelsEndpointClient for TestModelsEndpoint {
    fn has_command_auth(&self) -> bool {
        false
    }

    fn uses_EfficiencyCode_backend(&self) -> ModelsEndpointFuture<'_, bool> {
        Box::pin(async { true })
    }

    fn list_models<'a>(
        &'a self,
        _client_version: &'a str,
        _http_client_factory: HttpClientFactory,
    ) -> ModelsEndpointFuture<'a, CoreResult<(Vec<ModelInfo>, Option<String>)>> {
        Box::pin(async move {
            self.fetch_count.fetch_add(1, Ordering::SeqCst);
            Ok((self.models.clone(), None))
        })
    }
}

fn remote_model(slug: &str) -> ModelInfo {
    ModelInfo {
        visibility: ModelVisibility::List,
        used_fallback_model_metadata: false,
        ..model_info_from_slug(slug)
    }
}

fn models_manager(
    cache: Arc<dyn ModelsCache>,
    endpoint: Arc<TestModelsEndpoint>,
) -> SharedModelsManager {
    Arc::new(OpenAiModelsManager::new_with_cache(
        cache,
        endpoint,
        Some(AuthManager::from_auth_for_testing(
            EfficiencyCodeAuth::create_dummy_chatgpt_auth_for_testing(),
        )),
    ))
}

async fn run_agent_with_model(models_manager: SharedModelsManager, model_slug: &str) -> Result<()> {
    let server = responses::start_mock_server().await;
    let response_mock = mount_sse_once(
        &server,
        sse(vec![
            ev_response_created("resp-1"),
            ev_assistant_message("msg-1", "done"),
            ev_completed("resp-1"),
        ]),
    )
    .await;
    let mut builder = test_EfficiencyCode()
        .with_auth(EfficiencyCodeAuth::create_dummy_chatgpt_auth_for_testing())
        .with_models_manager(models_manager);
    let test = builder.build(&server).await?;
    let available_models = test
        .thread_manager
        .get_models_manager()
        .list_models(
            RefreshStrategy::OnlineIfUncached,
            EfficiencyCode_core::test_support::default_http_client_factory(),
        )
        .await;
    assert!(
        available_models
            .iter()
            .any(|model| model.model == model_slug)
    );

    submit_thread_settings(
        &test.EfficiencyCode,
        ThreadSettingsOverrides {
            model: Some(model_slug.to_string()),
            ..Default::default()
        },
    )
    .await?;
    test.EfficiencyCode
        .submit(Op::UserInput {
            items: vec![UserInput::Text {
                text: "hello".to_string(),
                text_elements: Vec::new(),
            }],
            final_output_json_schema: None,
            responsesapi_client_metadata: None,
            additional_context: Default::default(),
            thread_settings: Default::default(),
        })
        .await?;
    loop {
        if matches!(
            wait_for_event(&test.EfficiencyCode, |_| true).await,
            EventMsg::TurnComplete(_)
        ) {
            break;
        }
    }

    assert_eq!(
        response_mock.single_request().body_json()["model"],
        model_slug
    );
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn injected_cache_hit_drives_agent_model_selection() -> Result<()> {
    let model_slug = "injected-cache-model";
    let cache = Arc::new(TestModelsCache {
        entry: Some(ModelsCacheEntry {
            fetched_at: Utc::now(),
            etag: None,
            client_version: Some(EfficiencyCode_models_manager::client_version_to_whole()),
            models: vec![remote_model(model_slug)],
        }),
        load_error: false,
        stored_entries: Mutex::new(Vec::new()),
    });
    let endpoint = TestModelsEndpoint::new(Vec::new());

    run_agent_with_model(models_manager(cache, endpoint.clone()), model_slug).await?;

    assert_eq!(endpoint.fetch_count.load(Ordering::SeqCst), 0);
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn injected_cache_error_falls_back_for_agent_model_selection() -> Result<()> {
    let model_slug = "injected-cache-fallback-model";
    let cache = Arc::new(TestModelsCache {
        entry: None,
        load_error: true,
        stored_entries: Mutex::new(Vec::new()),
    });
    let endpoint = TestModelsEndpoint::new(vec![remote_model(model_slug)]);

    run_agent_with_model(models_manager(cache.clone(), endpoint.clone()), model_slug).await?;

    assert!(endpoint.fetch_count.load(Ordering::SeqCst) >= 1);
    assert!(
        !cache
            .stored_entries
            .lock()
            .expect("stored entries lock should not be poisoned")
            .is_empty()
    );
    Ok(())
}
