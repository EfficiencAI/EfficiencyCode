#[cfg(EfficiencyCode_bazel)]
pub use code_mode_proto::EfficiencyCode::code_mode::v1::*;

#[cfg(not(EfficiencyCode_bazel))]
tonic::include_proto!("efficiency_code.code_mode.v1");

pub const MAX_IDENTIFIER_BYTES: usize = 256;
pub const MAX_TOOL_ERROR_BYTES: usize = 64 * 1_024;
