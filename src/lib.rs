use candid::{CandidType, Deserialize, Nat, Principal};
use icrc_ledger_types::icrc1::transfer::TransferError;
use icrc_ledger_types::icrc2::approve::ApproveError;
use icrc_ledger_types::icrc2::transfer_from::TransferFromError;
use serde::Serialize;

pub mod log;

// "ryjl3-tyaaa-aaaaa-aaaba-cai"
pub const ICP_LEDGER_ID: Principal = Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 2, 1, 1]);

// "buwm7-7yaaa-aaaar-qagva-cai"
pub const NICP_LEDGER_ID: Principal = Principal::from_slice(&[0, 0, 0, 0, 2, 48, 1, 170, 1, 1]);

// "jcmow-hyaaa-aaaaq-aadlq-cai"
pub const WTN_LEDGER_ID: Principal = Principal::from_slice(&[0, 0, 0, 0, 2, 0, 0, 215, 1, 1]);

// "n76cn-tyaaa-aaaam-acc5a-cai"
pub const WATER_NEURON_ID: Principal = Principal::from_slice(&[0, 0, 0, 0, 1, 128, 16, 186, 1, 1]);

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
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum BoomerangConversionError {
    ApproveError(ApproveError),
    MissingIcpBalance,
    MissingNicpBalance,
    ConversionError(ConversionError),
    TransferError(TransferError),
}

#[derive(CandidType, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct BoomerangConversionSuccess {
    pub nicp_block_index: Nat,
    pub wtn_block_index: Option<Nat>,
}
