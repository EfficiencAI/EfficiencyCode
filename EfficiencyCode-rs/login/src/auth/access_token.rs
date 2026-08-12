const PERSONAL_ACCESS_TOKEN_PREFIX: &str = "at-";

pub(super) enum EfficiencyCodeAccessToken<'a> {
    PersonalAccessToken(&'a str),
    AgentIdentityJwt(&'a str),
}

pub(super) fn classify_EfficiencyCode_access_token(
    access_token: &str,
) -> EfficiencyCodeAccessToken<'_> {
    if access_token.starts_with(PERSONAL_ACCESS_TOKEN_PREFIX) {
        EfficiencyCodeAccessToken::PersonalAccessToken(access_token)
    } else {
        EfficiencyCodeAccessToken::AgentIdentityJwt(access_token)
    }
}

#[cfg(test)]
#[path = "access_token_tests.rs"]
mod tests;
