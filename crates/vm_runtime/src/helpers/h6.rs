//! Milestone H6 capability / host-boundary helper implementations.
//!
//! Spec: `PHASE-3-RUNTIME-HELPER-IMPLEMENTATION-PLAN.md` §20.7,
//! `PHASE-3-RUNTIME-HELPER-CONTRACTS.md` §8.10,
//! `PHASE-3-HOST-BOUNDARY-CONTRACT.md` §5–§8
//!
//! Bootstrap lives in `vm_runtime` (no dependency on `vm_host`, which depends
//! on this crate). Semantics mirror host root registry + error normalization.

use vm_core::error::language::{ErrorObj, ErrorStore};
use vm_core::error::registry::{RuntimeErrorCode, VmStructuralErrorCode};
use vm_core::error::VmError;
use vm_core::id::CapabilityId;
use vm_core::value::Value;

use crate::control::VmControl;
use crate::module::resolver::CapabilitySet;
use crate::runtime_error::{RuntimeFailure, RuntimeResult};

/// Call-scoped host root retained for the duration of a host call.
#[derive(Debug, Clone, PartialEq)]
pub struct HostRootEntry {
    pub id: u32,
    pub value: Value,
}

/// Bootstrap host-boundary session state for enter/exit helpers.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct HostBoundarySession {
    pub active: bool,
    pub boundary_id: u32,
    pub call_scoped_roots: Vec<HostRootEntry>,
    next_root_id: u32,
}

impl HostBoundarySession {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_call_scoped_root(&mut self, value: Value) -> u32 {
        let id = self.next_root_id;
        self.next_root_id = self.next_root_id.saturating_add(1);
        self.call_scoped_roots.push(HostRootEntry { id, value });
        id
    }

    pub fn clear_call_scoped_roots(&mut self) {
        self.call_scoped_roots.clear();
    }
}

fn type_error() -> RuntimeFailure {
    RuntimeFailure::language(RuntimeErrorCode::TypeError)
}

fn require_session(
    session: Option<&mut HostBoundarySession>,
) -> RuntimeResult<&mut HostBoundarySession> {
    session.ok_or_else(|| {
        RuntimeFailure::structural(
            VmStructuralErrorCode::InvalidFrameStateError,
            "host boundary helpers require HostBoundarySession in dispatch env",
        )
    })
}

/// Bootstrap: `args[0]` Int capability_id → Unit if granted, else CapabilityError.
pub fn helper_check_capability(
    args: &[Value],
    capabilities: &CapabilitySet,
) -> RuntimeResult<()> {
    let cap = match args.first() {
        Some(Value::Int(v)) if *v >= 0 => CapabilityId::new(*v as u32),
        _ => return Err(type_error()),
    };
    if capabilities.has(cap) {
        Ok(())
    } else {
        Err(RuntimeFailure::language(RuntimeErrorCode::CapabilityError))
    }
}

/// Bootstrap: enter host call.
/// `args[0]` optional Int boundary_id (default 0); remaining ObjectRef/Error values
/// registered as call-scoped host roots. Returns Unit.
pub fn helper_enter_host_call(
    args: &[Value],
    session: Option<&mut HostBoundarySession>,
) -> RuntimeResult<()> {
    let session = require_session(session)?;
    if session.active {
        return Err(RuntimeFailure::structural(
            VmStructuralErrorCode::InvalidFrameStateError,
            "nested host call enter is not supported in bootstrap",
        ));
    }
    let (boundary_id, root_values) = match args.first() {
        Some(Value::Int(v)) if *v >= 0 => (*v as u32, &args[1..]),
        Some(Value::None) | None => (0, args),
        Some(Value::ObjectRef(_)) | Some(Value::Error(_)) => (0, args),
        _ => return Err(type_error()),
    };
    session.active = true;
    session.boundary_id = boundary_id;
    session.clear_call_scoped_roots();
    for value in root_values {
        match value {
            Value::ObjectRef(_) | Value::Error(_) => {
                session.register_call_scoped_root(value.clone());
            }
            Value::None => {}
            _ => return Err(type_error()),
        }
    }
    Ok(())
}

/// Bootstrap: exit host call and normalize host result.
///
/// Layout:
/// - `args[0]` Int tag: `0` success + `args[1]` value
/// - `args[0]` Int tag: `1` language host error + `args[1]` String message → Raise
/// - `args[0]` Int tag: `2` structural host error + `args[1]` String message → structural failure
///
/// Clears call-scoped roots. Must not surface raw host exception types (bootstrap
/// uses only Value/String tags).
pub fn helper_exit_host_call(
    args: &[Value],
    session: Option<&mut HostBoundarySession>,
    store: &mut ErrorStore,
) -> RuntimeResult<VmControl> {
    let session = require_session(session)?;
    if !session.active {
        return Err(RuntimeFailure::structural(
            VmStructuralErrorCode::InvalidFrameStateError,
            "exit_host_call without active host call",
        ));
    }
    let tag = match args.first() {
        Some(Value::Int(v)) => *v,
        _ => return Err(type_error()),
    };
    let outcome = match tag {
        0 => {
            let value = args.get(1).cloned().unwrap_or(Value::None);
            Ok(VmControl::Normal(Some(value)))
        }
        1 => {
            let message = match args.get(1) {
                Some(Value::String(s)) => s.clone(),
                _ => return Err(type_error()),
            };
            let handle = store.allocate(ErrorObj::new(
                RuntimeErrorCode::InternalVMError,
                message,
            ));
            Ok(VmControl::Raise(handle))
        }
        2 => {
            let message = match args.get(1) {
                Some(Value::String(s)) => s.clone(),
                _ => return Err(type_error()),
            };
            Err(RuntimeFailure::Structural(VmError::new(
                VmStructuralErrorCode::BackendViolationError,
                message,
            )))
        }
        _ => Err(type_error()),
    };
    session.clear_call_scoped_roots();
    session.active = false;
    outcome
}

/// Host must not retain heap values without a registered root (protocol check).
pub fn validate_host_retention(
    value: &Value,
    retained_without_root: bool,
) -> RuntimeResult<()> {
    if retained_without_root
        && matches!(value, Value::ObjectRef(_) | Value::Error(_))
    {
        return Err(RuntimeFailure::structural(
            VmStructuralErrorCode::BackendViolationError,
            "host retained VM value without HostRootEntry",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use vm_core::id::ObjectId;

    #[test]
    fn check_capability_grants_and_rejects() {
        let mut caps = CapabilitySet::new();
        let err = helper_check_capability(&[Value::Int(3)], &caps).expect_err("miss");
        assert_eq!(
            err,
            RuntimeFailure::language(RuntimeErrorCode::CapabilityError)
        );
        caps.grant(CapabilityId::new(3));
        helper_check_capability(&[Value::Int(3)], &caps).expect("ok");
    }

    #[test]
    fn enter_registers_roots_exit_clears_and_returns_value() {
        let mut session = HostBoundarySession::new();
        helper_enter_host_call(
            &[
                Value::Int(7),
                Value::ObjectRef(ObjectId::new(1)),
            ],
            Some(&mut session),
        )
        .expect("enter");
        assert!(session.active);
        assert_eq!(session.boundary_id, 7);
        assert_eq!(session.call_scoped_roots.len(), 1);

        let mut store = ErrorStore::new();
        let control = helper_exit_host_call(
            &[Value::Int(0), Value::Int(99)],
            Some(&mut session),
            &mut store,
        )
        .expect("exit");
        assert_eq!(control, VmControl::Normal(Some(Value::Int(99))));
        assert!(!session.active);
        assert!(session.call_scoped_roots.is_empty());
    }

    #[test]
    fn exit_language_error_raises_without_leaking_host_type() {
        let mut session = HostBoundarySession::new();
        helper_enter_host_call(&[Value::Int(0)], Some(&mut session)).expect("enter");
        let mut store = ErrorStore::new();
        let control = helper_exit_host_call(
            &[Value::Int(1), Value::String("host failed".into())],
            Some(&mut session),
            &mut store,
        )
        .expect("exit");
        let VmControl::Raise(h) = control else {
            panic!("raise");
        };
        assert_eq!(
            store.get(h).expect("e").error_code,
            RuntimeErrorCode::InternalVMError
        );
    }

    #[test]
    fn exit_structural_stays_structural() {
        let mut session = HostBoundarySession::new();
        helper_enter_host_call(&[], Some(&mut session)).expect("enter");
        let mut store = ErrorStore::new();
        let err = helper_exit_host_call(
            &[Value::Int(2), Value::String("structural:bad".into())],
            Some(&mut session),
            &mut store,
        )
        .expect_err("struct");
        assert!(matches!(err, RuntimeFailure::Structural(_)));
        assert!(!session.active);
    }

    #[test]
    fn retention_without_root_rejected() {
        let err = validate_host_retention(&Value::ObjectRef(ObjectId::new(1)), true)
            .expect_err("root");
        assert!(matches!(err, RuntimeFailure::Structural(_)));
    }
}
