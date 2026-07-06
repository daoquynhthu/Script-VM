//! Phase 3 runtime identity types.
//!
//! Spec: `PHASE-3-NORMATIVE-KEYWORDS-GLOSSARY.md` §5.3,
//! `PHASE-3-RUNTIMEPLAN-SCHEMA-CLOSURE.md`, `PHASE-3-EIR-SCHEMA-CLOSURE.md`

pub use sir::id::{
    BindingId, BlockId, CapabilityId, CaseId, DiagnosticId, EffectId, EnumId, ExtensionId,
    FieldId, FunctionId, ModuleId, NodeId, RecordId, ScopeId, SymbolId, TypeId,
};

macro_rules! define_runtime_id {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(pub u32);

        impl $name {
            pub const INVALID: Self = Self(u32::MAX);

            #[must_use]
            pub const fn new(raw: u32) -> Self {
                Self(raw)
            }

            #[must_use]
            pub const fn raw(self) -> u32 {
                self.0
            }

            #[must_use]
            pub const fn is_valid(self) -> bool {
                self.0 != u32::MAX
            }
        }
    };
}

define_runtime_id!(SlotId);
define_runtime_id!(SlotLayoutId);
define_runtime_id!(EirFunctionId);
define_runtime_id!(EirBlockId);
define_runtime_id!(ControlRegionId);
define_runtime_id!(DeoptId);
define_runtime_id!(RuntimeHelperId);
define_runtime_id!(ObjectId);
define_runtime_id!(FrameId);
define_runtime_id!(CellId);
define_runtime_id!(CallSiteId);
define_runtime_id!(AccessSiteId);
define_runtime_id!(SafepointId);
define_runtime_id!(EirOpId);
define_runtime_id!(ConstantId);
define_runtime_id!(ShapeId);
define_runtime_id!(InterfaceId);
define_runtime_id!(FrameMapId);
define_runtime_id!(RootMapId);
define_runtime_id!(ErrorHandle);

/// Field position within a record shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FieldIndex(pub u32);

/// Case position within an enum shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CaseIndex(pub u32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_ids_reject_invalid_sentinel() {
        assert!(!SlotId::INVALID.is_valid());
        assert!(SlotId::new(0).is_valid());
    }
}