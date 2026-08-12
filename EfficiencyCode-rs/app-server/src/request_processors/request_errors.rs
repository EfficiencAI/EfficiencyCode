use super::*;
use EfficiencyCode_protocol::error::EfficiencyCodeErrorDetails;

pub(super) fn environment_selection_error(err: EfficiencyCodeErr) -> JSONRPCErrorError {
    match err.details() {
        EfficiencyCodeErrorDetails::InvalidRequest(message) => invalid_request(message.clone()),
        _ => internal_error(format!("failed to validate environment selections: {err}")),
    }
}
