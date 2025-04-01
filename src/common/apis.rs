// ================== common apis ==================

#[ic_cdk::query]
pub fn cycles_balance() -> candid::Nat {
    candid::Nat::from(ic_cdk::api::canister_balance128())
}

#[ic_cdk::update]
pub fn cycles_receive() -> candid::Nat {
    let available = ic_cdk::api::call::msg_cycles_available128();

    if available == 0 {
        return candid::Nat::from(0_u128);
    }

    let accepted = ic_cdk::api::call::msg_cycles_accept128(available);

    assert!(accepted == available);

    candid::Nat::from(accepted)
}
