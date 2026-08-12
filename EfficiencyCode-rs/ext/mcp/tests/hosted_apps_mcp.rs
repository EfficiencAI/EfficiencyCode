use std::sync::Arc;

use EfficiencyCode_config::McpServerTransportConfig;
use EfficiencyCode_core::McpManager;
use EfficiencyCode_core::config::Config;
use EfficiencyCode_core::config::ConfigBuilder;
use EfficiencyCode_core::plugins_manager_for_config;
use EfficiencyCode_extension_api::ExtensionRegistryBuilder;
use EfficiencyCode_extension_api::McpServerContribution;
use EfficiencyCode_extension_api::McpServerContributionContext;
use EfficiencyCode_extension_api::McpServerContributor;
use EfficiencyCode_login::EfficiencyCodeAuth;
use EfficiencyCode_mcp::EfficiencyCode_APPS_MCP_SERVER_NAME;
use pretty_assertions::assert_eq;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[tokio::test]
async fn contributes_hosted_plugin_runtime_without_an_executor() -> TestResult {
    let EfficiencyCode_home = tempfile::tempdir()?;
    let config = ConfigBuilder::default()
        .EfficiencyCode_home(EfficiencyCode_home.path().to_path_buf())
        .fallback_cwd(Some(EfficiencyCode_home.path().to_path_buf()))
        .cli_overrides(vec![
            ("features.apps".to_string(), true.into()),
            ("chatgpt_base_url".to_string(), "https://chatgpt.com".into()),
        ])
        .build()
        .await?;
    let auth = EfficiencyCodeAuth::create_dummy_chatgpt_auth_for_testing();
    let manager = installed_manager(&config);

    let servers = manager.effective_servers(&config, Some(&auth)).await;
    let server = servers
        .get(EfficiencyCode_APPS_MCP_SERVER_NAME)
        .ok_or("hosted plugin runtime should be contributed as a configured server")?
        .config();
    let McpServerTransportConfig::StreamableHttp { url, .. } = &server.transport else {
        panic!("hosted plugin runtime should use streamable HTTP");
    };
    assert_eq!(url, "https://chatgpt.com/backend-api/ps/mcp");

    Ok(())
}

#[tokio::test]
async fn runtime_overlay_preserves_disabled_server() -> TestResult {
    let EfficiencyCode_home = tempfile::tempdir()?;
    let config = ConfigBuilder::default()
        .EfficiencyCode_home(EfficiencyCode_home.path().to_path_buf())
        .fallback_cwd(Some(EfficiencyCode_home.path().to_path_buf()))
        .cli_overrides(vec![
            ("features.apps".to_string(), true.into()),
            (
                "mcp_servers.EfficiencyCode_apps.url".to_string(),
                "https://example.com/mcp".into(),
            ),
            (
                "mcp_servers.EfficiencyCode_apps.enabled".to_string(),
                false.into(),
            ),
        ])
        .build()
        .await?;
    let auth = EfficiencyCodeAuth::create_dummy_chatgpt_auth_for_testing();
    let manager = installed_manager(&config);

    let servers = manager.effective_servers(&config, Some(&auth)).await;
    let server = servers
        .get(EfficiencyCode_APPS_MCP_SERVER_NAME)
        .ok_or("hosted plugin runtime should remain configured")?;

    assert!(!server.enabled());
    Ok(())
}

#[tokio::test]
async fn default_fallback_overwrites_reserved_config_without_an_extension() -> TestResult {
    let EfficiencyCode_home = tempfile::tempdir()?;
    let config = ConfigBuilder::default()
        .EfficiencyCode_home(EfficiencyCode_home.path().to_path_buf())
        .fallback_cwd(Some(EfficiencyCode_home.path().to_path_buf()))
        .cli_overrides(vec![
            ("features.apps".to_string(), true.into()),
            (
                "mcp_servers.EfficiencyCode_apps.url".to_string(),
                "https://example.com/mcp".into(),
            ),
        ])
        .build()
        .await?;
    let auth = EfficiencyCodeAuth::create_dummy_chatgpt_auth_for_testing();
    let manager = McpManager::new(Arc::new(plugins_manager_for_config(&config)));

    let servers = manager.effective_servers(&config, Some(&auth)).await;
    let server = servers
        .get(EfficiencyCode_APPS_MCP_SERVER_NAME)
        .ok_or("default Apps MCP should be present")?
        .config();
    let McpServerTransportConfig::StreamableHttp { url, .. } = &server.transport else {
        panic!("default Apps MCP should use streamable HTTP");
    };
    assert_eq!(url, "https://chatgpt.com/backend-api/ps/mcp");

    Ok(())
}

#[tokio::test]
async fn later_extension_can_remove_same_name_registration() -> TestResult {
    let EfficiencyCode_home = tempfile::tempdir()?;
    let config = ConfigBuilder::default()
        .EfficiencyCode_home(EfficiencyCode_home.path().to_path_buf())
        .fallback_cwd(Some(EfficiencyCode_home.path().to_path_buf()))
        .cli_overrides(vec![("features.apps".to_string(), true.into())])
        .build()
        .await?;
    let auth = EfficiencyCodeAuth::create_dummy_chatgpt_auth_for_testing();
    let mut builder = ExtensionRegistryBuilder::new();
    EfficiencyCode_mcp_extension::install(&mut builder);
    builder.mcp_server_contributor(Arc::new(RemoveEfficiencyCodeApps));
    let manager = McpManager::new_with_extensions(
        Arc::new(plugins_manager_for_config(&config)),
        Arc::new(builder.build()),
        EfficiencyCode_core::EfficiencyCodeAppsToolsCache::default(),
    );

    let servers = manager.effective_servers(&config, Some(&auth)).await;

    assert!(!servers.contains_key(EfficiencyCode_APPS_MCP_SERVER_NAME));
    Ok(())
}

#[tokio::test]
async fn hosted_apps_mcp_requires_chatgpt_auth() -> TestResult {
    let EfficiencyCode_home = tempfile::tempdir()?;
    let config = ConfigBuilder::default()
        .EfficiencyCode_home(EfficiencyCode_home.path().to_path_buf())
        .fallback_cwd(Some(EfficiencyCode_home.path().to_path_buf()))
        .cli_overrides(vec![("features.apps".to_string(), true.into())])
        .build()
        .await?;
    let auth = EfficiencyCodeAuth::from_api_key("test");
    let manager = installed_manager(&config);

    let servers = manager.effective_servers(&config, Some(&auth)).await;
    assert!(!servers.contains_key(EfficiencyCode_APPS_MCP_SERVER_NAME));

    Ok(())
}

#[tokio::test]
async fn disabled_apps_remove_reserved_server_config_for_all_hosts() -> TestResult {
    let EfficiencyCode_home = tempfile::tempdir()?;
    let config = ConfigBuilder::default()
        .EfficiencyCode_home(EfficiencyCode_home.path().to_path_buf())
        .fallback_cwd(Some(EfficiencyCode_home.path().to_path_buf()))
        .cli_overrides(vec![
            ("features.apps".to_string(), false.into()),
            (
                "mcp_servers.EfficiencyCode_apps.url".to_string(),
                "https://example.com/mcp".into(),
            ),
        ])
        .build()
        .await?;
    let managers = [
        installed_manager(&config),
        McpManager::new(Arc::new(plugins_manager_for_config(&config))),
    ];
    for manager in managers {
        let servers = manager.runtime_servers(&config).await;
        assert!(!servers.contains_key(EfficiencyCode_APPS_MCP_SERVER_NAME));
    }
    Ok(())
}

fn installed_manager(config: &Config) -> McpManager {
    let mut builder = ExtensionRegistryBuilder::new();
    EfficiencyCode_mcp_extension::install(&mut builder);
    McpManager::new_with_extensions(
        Arc::new(plugins_manager_for_config(config)),
        Arc::new(builder.build()),
        EfficiencyCode_core::EfficiencyCodeAppsToolsCache::default(),
    )
}

struct RemoveEfficiencyCodeApps;

impl McpServerContributor<Config> for RemoveEfficiencyCodeApps {
    fn id(&self) -> &'static str {
        "remove_EfficiencyCode_apps"
    }

    fn contribute<'a>(
        &'a self,
        _context: McpServerContributionContext<'a, Config>,
    ) -> EfficiencyCode_extension_api::ExtensionFuture<'a, Vec<McpServerContribution>> {
        Box::pin(async move {
            vec![McpServerContribution::Remove {
                name: EfficiencyCode_APPS_MCP_SERVER_NAME.to_string(),
            }]
        })
    }
}
