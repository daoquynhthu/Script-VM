//! EIR test fixtures.

use vm_diag::source_span::SourceSpanId;

use crate::digest::Digest;
use crate::eir::schema::{
    BlockParameter, Branch, ConstantEntry, ConstantOp, EirBlock, EirFunction, EirModule, EirOp,
    EirOpKind, EirTerminator, Jump, OpMetadata, Return, SafepointRecord, StoreField, StoreOp,
};
use crate::eir::schema::{CheckOp, CheckType, GuardFailureAction, GuardKind, GuardOp};
use crate::eir::validate::{EirValidationContext, HelperRegistryView};
#[cfg(test)]
use crate::eir::wire::{EirBlockWire, EirFunctionWire, EirModuleWire, EirOpWire};
use crate::id::{
    AccessSiteId, CallSiteId, ConstantId, EirBlockId, EirFunctionId, FieldId, FieldIndex,
    FrameMapId, FunctionId, ModuleId, RootMapId, RuntimeHelperId, SafepointId, ShapeId, SlotId,
    SlotLayoutId, TypeId,
};
use crate::value::Value;
use crate::profile::Version;
use crate::runtime_plan::schema::SafepointKind;

fn minimal_entry_block(span: SourceSpanId) -> EirBlock {
    let entry = EirBlockId::new(0);
    EirBlock {
        block_id: entry,
        parameters: Vec::new(),
        ops: vec![EirOp {
            metadata: OpMetadata {
                source_span: Some(span),
                ..OpMetadata::default()
            },
            kind: EirOpKind::Constant(ConstantOp {
                dest: SlotId::new(0),
                constant: ConstantId::new(0),
            }),
        }],
        terminator: EirTerminator::Return(Return {
            value: Some(SlotId::new(0)),
        }),
        source_span: Some(span),
    }
}

#[cfg(test)]
fn minimal_entry_block_wire(span: SourceSpanId) -> EirBlockWire {
    let entry = EirBlockId::new(0);
    EirBlockWire {
        block_id: entry,
        parameters: Vec::new(),
        ops: vec![EirOpWire {
            metadata: OpMetadata {
                source_span: Some(span),
                ..OpMetadata::default()
            },
            kind_tag: 0,
            kind: Some(EirOpKind::Constant(ConstantOp {
                dest: SlotId::new(0),
                constant: ConstantId::new(0),
            })),
        }],
        terminator_kind_tag: Some(2),
        terminator: Some(EirTerminator::Return(Return {
            value: Some(SlotId::new(0)),
        })),
        source_span: Some(span),
    }
}

/// Build a minimal valid EIR module for validation tests.
#[must_use]
pub fn minimal_valid_eir_module() -> EirModule {
    let span = SourceSpanId::new(1);
    let entry = EirBlockId::new(0);
    let function = EirFunction {
        eir_function_id: EirFunctionId::new(0),
        function_id: Some(FunctionId::new(0)),
        module_id: ModuleId::new(0),
        entry_block: entry,
        blocks: vec![minimal_entry_block(span)],
        slot_layout: SlotLayoutId::new(0),
        frame_map: FrameMapId::new(0),
        source_span: Some(span),
    };
    let mut constants = crate::eir::schema::ConstantPool::default();
    constants.constants.insert(
        0,
        ConstantEntry {
            constant_id: ConstantId::new(0),
            value: Value::Int(0),
        },
    );
    EirModule {
        eir_version: Version::new(1, 0, 0),
        source_runtime_plan_digest: Digest(0xDEAD_BEEF),
        functions: vec![function],
        constants,
        source_map: Default::default(),
        root_maps: Default::default(),
        safepoints: Default::default(),
        deopt_points: Default::default(),
    }
}

#[cfg(test)]
fn minimal_valid_eir_module_wire() -> EirModuleWire {
    let span = SourceSpanId::new(1);
    let entry = EirBlockId::new(0);
    EirModuleWire {
        eir_version: Version::new(1, 0, 0),
        source_runtime_plan_digest: Digest(0xDEAD_BEEF),
        functions: vec![EirFunctionWire {
            eir_function_id: EirFunctionId::new(0),
            function_id: Some(FunctionId::new(0)),
            module_id: ModuleId::new(0),
            entry_block: entry,
            blocks: vec![minimal_entry_block_wire(span)],
            slot_layout: SlotLayoutId::new(0),
            frame_map: FrameMapId::new(0),
            source_span: Some(span),
        }],
        constants: Default::default(),
        source_map: Default::default(),
        root_maps: Default::default(),
        safepoints: Default::default(),
        deopt_points: Default::default(),
    }
}

/// Validation context aligned with `minimal_valid_eir_module`.
#[must_use]
pub fn minimal_eir_validation_context() -> EirValidationContext {
    EirValidationContext {
        slot_ids: [SlotId::new(0)].into_iter().collect(),
        helper_registry: HelperRegistryView::from_ids([RuntimeHelperId::new(0)]),
        expected_runtime_plan_digest: Digest(0xDEAD_BEEF),
        call_site_ids: [CallSiteId::new(0)].into_iter().collect(),
        access_site_ids: Default::default(),
        safepoint_ids: Default::default(),
        deopt_ids: Default::default(),
        type_ids: Default::default(),
        shape_ids: Default::default(),
        field_ids: Default::default(),
        case_ids: Default::default(),
        requires_write_barrier: false,
        barrier_access_site_ids: Default::default(),
        gc_may_run: false,
    }
}

/// Module with a branch to a non-existent block.
#[must_use]
pub fn eir_module_with_invalid_block_graph() -> EirModule {
    let mut module = minimal_valid_eir_module();
    let missing = EirBlockId::new(99);
    if let Some(function) = module.functions.first_mut() {
        if let Some(block) = function.blocks.first_mut() {
            block.terminator = EirTerminator::Branch(Branch {
                condition: SlotId::new(0),
                then_block: missing,
                else_block: function.entry_block,
            });
        }
    }
    module
}

/// Wire module whose block has no terminator.
#[cfg(test)]
#[must_use]
pub(crate) fn eir_module_wire_with_missing_terminator() -> EirModuleWire {
    let mut wire = minimal_valid_eir_module_wire();
    if let Some(function) = wire.functions.first_mut() {
        if let Some(block) = function.blocks.first_mut() {
            block.terminator_kind_tag = None;
            block.terminator = None;
        }
    }
    wire
}

/// Wire module containing an unknown op kind tag.
#[cfg(test)]
#[must_use]
pub(crate) fn eir_module_wire_with_unknown_op_kind() -> EirModuleWire {
    let mut wire = minimal_valid_eir_module_wire();
    if let Some(function) = wire.functions.first_mut() {
        if let Some(block) = function.blocks.first_mut() {
            block.ops.push(EirOpWire {
                metadata: OpMetadata::default(),
                kind_tag: 255,
                kind: None,
            });
        }
    }
    wire
}

/// Wire module with an unknown terminator kind tag.
#[cfg(test)]
#[must_use]
pub(crate) fn eir_module_wire_with_unknown_terminator_kind() -> EirModuleWire {
    let mut wire = minimal_valid_eir_module_wire();
    if let Some(function) = wire.functions.first_mut() {
        if let Some(block) = function.blocks.first_mut() {
            block.terminator_kind_tag = Some(200);
            block.terminator = None;
        }
    }
    wire
}

/// Module with a RuntimeHelperOp referencing an unknown helper.
#[must_use]
pub fn eir_module_with_invalid_helper() -> EirModule {
    let span = SourceSpanId::new(1);
    let mut module = minimal_valid_eir_module();
    if let Some(function) = module.functions.first_mut() {
        if let Some(block) = function.blocks.first_mut() {
            block.ops.push(EirOp {
                metadata: OpMetadata {
                    source_span: Some(span),
                    ..OpMetadata::default()
                },
                kind: EirOpKind::RuntimeHelper(crate::eir::schema::RuntimeHelperOp {
                    dest: None,
                    helper_id: RuntimeHelperId::new(99),
                    args: vec![SlotId::new(0)],
                    call_site: None,
                    access_site: None,
                    safepoint_id: None,
                    deopt_id: None,
                }),
            });
        }
    }
    module
}

/// Module with `helper_write_barrier` (id 1) and no source mapping metadata.
#[must_use]
pub fn eir_module_with_write_barrier_without_source() -> EirModule {
    let mut module = minimal_valid_eir_module();
    if let Some(function) = module.functions.first_mut() {
        if let Some(block) = function.blocks.first_mut() {
            block.ops.push(EirOp {
                metadata: OpMetadata::default(),
                kind: EirOpKind::RuntimeHelper(crate::eir::schema::RuntimeHelperOp {
                    dest: None,
                    helper_id: RuntimeHelperId::new(1),
                    args: vec![SlotId::new(0)],
                    call_site: None,
                    access_site: None,
                    safepoint_id: None,
                    deopt_id: None,
                }),
            });
        }
    }
    module
}

/// Module with a may-collect helper at a safepoint whose RootMap is missing from module metadata.
#[must_use]
pub fn eir_module_with_may_collect_without_root_map() -> EirModule {
    let span = SourceSpanId::new(1);
    let safepoint = SafepointId::new(0);
    let helper = RuntimeHelperId::new(0);
    let mut module = minimal_valid_eir_module();
    module.safepoints.records.insert(
        0,
        SafepointRecord {
            safepoint_id: safepoint,
            kind: SafepointKind::HelperCall,
            root_map: RootMapId::new(99),
            frame_map: None,
            source_span: Some(span),
        },
    );
    if let Some(function) = module.functions.first_mut() {
        if let Some(block) = function.blocks.first_mut() {
            block.ops.push(EirOp {
                metadata: OpMetadata {
                    source_span: Some(span),
                    ..OpMetadata::default()
                },
                kind: EirOpKind::RuntimeHelper(crate::eir::schema::RuntimeHelperOp {
                    dest: None,
                    helper_id: helper,
                    args: vec![SlotId::new(0)],
                    call_site: None,
                    access_site: None,
                    safepoint_id: Some(safepoint),
                    deopt_id: None,
                }),
            });
        }
    }
    module
}

/// Context where GC may run and helper registry marks helper 0 as may-collect.
#[must_use]
pub fn may_collect_validation_context() -> EirValidationContext {
    let safepoint = SafepointId::new(0);
    let helper = RuntimeHelperId::new(0);
    let mut ctx = minimal_eir_validation_context();
    ctx.gc_may_run = true;
    ctx.safepoint_ids.insert(safepoint);
    ctx.helper_registry = HelperRegistryView::from_ids([helper]).with_may_collect([helper]);
    ctx
}

/// Module with a may-raise CheckOp missing source mapping.
#[must_use]
pub fn eir_module_with_unmapped_raise_op() -> EirModule {
    use crate::eir::schema::{CheckBool, CheckOp};
    use crate::error::registry::RuntimeErrorCode;

    let mut module = minimal_valid_eir_module();
    if let Some(function) = module.functions.first_mut() {
        if let Some(block) = function.blocks.first_mut() {
            block.ops.push(EirOp {
                metadata: OpMetadata::default(),
                kind: EirOpKind::Check(CheckOp::Bool(CheckBool {
                    operand: SlotId::new(0),
                    failure_error: RuntimeErrorCode::TypeError,
                })),
            });
        }
    }
    module
}

/// Module referencing a ConstantId absent from the constant pool.
#[must_use]
pub fn eir_module_with_unknown_constant() -> EirModule {
    let span = SourceSpanId::new(1);
    let mut module = minimal_valid_eir_module();
    if let Some(function) = module.functions.first_mut() {
        if let Some(block) = function.blocks.first_mut() {
            block.ops[0] = EirOp {
                metadata: OpMetadata {
                    source_span: Some(span),
                    ..OpMetadata::default()
                },
                kind: EirOpKind::Constant(ConstantOp {
                    dest: SlotId::new(0),
                    constant: ConstantId::new(99),
                }),
            };
        }
    }
    module
}

/// Module with Jump args count mismatched against target block parameters.
#[must_use]
pub fn eir_module_with_invalid_block_argument_count() -> EirModule {
    let span = SourceSpanId::new(1);
    let entry = EirBlockId::new(0);
    let target = EirBlockId::new(1);
    let entry_block = minimal_entry_block(span);
    let target_block = EirBlock {
        block_id: target,
        parameters: vec![BlockParameter {
            slot: SlotId::new(0),
        }],
        ops: vec![EirOp {
            metadata: OpMetadata {
                source_span: Some(span),
                ..OpMetadata::default()
            },
            kind: EirOpKind::Constant(ConstantOp {
                dest: SlotId::new(0),
                constant: ConstantId::new(0),
            }),
        }],
        terminator: EirTerminator::Return(Return {
            value: Some(SlotId::new(0)),
        }),
        source_span: Some(span),
    };
    let mut module = minimal_valid_eir_module();
    if let Some(function) = module.functions.first_mut() {
        function.blocks = vec![
            EirBlock {
                block_id: entry,
                parameters: Vec::new(),
                ops: entry_block.ops,
                terminator: EirTerminator::Jump(Jump {
                    target,
                    args: Vec::new(),
                }),
                source_span: Some(span),
            },
            target_block,
        ];
    }
    module
}

/// Module with heap StoreField when write barrier is required but access site lacks policy.
#[must_use]
pub fn eir_module_with_heap_store_without_barrier() -> EirModule {
    let span = SourceSpanId::new(1);
    let access_site = AccessSiteId::new(0);
    let shape = ShapeId::new(0);
    let field = FieldId::new(0);
    let mut module = minimal_valid_eir_module();
    if let Some(function) = module.functions.first_mut() {
        if let Some(block) = function.blocks.first_mut() {
            block.ops.push(EirOp {
                metadata: OpMetadata {
                    source_span: Some(span),
                    ..OpMetadata::default()
                },
                kind: EirOpKind::Store(StoreOp::Field(StoreField {
                    receiver: SlotId::new(0),
                    value: SlotId::new(0),
                    record_shape: shape,
                    field_id: field,
                    field_index: FieldIndex(0),
                    access_site_id: access_site,
                    check_readonly: false,
                    check_mutability: false,
                    type_check: None,
                })),
            });
        }
    }
    module
}

/// Context requiring write barriers with shape/field/access-site bindings.
#[must_use]
pub fn barrier_required_validation_context() -> EirValidationContext {
    let mut ctx = minimal_eir_validation_context();
    ctx.access_site_ids.insert(AccessSiteId::new(0));
    ctx.shape_ids.insert(ShapeId::new(0));
    ctx.field_ids.insert(FieldId::new(0));
    ctx.requires_write_barrier = true;
    ctx
}

/// Module with CheckType referencing unknown TypeId.
#[must_use]
pub fn eir_module_with_unknown_type_id() -> EirModule {
    let span = SourceSpanId::new(1);
    let mut module = minimal_valid_eir_module();
    if let Some(function) = module.functions.first_mut() {
        if let Some(block) = function.blocks.first_mut() {
            block.ops.push(EirOp {
                metadata: OpMetadata {
                    source_span: Some(span),
                    ..OpMetadata::default()
                },
                kind: EirOpKind::Check(CheckOp::Type(CheckType {
                    operand: SlotId::new(0),
                    expected_type: TypeId::new(99),
                    failure_error: crate::error::registry::RuntimeErrorCode::TypeError,
                })),
            });
        }
    }
    module
}

/// Validation context with type 0 known.
#[must_use]
pub fn type_bound_validation_context() -> EirValidationContext {
    let mut ctx = minimal_eir_validation_context();
    ctx.type_ids.insert(TypeId::new(0));
    ctx
}

/// Module whose RuntimePlan digest does not match the validation context.
#[must_use]
pub fn eir_module_with_digest_mismatch() -> EirModule {
    let mut module = minimal_valid_eir_module();
    module.source_runtime_plan_digest = Digest(0xBAD_CAFE);
    module
}

/// Module referencing an unknown SlotId.
#[must_use]
pub fn eir_module_with_unknown_slot() -> EirModule {
    let span = SourceSpanId::new(1);
    let mut module = minimal_valid_eir_module();
    if let Some(function) = module.functions.first_mut() {
        if let Some(block) = function.blocks.first_mut() {
            block.ops[0] = EirOp {
                metadata: OpMetadata {
                    source_span: Some(span),
                    ..OpMetadata::default()
                },
                kind: EirOpKind::Constant(ConstantOp {
                    dest: SlotId::new(99),
                    constant: ConstantId::new(0),
                }),
            };
        }
    }
    module
}

/// Module with GuardOp Raise failure lacking failure action metadata.
#[must_use]
pub fn eir_module_with_guard_without_failure_action() -> EirModule {
    let span = SourceSpanId::new(1);
    let mut module = minimal_valid_eir_module();
    if let Some(function) = module.functions.first_mut() {
        if let Some(block) = function.blocks.first_mut() {
            block.ops.push(EirOp {
                metadata: OpMetadata {
                    source_span: Some(span),
                    ..OpMetadata::default()
                },
                kind: EirOpKind::Guard(GuardOp {
                    guard_kind: GuardKind::Type,
                    inputs: vec![SlotId::new(0)],
                    on_failure: GuardFailureAction::Raise,
                    deopt_id: None,
                    helper_id: None,
                    failure_error: None,
                }),
            });
        }
    }
    module
}

/// Module whose entry block id is not present in blocks list.
#[must_use]
pub fn eir_module_with_invalid_entry_block() -> EirModule {
    let mut module = minimal_valid_eir_module();
    if let Some(function) = module.functions.first_mut() {
        function.entry_block = EirBlockId::new(99);
    }
    module
}