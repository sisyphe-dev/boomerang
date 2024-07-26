use boomerang::{self_canister_id, BoomerangError, DepositSuccess};
use candid::{Nat, Principal};
use ic_base_types::PrincipalId;
use ic_cdk::{query, update};
use icp_ledger::{AccountIdentifier, Subaccount};

fn main() {}

#[query]
fn get_account_id(principal: Principal) -> AccountIdentifier {
    let boomerang_id = self_canister_id();
    let subaccount = Subaccount::from(&PrincipalId::from(principal));
    AccountIdentifier::new(PrincipalId::from(boomerang_id), Some(subaccount))
}

#[update]
async fn retrieve_nicp(target: Principal) -> Result<Nat, BoomerangError> {
    boomerang::icp_to_nicp::retrieve_nicp(target).await
}

#[update]
async fn notify_icp_deposit(client_id: Principal) -> Result<DepositSuccess, BoomerangError> {
    boomerang::icp_to_nicp::notify_icp_deposit(client_id).await
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

/*
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

if wtn_balance_e8s == 0_u64 {
    return Ok(BoomerangConversionSuccess {
        nicp_block_index,
        wtn_block_index: None,
    });
}

match handle_icrc1_transfer(Icrc1TransferArg {
    fee_e8s: TRANSFER_FEE,
    amount_e8s: wtn_balance_e8s.clone(),
    ledger_id: WTN_LEDGER_ID,
    to: client_id,
})
.await
{
    Ok(block_index) => {
        log!(INFO, "Transfered WTN at block index: {}", block_index);

        Ok(BoomerangConversionSuccess {
            nicp_block_index,
            wtn_block_index: Some(block_index),
        })
    }
    Err(error) => Err(BoomerangError::TransferError(error)),
}
    */
