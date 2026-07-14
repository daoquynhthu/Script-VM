//! EIR modules for interpreter execution tests.

use vm_core::digest::Digest;
use vm_core::eir::schema::{
    BinaryOp, BinaryOperator, BlockParameter, Branch, ConstantEntry, ConstantOp, EirBlock,
    EirFunction, EirModule, EirOp, EirOpKind, EirTerminator, LoadOp, LoadSlot, LoopBackedge,
    OpMetadata, Return, RuntimeHelperOp, StoreOp, StoreSlot,
};

use vm_core::id::{
    ConstantId, EirBlockId, EirFunctionId, FrameMapId, FunctionId, ModuleId, RuntimeHelperId,
    SafepointId, SlotId, SlotLayoutId,
};
use vm_core::profile::Version;
use vm_core::runtime_plan::schema::SafepointKind;
use vm_core::value::Value;
use vm_diag::source_span::SourceSpanId;
use vm_runtime::helpers::dispatch::{
    HELPER_ALLOC_OBJECT_ID, HELPER_GENERIC_CALL_ID, HELPER_PERFORM_UNWIND_ID,
};

/// Out-of-range helper id (all 47 registry helpers are dispatched).
pub const UNDISPATCHED_HELPER_ID: RuntimeHelperId = RuntimeHelperId::new(99);

fn const_op(dest: SlotId, id: u32, span: SourceSpanId) -> EirOp {
    EirOp {
        metadata: OpMetadata {
            source_span: Some(span),
            ..OpMetadata::default()
        },
        kind: EirOpKind::Constant(ConstantOp {
            dest,
            constant: ConstantId::new(id),
        }),
    }
}

fn return_slot(slot: SlotId) -> EirTerminator {
    EirTerminator::Return(Return {
        value: Some(slot),
    })
}

/// `return 42` — literal execution fixture.
#[must_use]
pub fn literal_return_module() -> EirModule {
    let span = SourceSpanId::new(1);
    let function = EirFunction {
        eir_function_id: EirFunctionId::new(0),
        function_id: Some(FunctionId::new(0)),
        module_id: ModuleId::new(0),
        entry_block: EirBlockId::new(0),
        blocks: vec![EirBlock {
            block_id: EirBlockId::new(0),
            parameters: vec![],
            ops: vec![const_op(SlotId::new(0), 0, span)],
            terminator: return_slot(SlotId::new(0)),
            source_span: Some(span),
        }],
        slot_layout: SlotLayoutId::new(0),
        frame_map: FrameMapId::new(0),
        source_span: Some(span),
    };
    let mut constants = vm_core::eir::schema::ConstantPool::default();
    constants.constants.insert(
        0,
        ConstantEntry {
            constant_id: ConstantId::new(0),
            value: Value::Int(42),
        },
    );
    base_module(function, constants)
}

/// Slot load/store round-trip fixture.
#[must_use]
pub fn slot_copy_module() -> EirModule {
    let span = SourceSpanId::new(2);
    let function = EirFunction {
        eir_function_id: EirFunctionId::new(0),
        function_id: Some(FunctionId::new(0)),
        module_id: ModuleId::new(0),
        entry_block: EirBlockId::new(0),
        blocks: vec![EirBlock {
            block_id: EirBlockId::new(0),
            parameters: vec![],
            ops: vec![
                const_op(SlotId::new(0), 0, span),
                EirOp {
                    metadata: OpMetadata::default(),
                    kind: EirOpKind::Store(StoreOp::Slot(StoreSlot {
                        dest: SlotId::new(1),
                        value: SlotId::new(0),
                        check_initialized: None,
                    })),
                },
                EirOp {
                    metadata: OpMetadata::default(),
                    kind: EirOpKind::Load(LoadOp::Slot(LoadSlot {
                        dest: SlotId::new(2),
                        source: SlotId::new(1),
                        require_initialized: true,
                    })),
                },
            ],
            terminator: return_slot(SlotId::new(2)),
            source_span: Some(span),
        }],
        slot_layout: SlotLayoutId::new(0),
        frame_map: FrameMapId::new(0),
        source_span: Some(span),
    };
    let mut constants = vm_core::eir::schema::ConstantPool::default();
    constants.constants.insert(
        0,
        ConstantEntry {
            constant_id: ConstantId::new(0),
            value: Value::Int(7),
        },
    );
    base_module(function, constants)
}

/// Branch on bool constant — then-path returns 1.
#[must_use]
pub fn branch_true_module() -> EirModule {
    let span = SourceSpanId::new(3);
    let entry = EirBlockId::new(0);
    let then_block = EirBlockId::new(1);
    let else_block = EirBlockId::new(2);
    let function = EirFunction {
        eir_function_id: EirFunctionId::new(0),
        function_id: Some(FunctionId::new(0)),
        module_id: ModuleId::new(0),
        entry_block: entry,
        blocks: vec![
            EirBlock {
                block_id: entry,
                parameters: vec![],
                ops: vec![const_op(SlotId::new(0), 0, span)],
                terminator: EirTerminator::Branch(Branch {
                    condition: SlotId::new(0),
                    then_block,
                    else_block,
                }),
                source_span: Some(span),
            },
            EirBlock {
                block_id: then_block,
                parameters: vec![],
                ops: vec![const_op(SlotId::new(1), 1, span)],
                terminator: return_slot(SlotId::new(1)),
                source_span: Some(span),
            },
            EirBlock {
                block_id: else_block,
                parameters: vec![],
                ops: vec![const_op(SlotId::new(1), 2, span)],
                terminator: return_slot(SlotId::new(1)),
                source_span: Some(span),
            },
        ],
        slot_layout: SlotLayoutId::new(0),
        frame_map: FrameMapId::new(0),
        source_span: Some(span),
    };
    let mut constants = vm_core::eir::schema::ConstantPool::default();
    constants.constants.insert(
        0,
        ConstantEntry {
            constant_id: ConstantId::new(0),
            value: Value::Bool(true),
        },
    );
    constants.constants.insert(
        1,
        ConstantEntry {
            constant_id: ConstantId::new(1),
            value: Value::Int(1),
        },
    );
    constants.constants.insert(
        2,
        ConstantEntry {
            constant_id: ConstantId::new(2),
            value: Value::Int(0),
        },
    );
    base_module(function, constants)
}

/// Branch with non-bool condition — negative test fixture.
#[must_use]
pub fn branch_non_bool_module() -> EirModule {
    let mut module = branch_true_module();
    module.constants.constants.insert(
        0,
        ConstantEntry {
            constant_id: ConstantId::new(0),
            value: Value::Int(1),
        },
    );
    module
}

/// Binary add: `return 3 + 4`.
#[must_use]
pub fn binary_add_module() -> EirModule {
    let span = SourceSpanId::new(4);
    let function = EirFunction {
        eir_function_id: EirFunctionId::new(0),
        function_id: Some(FunctionId::new(0)),
        module_id: ModuleId::new(0),
        entry_block: EirBlockId::new(0),
        blocks: vec![EirBlock {
            block_id: EirBlockId::new(0),
            parameters: vec![],
            ops: vec![
                const_op(SlotId::new(0), 0, span),
                const_op(SlotId::new(1), 1, span),
                EirOp {
                    metadata: OpMetadata::default(),
                    kind: EirOpKind::Binary(BinaryOp {
                        dest: SlotId::new(2),
                        op: BinaryOperator::Add,
                        left: SlotId::new(0),
                        right: SlotId::new(1),
                        overflow_policy: None,
                    }),
                },
            ],
            terminator: return_slot(SlotId::new(2)),
            source_span: Some(span),
        }],
        slot_layout: SlotLayoutId::new(0),
        frame_map: FrameMapId::new(0),
        source_span: Some(span),
    };
    let mut constants = vm_core::eir::schema::ConstantPool::default();
    constants.constants.insert(
        0,
        ConstantEntry {
            constant_id: ConstantId::new(0),
            value: Value::Int(3),
        },
    );
    constants.constants.insert(
        1,
        ConstantEntry {
            constant_id: ConstantId::new(1),
            value: Value::Int(4),
        },
    );
    base_module(function, constants)
}

/// Loop backedge safepoint hook fixture (one backedge then exit).
#[must_use]
pub fn loop_backedge_module() -> EirModule {
    let span = SourceSpanId::new(5);
    let init = EirBlockId::new(0);
    let header = EirBlockId::new(1);
    let body = EirBlockId::new(2);
    let exit = EirBlockId::new(3);
    let function = EirFunction {
        eir_function_id: EirFunctionId::new(0),
        function_id: Some(FunctionId::new(0)),
        module_id: ModuleId::new(0),
        entry_block: init,
        blocks: vec![
            EirBlock {
                block_id: init,
                parameters: vec![],
                ops: vec![const_op(SlotId::new(0), 0, span)],
                terminator: EirTerminator::Jump(vm_core::eir::schema::Jump {
                    target: header,
                    args: vec![SlotId::new(0)],
                }),
                source_span: Some(span),
            },
            EirBlock {
                block_id: header,
                parameters: vec![BlockParameter {
                    slot: SlotId::new(0),
                }],
                ops: vec![],
                terminator: EirTerminator::Branch(Branch {
                    condition: SlotId::new(0),
                    then_block: body,
                    else_block: exit,
                }),
                source_span: Some(span),
            },
            EirBlock {
                block_id: body,
                parameters: vec![],
                ops: vec![const_op(SlotId::new(1), 1, span)],
                terminator: EirTerminator::LoopBackedge(LoopBackedge {
                    target: header,
                    args: vec![SlotId::new(1)],
                    safepoint_id: SafepointId::new(0),
                }),
                source_span: Some(span),
            },
            EirBlock {
                block_id: exit,
                parameters: vec![],
                ops: vec![const_op(SlotId::new(2), 2, span)],
                terminator: return_slot(SlotId::new(2)),
                source_span: Some(span),
            },
        ],
        slot_layout: SlotLayoutId::new(0),
        frame_map: FrameMapId::new(0),
        source_span: Some(span),
    };
    let mut constants = vm_core::eir::schema::ConstantPool::default();
    constants.constants.insert(
        0,
        ConstantEntry {
            constant_id: ConstantId::new(0),
            value: Value::Bool(true),
        },
    );
    constants.constants.insert(
        1,
        ConstantEntry {
            constant_id: ConstantId::new(1),
            value: Value::Bool(false),
        },
    );
    constants.constants.insert(
        2,
        ConstantEntry {
            constant_id: ConstantId::new(2),
            value: Value::Int(99),
        },
    );
    let mut module = base_module(function, constants);
    module.safepoints.records.insert(
        0,
        vm_core::eir::schema::SafepointRecord {
            safepoint_id: SafepointId::new(0),
            kind: SafepointKind::LoopBackedge,
            root_map: vm_core::id::RootMapId::new(0),
            frame_map: None,
            source_span: Some(span),
        },
    );
    module
}

/// Undispatched helper — error propagation fixture.
#[must_use]
pub fn undispatched_helper_module() -> EirModule {
    let span = SourceSpanId::new(6);
    let function = EirFunction {
        eir_function_id: EirFunctionId::new(0),
        function_id: Some(FunctionId::new(0)),
        module_id: ModuleId::new(0),
        entry_block: EirBlockId::new(0),
        blocks: vec![EirBlock {
            block_id: EirBlockId::new(0),
            parameters: vec![],
            ops: vec![EirOp {
                metadata: OpMetadata::default(),
                kind: EirOpKind::RuntimeHelper(RuntimeHelperOp {
                    dest: None,
                    helper_id: UNDISPATCHED_HELPER_ID,
                    args: vec![],
                    call_site: None,
                    access_site: None,
                    safepoint_id: None,
                    deopt_id: None,
                }),
            }],
            terminator: return_slot(SlotId::new(0)),
            source_span: Some(span),
        }],
        slot_layout: SlotLayoutId::new(0),
        frame_map: FrameMapId::new(0),
        source_span: Some(span),
    };
    base_module(function, Default::default())
}

/// Raise terminator with valid Error value.
#[must_use]
pub fn raise_error_module(error_value: Value) -> EirModule {
    let span = SourceSpanId::new(7);
    let function = EirFunction {
        eir_function_id: EirFunctionId::new(0),
        function_id: Some(FunctionId::new(0)),
        module_id: ModuleId::new(0),
        entry_block: EirBlockId::new(0),
        blocks: vec![EirBlock {
            block_id: EirBlockId::new(0),
            parameters: vec![],
            ops: vec![const_op(SlotId::new(0), 0, span)],
            terminator: EirTerminator::Raise(vm_core::eir::schema::Raise {
                error: SlotId::new(0),
            }),
            source_span: Some(span),
        }],
        slot_layout: SlotLayoutId::new(0),
        frame_map: FrameMapId::new(0),
        source_span: Some(span),
    };
    let mut constants = vm_core::eir::schema::ConstantPool::default();
    constants.constants.insert(
        0,
        ConstantEntry {
            constant_id: ConstantId::new(0),
            value: error_value,
        },
    );
    base_module(function, constants)
}

/// Helper alloc_object dispatch fixture (Milestone H1).
#[must_use]
pub fn helper_alloc_object_module() -> EirModule {
    let span = SourceSpanId::new(9);
    let function = EirFunction {
        eir_function_id: EirFunctionId::new(0),
        function_id: Some(FunctionId::new(0)),
        module_id: ModuleId::new(0),
        entry_block: EirBlockId::new(0),
        blocks: vec![EirBlock {
            block_id: EirBlockId::new(0),
            parameters: vec![],
            ops: vec![EirOp {
                metadata: OpMetadata::default(),
                kind: EirOpKind::RuntimeHelper(RuntimeHelperOp {
                    dest: Some(SlotId::new(0)),
                    helper_id: HELPER_ALLOC_OBJECT_ID,
                    args: vec![],
                    call_site: None,
                    access_site: None,
                    safepoint_id: None,
                    deopt_id: None,
                }),
            }],
            terminator: return_slot(SlotId::new(0)),
            source_span: Some(span),
        }],
        slot_layout: SlotLayoutId::new(0),
        frame_map: FrameMapId::new(0),
        source_span: Some(span),
    };
    base_module(function, Default::default())
}

/// Helper perform_unwind dispatch fixture.
#[must_use]
pub fn helper_perform_unwind_module() -> EirModule {
    let span = SourceSpanId::new(8);
    let function = EirFunction {
        eir_function_id: EirFunctionId::new(0),
        function_id: Some(FunctionId::new(0)),
        module_id: ModuleId::new(0),
        entry_block: EirBlockId::new(0),
        blocks: vec![EirBlock {
            block_id: EirBlockId::new(0),
            parameters: vec![],
            ops: vec![EirOp {
                metadata: OpMetadata::default(),
                kind: EirOpKind::RuntimeHelper(RuntimeHelperOp {
                    dest: None,
                    helper_id: HELPER_PERFORM_UNWIND_ID,
                    args: vec![],
                    call_site: None,
                    access_site: None,
                    safepoint_id: None,
                    deopt_id: None,
                }),
            }],
            terminator: EirTerminator::Return(Return { value: None }),
            source_span: Some(span),
        }],
        slot_layout: SlotLayoutId::new(0),
        frame_map: FrameMapId::new(0),
        source_span: Some(span),
    };
    base_module(function, Default::default())
}

/// Nested generic_call: caller invokes callee that returns its first arg.
/// Caller: const 7 → slot1, const ObjectRef callee → slot0, generic_call(slot0, slot1) → slot2, return slot2.
/// Callee (eir id 1): return slot0.
#[must_use]
pub fn generic_call_nested_module(callee_object: Value) -> EirModule {
    let span = SourceSpanId::new(10);
    let caller = EirFunction {
        eir_function_id: EirFunctionId::new(0),
        function_id: Some(FunctionId::new(0)),
        module_id: ModuleId::new(0),
        entry_block: EirBlockId::new(0),
        blocks: vec![EirBlock {
            block_id: EirBlockId::new(0),
            parameters: vec![],
            ops: vec![
                const_op(SlotId::new(0), 0, span), // callee
                const_op(SlotId::new(1), 1, span), // arg 7
                EirOp {
                    metadata: OpMetadata::default(),
                    kind: EirOpKind::RuntimeHelper(RuntimeHelperOp {
                        dest: Some(SlotId::new(2)),
                        helper_id: HELPER_GENERIC_CALL_ID,
                        args: vec![SlotId::new(0), SlotId::new(1)],
                        call_site: None,
                        access_site: None,
                        safepoint_id: None,
                        deopt_id: None,
                    }),
                },
            ],
            terminator: return_slot(SlotId::new(2)),
            source_span: Some(span),
        }],
        slot_layout: SlotLayoutId::new(0),
        frame_map: FrameMapId::new(0),
        source_span: Some(span),
    };
    let callee = EirFunction {
        eir_function_id: EirFunctionId::new(1),
        function_id: Some(FunctionId::new(1)),
        module_id: ModuleId::new(0),
        entry_block: EirBlockId::new(0),
        blocks: vec![EirBlock {
            block_id: EirBlockId::new(0),
            parameters: vec![],
            ops: vec![],
            terminator: return_slot(SlotId::new(0)),
            source_span: Some(span),
        }],
        slot_layout: SlotLayoutId::new(0),
        frame_map: FrameMapId::new(0),
        source_span: Some(span),
    };
    let mut constants = vm_core::eir::schema::ConstantPool::default();
    constants.constants.insert(
        0,
        ConstantEntry {
            constant_id: ConstantId::new(0),
            value: callee_object,
        },
    );
    constants.constants.insert(
        1,
        ConstantEntry {
            constant_id: ConstantId::new(1),
            value: Value::Int(7),
        },
    );
    EirModule {
        eir_version: Version::new(1, 0, 0),
        source_runtime_plan_digest: Digest(0xCA11_B0D1),
        functions: vec![caller, callee],
        constants,
        source_map: Default::default(),
        root_maps: Default::default(),
        safepoints: Default::default(),
        deopt_points: Default::default(),
    }
}

/// Module init body fixture: returns constant 99.
#[must_use]
pub fn module_init_body_module() -> EirModule {
    let span = SourceSpanId::new(11);
    let function = EirFunction {
        eir_function_id: EirFunctionId::new(0),
        function_id: Some(FunctionId::new(0)),
        module_id: ModuleId::new(0),
        entry_block: EirBlockId::new(0),
        blocks: vec![EirBlock {
            block_id: EirBlockId::new(0),
            parameters: vec![],
            ops: vec![const_op(SlotId::new(0), 0, span)],
            terminator: return_slot(SlotId::new(0)),
            source_span: Some(span),
        }],
        slot_layout: SlotLayoutId::new(0),
        frame_map: FrameMapId::new(0),
        source_span: Some(span),
    };
    let mut constants = vm_core::eir::schema::ConstantPool::default();
    constants.constants.insert(
        0,
        ConstantEntry {
            constant_id: ConstantId::new(0),
            value: Value::Int(99),
        },
    );
    base_module(function, constants)
}

fn base_module(function: EirFunction, constants: vm_core::eir::schema::ConstantPool) -> EirModule {
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

