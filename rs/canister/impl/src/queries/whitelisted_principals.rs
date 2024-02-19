use crate::state;
use event_sink_canister::WhitelistedPrincipals;
use ic_cdk::query;

#[query]
fn whitelisted_principals() -> WhitelistedPrincipals {
    state::read(|s| s.whitelisted_principals())
}
