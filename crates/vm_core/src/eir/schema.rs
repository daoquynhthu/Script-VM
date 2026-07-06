//! Closed EIR schema types.
//!
//! Spec: `PHASE-3-EIR-SCHEMA-CLOSURE.md`

use std::collections::BTreeMap;

use vm_diag::source_span::SourceSpanId;

use crate::digest::Digest;
use crate::error::registry::RuntimeErrorCode;
use crate::id::{
    AccessSiteId, BindingId, CallSiteId, CapabilityId, CaseId, CaseIndex, ConstantId,
    ControlRegionId, DeoptId, EirBlockId, EirFunctionId, EirOpId, FieldId, FieldIndex,
    FrameMapId, FunctionId, ModuleId, NodeId, RecordId, RootMapId, RuntimeHelperId, SafepointId,
    ShapeId, SlotId, SlotLayoutId, TypeId,
};
use crate::profile::Version;
use crate::runtime_plan::schema::SafepointKind;
use crate::value::Value;

/// Metadata present on every EIR operation.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct OpMetadata {
    pub op_id: Option<EirOpId>,
    pub source_span: Option<SourceSpanId>,
    pub debug_origin: Option<NodeId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstantOp {
    pub dest: SlotId,
    pub constant: ConstantId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadSlot {
    pub dest: SlotId,
    pub source: SlotId,
    pub require_initialized: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadCell {
    pub dest: SlotId,
    pub cell_slot: SlotId,
    pub binding_id: Option<BindingId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadCapture {
    pub dest: SlotId,
    pub capture_index: u32,
    pub binding_id: Option<BindingId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadModuleSlot {
    pub dest: SlotId,
    pub module_id: ModuleId,
    pub module_slot: SlotId,
    pub binding_id: Option<BindingId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadField {
    pub dest: SlotId,
    pub receiver: SlotId,
    pub record_shape: ShapeId,
    pub field_id: FieldId,
    pub field_index: FieldIndex,
    pub access_site_id: AccessSiteId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadEnumPayload {
    pub dest: SlotId,
    pub receiver: SlotId,
    pub enum_shape: ShapeId,
    pub case_id: CaseId,
    pub case_index: CaseIndex,
    pub payload_index: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadOp {
    Slot(LoadSlot),
    Cell(LoadCell),
    Capture(LoadCapture),
    ModuleSlot(LoadModuleSlot),
    Field(LoadField),
    EnumPayload(LoadEnumPayload),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreSlot {
    pub dest: SlotId,
    pub value: SlotId,
    pub check_initialized: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreCell {
    pub cell_slot: SlotId,
    pub value: SlotId,
    pub binding_id: Option<BindingId>,
    pub check_mutability: bool,
    pub type_check: Option<TypeId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreModuleSlot {
    pub module_id: ModuleId,
    pub module_slot: SlotId,
    pub value: SlotId,
    pub binding_id: Option<BindingId>,
    pub check_mutability: bool,
    pub type_check: Option<TypeId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreField {
    pub receiver: SlotId,
    pub value: SlotId,
    pub record_shape: ShapeId,
    pub field_id: FieldId,
    pub field_index: FieldIndex,
    pub access_site_id: AccessSiteId,
    pub check_readonly: bool,
    pub check_mutability: bool,
    pub type_check: Option<TypeId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreListIndex {
    pub receiver: SlotId,
    pub index: SlotId,
    pub value: SlotId,
    pub access_site_id: AccessSiteId,
    pub check_readonly: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreMapEntry {
    pub receiver: SlotId,
    pub key: SlotId,
    pub value: SlotId,
    pub access_site_id: AccessSiteId,
    pub check_readonly: bool,
    pub check_hashable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreOp {
    Slot(StoreSlot),
    Cell(StoreCell),
    ModuleSlot(StoreModuleSlot),
    Field(StoreField),
    ListIndex(StoreListIndex),
    MapEntry(StoreMapEntry),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOperator {
    Plus,
    Minus,
    Not,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnaryOp {
    pub dest: SlotId,
    pub op: UnaryOperator,
    pub operand: SlotId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Identity,
    NotIdentity,
    Contains,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NumericOverflowPolicy {
    Checked,
    Wrapping,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryOp {
    pub dest: SlotId,
    pub op: BinaryOperator,
    pub left: SlotId,
    pub right: SlotId,
    pub overflow_policy: Option<NumericOverflowPolicy>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LogicalOperator {
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalOp {
    pub dest: SlotId,
    pub op: LogicalOperator,
    pub left_block: EirBlockId,
    pub right_block: EirBlockId,
    pub merge_block: EirBlockId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckBool {
    pub operand: SlotId,
    pub failure_error: RuntimeErrorCode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckType {
    pub operand: SlotId,
    pub expected_type: TypeId,
    pub failure_error: RuntimeErrorCode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckCallable {
    pub operand: SlotId,
    pub failure_error: RuntimeErrorCode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckArity {
    pub operand: SlotId,
    pub expected_arity: u32,
    pub failure_error: RuntimeErrorCode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckHashable {
    pub operand: SlotId,
    pub failure_error: RuntimeErrorCode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckReadonly {
    pub operand: SlotId,
    pub failure_error: RuntimeErrorCode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckCapability {
    pub operand: SlotId,
    pub capability_id: CapabilityId,
    pub failure_error: RuntimeErrorCode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckShape {
    pub operand: SlotId,
    pub shape_id: ShapeId,
    pub failure_error: RuntimeErrorCode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckOverflow {
    pub left: SlotId,
    pub right: SlotId,
    pub failure_error: RuntimeErrorCode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckDivisionByZero {
    pub divisor: SlotId,
    pub failure_error: RuntimeErrorCode,
}

/// Frozen CheckOp union (§13).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckOp {
    Bool(CheckBool),
    Type(CheckType),
    Callable(CheckCallable),
    Arity(CheckArity),
    Hashable(CheckHashable),
    Readonly(CheckReadonly),
    Capability(CheckCapability),
    Shape(CheckShape),
    Overflow(CheckOverflow),
    DivisionByZero(CheckDivisionByZero),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CallKind {
    Generic,
    KnownFunction,
    KnownBuiltin,
    RecordConstructor,
    EnumConstructor,
    BoundMethod,
    HostFunction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedArgumentSlot {
    pub name: String,
    pub slot: SlotId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallOp {
    pub dest: Option<SlotId>,
    pub callee: SlotId,
    pub positional_args: Vec<SlotId>,
    pub named_args: Vec<NamedArgumentSlot>,
    pub call_site_id: CallSiteId,
    pub result_type_check: Option<TypeId>,
    pub call_kind: CallKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccessKind {
    AttributeRead,
    AttributeWrite,
    MethodRead,
    IndexRead,
    IndexWrite,
    SliceRead,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessOp {
    pub kind: AccessKind,
    pub access_site_id: AccessSiteId,
    pub receiver: SlotId,
    pub dest: Option<SlotId>,
    pub index: Option<SlotId>,
    pub value: Option<SlotId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstructList {
    pub dest: SlotId,
    pub elements: Vec<SlotId>,
    pub safepoint_id: Option<SafepointId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstructMap {
    pub dest: SlotId,
    pub entries: Vec<(SlotId, SlotId)>,
    pub safepoint_id: Option<SafepointId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstructRecord {
    pub dest: SlotId,
    pub record_id: RecordId,
    pub shape_id: ShapeId,
    pub field_values: Vec<SlotId>,
    pub safepoint_id: Option<SafepointId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstructEnumValue {
    pub dest: SlotId,
    pub enum_shape: ShapeId,
    pub case_id: CaseId,
    pub case_index: CaseIndex,
    pub payload_slots: Vec<SlotId>,
    pub safepoint_id: Option<SafepointId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstructFunction {
    pub dest: SlotId,
    pub function_id: FunctionId,
    pub safepoint_id: Option<SafepointId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstructError {
    pub dest: SlotId,
    pub code: RuntimeErrorCode,
    pub message_slot: SlotId,
}

/// Frozen ConstructOp union (§16).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstructOp {
    List(ConstructList),
    Map(ConstructMap),
    Record(ConstructRecord),
    EnumValue(ConstructEnumValue),
    Function(ConstructFunction),
    Error(ConstructError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatternCheckLiteral {
    pub scrutinee: SlotId,
    pub expected: SlotId,
    pub failure_error: RuntimeErrorCode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatternCheckRecordShape {
    pub scrutinee: SlotId,
    pub record_shape: ShapeId,
    pub failure_error: RuntimeErrorCode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatternCheckEnumCase {
    pub scrutinee: SlotId,
    pub enum_shape: ShapeId,
    pub case_id: CaseId,
    pub failure_error: RuntimeErrorCode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatternCheckListLength {
    pub scrutinee: SlotId,
    pub expected_length: u32,
    pub failure_error: RuntimeErrorCode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatternCheckMapKey {
    pub scrutinee: SlotId,
    pub key: SlotId,
    pub failure_error: RuntimeErrorCode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatternBind {
    pub dest: SlotId,
    pub source: SlotId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatternBranch {
    pub success_block: EirBlockId,
    pub failure_block: EirBlockId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatternCommit {
    pub binding_slots: Vec<SlotId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatternRollback {
    pub restored_slots: Vec<SlotId>,
}

/// Frozen PatternOp union (§17).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternOp {
    CheckLiteral(PatternCheckLiteral),
    CheckRecordShape(PatternCheckRecordShape),
    CheckEnumCase(PatternCheckEnumCase),
    CheckListLength(PatternCheckListLength),
    CheckMapKey(PatternCheckMapKey),
    Bind(PatternBind),
    Branch(PatternBranch),
    Commit(PatternCommit),
    Rollback(PatternRollback),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeHelperOp {
    pub dest: Option<SlotId>,
    pub helper_id: RuntimeHelperId,
    pub args: Vec<SlotId>,
    pub call_site: Option<CallSiteId>,
    pub access_site: Option<AccessSiteId>,
    pub safepoint_id: Option<SafepointId>,
    pub deopt_id: Option<DeoptId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SafepointOp {
    pub safepoint_id: SafepointId,
    pub kind: SafepointKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GuardKind {
    Type,
    Shape,
    Arity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GuardFailureAction {
    Raise,
    HelperFallback,
    Deopt,
    Branch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuardOp {
    pub guard_kind: GuardKind,
    pub inputs: Vec<SlotId>,
    pub on_failure: GuardFailureAction,
    pub deopt_id: Option<DeoptId>,
    pub helper_id: Option<RuntimeHelperId>,
    pub failure_error: Option<RuntimeErrorCode>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DebugKind {
    Trace,
    Breakpoint,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebugOp {
    pub debug_kind: DebugKind,
    pub source_span: Option<SourceSpanId>,
}

/// Closed EIR operation union per frozen schema §6.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EirOpKind {
    Constant(ConstantOp),
    Load(LoadOp),
    Store(StoreOp),
    Unary(UnaryOp),
    Binary(BinaryOp),
    Logical(LogicalOp),
    Check(CheckOp),
    Call(CallOp),
    Access(AccessOp),
    Construct(ConstructOp),
    Pattern(PatternOp),
    RuntimeHelper(RuntimeHelperOp),
    Safepoint(SafepointOp),
    Guard(GuardOp),
    Debug(DebugOp),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EirOp {
    pub metadata: OpMetadata,
    pub kind: EirOpKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Jump {
    pub target: EirBlockId,
    pub args: Vec<SlotId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Branch {
    pub condition: SlotId,
    pub then_block: EirBlockId,
    pub else_block: EirBlockId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Return {
    pub value: Option<SlotId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Raise {
    pub error: SlotId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoopBackedge {
    pub target: EirBlockId,
    pub args: Vec<SlotId>,
    pub safepoint_id: SafepointId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwitchCase {
    pub value: SlotId,
    pub target: EirBlockId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Switch {
    pub scrutinee: SlotId,
    pub cases: Vec<SwitchCase>,
    pub default: EirBlockId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unwind {
    pub pending_control_slot: SlotId,
    pub target_region: Option<ControlRegionId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unreachable {
    pub reason: String,
}

/// Closed EIR terminator union per frozen schema §22.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EirTerminator {
    Jump(Jump),
    Branch(Branch),
    Return(Return),
    Raise(Raise),
    LoopBackedge(LoopBackedge),
    Switch(Switch),
    Unwind(Unwind),
    Unreachable(Unreachable),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockParameter {
    pub slot: SlotId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EirBlock {
    pub block_id: EirBlockId,
    pub parameters: Vec<BlockParameter>,
    pub ops: Vec<EirOp>,
    pub terminator: EirTerminator,
    pub source_span: Option<SourceSpanId>,
}

/// Owner of a GC root map (§2.2 GC metadata).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RootMapOwner {
    InterpreterFrame,
    EirFunction,
    RuntimeHelper,
    JitCompiledFunction,
    HostBoundary,
    ModuleInitialization,
}

/// Canonical RootMap metadata (§2 GC metadata).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootMap {
    pub root_map_id: RootMapId,
    pub owner: RootMapOwner,
    pub safepoint_id: Option<SafepointId>,
    pub frame_map_id: Option<FrameMapId>,
    pub roots: Vec<SlotId>,
    pub source_span: Option<SourceSpanId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RootMapTable {
    pub maps: BTreeMap<u32, RootMap>,
}

/// Safepoint record linking to RootMap when GC may run (§4 GC metadata).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SafepointRecord {
    pub safepoint_id: SafepointId,
    pub kind: SafepointKind,
    pub root_map: RootMapId,
    pub frame_map: Option<FrameMapId>,
    pub source_span: Option<SourceSpanId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EirFunction {
    pub eir_function_id: EirFunctionId,
    pub function_id: Option<FunctionId>,
    pub module_id: ModuleId,
    pub entry_block: EirBlockId,
    pub blocks: Vec<EirBlock>,
    pub slot_layout: SlotLayoutId,
    pub frame_map: FrameMapId,
    pub source_span: Option<SourceSpanId>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstantEntry {
    pub constant_id: ConstantId,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ConstantPool {
    pub constants: BTreeMap<u32, ConstantEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EirSourceMap {
    pub spans: BTreeMap<u32, SourceSpanId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SafepointTable {
    pub records: BTreeMap<u32, SafepointRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DeoptPointTable {
    pub records: BTreeMap<u32, DeoptId>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EirModule {
    pub eir_version: Version,
    pub source_runtime_plan_digest: Digest,
    pub functions: Vec<EirFunction>,
    pub constants: ConstantPool,
    pub source_map: EirSourceMap,
    pub root_maps: RootMapTable,
    pub safepoints: SafepointTable,
    pub deopt_points: DeoptPointTable,
}

/// Numeric tag for EIR op kinds used during ingest validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EirOpKindTag {
    Constant = 0,
    Load = 1,
    Store = 2,
    Unary = 3,
    Binary = 4,
    Logical = 5,
    Check = 6,
    Call = 7,
    Access = 8,
    Construct = 9,
    Pattern = 10,
    RuntimeHelper = 11,
    Safepoint = 12,
    Guard = 13,
    Debug = 14,
}

impl EirOpKindTag {
    pub const MAX: u8 = Self::Debug as u8;

    #[must_use]
    pub const fn from_u8(raw: u8) -> Option<Self> {
        match raw {
            0 => Some(Self::Constant),
            1 => Some(Self::Load),
            2 => Some(Self::Store),
            3 => Some(Self::Unary),
            4 => Some(Self::Binary),
            5 => Some(Self::Logical),
            6 => Some(Self::Check),
            7 => Some(Self::Call),
            8 => Some(Self::Access),
            9 => Some(Self::Construct),
            10 => Some(Self::Pattern),
            11 => Some(Self::RuntimeHelper),
            12 => Some(Self::Safepoint),
            13 => Some(Self::Guard),
            14 => Some(Self::Debug),
            _ => None,
        }
    }
}

/// Numeric tag for EIR terminator kinds used during ingest validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EirTerminatorKindTag {
    Jump = 0,
    Branch = 1,
    Return = 2,
    Raise = 3,
    LoopBackedge = 4,
    Switch = 5,
    Unwind = 6,
    Unreachable = 7,
}

impl EirTerminatorKindTag {
    #[must_use]
    pub const fn from_u8(raw: u8) -> Option<Self> {
        match raw {
            0 => Some(Self::Jump),
            1 => Some(Self::Branch),
            2 => Some(Self::Return),
            3 => Some(Self::Raise),
            4 => Some(Self::LoopBackedge),
            5 => Some(Self::Switch),
            6 => Some(Self::Unwind),
            7 => Some(Self::Unreachable),
            _ => None,
        }
    }
}