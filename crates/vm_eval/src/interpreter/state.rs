//! Interpreter state and activation records.
//!
//! Spec: `PHASE-3-FAST-INTERPRETER-DATA-STRUCTURES.md`, `PHASE-3-CONTROL-STATE-MODEL.md`

use std::collections::BTreeMap;

use vm_core::eir::schema::{ConstantPool, EirBlock, EirFunction};
use vm_core::id::{EirBlockId, EirFunctionId};
use vm_core::error::language::ErrorStore;
use vm_core::id::SafepointId;
use vm_diag::source_span::SourceSpanId;
use vm_runtime::call::callable::CallableRegistry;
use vm_runtime::call::contract::StubTypeContractChecker;
use vm_runtime::control::PendingControl;
use vm_runtime::frame::SlotArray;
use vm_runtime::heap::Heap;
use vm_runtime::unwind::UnwindContext;
use vm_runtime::write_barrier::NoopWriteBarrierHook;

/// Single interpreter activation record.
#[derive(Debug, Clone, PartialEq)]
pub struct InterpreterFrame {
    pub function_id: EirFunctionId,
    pub slots: SlotArray,
    pub current_block: EirBlockId,
    pub blocks: BTreeMap<u32, EirBlock>,
}

impl InterpreterFrame {
    pub fn new(function: &EirFunction, slot_count: usize) -> Self {
        let blocks = function
            .blocks
            .iter()
            .map(|b| (b.block_id.raw(), b.clone()))
            .collect();
        Self {
            function_id: function.eir_function_id,
            slots: SlotArray::with_capacity(slot_count),
            current_block: function.entry_block,
            blocks,
        }
    }

    #[must_use]
    pub fn current_block(&self) -> Option<&EirBlock> {
        self.blocks.get(&self.current_block.raw())
    }

    #[must_use]
    pub fn block(&self, id: EirBlockId) -> Option<&EirBlock> {
        self.blocks.get(&id.raw())
    }
}

/// Loop-backedge safepoint poll counter (bootstrap hook).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SafepointPollState {
    pub loop_backedge_count: u32,
}

impl SafepointPollState {
    pub fn record_loop_backedge(&mut self, _safepoint_id: SafepointId) {
        self.loop_backedge_count += 1;
    }
}

/// Mutable interpreter execution state.
#[derive(Debug)]
pub struct InterpreterState {
    pub frames: Vec<InterpreterFrame>,
    pub constants: ConstantPool,
    pub error_store: ErrorStore,
    pub heap: Heap,
    pub callable_registry: CallableRegistry,
    pub type_checker: StubTypeContractChecker,
    pub write_barrier: NoopWriteBarrierHook,
    pub unwind_ctx: UnwindContext,
    pub safepoint_polls: SafepointPollState,
    pub last_source_span: Option<SourceSpanId>,
    pub halted: bool,
}

impl InterpreterState {
    #[must_use]
    pub fn new(constants: ConstantPool) -> Self {
        Self {
            frames: Vec::new(),
            constants,
            error_store: ErrorStore::new(),
            heap: Heap::new(),
            callable_registry: CallableRegistry::new(),
            type_checker: StubTypeContractChecker::new(),
            write_barrier: NoopWriteBarrierHook,
            unwind_ctx: UnwindContext::with_pending(PendingControl::Return(None)),
            safepoint_polls: SafepointPollState::default(),
            last_source_span: None,
            halted: false,
        }
    }

    pub fn push_frame(&mut self, function: &EirFunction, slot_count: usize) {
        self.frames
            .push(InterpreterFrame::new(function, slot_count));
    }

    pub fn pop_frame(&mut self) -> Option<InterpreterFrame> {
        self.frames.pop()
    }

    #[must_use]
    pub fn current_frame(&self) -> Option<&InterpreterFrame> {
        self.frames.last()
    }

    pub fn current_frame_mut(&mut self) -> Option<&mut InterpreterFrame> {
        self.frames.last_mut()
    }
}