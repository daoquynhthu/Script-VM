//! Milestone H7 optimization-readiness helpers and metadata.
//!
//! Spec: `PHASE-3-RUNTIME-HELPER-IMPLEMENTATION-PLAN.md` §20.8,
//! `PHASE-3-RUNTIME-HELPER-CONTRACTS.md` §6, §8.4.4,
//! `PHASE-3-RUNTIME-HELPER-REGISTRY.md` §4–§6,
//! `PHASE-3-GC-METADATA-OWNERSHIP.md` §8 (safepoint/root policy hooks)

use std::collections::BTreeMap;

use vm_core::error::registry::RuntimeErrorCode;
use vm_core::id::{DeoptId, RuntimeHelperId, ShapeId};
use vm_core::value::Value;

use crate::heap::object::HeapObject;
use crate::heap::Heap;
use crate::runtime_error::{RuntimeFailure, RuntimeResult};

use super::registry::RuntimeHelperRegistry;
use super::schema::{
    HelperCallingConvention, HelperGcBehavior, HelperJitCallPolicy, RuntimeHelperDescriptor,
};

/// Shape classification for bootstrap shape checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShapeKind {
    List,
    Map,
    Record { field_count: u32 },
    Enum { min_cases: u32 },
    Function,
}

/// Bootstrap shape table: ShapeId → expected kind (not a public ABI).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ShapeRegistry {
    shapes: BTreeMap<u32, ShapeKind>,
}

impl ShapeRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, shape_id: ShapeId, kind: ShapeKind) {
        self.shapes.insert(shape_id.raw(), kind);
    }

    #[must_use]
    pub fn lookup(&self, shape_id: ShapeId) -> Option<ShapeKind> {
        self.shapes.get(&shape_id.raw()).copied()
    }
}

/// Bootstrap: `args[0]` value, `args[1]` Int ShapeId → Bool (does not raise on mismatch).
pub fn helper_check_shape(
    args: &[Value],
    heap: &Heap,
    shapes: &ShapeRegistry,
) -> RuntimeResult<Value> {
    let value = args
        .first()
        .ok_or_else(|| RuntimeFailure::language(RuntimeErrorCode::TypeError))?;
    let shape_raw = match args.get(1) {
        Some(Value::Int(v)) if *v >= 0 => *v as u32,
        _ => return Err(RuntimeFailure::language(RuntimeErrorCode::TypeError)),
    };
    let shape_id = ShapeId::new(shape_raw);
    let Some(expected) = shapes.lookup(shape_id) else {
        // Unknown shape id: treat as not matching (generic shape check fallback).
        return Ok(Value::Bool(false));
    };
    Ok(Value::Bool(value_matches_shape(value, expected, heap)?))
}

fn value_matches_shape(value: &Value, expected: ShapeKind, heap: &Heap) -> RuntimeResult<bool> {
    match value {
        Value::ObjectRef(id) => {
            let object = heap.get(*id)?;
            Ok(match (object, expected) {
                (HeapObject::List { .. }, ShapeKind::List) => true,
                (HeapObject::Map { .. }, ShapeKind::Map) => true,
                (HeapObject::RecordInstance { fields, .. }, ShapeKind::Record { field_count }) => {
                    fields.len() as u32 == field_count
                }
                (HeapObject::EnumValue { case_index, .. }, ShapeKind::Enum { min_cases }) => {
                    case_index.0 < min_cases
                }
                (HeapObject::Function, ShapeKind::Function) => true,
                _ => false,
            })
        }
        _ => Ok(false),
    }
}

/// JIT-facing helper call descriptor (internal; not public ABI).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HelperJitCallDescriptor {
    pub helper_id: RuntimeHelperId,
    pub name: String,
    pub jit_call_policy: HelperJitCallPolicy,
    pub calling_convention: HelperCallingConvention,
    pub may_allocate: bool,
    pub may_raise: bool,
    pub is_safepoint: bool,
    pub requires_roots_visible: bool,
    pub gc_behavior: HelperGcBehavior,
    pub is_jit_callable: bool,
    /// Optional deopt metadata link for JIT bailout paths.
    pub deopt_link: Option<DeoptId>,
}

/// Deopt metadata links for helpers that may deoptimize (internal table).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HelperDeoptLinkTable {
    links: BTreeMap<u32, DeoptId>,
}

impl HelperDeoptLinkTable {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn link(&mut self, helper_id: RuntimeHelperId, deopt_id: DeoptId) {
        self.links.insert(helper_id.raw(), deopt_id);
    }

    #[must_use]
    pub fn get(&self, helper_id: RuntimeHelperId) -> Option<DeoptId> {
        self.links.get(&helper_id.raw()).copied()
    }
}

/// Build the helper JIT readiness matrix from the canonical registry + deopt links.
#[must_use]
pub fn build_jit_readiness_matrix(
    registry: &RuntimeHelperRegistry,
    deopt_links: &HelperDeoptLinkTable,
) -> Vec<HelperJitCallDescriptor> {
    registry
        .descriptors()
        .map(|d| descriptor_to_jit_row(d, deopt_links.get(d.helper_id)))
        .collect()
}

fn descriptor_to_jit_row(
    d: &RuntimeHelperDescriptor,
    deopt_link: Option<DeoptId>,
) -> HelperJitCallDescriptor {
    HelperJitCallDescriptor {
        helper_id: d.helper_id,
        name: d.name.clone(),
        jit_call_policy: d.jit_call_policy,
        calling_convention: d.signature.calling_convention,
        may_allocate: d.may_allocate,
        may_raise: d.may_raise,
        is_safepoint: d.is_safepoint,
        requires_roots_visible: d.requires_roots_visible,
        gc_behavior: d.gc_behavior,
        is_jit_callable: d.is_jit_callable(),
        deopt_link,
    }
}

/// Safepoint / root-map policy validation for a single helper descriptor.
///
/// Helpers that are safepoints and may collect must require roots visible.
/// JIT-callable helpers must not use InterpreterOnly / NotJitCallable policies.
pub fn validate_helper_safepoint_root_policy(
    d: &RuntimeHelperDescriptor,
) -> RuntimeResult<()> {
    if d.is_safepoint && d.may_collect() && !d.requires_roots_visible {
        return Err(RuntimeFailure::structural(
            vm_core::error::registry::VmStructuralErrorCode::InvalidRootMapError,
            format!(
                "safepoint helper {} may collect without requires_roots_visible",
                d.helper_id.raw()
            ),
        ));
    }
    if d.is_jit_callable()
        && matches!(
            d.jit_call_policy,
            HelperJitCallPolicy::NotJitCallable | HelperJitCallPolicy::InterpreterOnly
        )
    {
        return Err(RuntimeFailure::structural(
            vm_core::error::registry::VmStructuralErrorCode::InvalidHelperError,
            format!(
                "helper {} marked JIT-callable with policy {:?}",
                d.helper_id.raw(),
                d.jit_call_policy
            ),
        ));
    }
    Ok(())
}

/// Validate full JIT readiness matrix rows.
pub fn validate_jit_readiness_matrix(
    rows: &[HelperJitCallDescriptor],
) -> RuntimeResult<()> {
    for row in rows {
        if row.is_safepoint
            && matches!(
                row.gc_behavior,
                HelperGcBehavior::MayAllocateMayCollect
                    | HelperGcBehavior::MayMoveObjects
                    | HelperGcBehavior::GcInternal
            )
            && !row.requires_roots_visible
        {
            return Err(RuntimeFailure::structural(
                vm_core::error::registry::VmStructuralErrorCode::InvalidRootMapError,
                format!(
                    "JIT matrix: safepoint helper {} lacks roots_visible",
                    row.helper_id.raw()
                ),
            ));
        }
        if row.is_jit_callable
            && matches!(
                row.jit_call_policy,
                HelperJitCallPolicy::NotJitCallable | HelperJitCallPolicy::InterpreterOnly
            )
        {
            return Err(RuntimeFailure::structural(
                vm_core::error::registry::VmStructuralErrorCode::InvalidHelperError,
                format!(
                    "JIT matrix: helper {} inconsistent jit policy",
                    row.helper_id.raw()
                ),
            ));
        }
        // Deopt-linked helpers must be JIT-callable (bailout from JIT).
        if row.deopt_link.is_some() && !row.is_jit_callable {
            return Err(RuntimeFailure::structural(
                vm_core::error::registry::VmStructuralErrorCode::InvalidDeoptError,
                format!(
                    "deopt link on non-JIT helper {}",
                    row.helper_id.raw()
                ),
            ));
        }
    }
    Ok(())
}

/// Helper ids currently dispatched through `dispatch_helper` (H1–H7 subset).
#[must_use]
pub fn dispatched_helper_ids() -> &'static [u32] {
    &[
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, // H1 type + H4 error + H7 shape
        11, 12, 13, 14, 15, 16, 17, 18, // H2/H3 access + bind
        21, 22, 23, 25, 26, 27, // construct + call
        29, 30, 31, 32, 33, // control/resource
        34, 35, 36, 37, 38, // module
        39, 40, 41, // host
        42, // display
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::registry::RuntimeHelperRegistry;
    use vm_core::id::{CaseIndex, EnumId, ObjectId};

    #[test]
    fn check_shape_matches_record_field_count() {
        let mut heap = Heap::new();
        let rec = heap
            .alloc_record_instance(vec![Value::Int(1), Value::Int(2)], false)
            .expect("rec");
        let mut shapes = ShapeRegistry::new();
        shapes.register(ShapeId::new(1), ShapeKind::Record { field_count: 2 });
        shapes.register(ShapeId::new(2), ShapeKind::Record { field_count: 3 });
        let ok = helper_check_shape(
            &[Value::ObjectRef(rec.id()), Value::Int(1)],
            &heap,
            &shapes,
        )
        .expect("check");
        assert_eq!(ok, Value::Bool(true));
        let bad = helper_check_shape(
            &[Value::ObjectRef(rec.id()), Value::Int(2)],
            &heap,
            &shapes,
        )
        .expect("check");
        assert_eq!(bad, Value::Bool(false));
    }

    #[test]
    fn check_shape_list_and_unknown() {
        let mut heap = Heap::new();
        let list = heap.alloc_list(vec![], false).expect("list");
        let mut shapes = ShapeRegistry::new();
        shapes.register(ShapeId::new(10), ShapeKind::List);
        let ok = helper_check_shape(
            &[Value::ObjectRef(list.id()), Value::Int(10)],
            &heap,
            &shapes,
        )
        .expect("ok");
        assert_eq!(ok, Value::Bool(true));
        let unknown = helper_check_shape(
            &[Value::ObjectRef(list.id()), Value::Int(99)],
            &heap,
            &shapes,
        )
        .expect("unknown");
        assert_eq!(unknown, Value::Bool(false));
    }

    #[test]
    fn check_shape_enum_case_bounds() {
        let mut heap = Heap::new();
        let en = heap
            .alloc_enum_value(EnumId::new(0), CaseIndex(1), None)
            .expect("enum");
        let mut shapes = ShapeRegistry::new();
        shapes.register(ShapeId::new(5), ShapeKind::Enum { min_cases: 2 });
        let ok = helper_check_shape(
            &[Value::ObjectRef(en.id()), Value::Int(5)],
            &heap,
            &shapes,
        )
        .expect("ok");
        assert_eq!(ok, Value::Bool(true));
        shapes.register(ShapeId::new(6), ShapeKind::Enum { min_cases: 1 });
        let bad = helper_check_shape(
            &[Value::ObjectRef(en.id()), Value::Int(6)],
            &heap,
            &shapes,
        )
        .expect("bad");
        assert_eq!(bad, Value::Bool(false));
    }

    #[test]
    fn jit_matrix_covers_canonical_registry() {
        let registry = RuntimeHelperRegistry::canonical().expect("reg");
        let mut links = HelperDeoptLinkTable::new();
        // Link a JIT-callable helper (numeric binary id 11)
        links.link(RuntimeHelperId::new(11), DeoptId::new(0));
        let matrix = build_jit_readiness_matrix(&registry, &links);
        assert_eq!(matrix.len(), 47);
        validate_jit_readiness_matrix(&matrix).expect("valid");
        let row = matrix.iter().find(|r| r.helper_id.raw() == 11).expect("11");
        assert!(row.is_jit_callable);
        assert_eq!(row.deopt_link, Some(DeoptId::new(0)));
    }

    #[test]
    fn deopt_link_on_interpreter_only_rejected() {
        let registry = RuntimeHelperRegistry::canonical().expect("reg");
        let mut links = HelperDeoptLinkTable::new();
        // id 3 raise is InterpreterOnly
        links.link(RuntimeHelperId::new(3), DeoptId::new(1));
        let matrix = build_jit_readiness_matrix(&registry, &links);
        let err = validate_jit_readiness_matrix(&matrix).expect_err("deopt");
        assert!(matches!(err, RuntimeFailure::Structural(_)));
    }

    #[test]
    fn canonical_safepoint_policies_hold() {
        let registry = RuntimeHelperRegistry::canonical().expect("reg");
        for d in registry.descriptors() {
            validate_helper_safepoint_root_policy(d).expect("policy");
        }
    }

    #[test]
    fn dispatched_ids_are_subset_of_registry() {
        let registry = RuntimeHelperRegistry::canonical().expect("reg");
        for id in dispatched_helper_ids() {
            assert!(
                registry.contains(RuntimeHelperId::new(*id)),
                "missing {id}"
            );
        }
    }

    #[test]
    fn non_object_fails_shape_match() {
        let heap = Heap::new();
        let mut shapes = ShapeRegistry::new();
        shapes.register(ShapeId::new(1), ShapeKind::List);
        let v = helper_check_shape(&[Value::Int(1), Value::Int(1)], &heap, &shapes).expect("x");
        assert_eq!(v, Value::Bool(false));
        let _ = ObjectId::new(0); // silence unused if any
    }
}
