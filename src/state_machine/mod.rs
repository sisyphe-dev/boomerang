use candid::{CandidType, Deserialize};
use ic_state_machine_tests::{
    CanisterId, CanisterStatusResultV2, PrincipalId, StateMachine, WasmResult,
};
use lazy_static::lazy_static;
use std::process::Command;

lazy_static! {
    static ref CARGO_BUILD_RESULT: Result<(), std::io::Error> = cargo_build();
}

fn cargo_build() -> Result<(), std::io::Error> {
    Command::new("cargo")
        .args(&[
            "build",
            "--target",
            "wasm32-unknown-unknown",
            "--release",
            "-p",
            "boomerang",
            "--locked",
        ])
        .spawn()?
        .wait()?;
    Ok(())
}

#[derive(Debug)]
pub struct BoomerangSetup {
    env: StateMachine,
    pub minter: PrincipalId,
    pub water_neuron_id: CanisterId,
    pub wtn_ledger_id: CanisterId,
    pub icp_ledger_id: CanisterId,
    pub nicp_ledger_id: CanisterId,
    pub governance_id: CanisterId,
}

#[derive(Deserialize, CandidType)]
pub enum LiquidArg {
    Init(InitArg),
    Upgrade(Option<UpgradeArg>),
}

#[derive(Deserialize, CandidType, Encode, Decode, PartialEq, Eq, Clone, Debug)]
pub struct InitArg {
    #[cbor(n(0), with = "crate::cbor::principal")]
    nicp_ledger_id: Principal,
    #[cbor(n(1), with = "crate::cbor::principal")]
    pub wtn_governance_id: Principal,
    #[cbor(n(2), with = "crate::cbor::principal")]
    pub wtn_ledger_id: Principal,
}

#[derive(Deserialize, CandidType, Encode, Decode, PartialEq, Eq, Clone, Debug)]
pub struct UpgradeArg {
    #[n(0)]
    pub governance_fee_share_percent: Option<u64>,
}

impl BoomerangSetup {
    fn new() -> Self {
        let env = StateMachine::new();
        let minter = PrincipalId::new_user_test_id(DEFAULT_PRINCIPAL_ID);

        let mut initial_balances = HashMap::new();
        initial_balances.insert(
            AccountIdentifier::new(minter.into(), None),
            Tokens::from_e8s(22_000_000 * E8S),
        );

        let icp_ledger_id = env
            .install_canister(
                icp_ledger_wasm(),
                Encode!(&LedgerCanisterInitPayload::builder()
                    .initial_values(initial_balances)
                    .transfer_fee(Tokens::from_e8s(10_000))
                    .minting_account(GOVERNANCE_CANISTER_ID.get().into())
                    .token_symbol_and_name("ICP", "Internet Computer")
                    .feature_flags(icp_ledger::FeatureFlags { icrc2: true })
                    .build()
                    .unwrap())
                .unwrap(),
                None,
            )
            .unwrap();

        let nicp_ledger_id = env.create_canister(None);
        let wtn_ledger_id = env.create_canister(None);

        let water_neuron_id = env
            .install_canister(
                water_neuron_wasm(),
                Encode!(&LiquidArg::Init(InitArg {
                    wtn_governance_id: sns.governance.into(),
                    wtn_ledger_id: sns.ledger.into(),
                    nicp_ledger_id: nicp_ledger_id.get().0,
                }))
                .unwrap(),
            )
            .unwrap();
    }
}
