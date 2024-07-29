use crate::state_machine::BoomerangSetup;
use ic_state_machine_tests::PrincipalId;

#[test]
fn check_get_staking_account_id() {
    let boomerang = BoomerangSetup::new();

    let caller = PrincipalId::new_user_test_id(212);
    println!("{caller}");
    let account_id = boomerang.get_staking_account_id(caller.0);
    println!("{account_id}");

    panic!();
}
