use candid::{CandidType, Deserialize, Nat, Principal};
use ic_base_types::PrincipalId;
use icp_ledger::{AccountIdentifier, Subaccount};
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::TransferError;
use icrc_ledger_types::icrc2::approve::ApproveError;
use icrc_ledger_types::icrc2::transfer_from::TransferFromError;
use serde::Serialize;

pub mod icp_to_nicp;
pub mod log;
pub mod nicp_to_icp;

// "ryjl3-tyaaa-aaaaa-aaaba-cai"
pub const ICP_LEDGER_ID: Principal = Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 2, 1, 1]);

// "buwm7-7yaaa-aaaar-qagva-cai"
pub const NICP_LEDGER_ID: Principal = Principal::from_slice(&[0, 0, 0, 0, 2, 48, 1, 170, 1, 1]);

// "jcmow-hyaaa-aaaaq-aadlq-cai"
pub const WTN_LEDGER_ID: Principal = Principal::from_slice(&[0, 0, 0, 0, 2, 0, 0, 215, 1, 1]);

// "tsbvt-pyaaa-aaaar-qafva-cai"
pub const WATER_NEURON_ID: Principal = Principal::from_slice(&[0, 0, 0, 0, 2, 48, 1, 106, 1, 1]);

pub const E8S: u64 = 100_000_000;
pub const TRANSFER_FEE: u64 = 10_000;

pub fn derive_account(principal: Principal, seed: u8) -> Account {
    let mut seed_derivation = vec![seed];
    seed_derivation.extend(&principal.as_slice()[1..]);
    let subaccount = Subaccount::from(&PrincipalId::from(Principal::from_slice(&seed_derivation)));

    Account {
        owner: self_canister_id(),
        subaccount: Some(subaccount.0),
    }
}

pub fn derive_account_id(principal: Principal, seed: u8) -> AccountIdentifier {
    let mut seed_derivation = vec![seed];
    seed_derivation.extend(&principal.as_slice()[1..]);
    let subaccount = Subaccount::from(&PrincipalId::from(Principal::from_slice(&seed_derivation)));
    AccountIdentifier::new(PrincipalId::from(self_canister_id()), Some(subaccount))
}

pub fn derive_subaccount(principal: Principal, seed: u8) -> Subaccount {
    let mut seed_derivation = vec![seed];
    seed_derivation.extend(&principal.as_slice()[1..]);
    Subaccount::from(&PrincipalId::from(Principal::from_slice(&seed_derivation)))
}

#[test]
fn check_canister_ids() {
    assert_eq!(
        Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(),
        ICP_LEDGER_ID
    );
    assert_eq!(
        Principal::from_text("buwm7-7yaaa-aaaar-qagva-cai").unwrap(),
        NICP_LEDGER_ID
    );
    assert_eq!(
        Principal::from_text("jcmow-hyaaa-aaaaq-aadlq-cai").unwrap(),
        WTN_LEDGER_ID
    );
    assert_eq!(
        Principal::from_text("tsbvt-pyaaa-aaaar-qafva-cai").unwrap(),
        WATER_NEURON_ID
    );
}

#[cfg(target = "wasm32-unknown-unkown")]
pub fn self_canister_id() -> Principal {
    ic_cdk::id()
}

#[cfg(not(target = "wasm32-unknown-unkown"))]
pub fn self_canister_id() -> Principal {
    Principal::anonymous()
}

pub struct Icrc1TransferArg {
    pub amount_e8s: Nat,
    pub fee_e8s: u64,
    pub ledger_id: Principal,
    pub to: Principal,
}

#[derive(Debug, PartialEq, Eq, CandidType, Serialize, Deserialize)]
pub enum GuardError {
    AlreadyProcessing,
    TooManyConcurrentRequests,
}

#[derive(CandidType, Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct ConversionArg {
    pub amount_e8s: u64,
    pub maybe_subaccount: Option<[u8; 32]>,
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ConversionError {
    TransferFromError(TransferFromError),
    TransferError(TransferError),
    AmountTooLow { minimum_amount_e8s: u64 },
    GuardError { guard_error: GuardError },
    GenericError { code: i32, message: String },
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct DepositSuccess {
    pub block_index: Nat,
    pub transfer_id: u64,
    pub nicp_amount: Option<u64>,
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct WithdrawalSuccess {
    block_index: Nat,
    withdrawal_id: u64,
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum BoomerangError {
    ApproveError(ApproveError),
    BalanceOfError(String),
    ConversionError(ConversionError),
    TransferError(TransferError),
    IcpNotAvailable,
}
