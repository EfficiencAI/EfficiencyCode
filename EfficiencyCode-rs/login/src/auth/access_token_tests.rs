use super::*;

#[test]
fn classifies_personal_access_tokens_by_prefix() {
    assert!(matches!(
        classify_EfficiencyCode_access_token("at-example"),
        EfficiencyCodeAccessToken::PersonalAccessToken("at-example")
    ));
    assert!(matches!(
        classify_EfficiencyCode_access_token("header.payload.signature"),
        EfficiencyCodeAccessToken::AgentIdentityJwt("header.payload.signature")
    ));
}
