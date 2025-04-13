use ic_canister_kit::types::CanisterId;

/// returns sorted token addresses, used to handle return values from pairs sorted in this order
pub fn sort_tokens(token_a: CanisterId, token_b: CanisterId) -> (CanisterId, CanisterId) {
    if token_a < token_b {
        (token_a, token_b)
    } else {
        (token_b, token_a)
    }
}
