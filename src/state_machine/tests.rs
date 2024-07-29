use crate::state_machine::{BoomerangSetup, USER_PRINCIPAL_ID};
use ic_state_machine_tests::PrincipalId;

#[test]
fn check_notify_icp_deposit() {
    let boomerang = BoomerangSetup::new();

    let caller = PrincipalId::new_user_test_id(USER_PRINCIPAL_ID);

    println!("{caller}");
    let account_id = boomerang.get_staking_account_id(caller.0);
    println!("{account_id}");

    let transfer_result = boomerang.icp_transfer(caller.0, None, 1_000_000, account_id);
    panic!();
}
