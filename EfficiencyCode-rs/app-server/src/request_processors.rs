use crate::bespoke_event_handling::apply_bespoke_event_handling;
use crate::command_exec::CommandExecManager;
use crate::command_exec::StartCommandExecParams;
use crate::config_manager::ConfigManager;
use crate::error_code::INPUT_TOO_LARGE_ERROR_CODE;
use crate::error_code::invalid_params;
use crate::models::supported_models;
use crate::outgoing_message::ConnectionId;
use crate::outgoing_message::ConnectionRequestId;
use crate::outgoing_message::OutgoingMessageSender;
use crate::outgoing_message::RequestContext;
use crate::outgoing_message::ThreadScopedOutgoingMessageSender;
use crate::skills_watcher::SkillsWatcher;
use crate::thread_status::ThreadWatchManager;
use crate::thread_status::resolve_thread_status;
use EfficiencyCode_analytics::AnalyticsEventsClient;
use EfficiencyCode_analytics::AnalyticsJsonRpcError;
use EfficiencyCode_analytics::InputError;
use EfficiencyCode_analytics::TurnSteerRequestError;
use EfficiencyCode_app_server_protocol::Account;
use EfficiencyCode_app_server_protocol::AccountLoginCompletedNotification;
use EfficiencyCode_app_server_protocol::AccountTokenUsageDailyBucket;
use EfficiencyCode_app_server_protocol::AccountTokenUsageSummary;
use EfficiencyCode_app_server_protocol::AccountUpdatedNotification;
use EfficiencyCode_app_server_protocol::AddCreditsNudgeCreditType;
use EfficiencyCode_app_server_protocol::AddCreditsNudgeEmailStatus;
use EfficiencyCode_app_server_protocol::AdditionalContextEntry;
use EfficiencyCode_app_server_protocol::AdditionalContextKind;
use EfficiencyCode_app_server_protocol::AppListUpdatedNotification;
use EfficiencyCode_app_server_protocol::AppSummary;
use EfficiencyCode_app_server_protocol::AppTemplateSummary;
use EfficiencyCode_app_server_protocol::AppTemplateUnavailableReason;
use EfficiencyCode_app_server_protocol::AppsInstalledParams;
use EfficiencyCode_app_server_protocol::AppsInstalledResponse;
use EfficiencyCode_app_server_protocol::AppsListParams;
use EfficiencyCode_app_server_protocol::AppsListResponse;
use EfficiencyCode_app_server_protocol::AppsReadParams;
use EfficiencyCode_app_server_protocol::AppsReadResponse;
use EfficiencyCode_app_server_protocol::AskForApproval;
use EfficiencyCode_app_server_protocol::AuthMode;
use EfficiencyCode_app_server_protocol::CancelLoginAccountParams;
use EfficiencyCode_app_server_protocol::CancelLoginAccountResponse;
use EfficiencyCode_app_server_protocol::CancelLoginAccountStatus;
use EfficiencyCode_app_server_protocol::ClientInfo;
use EfficiencyCode_app_server_protocol::ClientRequest;
use EfficiencyCode_app_server_protocol::ClientResponsePayload;
use EfficiencyCode_app_server_protocol::CollaborationModeListParams;
use EfficiencyCode_app_server_protocol::CollaborationModeListResponse;
use EfficiencyCode_app_server_protocol::CommandExecParams;
use EfficiencyCode_app_server_protocol::CommandExecResizeParams;
use EfficiencyCode_app_server_protocol::CommandExecTerminateParams;
use EfficiencyCode_app_server_protocol::CommandExecWriteParams;
use EfficiencyCode_app_server_protocol::ConfigWarningNotification;
use EfficiencyCode_app_server_protocol::ConsumeAccountRateLimitResetCreditOutcome;
use EfficiencyCode_app_server_protocol::ConsumeAccountRateLimitResetCreditParams;
use EfficiencyCode_app_server_protocol::ConsumeAccountRateLimitResetCreditResponse;
use EfficiencyCode_app_server_protocol::ConversationGitInfo;
use EfficiencyCode_app_server_protocol::ConversationSummary;
use EfficiencyCode_app_server_protocol::DeprecationNoticeNotification;
use EfficiencyCode_app_server_protocol::DynamicToolFunctionSpec;
use EfficiencyCode_app_server_protocol::DynamicToolNamespaceTool;
use EfficiencyCode_app_server_protocol::DynamicToolSpec;
use EfficiencyCode_app_server_protocol::EfficiencyCodeErrorInfo;
use EfficiencyCode_app_server_protocol::EnvironmentAddParams;
use EfficiencyCode_app_server_protocol::EnvironmentAddResponse;
use EfficiencyCode_app_server_protocol::EnvironmentInfoParams;
use EfficiencyCode_app_server_protocol::EnvironmentInfoResponse;
use EfficiencyCode_app_server_protocol::EnvironmentShellInfo;
use EfficiencyCode_app_server_protocol::EnvironmentStatusKind;
use EfficiencyCode_app_server_protocol::EnvironmentStatusParams;
use EfficiencyCode_app_server_protocol::EnvironmentStatusResponse;
use EfficiencyCode_app_server_protocol::ExperimentalFeature as ApiExperimentalFeature;
use EfficiencyCode_app_server_protocol::ExperimentalFeatureListParams;
use EfficiencyCode_app_server_protocol::ExperimentalFeatureListResponse;
use EfficiencyCode_app_server_protocol::ExperimentalFeatureStage as ApiExperimentalFeatureStage;
use EfficiencyCode_app_server_protocol::FeedbackUploadParams;
use EfficiencyCode_app_server_protocol::FeedbackUploadResponse;
use EfficiencyCode_app_server_protocol::GetAccountParams;
use EfficiencyCode_app_server_protocol::GetAccountRateLimitsResponse;
use EfficiencyCode_app_server_protocol::GetAccountResponse;
use EfficiencyCode_app_server_protocol::GetAccountTokenUsageResponse;
use EfficiencyCode_app_server_protocol::GetAuthStatusParams;
use EfficiencyCode_app_server_protocol::GetAuthStatusResponse;
use EfficiencyCode_app_server_protocol::GetConversationSummaryParams;
use EfficiencyCode_app_server_protocol::GetConversationSummaryResponse;
use EfficiencyCode_app_server_protocol::GetWorkspaceMessagesResponse;
use EfficiencyCode_app_server_protocol::GitDiffToRemoteParams;
use EfficiencyCode_app_server_protocol::GitDiffToRemoteResponse;
use EfficiencyCode_app_server_protocol::GitInfo as ApiGitInfo;
use EfficiencyCode_app_server_protocol::HookMetadata;
use EfficiencyCode_app_server_protocol::HooksListParams;
use EfficiencyCode_app_server_protocol::HooksListResponse;
use EfficiencyCode_app_server_protocol::InitializeParams;
use EfficiencyCode_app_server_protocol::InitializeResponse;
use EfficiencyCode_app_server_protocol::InstalledApp;
use EfficiencyCode_app_server_protocol::JSONRPCErrorError;
use EfficiencyCode_app_server_protocol::ListMcpServerStatusParams;
use EfficiencyCode_app_server_protocol::ListMcpServerStatusResponse;
use EfficiencyCode_app_server_protocol::LoginAccountParams;
use EfficiencyCode_app_server_protocol::LoginAccountResponse;
use EfficiencyCode_app_server_protocol::LoginApiKeyParams;
use EfficiencyCode_app_server_protocol::LoginAppBrand;
use EfficiencyCode_app_server_protocol::LogoutAccountResponse;
use EfficiencyCode_app_server_protocol::MarketplaceAddParams;
use EfficiencyCode_app_server_protocol::MarketplaceAddResponse;
use EfficiencyCode_app_server_protocol::MarketplaceInterface;
use EfficiencyCode_app_server_protocol::MarketplaceRemoveParams;
use EfficiencyCode_app_server_protocol::MarketplaceRemoveResponse;
use EfficiencyCode_app_server_protocol::MarketplaceUpgradeErrorInfo;
use EfficiencyCode_app_server_protocol::MarketplaceUpgradeParams;
use EfficiencyCode_app_server_protocol::MarketplaceUpgradeResponse;
use EfficiencyCode_app_server_protocol::McpResourceReadParams;
use EfficiencyCode_app_server_protocol::McpResourceReadResponse;
use EfficiencyCode_app_server_protocol::McpServerOauthClientRegistration;
use EfficiencyCode_app_server_protocol::McpServerOauthLoginCompletedNotification;
use EfficiencyCode_app_server_protocol::McpServerOauthLoginParams;
use EfficiencyCode_app_server_protocol::McpServerOauthLoginResponse;
use EfficiencyCode_app_server_protocol::McpServerRefreshResponse;
use EfficiencyCode_app_server_protocol::McpServerStatus;
use EfficiencyCode_app_server_protocol::McpServerStatusDetail;
use EfficiencyCode_app_server_protocol::McpServerToolCallParams;
use EfficiencyCode_app_server_protocol::McpServerToolCallResponse;
use EfficiencyCode_app_server_protocol::MemoryResetResponse;
use EfficiencyCode_app_server_protocol::MockExperimentalMethodParams;
use EfficiencyCode_app_server_protocol::MockExperimentalMethodResponse;
use EfficiencyCode_app_server_protocol::ModelListParams;
use EfficiencyCode_app_server_protocol::ModelListResponse;
use EfficiencyCode_app_server_protocol::PermissionProfileListParams;
use EfficiencyCode_app_server_protocol::PermissionProfileListResponse;
use EfficiencyCode_app_server_protocol::PermissionProfileSummary;
use EfficiencyCode_app_server_protocol::PluginDetail;
use EfficiencyCode_app_server_protocol::PluginInstallParams;
use EfficiencyCode_app_server_protocol::PluginInstallResponse;
use EfficiencyCode_app_server_protocol::PluginInstalledParams;
use EfficiencyCode_app_server_protocol::PluginInstalledResponse;
use EfficiencyCode_app_server_protocol::PluginInterface;
use EfficiencyCode_app_server_protocol::PluginListMarketplaceKind;
use EfficiencyCode_app_server_protocol::PluginListParams;
use EfficiencyCode_app_server_protocol::PluginListResponse;
use EfficiencyCode_app_server_protocol::PluginMarketplaceEntry;
use EfficiencyCode_app_server_protocol::PluginReadParams;
use EfficiencyCode_app_server_protocol::PluginReadResponse;
use EfficiencyCode_app_server_protocol::PluginShareCheckoutParams;
use EfficiencyCode_app_server_protocol::PluginShareCheckoutResponse;
use EfficiencyCode_app_server_protocol::PluginShareContext;
use EfficiencyCode_app_server_protocol::PluginShareDeleteParams;
use EfficiencyCode_app_server_protocol::PluginShareDeleteResponse;
use EfficiencyCode_app_server_protocol::PluginShareDiscoverability;
use EfficiencyCode_app_server_protocol::PluginShareListItem;
use EfficiencyCode_app_server_protocol::PluginShareListParams;
use EfficiencyCode_app_server_protocol::PluginShareListResponse;
use EfficiencyCode_app_server_protocol::PluginSharePrincipal;
use EfficiencyCode_app_server_protocol::PluginSharePrincipalType;
use EfficiencyCode_app_server_protocol::PluginShareSaveParams;
use EfficiencyCode_app_server_protocol::PluginShareSaveResponse;
use EfficiencyCode_app_server_protocol::PluginShareTarget;
use EfficiencyCode_app_server_protocol::PluginShareUpdateDiscoverability;
use EfficiencyCode_app_server_protocol::PluginShareUpdateTargetsParams;
use EfficiencyCode_app_server_protocol::PluginShareUpdateTargetsResponse;
use EfficiencyCode_app_server_protocol::PluginSkillReadParams;
use EfficiencyCode_app_server_protocol::PluginSkillReadResponse;
use EfficiencyCode_app_server_protocol::PluginSource;
use EfficiencyCode_app_server_protocol::PluginSummary;
use EfficiencyCode_app_server_protocol::PluginUninstallParams;
use EfficiencyCode_app_server_protocol::PluginUninstallResponse;
use EfficiencyCode_app_server_protocol::RateLimitResetCredit;
use EfficiencyCode_app_server_protocol::RateLimitResetCreditStatus;
use EfficiencyCode_app_server_protocol::RateLimitResetCreditsSummary;
use EfficiencyCode_app_server_protocol::RateLimitResetType;
use EfficiencyCode_app_server_protocol::RequestId;
use EfficiencyCode_app_server_protocol::ReviewDelivery as ApiReviewDelivery;
use EfficiencyCode_app_server_protocol::ReviewStartParams;
use EfficiencyCode_app_server_protocol::ReviewStartResponse;
use EfficiencyCode_app_server_protocol::ReviewTarget as ApiReviewTarget;
use EfficiencyCode_app_server_protocol::SandboxMode;
use EfficiencyCode_app_server_protocol::SendAddCreditsNudgeEmailParams;
use EfficiencyCode_app_server_protocol::SendAddCreditsNudgeEmailResponse;
use EfficiencyCode_app_server_protocol::ServerNotification;
use EfficiencyCode_app_server_protocol::ServerRequestResolvedNotification;
use EfficiencyCode_app_server_protocol::SkillSummary;
use EfficiencyCode_app_server_protocol::SkillsConfigWriteParams;
use EfficiencyCode_app_server_protocol::SkillsConfigWriteResponse;
use EfficiencyCode_app_server_protocol::SkillsExtraRootsSetParams;
use EfficiencyCode_app_server_protocol::SkillsExtraRootsSetResponse;
use EfficiencyCode_app_server_protocol::SkillsListParams;
use EfficiencyCode_app_server_protocol::SkillsListResponse;
use EfficiencyCode_app_server_protocol::SortDirection;
use EfficiencyCode_app_server_protocol::Thread;
use EfficiencyCode_app_server_protocol::ThreadApproveGuardianDeniedActionParams;
use EfficiencyCode_app_server_protocol::ThreadApproveGuardianDeniedActionResponse;
use EfficiencyCode_app_server_protocol::ThreadArchiveParams;
use EfficiencyCode_app_server_protocol::ThreadArchiveResponse;
use EfficiencyCode_app_server_protocol::ThreadArchivedNotification;
use EfficiencyCode_app_server_protocol::ThreadBackgroundTerminal;
use EfficiencyCode_app_server_protocol::ThreadBackgroundTerminalsCleanParams;
use EfficiencyCode_app_server_protocol::ThreadBackgroundTerminalsCleanResponse;
use EfficiencyCode_app_server_protocol::ThreadBackgroundTerminalsListParams;
use EfficiencyCode_app_server_protocol::ThreadBackgroundTerminalsListResponse;
use EfficiencyCode_app_server_protocol::ThreadBackgroundTerminalsTerminateParams;
use EfficiencyCode_app_server_protocol::ThreadBackgroundTerminalsTerminateResponse;
use EfficiencyCode_app_server_protocol::ThreadClosedNotification;
use EfficiencyCode_app_server_protocol::ThreadCompactStartParams;
use EfficiencyCode_app_server_protocol::ThreadCompactStartResponse;
use EfficiencyCode_app_server_protocol::ThreadDecrementElicitationParams;
use EfficiencyCode_app_server_protocol::ThreadDecrementElicitationResponse;
use EfficiencyCode_app_server_protocol::ThreadDeleteParams;
use EfficiencyCode_app_server_protocol::ThreadDeleteResponse;
use EfficiencyCode_app_server_protocol::ThreadDeletedNotification;
use EfficiencyCode_app_server_protocol::ThreadForkParams;
use EfficiencyCode_app_server_protocol::ThreadForkResponse;
use EfficiencyCode_app_server_protocol::ThreadGoal;
use EfficiencyCode_app_server_protocol::ThreadGoalClearParams;
use EfficiencyCode_app_server_protocol::ThreadGoalClearResponse;
use EfficiencyCode_app_server_protocol::ThreadGoalClearedNotification;
use EfficiencyCode_app_server_protocol::ThreadGoalGetParams;
use EfficiencyCode_app_server_protocol::ThreadGoalGetResponse;
use EfficiencyCode_app_server_protocol::ThreadGoalSetParams;
use EfficiencyCode_app_server_protocol::ThreadGoalSetResponse;
use EfficiencyCode_app_server_protocol::ThreadGoalStatus;
use EfficiencyCode_app_server_protocol::ThreadGoalUpdatedNotification;
use EfficiencyCode_app_server_protocol::ThreadHistoryBuilder;
#[cfg(test)]
use EfficiencyCode_app_server_protocol::ThreadHistoryMode;
use EfficiencyCode_app_server_protocol::ThreadIncrementElicitationParams;
use EfficiencyCode_app_server_protocol::ThreadIncrementElicitationResponse;
use EfficiencyCode_app_server_protocol::ThreadInjectItemsParams;
use EfficiencyCode_app_server_protocol::ThreadInjectItemsResponse;
use EfficiencyCode_app_server_protocol::ThreadItem;
use EfficiencyCode_app_server_protocol::ThreadItemEntry;
use EfficiencyCode_app_server_protocol::ThreadItemsListParams;
use EfficiencyCode_app_server_protocol::ThreadItemsListResponse;
use EfficiencyCode_app_server_protocol::ThreadListCwdFilter;
use EfficiencyCode_app_server_protocol::ThreadListParams;
use EfficiencyCode_app_server_protocol::ThreadListResponse;
use EfficiencyCode_app_server_protocol::ThreadLoadedListParams;
use EfficiencyCode_app_server_protocol::ThreadLoadedListResponse;
use EfficiencyCode_app_server_protocol::ThreadMemoryModeSetParams;
use EfficiencyCode_app_server_protocol::ThreadMemoryModeSetResponse;
use EfficiencyCode_app_server_protocol::ThreadMetadataGitInfoUpdateParams;
use EfficiencyCode_app_server_protocol::ThreadMetadataUpdateParams;
use EfficiencyCode_app_server_protocol::ThreadMetadataUpdateResponse;
use EfficiencyCode_app_server_protocol::ThreadNameUpdatedNotification;
use EfficiencyCode_app_server_protocol::ThreadReadParams;
use EfficiencyCode_app_server_protocol::ThreadReadResponse;
use EfficiencyCode_app_server_protocol::ThreadRealtimeAppendAudioParams;
use EfficiencyCode_app_server_protocol::ThreadRealtimeAppendAudioResponse;
use EfficiencyCode_app_server_protocol::ThreadRealtimeAppendSpeechParams;
use EfficiencyCode_app_server_protocol::ThreadRealtimeAppendSpeechResponse;
use EfficiencyCode_app_server_protocol::ThreadRealtimeAppendTextParams;
use EfficiencyCode_app_server_protocol::ThreadRealtimeAppendTextResponse;
use EfficiencyCode_app_server_protocol::ThreadRealtimeListVoicesResponse;
use EfficiencyCode_app_server_protocol::ThreadRealtimeStartParams;
use EfficiencyCode_app_server_protocol::ThreadRealtimeStartResponse;
use EfficiencyCode_app_server_protocol::ThreadRealtimeStartTransport;
use EfficiencyCode_app_server_protocol::ThreadRealtimeStopParams;
use EfficiencyCode_app_server_protocol::ThreadRealtimeStopResponse;
use EfficiencyCode_app_server_protocol::ThreadResumeInitialTurnsPageParams;
use EfficiencyCode_app_server_protocol::ThreadResumeParams;
use EfficiencyCode_app_server_protocol::ThreadResumeResponse;
use EfficiencyCode_app_server_protocol::ThreadRollbackParams;
use EfficiencyCode_app_server_protocol::ThreadSearchOccurrence;
use EfficiencyCode_app_server_protocol::ThreadSearchOccurrencesParams;
use EfficiencyCode_app_server_protocol::ThreadSearchOccurrencesResponse;
use EfficiencyCode_app_server_protocol::ThreadSearchParams;
use EfficiencyCode_app_server_protocol::ThreadSearchResponse;
use EfficiencyCode_app_server_protocol::ThreadSearchResult;
use EfficiencyCode_app_server_protocol::ThreadSearchSortKey;
use EfficiencyCode_app_server_protocol::ThreadSearchTextRange;
use EfficiencyCode_app_server_protocol::ThreadSetNameParams;
use EfficiencyCode_app_server_protocol::ThreadSetNameResponse;
use EfficiencyCode_app_server_protocol::ThreadSettings;
use EfficiencyCode_app_server_protocol::ThreadSettingsUpdateParams;
use EfficiencyCode_app_server_protocol::ThreadSettingsUpdateResponse;
use EfficiencyCode_app_server_protocol::ThreadShellCommandParams;
use EfficiencyCode_app_server_protocol::ThreadShellCommandResponse;
use EfficiencyCode_app_server_protocol::ThreadSortKey;
use EfficiencyCode_app_server_protocol::ThreadSourceKind;
use EfficiencyCode_app_server_protocol::ThreadStartParams;
use EfficiencyCode_app_server_protocol::ThreadStartResponse;
use EfficiencyCode_app_server_protocol::ThreadStartedNotification;
use EfficiencyCode_app_server_protocol::ThreadStatus;
use EfficiencyCode_app_server_protocol::ThreadTurnsListParams;
use EfficiencyCode_app_server_protocol::ThreadTurnsListResponse;
use EfficiencyCode_app_server_protocol::ThreadUnarchiveParams;
use EfficiencyCode_app_server_protocol::ThreadUnarchiveResponse;
use EfficiencyCode_app_server_protocol::ThreadUnarchivedNotification;
use EfficiencyCode_app_server_protocol::ThreadUnsubscribeParams;
use EfficiencyCode_app_server_protocol::ThreadUnsubscribeResponse;
use EfficiencyCode_app_server_protocol::ThreadUnsubscribeStatus;
use EfficiencyCode_app_server_protocol::Turn;
use EfficiencyCode_app_server_protocol::TurnEnvironmentParams;
use EfficiencyCode_app_server_protocol::TurnError;
use EfficiencyCode_app_server_protocol::TurnInterruptParams;
use EfficiencyCode_app_server_protocol::TurnInterruptResponse;
use EfficiencyCode_app_server_protocol::TurnItemsView;
use EfficiencyCode_app_server_protocol::TurnStartParams;
use EfficiencyCode_app_server_protocol::TurnStartResponse;
use EfficiencyCode_app_server_protocol::TurnStatus;
use EfficiencyCode_app_server_protocol::TurnSteerParams;
use EfficiencyCode_app_server_protocol::TurnSteerResponse;
use EfficiencyCode_app_server_protocol::UserInput as V2UserInput;
use EfficiencyCode_app_server_protocol::WindowsSandboxReadiness;
use EfficiencyCode_app_server_protocol::WindowsSandboxReadinessResponse;
use EfficiencyCode_app_server_protocol::WindowsSandboxSetupCompletedNotification;
use EfficiencyCode_app_server_protocol::WindowsSandboxSetupMode;
use EfficiencyCode_app_server_protocol::WindowsSandboxSetupStartParams;
use EfficiencyCode_app_server_protocol::WindowsSandboxSetupStartResponse;
use EfficiencyCode_app_server_protocol::WorkspaceMessage;
use EfficiencyCode_app_server_protocol::WorkspaceMessageType;
use EfficiencyCode_arg0::Arg0DispatchPaths;
use EfficiencyCode_backend_client::AddCreditsNudgeCreditType as BackendAddCreditsNudgeCreditType;
use EfficiencyCode_backend_client::Client as BackendClient;
use EfficiencyCode_backend_client::ConsumeRateLimitResetCreditCode as BackendConsumeRateLimitResetCreditCode;
use EfficiencyCode_backend_client::EfficiencyCodeWorkspaceMessage as BackendWorkspaceMessage;
use EfficiencyCode_backend_client::EfficiencyCodeWorkspaceMessageType as BackendWorkspaceMessageType;
use EfficiencyCode_backend_client::EfficiencyCodeWorkspaceMessagesResponse as BackendWorkspaceMessagesResponse;
use EfficiencyCode_backend_client::RateLimitResetCreditDetails as BackendRateLimitResetCreditDetails;
use EfficiencyCode_backend_client::RateLimitResetCreditsDetails as BackendRateLimitResetCreditsDetails;
use EfficiencyCode_backend_client::RequestError as BackendRequestError;
use EfficiencyCode_backend_client::TokenUsageProfile;
use EfficiencyCode_chatgpt::connectors;
use EfficiencyCode_chatgpt::workspace_settings;
use EfficiencyCode_config::CloudConfigBundleLoadError;
use EfficiencyCode_config::CloudConfigBundleLoadErrorCode;
use EfficiencyCode_config::ConfigLayerStack;
use EfficiencyCode_config::loader::project_trust_key;
use EfficiencyCode_config::types::McpServerTransportConfig;
use EfficiencyCode_connectors::AppInfo;
use EfficiencyCode_core::EfficiencyCodeThread;
use EfficiencyCode_core::EfficiencyCodeThreadSettingsOverrides;
use EfficiencyCode_core::ForkSnapshot;
use EfficiencyCode_core::McpManager;
use EfficiencyCode_core::NewThread;
#[cfg(test)]
use EfficiencyCode_core::SessionMeta;
use EfficiencyCode_core::StartThreadOptions;
use EfficiencyCode_core::SteerInputError;
use EfficiencyCode_core::ThreadConfigSnapshot;
use EfficiencyCode_core::ThreadManager;
use EfficiencyCode_core::config::Config;
use EfficiencyCode_core::config::ConfigOverrides;
use EfficiencyCode_core::config::NetworkProxyAuditMetadata;
use EfficiencyCode_core::config::edit::ConfigEdit;
use EfficiencyCode_core::config::edit::ConfigEditsBuilder;
use EfficiencyCode_core::connectors::AccessibleConnectorsStatus;
use EfficiencyCode_core::exec::ExecCapturePolicy;
use EfficiencyCode_core::exec::ExecExpiration;
use EfficiencyCode_core::exec::ExecParams;
use EfficiencyCode_core::exec_env::create_env;
use EfficiencyCode_core::path_utils;
#[cfg(test)]
use EfficiencyCode_core::read_head_for_summary;
use EfficiencyCode_core::sandboxing::SandboxPermissions;
use EfficiencyCode_core::truncate_rollout_after_turn_id;
use EfficiencyCode_core::truncate_rollout_before_turn_id;
use EfficiencyCode_core::windows_sandbox::WindowsSandboxLevelExt;
use EfficiencyCode_core::windows_sandbox::WindowsSandboxSetupMode as CoreWindowsSandboxSetupMode;
use EfficiencyCode_core::windows_sandbox::WindowsSandboxSetupRequest;
use EfficiencyCode_core::windows_sandbox::sandbox_setup_is_complete;
use EfficiencyCode_core_plugins::PluginInstallError as CorePluginInstallError;
use EfficiencyCode_core_plugins::PluginInstallRequest;
use EfficiencyCode_core_plugins::PluginReadRequest;
use EfficiencyCode_core_plugins::PluginUninstallError as CorePluginUninstallError;
use EfficiencyCode_core_plugins::PluginsManager;
use EfficiencyCode_core_plugins::loader::load_plugin_apps;
use EfficiencyCode_core_plugins::manifest::PluginManifestInterface;
use EfficiencyCode_core_plugins::marketplace::MarketplaceError;
use EfficiencyCode_core_plugins::marketplace::MarketplacePluginSource;
use EfficiencyCode_core_plugins::marketplace_add::MarketplaceAddError;
use EfficiencyCode_core_plugins::marketplace_add::MarketplaceAddRequest;
use EfficiencyCode_core_plugins::marketplace_add::add_marketplace as add_marketplace_to_EfficiencyCode_home;
use EfficiencyCode_core_plugins::marketplace_remove::MarketplaceRemoveError;
use EfficiencyCode_core_plugins::marketplace_remove::MarketplaceRemoveRequest as CoreMarketplaceRemoveRequest;
use EfficiencyCode_core_plugins::marketplace_remove::remove_marketplace;
use EfficiencyCode_core_plugins::remote::RemoteMarketplace;
use EfficiencyCode_core_plugins::remote::RemoteMarketplaceSource;
use EfficiencyCode_core_plugins::remote::RemotePluginCatalogError;
use EfficiencyCode_core_plugins::remote::RemotePluginDetail as RemoteCatalogPluginDetail;
use EfficiencyCode_core_plugins::remote::RemotePluginServiceConfig;
use EfficiencyCode_core_plugins::remote::RemotePluginShareContext as RemoteCatalogPluginShareContext;
use EfficiencyCode_core_plugins::remote::RemotePluginShareSummary as RemoteCatalogPluginShareSummary;
use EfficiencyCode_core_plugins::remote::RemotePluginSummary as RemoteCatalogPluginSummary;
use EfficiencyCode_exec_server::EnvironmentManager;
use EfficiencyCode_exec_server::EnvironmentObservedStatus;
use EfficiencyCode_exec_server::LOCAL_ENVIRONMENT_ID;
use EfficiencyCode_exec_server::LOCAL_FS;
use EfficiencyCode_features::FEATURES;
use EfficiencyCode_features::Feature;
use EfficiencyCode_features::Stage;
use EfficiencyCode_feedback::EfficiencyCodeFeedback;
use EfficiencyCode_feedback::FeedbackAttachmentPath;
use EfficiencyCode_feedback::FeedbackUploadOptions;
use EfficiencyCode_git_utils::git_diff_to_remote;
use EfficiencyCode_git_utils::resolve_root_git_project_for_trust;
use EfficiencyCode_login::AuthManager;
use EfficiencyCode_login::EfficiencyCode_OPEN_APP_URL;
use EfficiencyCode_login::EfficiencyCodeAuth;
use EfficiencyCode_login::LoginSuccessPage;
use EfficiencyCode_login::LoginSuccessPageBrand;
use EfficiencyCode_login::ServerOptions as LoginServerOptions;
use EfficiencyCode_login::ShutdownHandle;
use EfficiencyCode_login::complete_device_code_login;
use EfficiencyCode_login::login_with_api_key;
use EfficiencyCode_login::login_with_bedrock_api_key;
use EfficiencyCode_login::oauth_client_id;
use EfficiencyCode_login::request_device_code;
use EfficiencyCode_login::run_login_server;
use EfficiencyCode_mcp::McpRuntimeContext;
use EfficiencyCode_mcp::McpServerStatusSnapshot;
use EfficiencyCode_mcp::McpSnapshotDetail;
use EfficiencyCode_mcp::collect_mcp_server_status_snapshot_with_detail;
use EfficiencyCode_mcp::discover_supported_scopes;
use EfficiencyCode_mcp::read_mcp_resource as read_mcp_resource_without_thread;
use EfficiencyCode_mcp::resolve_oauth_scopes;
use EfficiencyCode_memories_write::clear_memory_roots_contents;
use EfficiencyCode_model_provider::create_model_provider;
use EfficiencyCode_models_manager::collaboration_mode_presets::builtin_collaboration_mode_presets;
use EfficiencyCode_protocol::ThreadId;
use EfficiencyCode_protocol::config_types::CollaborationMode;
use EfficiencyCode_protocol::config_types::ForcedLoginMethod;
use EfficiencyCode_protocol::config_types::Personality;
use EfficiencyCode_protocol::config_types::ReasoningSummary;
use EfficiencyCode_protocol::config_types::TrustLevel;
use EfficiencyCode_protocol::config_types::WindowsSandboxLevel;
use EfficiencyCode_protocol::error::EfficiencyCodeErr;
use EfficiencyCode_protocol::error::Result as EfficiencyCodeResult;
#[cfg(test)]
use EfficiencyCode_protocol::items::TurnItem;
use EfficiencyCode_protocol::models::ResponseItem;
use EfficiencyCode_protocol::openai_models::ReasoningEffort;
#[cfg(test)]
use EfficiencyCode_protocol::permissions::FileSystemSandboxPolicy;
use EfficiencyCode_protocol::protocol::AgentStatus;
use EfficiencyCode_protocol::protocol::ConversationAudioParams;
use EfficiencyCode_protocol::protocol::ConversationSpeechParams;
use EfficiencyCode_protocol::protocol::ConversationStartParams;
use EfficiencyCode_protocol::protocol::ConversationStartTransport;
use EfficiencyCode_protocol::protocol::ConversationTextParams;
use EfficiencyCode_protocol::protocol::EventMsg;
#[cfg(test)]
use EfficiencyCode_protocol::protocol::GitInfo as CoreGitInfo;
use EfficiencyCode_protocol::protocol::McpAuthStatus as CoreMcpAuthStatus;
use EfficiencyCode_protocol::protocol::Op;
use EfficiencyCode_protocol::protocol::RealtimeVoicesList;
use EfficiencyCode_protocol::protocol::ReviewDelivery as CoreReviewDelivery;
use EfficiencyCode_protocol::protocol::ReviewRequest;
use EfficiencyCode_protocol::protocol::ReviewTarget as CoreReviewTarget;
use EfficiencyCode_protocol::protocol::SessionConfiguredEvent;
#[cfg(test)]
use EfficiencyCode_protocol::protocol::SessionMetaLine;
use EfficiencyCode_protocol::protocol::TurnEnvironmentSelection;
use EfficiencyCode_protocol::protocol::TurnEnvironmentSelections;
use EfficiencyCode_protocol::protocol::W3cTraceContext;
use EfficiencyCode_protocol::protocol::strip_user_message_prefix;
use EfficiencyCode_protocol::user_input::MAX_USER_INPUT_TEXT_CHARS;
use EfficiencyCode_protocol::user_input::UserInput as CoreInputItem;
use EfficiencyCode_rmcp_client::McpOAuthClientRegistration;
use EfficiencyCode_rmcp_client::StreamableHttpRedirectMode;
use EfficiencyCode_rmcp_client::perform_oauth_login_return_url;
use EfficiencyCode_rollout::InitialHistory;
use EfficiencyCode_rollout::ResumedHistory;
use EfficiencyCode_rollout::RolloutItem;
use EfficiencyCode_rollout::is_persisted_rollout_item;
use EfficiencyCode_rollout::state_db::StateDbHandle;
use EfficiencyCode_rollout::state_db::reconcile_rollout;
use EfficiencyCode_state::ThreadMetadata;
use EfficiencyCode_state::log_db::LogDbLayer;
use EfficiencyCode_thread_store::ArchiveThreadParams as StoreArchiveThreadParams;
use EfficiencyCode_thread_store::ArchiveThreadsParams as StoreArchiveThreadsParams;
use EfficiencyCode_thread_store::DeleteThreadsParams as StoreDeleteThreadsParams;
use EfficiencyCode_thread_store::GitInfoPatch as StoreGitInfoPatch;
use EfficiencyCode_thread_store::ItemSortKey as StoreItemSortKey;
use EfficiencyCode_thread_store::ListItemsParams as StoreListItemsParams;
use EfficiencyCode_thread_store::ListThreadsParams as StoreListThreadsParams;
use EfficiencyCode_thread_store::ListTurnsParams as StoreListTurnsParams;
use EfficiencyCode_thread_store::LoadThreadHistoryParams as StoreLoadThreadHistoryParams;
use EfficiencyCode_thread_store::LocalThreadStore;
use EfficiencyCode_thread_store::ReadThreadByRolloutPathParams as StoreReadThreadByRolloutPathParams;
use EfficiencyCode_thread_store::ReadThreadParams as StoreReadThreadParams;
use EfficiencyCode_thread_store::SearchThreadOccurrencesParams as StoreSearchThreadOccurrencesParams;
use EfficiencyCode_thread_store::SearchThreadsParams as StoreSearchThreadsParams;
use EfficiencyCode_thread_store::SortDirection as StoreSortDirection;
use EfficiencyCode_thread_store::StoredThread;
use EfficiencyCode_thread_store::StoredTurn;
use EfficiencyCode_thread_store::StoredTurnItemsView;
use EfficiencyCode_thread_store::StoredTurnStatus;
use EfficiencyCode_thread_store::ThreadMetadataPatch as StoreThreadMetadataPatch;
use EfficiencyCode_thread_store::ThreadRelationFilter as StoreThreadRelationFilter;
use EfficiencyCode_thread_store::ThreadSortKey as StoreThreadSortKey;
use EfficiencyCode_thread_store::ThreadStore;
use EfficiencyCode_thread_store::ThreadStoreError;
use EfficiencyCode_utils_absolute_path::AbsolutePathBuf;
use EfficiencyCode_utils_pty::DEFAULT_OUTPUT_BYTES_CAP;
use chrono::Duration as ChronoDuration;
use chrono::SecondsFormat;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Error as IoError;
use std::path::Path;
use std::path::PathBuf;
use std::result::Result;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;
use tokio::sync::Mutex;
use tokio::sync::Semaphore;
use tokio::sync::SemaphorePermit;
use tokio::sync::broadcast;
use tokio::sync::oneshot;
use tokio::sync::watch;
use tokio_util::sync::CancellationToken;
use tokio_util::sync::DropGuard;
use tokio_util::task::TaskTracker;
use toml::Value as TomlValue;
use tracing::Instrument;
use tracing::error;
use tracing::info;
use tracing::warn;
use uuid::Uuid;

#[cfg(test)]
use EfficiencyCode_app_server_protocol::ServerRequest;

mod account_processor;
mod apps_processor;
mod bedrock_auth;
mod catalog_processor;
mod command_exec_processor;
mod config_processor;
mod diagnostics;
mod environment_processor;
mod feedback_doctor_report;
mod feedback_processor;
mod fs_processor;
mod git_processor;
mod initialize_processor;
mod marketplace_processor;
mod mcp_processor;
mod plugins;
mod process_exec_processor;
mod remote_control_processor;
mod search;
mod thread_enrichment;
mod thread_fork_goal;
mod thread_processor;
mod thread_sections;
mod token_usage_replay;
mod turn_processor;
mod windows_sandbox_processor;

pub(crate) use account_processor::AccountRequestProcessor;
pub(crate) use apps_processor::AppsRequestProcessor;
pub(crate) use catalog_processor::CatalogRequestProcessor;
pub(crate) use command_exec_processor::CommandExecRequestProcessor;
pub(crate) use config_processor::ConfigRequestProcessor;
pub(crate) use diagnostics::read_server_diagnostics;
pub(crate) use environment_processor::EnvironmentRequestProcessor;
pub(crate) use feedback_processor::FeedbackRequestProcessor;
pub(crate) use fs_processor::FsRequestProcessor;
pub(crate) use git_processor::GitRequestProcessor;
pub(crate) use initialize_processor::InitializeRequestProcessor;
pub(crate) use marketplace_processor::MarketplaceRequestProcessor;
pub(crate) use mcp_processor::McpRequestProcessor;
pub(crate) use plugins::PluginRequestProcessor;
pub(crate) use process_exec_processor::ProcessExecRequestProcessor;
pub(crate) use remote_control_processor::RemoteControlRequestProcessor;
pub(crate) use search::SearchRequestProcessor;
pub(crate) use thread_goal_processor::ThreadGoalRequestProcessor;
pub(crate) use thread_processor::ThreadRequestProcessor;
pub(crate) use turn_processor::TurnRequestProcessor;
pub(crate) use windows_sandbox_processor::WindowsSandboxRequestProcessor;

use crate::error_code::internal_error;
use crate::error_code::invalid_request;
use crate::filters::compute_source_filters;
use crate::filters::source_kind_matches;
use crate::thread_state::ConnectionCapabilities;
use crate::thread_state::ThreadListenerCommand;
use crate::thread_state::ThreadState;
use crate::thread_state::ThreadStateManager;
use token_usage_replay::restored_token_usage_turn_id;
use token_usage_replay::send_thread_token_usage_update_to_connection;

fn resolve_request_cwd(cwd: Option<PathBuf>) -> Result<Option<AbsolutePathBuf>, JSONRPCErrorError> {
    cwd.map(|cwd| {
        AbsolutePathBuf::relative_to_current_dir(path_utils::normalize_for_native_workdir(cwd))
            .map_err(|err| invalid_request(format!("invalid cwd: {err}")))
    })
    .transpose()
}

fn resolve_turn_environment_selections(
    thread_manager: &ThreadManager,
    environments: Option<Vec<TurnEnvironmentParams>>,
) -> Result<Option<Vec<TurnEnvironmentSelection>>, JSONRPCErrorError> {
    let Some(environments) = environments else {
        return Ok(None);
    };
    let mut selections = Vec::with_capacity(environments.len());
    for environment in environments {
        let environment_id = environment.environment_id;
        let cwd = environment
            .cwd
            .to_inferred_path_uri()
            .ok_or_else(|| {
                invalid_request(format!(
                    "invalid cwd for environment `{environment_id}`: path `{}` does not use absolute POSIX or Windows path syntax",
                    environment.cwd
                ))
            })?;
        let workspace_roots = environment
            .runtime_workspace_roots
            .map(|roots| {
                let mut resolved_roots = Vec::new();
                for root in roots {
                    let root = root.to_inferred_path_uri().ok_or_else(|| {
                        invalid_request(format!(
                            "invalid runtime workspace root for environment `{environment_id}`: path `{root}` does not use absolute POSIX or Windows path syntax"
                        ))
                    })?;
                    if !resolved_roots.contains(&root) {
                        resolved_roots.push(root);
                    }
                }
                Ok::<_, JSONRPCErrorError>(resolved_roots)
            })
            .transpose()?
            .unwrap_or_else(|| vec![cwd.clone()]);
        selections.push(TurnEnvironmentSelection {
            environment_id,
            cwd,
            workspace_roots,
        });
    }
    thread_manager
        .validate_environment_selections(&selections)
        .map_err(environment_selection_error)?;
    Ok(Some(selections))
}

fn resolve_runtime_workspace_roots(workspace_roots: Vec<AbsolutePathBuf>) -> Vec<AbsolutePathBuf> {
    let mut resolved_roots = Vec::new();
    for root in workspace_roots {
        if !resolved_roots.iter().any(|existing| existing == &root) {
            resolved_roots.push(root);
        }
    }
    resolved_roots
}

mod config_errors;
mod request_errors;
mod thread_delete;
mod thread_goal_processor;
mod thread_lifecycle;
mod thread_resume_redaction;
mod thread_summary;

use self::config_errors::*;
use self::request_errors::*;
use self::thread_goal_processor::api_thread_goal_from_state;
use self::thread_lifecycle::*;
use self::thread_resume_redaction::*;
use self::thread_summary::*;

pub(crate) use self::thread_lifecycle::populate_thread_turns_from_history;
pub(crate) use self::thread_processor::thread_from_stored_thread;
#[cfg(test)]
pub(crate) use self::thread_summary::read_summary_from_rollout;
#[cfg(test)]
pub(crate) use self::thread_summary::summary_to_thread;
pub(crate) use self::thread_summary::thread_settings_from_config_snapshot;
pub(crate) use self::thread_summary::thread_settings_from_core_snapshot;

pub(crate) fn build_legacy_api_turns_from_rollout_items(items: &[RolloutItem]) -> Vec<Turn> {
    let mut builder = ThreadHistoryBuilder::new();
    for item in items {
        if is_persisted_rollout_item(
            item,
            EfficiencyCode_protocol::protocol::ThreadHistoryMode::Legacy,
        ) {
            builder.handle_rollout_item(item);
        }
    }
    builder.finish()
}
