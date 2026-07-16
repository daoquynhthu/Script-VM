//! Phase 2 SIR identity types.
//!
//! Spec: `PHASE-2-IR-SPEC.md` §6

macro_rules! define_sir_id {
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

define_sir_id!(ModuleId);
define_sir_id!(SymbolId);
define_sir_id!(BindingId);
define_sir_id!(ScopeId);
define_sir_id!(NodeId);
define_sir_id!(TypeId);
define_sir_id!(RecordId);
define_sir_id!(EnumId);
define_sir_id!(FieldId);
define_sir_id!(CaseId);
define_sir_id!(FunctionId);
define_sir_id!(BlockId);
define_sir_id!(EffectId);
define_sir_id!(CapabilityId);
define_sir_id!(DiagnosticId);
define_sir_id!(ExtensionId);
define_sir_id!(ControlRegionId);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sir_ids_construct_and_compare() {
        let a = NodeId::new(1);
        let b = NodeId::new(1);
        let invalid = NodeId::INVALID;
        assert_eq!(a, b);
        assert!(a.is_valid());
        assert!(!invalid.is_valid());
    }
}