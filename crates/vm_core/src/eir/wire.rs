//! Pre-resolution EIR wire types for validation before closed schema materialization.
//!
//! Internal only: malformed op/terminator tags are rejected here without extending
//! the frozen `EirOpKind` / `EirTerminator` unions.

use crate::digest::Digest;
use crate::eir::schema::{
    ConstantPool, DeoptPointTable, EirOpKind, EirOpKindTag, EirSourceMap, EirTerminator,
    EirTerminatorKindTag, OpMetadata, RootMapTable, SafepointTable,
};
use crate::eir::validate::EirValidationError;
use crate::id::{EirBlockId, EirFunctionId, FrameMapId, FunctionId, ModuleId, SlotLayoutId};
use crate::profile::Version;

/// Op wire row: kind tag is validated before resolution into `EirOpKind`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EirOpWire {
    pub metadata: OpMetadata,
    pub kind_tag: u8,
    pub kind: Option<EirOpKind>,
}

/// Block wire row validated before closed `EirBlock` materialization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EirBlockWire {
    pub block_id: EirBlockId,
    pub parameters: Vec<crate::eir::schema::BlockParameter>,
    pub ops: Vec<EirOpWire>,
    pub terminator_kind_tag: Option<u8>,
    pub terminator: Option<EirTerminator>,
    pub source_span: Option<vm_diag::source_span::SourceSpanId>,
}

/// Function wire container.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EirFunctionWire {
    pub eir_function_id: EirFunctionId,
    pub function_id: Option<FunctionId>,
    pub module_id: ModuleId,
    pub entry_block: EirBlockId,
    pub blocks: Vec<EirBlockWire>,
    pub slot_layout: SlotLayoutId,
    pub frame_map: FrameMapId,
    pub source_span: Option<vm_diag::source_span::SourceSpanId>,
}

/// Module wire container validated before execution.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct EirModuleWire {
    pub eir_version: Version,
    pub source_runtime_plan_digest: Digest,
    pub functions: Vec<EirFunctionWire>,
    pub constants: ConstantPool,
    pub source_map: EirSourceMap,
    pub root_maps: RootMapTable,
    pub safepoints: SafepointTable,
    pub deopt_points: DeoptPointTable,
}

pub(crate) fn validate_op_kind_tag(raw: u8) -> Result<EirOpKindTag, EirValidationError> {
    EirOpKindTag::from_u8(raw).ok_or(EirValidationError::UnknownOpKind(raw))
}

pub(crate) fn validate_terminator_kind_tag(raw: u8) -> Result<EirTerminatorKindTag, EirValidationError> {
    EirTerminatorKindTag::from_u8(raw).ok_or(EirValidationError::UnknownTerminatorKind(raw))
}