//! Internal heap object representations.
//!
//! Spec: `PHASE-3-VM-RUNTIME-ROUND1.md` §3–§4

use vm_core::id::{CaseIndex, EnumId};
use vm_core::value::Value;

use crate::readonly::ReadOnlyViewObj;
use crate::value::ValueKey;

/// Object kind for hashability and mutation policy classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HeapObjectKind {
    List,
    Map,
    RecordInstance,
    ReadOnlyView,
    EnumValue,
    Function,
    Module,
    Resource,
}

impl HeapObjectKind {
    #[must_use]
    pub const fn is_mutable_aggregate(self) -> bool {
        matches!(
            self,
            Self::List | Self::Map | Self::RecordInstance
        )
    }

    #[must_use]
    pub const fn is_hashable_immediate(self) -> bool {
        false
    }
}

/// Bootstrap heap object enum (internal layout; not public ABI).
#[derive(Debug, Clone, PartialEq)]
pub enum HeapObject {
    List {
        elements: Vec<Value>,
        readonly: bool,
    },
    Map {
        entries: Vec<(ValueKey, Value)>,
        readonly: bool,
    },
    RecordInstance {
        fields: Vec<Value>,
        readonly: bool,
    },
    ReadOnlyView(ReadOnlyViewObj),
    EnumValue {
        enum_id: EnumId,
        case_index: CaseIndex,
        payload: Option<Value>,
    },
    Function,
    Module,
    Resource,
}

impl HeapObject {
    #[must_use]
    pub const fn kind(&self) -> HeapObjectKind {
        match self {
            Self::List { .. } => HeapObjectKind::List,
            Self::Map { .. } => HeapObjectKind::Map,
            Self::RecordInstance { .. } => HeapObjectKind::RecordInstance,
            Self::ReadOnlyView(_) => HeapObjectKind::ReadOnlyView,
            Self::EnumValue { .. } => HeapObjectKind::EnumValue,
            Self::Function => HeapObjectKind::Function,
            Self::Module => HeapObjectKind::Module,
            Self::Resource => HeapObjectKind::Resource,
        }
    }

    #[must_use]
    pub const fn is_readonly(&self) -> bool {
        match self {
            Self::List { readonly, .. }
            | Self::Map { readonly, .. }
            | Self::RecordInstance { readonly, .. } => *readonly,
            Self::ReadOnlyView(_) => true,
            Self::EnumValue { .. }
            | Self::Function
            | Self::Module
            | Self::Resource => false,
        }
    }
}