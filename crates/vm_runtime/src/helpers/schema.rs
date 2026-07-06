//! Runtime helper descriptor schema.
//!
//! Spec: `PHASE-3-RUNTIME-HELPER-REGISTRY.md`, `PHASE-3-RUNTIME-HELPER-CONTRACTS.md`

use vm_core::id::{CapabilityId, EffectId, RuntimeHelperId};

/// Internal helper family classification (contracts §3.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuntimeHelperFamily {
    Call,
    Access,
    Construction,
    TypeCheck,
    Pattern,
    Error,
    Unwind,
    Resource,
    Module,
    Capability,
    Allocation,
    WriteBarrier,
    Display,
    Numeric,
    Debug,
}

/// Helper result type (registry §4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HelperResultType {
    Value,
    VmControl,
    Unit,
    Bool,
    ErrorRef,
    HelperInternal,
}

/// Internal calling convention (contracts §3.6).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HelperCallingConvention {
    InterpreterDirect,
    JitRuntimeCall,
    HostBoundaryCall,
    GcRuntimeCall,
    InternalOnly,
}

/// GC behavior declaration (contracts §5.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HelperGcBehavior {
    NoAllocation,
    MayAllocateNoCollection,
    MayAllocateMayCollect,
    MayMoveObjects,
    GcInternal,
}

/// JIT call policy (contracts §6).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HelperJitCallPolicy {
    NotJitCallable,
    InterpreterOnly,
    JitRuntimeCall,
    HostBoundaryCall,
    GcRuntimeCall,
    InternalOnly,
}

/// Source mapping policy for may-raise helpers (registry §5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HelperSourceMappingPolicy {
    NotRequired,
    RequiredOnRaise,
}

/// Minimal helper signature (full param list deferred to interpreter WP).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeHelperSignature {
    pub result: HelperResultType,
    pub calling_convention: HelperCallingConvention,
}

/// Canonical runtime helper descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeHelperDescriptor {
    pub helper_id: RuntimeHelperId,
    pub name: String,
    pub family: RuntimeHelperFamily,
    pub signature: RuntimeHelperSignature,
    pub may_allocate: bool,
    pub may_raise: bool,
    pub may_unwind: bool,
    pub is_safepoint: bool,
    pub requires_roots_visible: bool,
    pub required_capability: Option<CapabilityId>,
    pub effect: Option<EffectId>,
    pub gc_behavior: HelperGcBehavior,
    pub jit_call_policy: HelperJitCallPolicy,
    pub source_mapping_policy: HelperSourceMappingPolicy,
}

impl RuntimeHelperDescriptor {
    /// Whether this helper may trigger GC collection per frozen GC metadata rules.
    #[must_use]
    pub fn may_collect(&self) -> bool {
        matches!(
            self.gc_behavior,
            HelperGcBehavior::MayAllocateMayCollect
                | HelperGcBehavior::MayMoveObjects
                | HelperGcBehavior::GcInternal
        )
    }

    /// Whether this helper is callable from JIT per its policy.
    #[must_use]
    pub fn is_jit_callable(&self) -> bool {
        !matches!(
            self.jit_call_policy,
            HelperJitCallPolicy::NotJitCallable | HelperJitCallPolicy::InterpreterOnly
        )
    }
}