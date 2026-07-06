//! Test fixtures for RuntimePlan validation.

use std::collections::BTreeMap;

use vm_diag::source_span::SourceSpanId;

use crate::digest::Digest;
use crate::id::{
    AccessSiteId, BindingId, CallSiteId, DeoptId, EirFunctionId, FieldId, FieldIndex,
    FunctionId, ModuleId, RecordId, SafepointId, ShapeId, SlotId, SlotLayoutId, TypeId,
};
use crate::profile::{RuntimeTargetProfile, Version};
use crate::runtime_plan::schema::{
    CapabilityEnvironmentMutability, CapabilityGatePlan, ExportPlan, FunctionPlan, FunctionPlanTable,
    IdRange, ImportPlan, ModulePlan, ModulePlanTable, Mutability, ParameterLayout,
    RecordFieldShape, RecordShape, RuntimePlan, RuntimePlanSourceMap, RuntimeTypeDesc, ShapePlan,
    SlotDescriptor, SlotKind, SlotLayout, SlotLayoutOwner, SlotLayoutTable, SlotStorage,
    TypeCheckStrategy, TypePlan,
};

/// Build the smallest plan that satisfies frozen validation requirements.
#[must_use]
pub fn minimal_valid_plan() -> RuntimePlan {
    let module_id = ModuleId::new(0);
    let function_id = FunctionId::new(0);
    let init_eir = EirFunctionId::new(0);
    let entry_eir = EirFunctionId::new(1);
    let module_layout = SlotLayoutId::new(0);
    let function_layout = SlotLayoutId::new(1);
    let state_slot = SlotId::new(0);
    let object_slot = SlotId::new(1);
    let local_slot = SlotId::new(2);
    let span = SourceSpanId::new(0);
    let vm_version = Version::new(0, 1, 0);

    let module_layout_entry = SlotLayout {
        slot_layout_id: module_layout,
        owner: SlotLayoutOwner::Module,
        slots: vec![
            SlotDescriptor {
                slot_id: state_slot,
                kind: SlotKind::Module,
                binding_id: None,
                type_hint: None,
                mutability: Some(Mutability::Mutable),
                storage: SlotStorage::RuntimeInternal,
                initialized_at_entry: false,
                source_span: Some(span),
            },
            SlotDescriptor {
                slot_id: object_slot,
                kind: SlotKind::Module,
                binding_id: None,
                type_hint: None,
                mutability: Some(Mutability::Mutable),
                storage: SlotStorage::ModuleRef,
                initialized_at_entry: false,
                source_span: Some(span),
            },
        ],
        hidden_slots: Vec::new(),
        max_slot_count: 4,
    };

    let function_layout_entry = SlotLayout {
        slot_layout_id: function_layout,
        owner: SlotLayoutOwner::Function,
        slots: vec![SlotDescriptor {
            slot_id: local_slot,
            kind: SlotKind::Local,
            binding_id: Some(BindingId::new(0)),
            type_hint: None,
            mutability: Some(Mutability::Mutable),
            storage: SlotStorage::Value,
            initialized_at_entry: true,
            source_span: Some(span),
        }],
        hidden_slots: Vec::new(),
        max_slot_count: 4,
    };

    let mut slot_layouts = BTreeMap::new();
    slot_layouts.insert(module_layout.raw(), module_layout_entry);
    slot_layouts.insert(function_layout.raw(), function_layout_entry);

    let mut modules = BTreeMap::new();
    modules.insert(
        module_id.raw(),
        ModulePlan {
            module_id,
            module_slot_layout: module_layout,
            initialization_function: init_eir,
            import_plan: ImportPlan::default(),
            export_plan: ExportPlan {
                exports: Vec::new(),
                seal_after_init: true,
            },
            module_state_slot: state_slot,
            module_object_slot: object_slot,
            source_order: Vec::new(),
            source_span: Some(span),
        },
    );

    let mut functions = BTreeMap::new();
    functions.insert(
        function_id.raw(),
        FunctionPlan {
            function_id,
            module_id,
            entry_eir_function: entry_eir,
            parameter_layout: ParameterLayout::default(),
            local_slot_layout: function_layout,
            capture_layout: Default::default(),
            default_argument_plan: Default::default(),
            return_type: None,
            effect: None,
            required_capabilities: Vec::new(),
            call_site_range: IdRange::empty(CallSiteId::new(0)),
            access_site_range: IdRange::empty(AccessSiteId::new(0)),
            safepoint_range: IdRange::empty(SafepointId::new(0)),
            deopt_range: IdRange::empty(DeoptId::new(0)),
            source_span: Some(span),
        },
    );

    let field_id = FieldId::new(0);
    let mut field_index_by_id = BTreeMap::new();
    field_index_by_id.insert(field_id.raw(), FieldIndex(0));
    let mut record_shapes = BTreeMap::new();
    record_shapes.insert(
        0,
        RecordShape {
            record_id: RecordId::new(0),
            shape_id: ShapeId::new(0),
            fields: vec![RecordFieldShape {
                field_id,
                field_index: FieldIndex(0),
            }],
            methods: Vec::new(),
            field_index_by_id,
        },
    );

    let mut runtime_types = BTreeMap::new();
    let type_id = TypeId::new(0);
    runtime_types.insert(
        type_id.raw(),
        RuntimeTypeDesc {
            type_id,
            name: "Int".into(),
        },
    );

    RuntimePlan {
        plan_version: Version::new(1, 0, 0),
        source_sir_digest: Digest(0xDEAD_BEEF),
        phase2_schema_version: Version::new(1, 0, 0),
        vm_version,
        target_profile: RuntimeTargetProfile::bootstrap(),
        module_plans: ModulePlanTable { modules },
        function_plans: FunctionPlanTable { functions },
        type_plan: TypePlan {
            runtime_types,
            type_check_strategies: BTreeMap::from([(type_id.raw(), TypeCheckStrategy::Exact)]),
        },
        shape_plan: ShapePlan {
            record_shapes,
            enum_shapes: BTreeMap::new(),
        },
        slot_layouts: SlotLayoutTable { layouts: slot_layouts },
        call_site_table: Default::default(),
        access_site_table: Default::default(),
        safepoint_seed_table: Default::default(),
        deopt_seed_table: Default::default(),
        helper_requirements: Default::default(),
        capability_gate_plan: CapabilityGatePlan {
            gates: Vec::new(),
            environment_digest: None,
            mutability_policy: CapabilityEnvironmentMutability::Immutable,
        },
        source_map: RuntimePlanSourceMap::default(),
        diagnostics: None,
    }
}