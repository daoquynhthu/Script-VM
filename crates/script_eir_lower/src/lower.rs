//! Lower validated `IrUnit` SIR to EIR (T-P3L bootstrap).
//!
//! Spec anchors: PHASE-3-SIR-LOWERING-ROUND1 / CONTROL-LOWERING-ROUND2 (subset),
//! PHASE-3-EIR-SCHEMA-CLOSURE (ops used by vm_eval).
//!
//! Non-goals: full RuntimePlan packaging, full control-region EIR fidelity, short-circuit and/or.

use std::collections::HashMap;

use sir::{BindingId, BinaryOp as SirBin, IrUnit, NodeId, SirNode, UnaryOp as SirUn};
use vm_core::digest::Digest;
use vm_core::eir::schema::{
    BinaryOp, BinaryOperator, Branch, ConstantEntry, ConstantOp, ConstantPool, EirBlock,
    EirFunction, EirModule, EirOp, EirOpKind, EirTerminator, Jump, LoadOp, LoadSlot, OpMetadata,
    Raise, Return, RuntimeHelperOp, StoreOp, StoreSlot, UnaryOp, UnaryOperator,
};
use vm_core::id::{
    ConstantId, EirBlockId, EirFunctionId, FrameMapId, FunctionId, ModuleId, ObjectId, SlotId,
    SlotLayoutId,
};
use vm_core::profile::Version;
use vm_core::value::Value;
use vm_diag::source_span::SourceSpanId;
use vm_runtime::helpers::dispatch::{
    HELPER_CONSTRUCT_ERROR_ID, HELPER_CONSTRUCT_LIST_ID, HELPER_DISPLAY_ID, HELPER_GENERIC_CALL_ID,
};

use crate::error::EirLowerError;
use crate::program::{user_fn_target, EirProgram};

const SPAN: SourceSpanId = SourceSpanId::new(1);
const FN_OBJECT_BASE: u32 = 2000;
const PRINT_OBJECT_ID: u32 = 1999;

/// Lower a structural-validated SIR unit to an executable EIR program.
pub fn lower_sir_to_eir(unit: &IrUnit) -> Result<EirProgram, EirLowerError> {
    SirEirLower::new(unit).lower()
}

struct FnRec {
    name: String,
    node: NodeId,
    params: Vec<BindingId>,
    body: NodeId,
    eir_id: u32,
    function_id: u32,
    object_id: ObjectId,
}

struct SirEirLower<'a> {
    unit: &'a IrUnit,
    constants: ConstantPool,
    next_const: u32,
    fn_table: Vec<FnRec>,
    /// binding id → function object constant (for Name of functions)
    binding_fn_const: HashMap<u32, ConstantId>,
    /// symbol text → constant ObjectRef for callables (print + user fns)
    name_fn_const: HashMap<String, ConstantId>,
    callables: Vec<(ObjectId, vm_runtime::call::callable::CallableTarget)>,
}

impl<'a> SirEirLower<'a> {
    fn new(unit: &'a IrUnit) -> Self {
        Self {
            unit,
            constants: ConstantPool::default(),
            next_const: 0,
            fn_table: Vec::new(),
            binding_fn_const: HashMap::new(),
            name_fn_const: HashMap::new(),
            callables: Vec::new(),
        }
    }

    fn intern_const(&mut self, value: Value) -> ConstantId {
        let id = ConstantId::new(self.next_const);
        self.constants.constants.insert(
            self.next_const,
            ConstantEntry {
                constant_id: id,
                value,
            },
        );
        self.next_const += 1;
        id
    }

    fn binding_name(&self, id: BindingId) -> Result<String, EirLowerError> {
        let b = self
            .unit
            .bindings
            .iter()
            .find(|b| b.binding_id == id)
            .ok_or_else(|| EirLowerError::new(format!("missing binding {}", id.raw())))?;
        self.unit
            .symbol_text(b.symbol_id)
            .map(std::string::ToString::to_string)
            .ok_or_else(|| EirLowerError::new("missing symbol for binding"))
    }

    fn node(&self, id: NodeId) -> Result<&SirNode, EirLowerError> {
        self.unit
            .node(id)
            .map(|e| &e.kind)
            .ok_or_else(|| EirLowerError::new(format!("missing node {}", id.raw())))
    }

    fn lower(mut self) -> Result<EirProgram, EirLowerError> {
        // Collect functions from SIR node table.
        let mut fn_index = 0u32;
        for entry in &self.unit.nodes {
            if let SirNode::Function {
                binding,
                params,
                body,
            } = &entry.kind
            {
                let binding = *binding;
                let body = *body;
                let name = self.binding_name(binding)?;
                let eir_id = fn_index + 1;
                let object_id = ObjectId::new(FN_OBJECT_BASE + fn_index);
                let cid = self.intern_const(Value::ObjectRef(object_id));
                self.binding_fn_const.insert(binding.raw(), cid);
                self.name_fn_const.insert(name.clone(), cid);
                self.callables
                    .push((object_id, user_fn_target(object_id, eir_id, eir_id)));
                self.fn_table.push(FnRec {
                    name,
                    node: entry.node_id,
                    params: params.clone(),
                    body,
                    eir_id,
                    function_id: eir_id,
                    object_id,
                });
                fn_index += 1;
            }
        }

        // print identity function
        let print_obj = ObjectId::new(PRINT_OBJECT_ID);
        let print_eir = fn_index + 1;
        let print_cid = self.intern_const(Value::ObjectRef(print_obj));
        self.name_fn_const.insert("print".into(), print_cid);
        self.callables
            .push((print_obj, user_fn_target(print_obj, print_eir, print_eir)));

        let mut user_fns = Vec::new();
        // clone fn_table to avoid borrow issues
        let table = self.fn_table.clone();
        for rec in &table {
            user_fns.push(self.lower_function(rec)?);
        }
        let print_fn = self.make_print(print_eir);
        let main = self.lower_main()?;

        let mut functions = vec![main];
        functions.extend(user_fns);
        functions.push(print_fn);

        Ok(EirProgram {
            module: EirModule {
                eir_version: Version::new(1, 0, 0),
                source_runtime_plan_digest: Digest(0x51E1_0001),
                functions,
                constants: self.constants,
                source_map: Default::default(),
                root_maps: Default::default(),
                safepoints: Default::default(),
                deopt_points: Default::default(),
            },
            entry: EirFunctionId::new(0),
            callables: self.callables,
        })
    }

    fn make_print(&self, eir_id: u32) -> EirFunction {
        EirFunction {
            eir_function_id: EirFunctionId::new(eir_id),
            function_id: Some(FunctionId::new(eir_id)),
            module_id: ModuleId::new(0),
            entry_block: EirBlockId::new(0),
            blocks: vec![EirBlock {
                block_id: EirBlockId::new(0),
                parameters: vec![],
                ops: vec![],
                terminator: EirTerminator::Return(Return {
                    value: Some(SlotId::new(0)),
                }),
                source_span: Some(SPAN),
            }],
            slot_layout: SlotLayoutId::new(0),
            frame_map: FrameMapId::new(0),
            source_span: Some(SPAN),
        }
    }

    fn lower_function(&mut self, rec: &FnRec) -> Result<EirFunction, EirLowerError> {
        let mut fb = FuncBuilder::new(rec.eir_id, rec.function_id);
        for (i, p) in rec.params.iter().enumerate() {
            let name = self.binding_name(*p)?;
            fb.bind_binding(*p, name, SlotId::new(i as u32));
            fb.next_slot = fb.next_slot.max(i as u32 + 1);
        }
        let last = self.lower_node_stmt(&mut fb, rec.body)?;
        if !fb.terminated {
            let slot = last.unwrap_or_else(|| {
                let s = fb.alloc_slot();
                let cid = self.intern_const(Value::None);
                fb.push_op(EirOpKind::Constant(ConstantOp {
                    dest: s,
                    constant: cid,
                }));
                s
            });
            fb.finish_return(Some(slot));
        }
        Ok(fb.into_function())
    }

    fn lower_main(&mut self) -> Result<EirFunction, EirLowerError> {
        let mut fb = FuncBuilder::new(0, 0);
        let items = match self.node(self.unit.root_node)?.clone() {
            SirNode::ModuleBody { items } => items,
            _ => return Err(EirLowerError::new("root is not ModuleBody")),
        };
        let mut last = None;
        for item in items {
            let kind = self.node(item)?.clone();
            match kind {
                SirNode::Function { .. } => {}
                SirNode::ExportMarker { item: inner } => {
                    if matches!(self.node(inner)?.clone(), SirNode::Function { .. }) {
                        continue;
                    }
                    last = self.lower_node_stmt(&mut fb, inner)?;
                }
                _ => {
                    last = self.lower_node_stmt(&mut fb, item)?;
                }
            }
        }
        if !fb.terminated {
            let slot = last.unwrap_or_else(|| {
                let s = fb.alloc_slot();
                let cid = self.intern_const(Value::None);
                fb.push_op(EirOpKind::Constant(ConstantOp {
                    dest: s,
                    constant: cid,
                }));
                s
            });
            fb.finish_return(Some(slot));
        }
        Ok(fb.into_function())
    }

    fn lower_node_stmt(
        &mut self,
        fb: &mut FuncBuilder,
        id: NodeId,
    ) -> Result<Option<SlotId>, EirLowerError> {
        if fb.terminated {
            return Ok(None);
        }
        match self.node(id)?.clone() {
            SirNode::Let { binding, init } | SirNode::Const { binding, init } => {
                let v = self.lower_node_expr(fb, init)?;
                let name = self.binding_name(binding)?;
                let slot = fb.alloc_slot();
                fb.push_op(EirOpKind::Store(StoreOp::Slot(StoreSlot {
                    dest: slot,
                    value: v,
                    check_initialized: None,
                })));
                fb.bind_binding(binding, name, slot);
                Ok(Some(slot))
            }
            SirNode::Block { stmts } => {
                let mut last = None;
                for s in stmts {
                    last = self.lower_node_stmt(fb, s)?;
                }
                Ok(last)
            }
            SirNode::ExprStmt { expr } => Ok(Some(self.lower_node_expr(fb, expr)?)),
            SirNode::Return { value } => {
                let slot = match value {
                    Some(v) => Some(self.lower_node_expr(fb, v)?),
                    None => None,
                };
                fb.finish_return(slot);
                Ok(slot)
            }
            SirNode::Assign { binding, value } => {
                let v = self.lower_node_expr(fb, value)?;
                let dest = fb.lookup_binding(binding).ok_or_else(|| {
                    EirLowerError::new(format!("unbound assign {}", binding.raw()))
                })?;
                fb.push_op(EirOpKind::Store(StoreOp::Slot(StoreSlot {
                    dest,
                    value: v,
                    check_initialized: None,
                })));
                Ok(Some(dest))
            }
            SirNode::If {
                cond,
                then_block,
                elifs,
                else_block,
            } => self.lower_if(fb, cond, then_block, &elifs, else_block),
            SirNode::While { cond, body } => self.lower_while(fb, cond, body),
            SirNode::Import { .. } | SirNode::ExportMarker { .. } | SirNode::Function { .. } => {
                Ok(None)
            }
            SirNode::Assert { cond } => {
                // assert cond ⇒ if not cond: raise AssertionError
                let c = self.lower_node_expr(fb, cond)?;
                let not_c = fb.alloc_slot();
                fb.push_op(EirOpKind::Unary(UnaryOp {
                    dest: not_c,
                    op: UnaryOperator::Not,
                    operand: c,
                }));
                let fail = fb.new_block_id();
                let ok = fb.new_block_id();
                fb.finish_branch(not_c, fail, ok);
                fb.start_block(fail);
                self.emit_raise_string(fb, "assertion failed")?;
                fb.start_block(ok);
                Ok(Some(c))
            }
            SirNode::Raise { value } => {
                let v = self.lower_node_expr(fb, value)?;
                self.emit_raise_value(fb, v)?;
                Ok(None)
            }
            SirNode::Break => {
                let target = fb
                    .loop_stack
                    .last()
                    .map(|l| l.break_target)
                    .ok_or_else(|| EirLowerError::new("break outside loop in EIR lower"))?;
                fb.finish_jump(target);
                Ok(None)
            }
            SirNode::Continue => {
                let target = fb
                    .loop_stack
                    .last()
                    .map(|l| l.continue_target)
                    .ok_or_else(|| EirLowerError::new("continue outside loop in EIR lower"))?;
                fb.finish_jump(target);
                Ok(None)
            }
            SirNode::For {
                binding,
                iter,
                body,
            } => self.lower_for(fb, binding, iter, body),
            other => {
                // Expression used as statement
                if matches!(
                    other,
                    SirNode::LiteralNil
                        | SirNode::LiteralBool { .. }
                        | SirNode::LiteralInt { .. }
                        | SirNode::LiteralFloat { .. }
                        | SirNode::LiteralString { .. }
                        | SirNode::Name { .. }
                        | SirNode::Call { .. }
                        | SirNode::Unary { .. }
                        | SirNode::Binary { .. }
                        | SirNode::List { .. }
                        | SirNode::SymbolRef { .. }
                ) {
                    Ok(Some(self.lower_node_expr(fb, id)?))
                } else {
                    Err(EirLowerError::new(format!(
                        "unsupported stmt node: {other:?}"
                    )))
                }
            }
        }
    }

    fn lower_if(
        &mut self,
        fb: &mut FuncBuilder,
        cond: NodeId,
        then_block: NodeId,
        elifs: &[(NodeId, NodeId)],
        else_block: Option<NodeId>,
    ) -> Result<Option<SlotId>, EirLowerError> {
        if let Some((ec, eb)) = elifs.first() {
            // Desugar elif chain into nested if nodes by recursive call with rebuilt else.
            // Simpler: treat remaining elifs as nested via sequential lower_if on synthetic structure —
            // here we only lower first elif as else branch containing nested if.
            let rest = &elifs[1..];
            // Lower as: if cond then_block else { if ec eb rest else_block }
            // Implement by recursive lower_if for the else path manually:
            let c = self.lower_node_expr(fb, cond)?;
            let then_id = fb.new_block_id();
            let else_id = fb.new_block_id();
            let merge_id = fb.new_block_id();
            fb.finish_branch(c, then_id, else_id);
            fb.start_block(then_id);
            let then_slot = self.lower_node_stmt(fb, then_block)?;
            if !fb.terminated {
                fb.finish_jump(merge_id);
            }
            fb.start_block(else_id);
            let else_slot = self.lower_if(fb, *ec, *eb, rest, else_block)?;
            if !fb.terminated {
                fb.finish_jump(merge_id);
            }
            fb.start_block(merge_id);
            return Ok(then_slot.or(else_slot));
        }

        let c = self.lower_node_expr(fb, cond)?;
        let then_id = fb.new_block_id();
        let else_id = fb.new_block_id();
        let merge_id = fb.new_block_id();
        fb.finish_branch(c, then_id, else_id);
        fb.start_block(then_id);
        let then_slot = self.lower_node_stmt(fb, then_block)?;
        if !fb.terminated {
            fb.finish_jump(merge_id);
        }
        fb.start_block(else_id);
        let else_slot = if let Some(e) = else_block {
            self.lower_node_stmt(fb, e)?
        } else {
            None
        };
        if !fb.terminated {
            fb.finish_jump(merge_id);
        }
        fb.start_block(merge_id);
        Ok(then_slot.or(else_slot))
    }

    fn lower_while(
        &mut self,
        fb: &mut FuncBuilder,
        cond: NodeId,
        body: NodeId,
    ) -> Result<Option<SlotId>, EirLowerError> {
        let header = fb.new_block_id();
        let body_id = fb.new_block_id();
        let exit = fb.new_block_id();
        fb.finish_jump(header);
        fb.start_block(header);
        let c = self.lower_node_expr(fb, cond)?;
        fb.finish_branch(c, body_id, exit);
        fb.start_block(body_id);
        fb.loop_stack.push(LoopTargets {
            continue_target: header,
            break_target: exit,
        });
        let _ = self.lower_node_stmt(fb, body)?;
        fb.loop_stack.pop();
        if !fb.terminated {
            fb.finish_jump(header);
        }
        fb.start_block(exit);
        Ok(None)
    }

    fn emit_raise_string(
        &mut self,
        fb: &mut FuncBuilder,
        msg: &str,
    ) -> Result<(), EirLowerError> {
        // AssertionError is index 6 in RuntimeErrorCode::ALL.
        let code_slot = fb.alloc_slot();
        let code_c = self.intern_const(Value::Int(6));
        fb.push_op(EirOpKind::Constant(ConstantOp {
            dest: code_slot,
            constant: code_c,
        }));
        let msg_slot = fb.alloc_slot();
        let msg_c = self.intern_const(Value::String(msg.into()));
        fb.push_op(EirOpKind::Constant(ConstantOp {
            dest: msg_slot,
            constant: msg_c,
        }));
        let err_slot = fb.alloc_slot();
        fb.push_op(EirOpKind::RuntimeHelper(RuntimeHelperOp {
            dest: Some(err_slot),
            helper_id: HELPER_CONSTRUCT_ERROR_ID,
            args: vec![code_slot, msg_slot],
            call_site: None,
            access_site: None,
            safepoint_id: None,
            deopt_id: None,
        }));
        fb.seal_block(EirTerminator::Raise(Raise { error: err_slot }));
        Ok(())
    }

    fn emit_raise_value(
        &mut self,
        fb: &mut FuncBuilder,
        value: SlotId,
    ) -> Result<(), EirLowerError> {
        // If value is already Error, raise it; else wrap String/other as AssertionError message.
        // Bootstrap: always re-wrap via display path — construct_error(String).
        // Prefer: raise Error values directly when produced by construct_error.
        // For string/int, build message via display helper.
        let msg = fb.alloc_slot();
        fb.push_op(EirOpKind::RuntimeHelper(RuntimeHelperOp {
            dest: Some(msg),
            helper_id: HELPER_DISPLAY_ID,
            args: vec![value],
            call_site: None,
            access_site: None,
            safepoint_id: None,
            deopt_id: None,
        }));
        let code_slot = fb.alloc_slot();
        let code_c = self.intern_const(Value::Int(6));
        fb.push_op(EirOpKind::Constant(ConstantOp {
            dest: code_slot,
            constant: code_c,
        }));
        let err_slot = fb.alloc_slot();
        fb.push_op(EirOpKind::RuntimeHelper(RuntimeHelperOp {
            dest: Some(err_slot),
            helper_id: HELPER_CONSTRUCT_ERROR_ID,
            args: vec![code_slot, msg],
            call_site: None,
            access_site: None,
            safepoint_id: None,
            deopt_id: None,
        }));
        fb.seal_block(EirTerminator::Raise(Raise { error: err_slot }));
        Ok(())
    }

    fn lower_node_expr(
        &mut self,
        fb: &mut FuncBuilder,
        id: NodeId,
    ) -> Result<SlotId, EirLowerError> {
        match self.node(id)?.clone() {
            SirNode::LiteralNil => {
                let s = fb.alloc_slot();
                let cid = self.intern_const(Value::None);
                fb.push_op(EirOpKind::Constant(ConstantOp {
                    dest: s,
                    constant: cid,
                }));
                Ok(s)
            }
            SirNode::LiteralBool { value } => {
                let s = fb.alloc_slot();
                let cid = self.intern_const(Value::Bool(value));
                fb.push_op(EirOpKind::Constant(ConstantOp {
                    dest: s,
                    constant: cid,
                }));
                Ok(s)
            }
            SirNode::LiteralInt { value } => {
                let n: i64 = value
                    .parse()
                    .map_err(|_| EirLowerError::new(format!("bad int {value}")))?;
                let s = fb.alloc_slot();
                let cid = self.intern_const(Value::Int(n));
                fb.push_op(EirOpKind::Constant(ConstantOp {
                    dest: s,
                    constant: cid,
                }));
                Ok(s)
            }
            SirNode::LiteralFloat { value } => {
                let n: f64 = value
                    .parse()
                    .map_err(|_| EirLowerError::new(format!("bad float {value}")))?;
                let s = fb.alloc_slot();
                let cid = self.intern_const(Value::Float(n));
                fb.push_op(EirOpKind::Constant(ConstantOp {
                    dest: s,
                    constant: cid,
                }));
                Ok(s)
            }
            SirNode::LiteralString { value } => {
                let s = fb.alloc_slot();
                let cid = self.intern_const(Value::String(value));
                fb.push_op(EirOpKind::Constant(ConstantOp {
                    dest: s,
                    constant: cid,
                }));
                Ok(s)
            }
            SirNode::Name { binding } => {
                if let Some(slot) = fb.lookup_binding(binding) {
                    let dest = fb.alloc_slot();
                    fb.push_op(EirOpKind::Load(LoadOp::Slot(LoadSlot {
                        dest,
                        source: slot,
                        require_initialized: true,
                    })));
                    return Ok(dest);
                }
                if let Some(cid) = self.binding_fn_const.get(&binding.raw()).copied() {
                    let dest = fb.alloc_slot();
                    fb.push_op(EirOpKind::Constant(ConstantOp {
                        dest,
                        constant: cid,
                    }));
                    return Ok(dest);
                }
                // print / other globals by name
                let name = self.binding_name(binding)?;
                if let Some(cid) = self.name_fn_const.get(&name).copied() {
                    let dest = fb.alloc_slot();
                    fb.push_op(EirOpKind::Constant(ConstantOp {
                        dest,
                        constant: cid,
                    }));
                    return Ok(dest);
                }
                Err(EirLowerError::new(format!("unresolved binding {name}")))
            }
            SirNode::SymbolRef { symbol } => {
                let name = self
                    .unit
                    .symbol_text(symbol)
                    .ok_or_else(|| EirLowerError::new("missing symbol"))?
                    .to_string();
                if let Some(cid) = self.name_fn_const.get(&name).copied() {
                    let dest = fb.alloc_slot();
                    fb.push_op(EirOpKind::Constant(ConstantOp {
                        dest,
                        constant: cid,
                    }));
                    return Ok(dest);
                }
                Err(EirLowerError::new(format!("unresolved symbol {name}")))
            }
            SirNode::Call { callee, args } => {
                // Host print: print(x) → helper_display(x) [stdout side-effect in interpreter]
                if self.is_print_callee(callee) {
                    let arg = if let Some(a) = args.first() {
                        self.lower_node_expr(fb, *a)?
                    } else {
                        let s = fb.alloc_slot();
                        let cid = self.intern_const(Value::None);
                        fb.push_op(EirOpKind::Constant(ConstantOp {
                            dest: s,
                            constant: cid,
                        }));
                        s
                    };
                    let dest = fb.alloc_slot();
                    fb.push_op(EirOpKind::RuntimeHelper(RuntimeHelperOp {
                        dest: Some(dest),
                        helper_id: HELPER_DISPLAY_ID,
                        args: vec![arg],
                        call_site: None,
                        access_site: None,
                        safepoint_id: None,
                        deopt_id: None,
                    }));
                    return Ok(dest);
                }
                let c = self.lower_node_expr(fb, callee)?;
                let mut arg_slots = vec![c];
                for a in args {
                    arg_slots.push(self.lower_node_expr(fb, a)?);
                }
                let dest = fb.alloc_slot();
                fb.push_op(EirOpKind::RuntimeHelper(RuntimeHelperOp {
                    dest: Some(dest),
                    helper_id: HELPER_GENERIC_CALL_ID,
                    args: arg_slots,
                    call_site: None,
                    access_site: None,
                    safepoint_id: None,
                    deopt_id: None,
                }));
                Ok(dest)
            }
            SirNode::Unary { op, expr } => {
                let operand = self.lower_node_expr(fb, expr)?;
                let dest = fb.alloc_slot();
                let uop = match op {
                    SirUn::Neg => UnaryOperator::Minus,
                    SirUn::Not => UnaryOperator::Not,
                };
                fb.push_op(EirOpKind::Unary(UnaryOp {
                    dest,
                    op: uop,
                    operand,
                }));
                Ok(dest)
            }
            SirNode::Binary { op, left, right } => match op {
                SirBin::And => self.lower_and_or(fb, left, right, true),
                SirBin::Or => self.lower_and_or(fb, left, right, false),
                _ => {
                    let l = self.lower_node_expr(fb, left)?;
                    let r = self.lower_node_expr(fb, right)?;
                    let dest = fb.alloc_slot();
                    fb.push_op(EirOpKind::Binary(BinaryOp {
                        dest,
                        op: map_bin(op)?,
                        left: l,
                        right: r,
                        overflow_policy: None,
                    }));
                    Ok(dest)
                }
            },
            SirNode::List { elements } => {
                let mut args = Vec::new();
                for e in elements {
                    args.push(self.lower_node_expr(fb, e)?);
                }
                let dest = fb.alloc_slot();
                fb.push_op(EirOpKind::RuntimeHelper(RuntimeHelperOp {
                    dest: Some(dest),
                    helper_id: HELPER_CONSTRUCT_LIST_ID,
                    args,
                    call_site: None,
                    access_site: None,
                    safepoint_id: None,
                    deopt_id: None,
                }));
                Ok(dest)
            }
            other => Err(EirLowerError::new(format!(
                "not an expression node: {other:?}"
            ))),
        }
    }

    /// Short-circuit: `a and b` ⇒ if a then b else false; `a or b` ⇒ if a then true else b.
    fn lower_and_or(
        &mut self,
        fb: &mut FuncBuilder,
        left: NodeId,
        right: NodeId,
        is_and: bool,
    ) -> Result<SlotId, EirLowerError> {
        let l = self.lower_node_expr(fb, left)?;
        let then_id = fb.new_block_id();
        let else_id = fb.new_block_id();
        let merge_id = fb.new_block_id();
        let result = fb.alloc_slot();
        fb.finish_branch(l, then_id, else_id);

        fb.start_block(then_id);
        if is_and {
            let r = self.lower_node_expr(fb, right)?;
            fb.push_op(EirOpKind::Store(StoreOp::Slot(StoreSlot {
                dest: result,
                value: r,
                check_initialized: None,
            })));
        } else {
            let t = fb.alloc_slot();
            let cid = self.intern_const(Value::Bool(true));
            fb.push_op(EirOpKind::Constant(ConstantOp {
                dest: t,
                constant: cid,
            }));
            fb.push_op(EirOpKind::Store(StoreOp::Slot(StoreSlot {
                dest: result,
                value: t,
                check_initialized: None,
            })));
        }
        if !fb.terminated {
            fb.finish_jump(merge_id);
        }

        fb.start_block(else_id);
        if is_and {
            let f = fb.alloc_slot();
            let cid = self.intern_const(Value::Bool(false));
            fb.push_op(EirOpKind::Constant(ConstantOp {
                dest: f,
                constant: cid,
            }));
            fb.push_op(EirOpKind::Store(StoreOp::Slot(StoreSlot {
                dest: result,
                value: f,
                check_initialized: None,
            })));
        } else {
            let r = self.lower_node_expr(fb, right)?;
            fb.push_op(EirOpKind::Store(StoreOp::Slot(StoreSlot {
                dest: result,
                value: r,
                check_initialized: None,
            })));
        }
        if !fb.terminated {
            fb.finish_jump(merge_id);
        }

        fb.start_block(merge_id);
        Ok(result)
    }

    fn is_print_callee(&self, callee: NodeId) -> bool {
        match self.node(callee) {
            Ok(SirNode::Name { binding }) => self
                .binding_name(*binding)
                .map(|n| n == "print")
                .unwrap_or(false),
            Ok(SirNode::SymbolRef { symbol }) => self
                .unit
                .symbol_text(*symbol)
                .map(|s| s == "print")
                .unwrap_or(false),
            _ => false,
        }
    }

    /// Bootstrap: only list-literal iterators, unrolled (no runtime len helper).
    fn lower_for(
        &mut self,
        fb: &mut FuncBuilder,
        binding: BindingId,
        iter: NodeId,
        body: NodeId,
    ) -> Result<Option<SlotId>, EirLowerError> {
        let elements = match self.node(iter)?.clone() {
            SirNode::List { elements } => elements,
            _ => {
                return Err(EirLowerError::new(
                    "for-loop EIR bootstrap only supports list-literal iterators",
                ));
            }
        };
        let name = self.binding_name(binding)?;
        let mut last = None;
        for elem in elements {
            let v = self.lower_node_expr(fb, elem)?;
            let slot = fb.lookup_binding(binding).unwrap_or_else(|| {
                let s = fb.alloc_slot();
                fb.bind_binding(binding, name.clone(), s);
                s
            });
            // re-bind each iteration into same slot
            if fb.lookup_binding(binding).is_none() {
                fb.bind_binding(binding, name.clone(), slot);
            }
            fb.push_op(EirOpKind::Store(StoreOp::Slot(StoreSlot {
                dest: slot,
                value: v,
                check_initialized: None,
            })));
            last = self.lower_node_stmt(fb, body)?;
            if fb.terminated {
                break;
            }
        }
        Ok(last)
    }
}

fn map_bin(op: SirBin) -> Result<BinaryOperator, EirLowerError> {
    Ok(match op {
        SirBin::Add => BinaryOperator::Add,
        SirBin::Sub => BinaryOperator::Subtract,
        SirBin::Mul => BinaryOperator::Multiply,
        SirBin::Div => BinaryOperator::Divide,
        SirBin::Rem => BinaryOperator::Modulo,
        SirBin::Eq => BinaryOperator::Equal,
        SirBin::NotEq => BinaryOperator::NotEqual,
        SirBin::Lt => BinaryOperator::Less,
        SirBin::LtEq => BinaryOperator::LessEqual,
        SirBin::Gt => BinaryOperator::Greater,
        SirBin::GtEq => BinaryOperator::GreaterEqual,
        SirBin::Is => BinaryOperator::Identity,
        SirBin::In => BinaryOperator::Contains,
        SirBin::And | SirBin::Or => {
            return Err(EirLowerError::new("and/or handled via short-circuit"));
        }
    })
}

struct LoopTargets {
    continue_target: EirBlockId,
    break_target: EirBlockId,
}

struct FuncBuilder {
    eir_id: u32,
    function_id: u32,
    by_binding: HashMap<u32, SlotId>,
    by_name: HashMap<String, SlotId>,
    next_slot: u32,
    blocks: Vec<EirBlock>,
    current_id: EirBlockId,
    current_ops: Vec<EirOp>,
    next_block: u32,
    terminated: bool,
    loop_stack: Vec<LoopTargets>,
}

impl FuncBuilder {
    fn new(eir_id: u32, function_id: u32) -> Self {
        Self {
            eir_id,
            function_id,
            by_binding: HashMap::new(),
            by_name: HashMap::new(),
            next_slot: 0,
            blocks: Vec::new(),
            current_id: EirBlockId::new(0),
            current_ops: Vec::new(),
            next_block: 1,
            terminated: false,
            loop_stack: Vec::new(),
        }
    }

    fn bind_binding(&mut self, id: BindingId, name: String, slot: SlotId) {
        self.by_binding.insert(id.raw(), slot);
        self.by_name.insert(name, slot);
    }

    fn lookup_binding(&self, id: BindingId) -> Option<SlotId> {
        self.by_binding.get(&id.raw()).copied()
    }

    fn alloc_slot(&mut self) -> SlotId {
        let s = SlotId::new(self.next_slot);
        self.next_slot += 1;
        s
    }

    fn push_op(&mut self, kind: EirOpKind) {
        self.current_ops.push(EirOp {
            metadata: OpMetadata {
                source_span: Some(SPAN),
                ..OpMetadata::default()
            },
            kind,
        });
    }

    fn new_block_id(&mut self) -> EirBlockId {
        let id = EirBlockId::new(self.next_block);
        self.next_block += 1;
        id
    }

    fn seal_block(&mut self, terminator: EirTerminator) {
        self.blocks.push(EirBlock {
            block_id: self.current_id,
            parameters: vec![],
            ops: std::mem::take(&mut self.current_ops),
            terminator,
            source_span: Some(SPAN),
        });
        self.terminated = true;
    }

    fn finish_return(&mut self, value: Option<SlotId>) {
        self.seal_block(EirTerminator::Return(Return { value }));
    }

    fn finish_jump(&mut self, target: EirBlockId) {
        self.seal_block(EirTerminator::Jump(Jump {
            target,
            args: vec![],
        }));
    }

    fn finish_branch(&mut self, cond: SlotId, then_block: EirBlockId, else_block: EirBlockId) {
        self.seal_block(EirTerminator::Branch(Branch {
            condition: cond,
            then_block,
            else_block,
        }));
    }

    fn start_block(&mut self, id: EirBlockId) {
        self.current_id = id;
        self.current_ops.clear();
        self.terminated = false;
    }

    fn into_function(mut self) -> EirFunction {
        if !self.terminated {
            self.finish_return(None);
        }
        EirFunction {
            eir_function_id: EirFunctionId::new(self.eir_id),
            function_id: Some(FunctionId::new(self.function_id)),
            module_id: ModuleId::new(0),
            entry_block: EirBlockId::new(0),
            blocks: self.blocks,
            slot_layout: SlotLayoutId::new(0),
            frame_map: FrameMapId::new(0),
            source_span: Some(SPAN),
        }
    }
}

// FnRec needs Clone
impl Clone for FnRec {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            node: self.node,
            params: self.params.clone(),
            body: self.body,
            eir_id: self.eir_id,
            function_id: self.function_id,
            object_id: self.object_id,
        }
    }
}
