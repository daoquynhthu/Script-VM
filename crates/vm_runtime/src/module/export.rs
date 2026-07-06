//! Export table representation and sealing.
//!
//! Spec: `PHASE-3-MODULE-RUNTIME-CONTRACT.md` §8

use std::collections::BTreeMap;

use vm_core::error::registry::VmStructuralErrorCode;
use vm_core::id::{BindingId, SlotId, TypeId};
use vm_core::runtime_plan::schema::{ExportPlan, ExportPlanEntry};
use vm_diag::source_span::SourceSpanId;

use crate::runtime_error::{RuntimeFailure, RuntimeResult};

/// Single exported binding entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportEntry {
    pub name: String,
    pub binding_id: BindingId,
    pub slot_id: SlotId,
    pub initialized: bool,
    pub type_id: Option<TypeId>,
    pub source_span: SourceSpanId,
}

impl ExportEntry {
    #[must_use]
    pub fn from_plan(entry: &ExportPlanEntry) -> Self {
        Self {
            name: entry.exported_name.clone(),
            binding_id: entry.binding_id,
            slot_id: entry.slot_id,
            initialized: false,
            type_id: entry.interface_type,
            source_span: entry.source_span,
        }
    }
}

/// Module export table with seal-after-init semantics.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ExportTable {
    entries: BTreeMap<String, ExportEntry>,
    sealed: bool,
}

impl ExportTable {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn is_sealed(&self) -> bool {
        self.sealed
    }

    pub fn from_export_plan(plan: &ExportPlan) -> RuntimeResult<Self> {
        let mut table = Self::new();
        for entry in &plan.exports {
            table.insert_entry(ExportEntry::from_plan(entry))?;
        }
        Ok(table)
    }

    pub fn insert_entry(&mut self, entry: ExportEntry) -> RuntimeResult<()> {
        self.reject_if_sealed("insert export entry")?;
        if self.entries.contains_key(&entry.name) {
            return Err(RuntimeFailure::structural(
                VmStructuralErrorCode::InvalidRuntimePlanError,
                format!("duplicate export name: {}", entry.name),
            ));
        }
        self.entries.insert(entry.name.clone(), entry);
        Ok(())
    }

    pub fn mark_initialized(&mut self, name: &str) -> RuntimeResult<()> {
        self.reject_if_sealed("mark export initialized")?;
        let entry = self.entries.get_mut(name).ok_or_else(|| {
            RuntimeFailure::language(vm_core::error::registry::RuntimeErrorCode::ImportError)
        })?;
        entry.initialized = true;
        Ok(())
    }

    pub fn seal(&mut self) -> RuntimeResult<()> {
        if self.sealed {
            return Err(RuntimeFailure::structural(
                VmStructuralErrorCode::InvalidRuntimePlanError,
                "export table already sealed",
            ));
        }
        self.sealed = true;
        Ok(())
    }

    #[must_use]
    pub fn get(&self, name: &str) -> Option<&ExportEntry> {
        self.entries.get(name)
    }

    pub fn entries(&self) -> impl Iterator<Item = (&str, &ExportEntry)> {
        self.entries.iter().map(|(k, v)| (k.as_str(), v))
    }

    fn reject_if_sealed(&self, action: &str) -> RuntimeResult<()> {
        if self.sealed {
            Err(RuntimeFailure::structural(
                VmStructuralErrorCode::InvalidRuntimePlanError,
                format!("export table mutation after sealing: {action}"),
            ))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vm_core::id::BindingId;

    fn sample_entry(name: &str, slot: u32) -> ExportEntry {
        ExportEntry {
            name: name.to_string(),
            binding_id: BindingId::new(0),
            slot_id: SlotId::new(slot),
            initialized: false,
            type_id: None,
            source_span: SourceSpanId::new(0),
        }
    }

    #[test]
    fn duplicate_export_name_rejected() {
        let mut table = ExportTable::new();
        table
            .insert_entry(sample_entry("foo", 0))
            .expect("first insert");
        let err = table
            .insert_entry(sample_entry("foo", 1))
            .expect_err("duplicate");
        assert!(matches!(err, RuntimeFailure::Structural(_)));
    }

    #[test]
    fn mutation_after_seal_rejected() {
        let mut table = ExportTable::new();
        table
            .insert_entry(sample_entry("x", 0))
            .expect("insert");
        table.seal().expect("seal");
        let err = table
            .insert_entry(sample_entry("y", 1))
            .expect_err("sealed");
        assert!(matches!(err, RuntimeFailure::Structural(_)));
    }

    #[test]
    fn seal_marks_table_immutable() {
        let mut table = ExportTable::new();
        table
            .insert_entry(sample_entry("a", 0))
            .expect("insert");
        assert!(!table.is_sealed());
        table.seal().expect("seal");
        assert!(table.is_sealed());
    }
}