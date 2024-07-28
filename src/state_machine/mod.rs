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
            "cycles-manager",
            "--locked",
        ])
        .spawn()?
        .wait()?;
    Ok(())
}

pub struct BoomerangSetup {
    env: StateMachine,
    nicp_ledger_id: CanisterId,
    icp_ledger_id: CanisterId,
}
