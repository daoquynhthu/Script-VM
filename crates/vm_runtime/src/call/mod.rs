//! Call execution protocol substrate.
//!
//! Spec: `PHASE-3-CALL-EXECUTION-PROTOCOL.md`

pub mod bind;
pub mod builtin;
pub mod callable;
pub mod contract;
pub mod default;
pub mod input;
pub mod runtime;

pub use bind::{bind_arguments, ArgumentBinding, ParameterSpec};
pub use builtin::{
    validate_builtin_call, BuiltinCallDescriptor, HELPER_CALL_BUILTIN_ID, HELPER_CHECK_ARITY_ID,
    HELPER_GENERIC_CALL_ID,
};
pub use callable::{
    check_callable, BuiltinFunctionTarget, BoundMethodTarget, CallableRegistry, CallableTarget,
    EnumCaseConstructorTarget, HostFunctionTarget, RecordConstructorTarget, UserFunctionTarget,
};
pub use contract::{
    check_parameter_contracts, check_return_contract, StubTypeContractChecker, TypeContractChecker,
};
pub use default::{fill_defaults, DefaultEvaluator};
pub use input::{CallFrameInput, NamedArgumentValue};
pub use runtime::{CallRuntime, PreparedCall};