use ic_state_machine_tests::{
    CanisterId, CanisterStatusResultV2, PrincipalId, StateMachine, WasmResult,
};
use lazy_static::lazy_static;


lazy_static!{
    static ref CARGO_BUILD_RESULT: Result<(), std::io::Error> = cargo_build();
}

pub struct BoomerangSetup {
    env: StateMachine
}