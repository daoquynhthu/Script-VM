//! GC metadata ownership runtime structures.
//!
//! Spec: `PHASE-3-GC-METADATA-OWNERSHIP.md`, `PHASE-3-GC-SAFEPOINT-ROOT-MODEL.md`

pub mod frame_map;
pub mod pending_control;
pub mod profile;
pub mod root_location;
pub mod root_map;
pub mod safepoint;
pub mod validate;

pub use frame_map::{
    BindingVisibility, FrameMap, FrameMapTable, RegionStateSchema, VisibleBinding,
};
pub use pending_control::{pending_control_roots, PendingControlRootMetadata};
pub use profile::{CollectionModel, GcProfile};
pub use root_location::RootLocation;
pub use root_map::{RootMap, RootMapTable};
pub use safepoint::{
    SafepointLocation, SafepointOwner, SafepointRecord, SafepointTable,
};
pub use validate::{
    validate_frame_map_layouts, validate_moving_gc_policies, validate_root_map_slots,
    validate_safepoint_frame_map_for_deopt, validate_safepoint_root_link,
    validate_safepoint_table,
};