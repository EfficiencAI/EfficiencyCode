use EfficiencyCode_utils_absolute_path::test_support::PathExt;
use pretty_assertions::assert_eq;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use uuid::Uuid;

use super::*;

#[tokio::test]
async fn sqlite_sink_filters_noisy_targets_without_dropping_useful_diagnostics() {
    let EfficiencyCode_home = std::env::temp_dir().join(format!(
        "EfficiencyCode-state-log-db-filter-{}",
        Uuid::new_v4()
    ));
    let _cleanup = scopeguard::guard(EfficiencyCode_home.clone(), |EfficiencyCode_home| {
        let _ = std::fs::remove_dir_all(EfficiencyCode_home);
    });
    let runtime = StateRuntime::init(
        crate::SqliteConfig::new_for_testing(EfficiencyCode_home.as_path().abs()),
        "test-provider".to_string(),
    )
    .await
    .expect("initialize runtime");
    let layer = start(runtime.clone());

    let guard = tracing_subscriber::registry()
        .with(layer.clone().with_filter(default_filter()))
        .set_default();

    tracing::trace!(target: "opentelemetry_sdk", "dropped-trace");
    tracing::debug!(target: "opentelemetry_sdk", "dropped-debug");
    tracing::info!(target: "opentelemetry_sdk", "retained-info");
    tracing::debug!(target: "rmcp::transport", "dropped-rmcp-debug");
    tracing::info!(target: "rmcp::transport", "retained-rmcp-info");
    tracing::debug!(
        target: "EfficiencyCode_rmcp_client::oauth",
        "dropped-EfficiencyCode-rmcp-client-debug"
    );
    tracing::info!(
        target: "EfficiencyCode_rmcp_client::oauth",
        "retained-EfficiencyCode-rmcp-client-info"
    );
    tracing::trace!(target: "EfficiencyCode_http_client::transport", "dropped-request-body");
    tracing::debug!(target: "EfficiencyCode_http_client::transport", "retained-request-diagnostic");
    tracing::trace!(target: "EfficiencyCode_api::sse", "dropped-sse-parent");
    tracing::trace!(target: "EfficiencyCode_api::sse::responses", "dropped-sse-payload");
    tracing::debug!(target: "EfficiencyCode_api::sse::responses", "retained-sse-diagnostic");
    tracing::trace!(target: "EfficiencyCode_state", "retained-trace");
    tracing::trace!(
        target: "EfficiencyCode_tui::streaming::controller",
        "dropped-controller-trace"
    );
    tracing::debug!(
        target: "EfficiencyCode_tui::streaming::controller",
        "retained-controller-debug"
    );
    tracing::trace!(
        target: "EfficiencyCode_tui::streaming::table_holdback",
        "dropped-table-holdback-trace"
    );
    tracing::debug!(
        target: "EfficiencyCode_tui::streaming::table_holdback",
        "retained-table-holdback-debug"
    );
    tracing::trace!(
        target: "EfficiencyCode_tui::streaming::commit_tick",
        "retained-commit-tick-trace"
    );
    tracing::trace!(
        target: "EfficiencyCode_api::responses_websocket_timing",
        payload = "complete timing payload",
        "dropped-websocket-timing"
    );

    layer.flush().await;
    drop(guard);

    let logs = runtime
        .query_logs(&crate::LogQuery::default())
        .await
        .expect("query logs after flush");
    assert_eq!(
        logs.iter()
            .map(|row| (
                row.level.as_str(),
                row.target.as_str(),
                row.message.as_deref()
            ))
            .collect::<Vec<_>>(),
        vec![
            ("INFO", "opentelemetry_sdk", Some("retained-info")),
            ("INFO", "rmcp::transport", Some("retained-rmcp-info")),
            (
                "INFO",
                "EfficiencyCode_rmcp_client::oauth",
                Some("retained-EfficiencyCode-rmcp-client-info")
            ),
            (
                "DEBUG",
                "EfficiencyCode_http_client::transport",
                Some("retained-request-diagnostic")
            ),
            (
                "DEBUG",
                "EfficiencyCode_api::sse::responses",
                Some("retained-sse-diagnostic")
            ),
            ("TRACE", "EfficiencyCode_state", Some("retained-trace")),
            (
                "DEBUG",
                "EfficiencyCode_tui::streaming::controller",
                Some("retained-controller-debug"),
            ),
            (
                "DEBUG",
                "EfficiencyCode_tui::streaming::table_holdback",
                Some("retained-table-holdback-debug"),
            ),
            (
                "TRACE",
                "EfficiencyCode_tui::streaming::commit_tick",
                Some("retained-commit-tick-trace"),
            ),
        ]
    );
}
