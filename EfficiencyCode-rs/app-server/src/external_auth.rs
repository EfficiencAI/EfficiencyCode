use std::sync::Arc;
use std::sync::RwLock;

use EfficiencyCode_app_server_protocol::ChatgptAuthTokensRefreshParams;
use EfficiencyCode_app_server_protocol::ChatgptAuthTokensRefreshReason;
use EfficiencyCode_app_server_protocol::ChatgptAuthTokensRefreshResponse;
use EfficiencyCode_app_server_protocol::ServerRequestPayload;
use EfficiencyCode_login::EfficiencyCodeAuth;
use EfficiencyCode_login::ExternalAuthFuture;
use EfficiencyCode_login::auth::ExternalAuth;
use EfficiencyCode_login::auth::ExternalAuthRefreshContext;
use EfficiencyCode_login::auth::ExternalAuthRefreshReason;
use tokio::time::Duration;
use tokio::time::timeout;

use crate::outgoing_message::OutgoingMessageSender;

const EXTERNAL_AUTH_REFRESH_TIMEOUT: Duration = Duration::from_secs(10);

pub(crate) struct ExternalAuthBridge {
    outgoing: Arc<OutgoingMessageSender>,
    auth: RwLock<EfficiencyCodeAuth>,
}

impl ExternalAuthBridge {
    pub(crate) fn new(outgoing: Arc<OutgoingMessageSender>, auth: EfficiencyCodeAuth) -> Self {
        Self {
            outgoing,
            auth: RwLock::new(auth),
        }
    }

    async fn refresh(
        &self,
        context: ExternalAuthRefreshContext,
    ) -> std::io::Result<EfficiencyCodeAuth> {
        let reason = match context.reason {
            ExternalAuthRefreshReason::Unauthorized => ChatgptAuthTokensRefreshReason::Unauthorized,
        };
        let params = ChatgptAuthTokensRefreshParams {
            reason,
            previous_account_id: context.previous_account_id,
        };

        let (request_id, rx) = self
            .outgoing
            .send_request(ServerRequestPayload::ChatgptAuthTokensRefresh(params))
            .await;
        let result = match timeout(EXTERNAL_AUTH_REFRESH_TIMEOUT, rx).await {
            Ok(result) => {
                let result = result.map_err(|err| {
                    std::io::Error::other(format!("auth refresh request canceled: {err}"))
                })?;
                result.map_err(|err| {
                    std::io::Error::other(format!(
                        "auth refresh request failed: code={} message={}",
                        err.code, err.message
                    ))
                })?
            }
            Err(_) => {
                let _canceled = self.outgoing.cancel_request(&request_id).await;
                return Err(std::io::Error::other(format!(
                    "auth refresh request timed out after {}s",
                    EXTERNAL_AUTH_REFRESH_TIMEOUT.as_secs()
                )));
            }
        };

        let response: ChatgptAuthTokensRefreshResponse =
            serde_json::from_value(result).map_err(std::io::Error::other)?;
        let auth = EfficiencyCodeAuth::from_external_chatgpt_tokens(
            response.access_token.as_str(),
            response.chatgpt_account_id.as_str(),
            response.chatgpt_plan_type.as_deref(),
        )?;
        *self
            .auth
            .write()
            .map_err(|_| std::io::Error::other("external auth lock is poisoned"))? = auth.clone();
        Ok(auth)
    }
}

impl ExternalAuth for ExternalAuthBridge {
    fn resolve(&self) -> ExternalAuthFuture<'_, EfficiencyCodeAuth> {
        Box::pin(async {
            self.auth
                .read()
                .map(|auth| auth.clone())
                .map_err(|_| std::io::Error::other("external auth lock is poisoned"))
        })
    }

    fn refresh(
        &self,
        context: ExternalAuthRefreshContext,
    ) -> ExternalAuthFuture<'_, EfficiencyCodeAuth> {
        Box::pin(ExternalAuthBridge::refresh(self, context))
    }
}
