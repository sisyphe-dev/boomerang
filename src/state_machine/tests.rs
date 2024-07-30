use crate::state_machine::{BoomerangSetup, ONE_MONTH_SECONDS, USER_PRINCIPAL_ID};
use crate::{E8S, TRANSFER_FEE};
use ic_state_machine_tests::PrincipalId;
use icrc_ledger_types::icrc1::account::Account;
use std::time::Duration;

#[test]
fn check_e2e() {
    let boomerang = BoomerangSetup::new();

    let caller = PrincipalId::new_user_test_id(USER_PRINCIPAL_ID);

    let account_id = boomerang.get_staking_account_id(caller.0);

    assert!(boomerang
        .icp_transfer(caller.0, None, 1_000 * E8S, account_id)
        .is_ok());

    assert!(boomerang.notify_icp_deposit(caller.0).is_ok());

    assert!(boomerang.retrieve_nicp(caller.0).is_ok());

    let balance: u64 = boomerang.nicp_balance(caller.0).0.try_into().unwrap();
    assert_eq!(balance, 1_000 * E8S - 3 * TRANSFER_FEE);

    boomerang
        .nicp_transfer(
            boomerang.water_neuron_id.into(),
            None,
            balance,
            Account {
                owner: caller.0,
                subaccount: None,
            },
        )
        .unwrap();

    let account = boomerang.get_unstaking_account(caller.0);

    assert!(boomerang
        .nicp_transfer(caller.0, None, balance - TRANSFER_FEE, account)
        .is_ok());

    assert!(boomerang.notify_nicp_deposit(caller.0).is_ok());

    assert!(boomerang.try_retrieve_icp(caller.0).is_err());

    boomerang
        .env
        .advance_time(Duration::from_secs(7 * ONE_MONTH_SECONDS));
    boomerang.env.tick();
    

    let res = boomerang.try_retrieve_icp(caller.0);

    println!("{:?}", res);

    let balance = boomerang.icp_balance(caller.0);
    println!("{balance}");

    panic!();
}