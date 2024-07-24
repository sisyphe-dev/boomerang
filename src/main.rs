use boomerang::log::INFO;
use boomerang::{
    BoomerangConversionError, BoomerangConversionSuccess, ConversionError, DepositSuccess, ConversionArg,
    Icrc1TransferArg, ICP_LEDGER_ID, NICP_LEDGER_ID, WATER_NEURON_ID, WTN_LEDGER_ID,
};
use candid::{Nat, Principal};
use ic_base_types::PrincipalId;
use ic_canister_log::log;
use ic_cdk::{query, update};
use icp_ledger::{AccountIdentifier, Subaccount};
use icrc_ledger_client_cdk::{CdkRuntime, ICRC1Client};
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::TransferArg;
use icrc_ledger_types::icrc1::transfer::TransferError;
use icrc_ledger_types::icrc2::approve::ApproveArgs;

fn main() {
    
}

const E8S: u64 = 100_000_000;

#[query]
fn get_account_id(principal: Principal) -> AccountIdentifier {
    let subaccount = Subaccount::from(&PrincipalId::from(principal));
    let boomerang_id = ic_cdk::id();

    AccountIdentifier::new(PrincipalId::from(boomerang_id), Some(subaccount))
}

#[update]
async fn notify_icp_transfer(
    client_id: Principal,
) -> Result<BoomerangConversionSuccess, BoomerangConversionError> {
    let boomerang_id = ic_cdk::id();

    let subaccount = Subaccount::from(&PrincipalId::from(client_id));

    let boomerang_account = Account {
        owner: boomerang_id,
        subaccount: Some(subaccount.0),
    };

    let client = ICRC1Client {
        runtime: CdkRuntime,
        ledger_canister_id: ICP_LEDGER_ID,
    };

    let balance_e8s = match client.balance_of(boomerang_account).await {
        Ok(balance) => {
            log!(INFO, "Subaccount Balance: {}", balance);
            balance
        },
        Err(_) => {
            return Err(BoomerangConversionError::MissingIcpBalance);
        }
    };

    let spender = Account {
        owner: WATER_NEURON_ID,
        subaccount: None,
    };

    let approve_args = ApproveArgs {
        from_subaccount: Some(subaccount.0),
        spender,
        amount: balance_e8s.clone(),
        expected_allowance: None,
        expires_at: None,
        fee: None,
        memo: None,
        created_at_time: None,
    };

    match client.approve(approve_args).await.unwrap() {
        Ok(block_index) => {
            log! {INFO, "Approval occured at block index: {}", block_index};
        }
        Err(error) => {
            return Err(BoomerangConversionError::ApproveError(error));
        }
    };

    let conversion_arg = ConversionArg {
        amount_e8s: balance_e8s.0.to_u64_digits()[0]-20_000,
        maybe_subaccount: Some(subaccount.0), 
    };

    let conversion_result: (Result<DepositSuccess, ConversionError>,) =
        ic_cdk::call(WATER_NEURON_ID, "icp_to_nicp", (conversion_arg,))
            .await
            .unwrap();

    match conversion_result.0 {
        Ok(success) => {
            log!(
                INFO,
                "Transfered {} ICP at block index: {}",
                balance_e8s.clone() / E8S,
                success.block_index
            );
        }
        Err(error) => {
            return Err(BoomerangConversionError::ConversionError(error));
        }
    }

    let nicp_client = ICRC1Client {
        runtime: CdkRuntime,
        ledger_canister_id: NICP_LEDGER_ID,
    };

    let boomerang_account = Account {
        owner: boomerang_id,
        subaccount: Some(subaccount.0),
    };

    let nicp_balance_e8s = match nicp_client.balance_of(boomerang_account).await {
        Ok(balance) => balance,
        Err(_) => {
            return Err(BoomerangConversionError::MissingNicpBalance);
        }
    };

    log!(INFO, "Nicp balance: {}", nicp_balance_e8s);

    let nicp_fee_e8s = 10_000;

    let nicp_block_index = match handle_icrc1_transfer(Icrc1TransferArg {
        fee_e8s: nicp_fee_e8s,
        amount_e8s: nicp_balance_e8s.clone(),
        ledger_id: NICP_LEDGER_ID,
        to: client_id,
    })
    .await
    {
        Ok(block_index) => {
            log!(
                INFO,
                "Transfered nICP at block index: {}",
                block_index
            );

            block_index
        }
        Err(_) => {
            return Err(BoomerangConversionError::MissingNicpBalance);
        }
    };

    let wtn_client = ICRC1Client {
        runtime: CdkRuntime,
        ledger_canister_id: WTN_LEDGER_ID,
    };

    let wtn_balance_e8s = match wtn_client.balance_of(boomerang_account).await {
        Ok(balance) => balance,
        Err(_) => {
            return Ok(BoomerangConversionSuccess {
                nicp_block_index,
                wtn_block_index: None,
            });
        }
    };

    if wtn_balance_e8s == Nat::from(0u64) {
        return Ok(BoomerangConversionSuccess {
            nicp_block_index,
            wtn_block_index: None,
        });
    }

    let wtn_fee_e8s = 10_000;

    match handle_icrc1_transfer(Icrc1TransferArg {
        fee_e8s: wtn_fee_e8s,
        amount_e8s: wtn_balance_e8s.clone(),
        ledger_id: WTN_LEDGER_ID,
        to: client_id,
    })
    .await
    {
        Ok(block_index) => {
            log!(
                INFO,
                "Transfered WTN at block index: {}",
                block_index
            );

            Ok(BoomerangConversionSuccess {
                nicp_block_index,
                wtn_block_index: Some(block_index),
            })
        }
        Err(error) => Err(BoomerangConversionError::TransferError(error)),
    }
}

async fn handle_icrc1_transfer(arg: Icrc1TransferArg) -> Result<Nat, TransferError> {
    let user_account = Account {
        owner: arg.to,
        subaccount: None,
    };

    let subaccount = Subaccount::from(&PrincipalId::from(arg.to));

    let transfer_args = TransferArg {
        memo: None,
        amount: arg.amount_e8s.clone() - arg.fee_e8s,
        fee: Some(arg.fee_e8s.into()),
        from_subaccount: Some(subaccount.0),
        to: user_account,
        created_at_time: None,
    };

    let client = ICRC1Client {
        runtime: CdkRuntime,
        ledger_canister_id: NICP_LEDGER_ID,
    };

    let res = client.transfer(transfer_args).await.unwrap();

    log!(INFO, "{:?}", res);
    res

}

/// Checks the real candid interface against the one declared in the did file
/// Check that the types used to interact with the NNS governance canister are matching.
#[test]
fn check_candid_interface_compatibility() {
    fn source_to_str(source: &candid_parser::utils::CandidSource) -> String {
        match source {
            candid_parser::utils::CandidSource::File(f) => {
                std::fs::read_to_string(f).unwrap_or_else(|_| "".to_string())
            }
            candid_parser::utils::CandidSource::Text(t) => t.to_string(),
        }
    }

    fn check_service_equal(
        new_name: &str,
        new: candid_parser::utils::CandidSource,
        old_name: &str,
        old: candid_parser::utils::CandidSource,
    ) {
        let new_str = source_to_str(&new);
        let old_str = source_to_str(&old);
        match candid_parser::utils::service_equal(new, old) {
            Ok(_) => {}
            Err(e) => {
                eprintln!(
                    "{} is not compatible with {}!\n\n\
            {}:\n\
            {}\n\n\
            {}:\n\
            {}\n",
                    new_name, old_name, new_name, new_str, old_name, old_str
                );
                panic!("{:?}", e);
            }
        }
    }

    candid::export_service!();

    let new_interface = __export_service();

    // check the public interface against the actual one
    let old_interface = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("boomerang.did");

    check_service_equal(
        "actual cycles-manager candid interface",
        candid_parser::utils::CandidSource::Text(&new_interface),
        "declared candid interface in boomerang.did file",
        candid_parser::utils::CandidSource::File(old_interface.as_path()),
    );
}
