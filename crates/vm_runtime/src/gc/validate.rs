//! GC metadata validation.
//!
//! Spec: `PHASE-3-GC-METADATA-OWNERSHIP.md` §8, `PHASE-3-VALIDATION-MATRIX.md` P3-V9

use std::collections::BTreeSet;

use vm_core::error::registry::VmStructuralErrorCode;
use vm_core::id::{SlotId, SlotLayoutId};

use crate::runtime_error::{RuntimeFailure, RuntimeResult};

use super::frame_map::FrameMapTable;
use super::profile::GcProfile;
use super::root_map::RootMapTable;
use super::safepoint::{SafepointRecord, SafepointTable};

fn root_map_error(message: impl Into<String>) -> RuntimeFailure {
    RuntimeFailure::structural(VmStructuralErrorCode::InvalidRootMapError, message)
}

/// Reject safepoint records that may run GC without a resolvable RootMap.
pub fn validate_safepoint_root_link(
    record: &SafepointRecord,
    root_maps: &RootMapTable,
) -> RuntimeResult<()> {
    if record.gc_may_run() && root_maps.get(record.root_map).is_none() {
        return Err(root_map_error(format!(
            "safepoint {} requires RootMap {} when GC may run",
            record.safepoint_id.raw(),
            record.root_map.raw()
        )));
    }
    Ok(())
}

/// Reject RootMap entries referencing unknown slot ids.
pub fn validate_root_map_slots(
    root_maps: &RootMapTable,
    known_slots: &BTreeSet<SlotId>,
) -> RuntimeResult<()> {
    for map in root_maps.maps.values() {
        for location in &map.roots {
            if let Some(slot) = location.referenced_slot() {
                if !known_slots.contains(&slot) {
                    return Err(root_map_error(format!(
                        "RootMap {} references unknown SlotId {}",
                        map.root_map_id.raw(),
                        slot.raw()
                    )));
                }
            }
        }
    }
    Ok(())
}

/// Reject FrameMap entries referencing unknown slot layouts.
pub fn validate_frame_map_layouts(
    frame_maps: &FrameMapTable,
    known_layouts: &BTreeSet<SlotLayoutId>,
) -> RuntimeResult<()> {
    for map in frame_maps.maps.values() {
        if !known_layouts.contains(&map.slot_layout) {
            return Err(RuntimeFailure::structural(
                VmStructuralErrorCode::InvalidFrameStateError,
                format!(
                    "FrameMap {} references unknown SlotLayoutId {}",
                    map.frame_map_id.raw(),
                    map.slot_layout.raw()
                ),
            ));
        }
    }
    Ok(())
}

/// Validate all safepoints in a table.
pub fn validate_safepoint_table(
    table: &SafepointTable,
    root_maps: &RootMapTable,
) -> RuntimeResult<()> {
    for record in table.records.values() {
        validate_safepoint_root_link(record, root_maps)?;
    }
    Ok(())
}

/// Apply moving-GC root policy across all root maps.
pub fn validate_moving_gc_policies(
    profile: &GcProfile,
    root_maps: &RootMapTable,
) -> RuntimeResult<()> {
    for map in root_maps.maps.values() {
        profile.validate_root_map_policy(map)?;
    }
    Ok(())
}

/// Reject deopt-linked safepoints without frame map when required.
pub fn validate_safepoint_frame_map_for_deopt(record: &SafepointRecord) -> RuntimeResult<()> {
    if record.deopt_id.is_some() && record.frame_map.is_none() {
        return Err(RuntimeFailure::structural(
            VmStructuralErrorCode::InvalidDeoptError,
            format!(
                "DeoptPoint at safepoint {} requires FrameMap",
                record.safepoint_id.raw()
            ),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gc::frame_map::FrameMap;
    use crate::gc::root_map::RootMap;
    use crate::gc::root_location::RootLocation;
    use crate::gc::safepoint::{SafepointLocation, SafepointOwner, SafepointRecord};
    use vm_core::eir::schema::RootMapOwner;
    use vm_core::id::{
        EirFunctionId, FrameMapId, ModuleId, RootMapId, SafepointId, SlotId, SlotLayoutId,
    };
    use vm_core::runtime_plan::schema::SafepointKind;

    fn safepoint(root_map: RootMapId) -> SafepointRecord {
        SafepointRecord {
            safepoint_id: SafepointId::new(0),
            kind: SafepointKind::Allocation,
            owner: SafepointOwner::Interpreter,
            location: SafepointLocation::HostBoundary,
            root_map,
            frame_map: None,
            deopt_id: None,
            source_span: None,
        }
    }

    #[test]
    fn safepoint_without_root_map_rejected_when_gc_may_run() {
        let record = safepoint(RootMapId::new(99));
        let root_maps = RootMapTable::new();
        let err = validate_safepoint_root_link(&record, &root_maps).expect_err("missing");
        assert!(matches!(err, RuntimeFailure::Structural(_)));
    }

    #[test]
    fn unknown_slot_root_rejected() {
        let mut root_maps = RootMapTable::new();
        root_maps.maps.insert(
            0,
            RootMap {
                root_map_id: RootMapId::new(0),
                owner: RootMapOwner::InterpreterFrame,
                safepoint_id: None,
                frame_map_id: None,
                roots: vec![RootLocation::Slot(SlotId::new(99))],
                source_span: None,
                updateable: false,
            },
        );
        let known = BTreeSet::from([SlotId::new(0)]);
        let err = validate_root_map_slots(&root_maps, &known).expect_err("unknown slot");
        assert!(matches!(err, RuntimeFailure::Structural(_)));
    }

    #[test]
    fn frame_map_unknown_layout_rejected() {
        let mut frame_maps = FrameMapTable::new();
        frame_maps.maps.insert(
            0,
            FrameMap {
                frame_map_id: FrameMapId::new(0),
                owner_function: EirFunctionId::new(0),
                source_function: None,
                module_id: ModuleId::new(0),
                slot_layout: SlotLayoutId::new(99),
                visible_bindings: vec![],
                region_state_schema: Default::default(),
                source_span: None,
            },
        );
        let known = BTreeSet::from([SlotLayoutId::new(0)]);
        let err = validate_frame_map_layouts(&frame_maps, &known).expect_err("layout");
        assert!(matches!(err, RuntimeFailure::Structural(_)));
    }
}