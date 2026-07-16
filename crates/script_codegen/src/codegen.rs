//! Lower Phase 1 AST to bootstrap EIR for `vm_eval`.
//!
//! Spec: Phase 1 language surface; Phase 3 EIR schema (bootstrap subset).
//! Non-goals: full SIR schema fidelity, production optimizer IR.

use std::collections::HashMap;

use script_parse::{
    BinaryOp as AstBin, Block, Decl, Expr, Item, Module, Stmt, UnaryOp as AstUn,
};
use script_sema::analyze_module;
use vm_core::digest::Digest;
use vm_core::eir::schema::{
    BinaryOp, BinaryOperator, Branch, ConstantEntry, ConstantOp, ConstantPool, EirBlock,
    EirFunction, EirModule, EirOp, EirOpKind, EirTerminator, Jump, LoadOp, LoadSlot, OpMetadata,
    Return, RuntimeHelperOp, StoreOp, StoreSlot, UnaryOp, UnaryOperator,
};
use vm_core::id::{
    ConstantId, EirBlockId, EirFunctionId, FrameMapId, FunctionId, ModuleId, ObjectId, SlotId,
    SlotLayoutId,
};
use vm_core::profile::Version;
use vm_core::value::Value;
use vm_diag::source_span::SourceSpanId;
use vm_runtime::helpers::dispatch::HELPER_GENERIC_CALL_ID;

use crate::error::CodegenError;
use crate::program::{user_fn_target, CompiledProgram};

const SPAN: SourceSpanId = SourceSpanId::new(1);
/// Object ids for user functions start here (avoid clashing with test fixtures).
const FN_OBJECT_BASE: u32 = 1000;
const PRINT_OBJECT_ID: u32 = 999;

/// Compile source: parse → sema → EIR.
pub fn compile_source(source: &str) -> Result<CompiledProgram, CodegenError> {
    let module = script_parse::parse_module(source)
        .map_err(|e| CodegenError::new(e.to_string()))?;
    compile_module(&module)
}

/// Compile a parsed module after semantic check.
pub fn compile_module(module: &Module) -> Result<CompiledProgram, CodegenError> {
    let sema = analyze_module(module);
    if !sema.ok() {
        let msgs: Vec<_> = sema.errors.iter().map(|e| e.message.clone()).collect();
        return Err(CodegenError::new(format!("sema failed: {}", msgs.join("; "))));
    }
    Codegen::new().compile(module)
}

struct FnInfo {
    name: String,
    params: Vec<String>,
    body: Block,
    eir_id: u32,
    function_id: u32,
    object_id: ObjectId,
}

struct Codegen {
    constants: ConstantPool,
    next_const: u32,
    fn_table: HashMap<String, FnInfo>,
    callables: Vec<(ObjectId, vm_runtime::call::callable::CallableTarget)>,
    /// Name → constant id holding ObjectRef for that function.
    fn_const: HashMap<String, ConstantId>,
}

impl Codegen {
    fn new() -> Self {
        Self {
            constants: ConstantPool::default(),
            next_const: 0,
            fn_table: HashMap::new(),
            callables: Vec::new(),
            fn_const: HashMap::new(),
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

    fn compile(mut self, module: &Module) -> Result<CompiledProgram, CodegenError> {
        // Collect function declarations (module scope).
        let mut fn_index = 0u32;
        for item in &module.items {
            let func = match item {
                Item::Decl(Decl::Function {
                    name,
                    params,
                    body,
                    ..
                }) => Some((name, params, body)),
                Item::Decl(Decl::Export { item: inner, .. }) => match inner.as_ref() {
                    Decl::Function {
                        name,
                        params,
                        body,
                        ..
                    } => Some((name, params, body)),
                    _ => None,
                },
                _ => None,
            };
            if let Some((name, params, body)) = func {
                let eir_id = fn_index + 1; // 0 reserved for $main
                let function_id = eir_id;
                let object_id = ObjectId::new(FN_OBJECT_BASE + fn_index);
                let info = FnInfo {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                    eir_id,
                    function_id,
                    object_id,
                };
                let cid = self.intern_const(Value::ObjectRef(object_id));
                self.fn_const.insert(name.clone(), cid);
                self.callables.push((
                    object_id,
                    user_fn_target(object_id, function_id, eir_id),
                ));
                self.fn_table.insert(name.clone(), info);
                fn_index += 1;
            }
        }

        // Bootstrap `print`: identity user function (returns first arg / None).
        let print_obj = ObjectId::new(PRINT_OBJECT_ID);
        let print_eir = fn_index + 1;
        let print_cid = self.intern_const(Value::ObjectRef(print_obj));
        self.fn_const.insert("print".into(), print_cid);
        self.callables.push((
            print_obj,
            user_fn_target(print_obj, print_eir, print_eir),
        ));

        // Lower user functions (stable eir ids).
        let fn_names: Vec<String> = self.fn_table.keys().cloned().collect();
        let mut user_fns = Vec::new();
        for name in fn_names {
            let info = self.fn_table.get(&name).unwrap().clone();
            user_fns.push(self.lower_function(&info)?);
        }
        // print body: return first argument (slot 0)
        let print_fn = self.make_print_function(print_eir);

        // $main from top-level statements / non-function decls
        let main = self.lower_main(module)?;

        let mut functions = vec![main];
        functions.extend(user_fns);
        functions.push(print_fn);

        let module_eir = EirModule {
            eir_version: Version::new(1, 0, 0),
            source_runtime_plan_digest: Digest(0xC0DE_6E00),
            functions,
            constants: self.constants,
            source_map: Default::default(),
            root_maps: Default::default(),
            safepoints: Default::default(),
            deopt_points: Default::default(),
        };

        Ok(CompiledProgram {
            module: module_eir,
            entry: EirFunctionId::new(0),
            callables: self.callables,
        })
    }

    fn make_print_function(&self, eir_id: u32) -> EirFunction {
        // Returns slot 0 (first arg); if no args, return None via empty — arity usually 1.
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

    fn lower_function(&mut self, info: &FnInfo) -> Result<EirFunction, CodegenError> {
        let mut fb = FuncBuilder::new(info.eir_id, info.function_id);
        for (i, p) in info.params.iter().enumerate() {
            fb.bind_name(p.clone(), SlotId::new(i as u32));
            fb.next_slot = fb.next_slot.max(i as u32 + 1);
        }
        let result_slot = self.lower_block_stmts(&mut fb, &info.body.stmts)?;
        // If body didn't return, return last expr result or None.
        if !fb.terminated {
            let slot = result_slot.unwrap_or_else(|| {
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

    fn lower_main(&mut self, module: &Module) -> Result<EirFunction, CodegenError> {
        let mut fb = FuncBuilder::new(0, 0);
        let mut last: Option<SlotId> = None;
        for item in &module.items {
            match item {
                Item::Decl(Decl::Function { .. }) => {}
                Item::Decl(Decl::Export { item: inner, .. })
                    if matches!(inner.as_ref(), Decl::Function { .. }) => {}
                Item::Decl(d) => {
                    last = self.lower_decl(&mut fb, d)?;
                }
                Item::Stmt(s) => {
                    last = self.lower_stmt(&mut fb, s)?;
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

    fn lower_decl(
        &mut self,
        fb: &mut FuncBuilder,
        decl: &Decl,
    ) -> Result<Option<SlotId>, CodegenError> {
        match decl {
            Decl::Let { name, value, .. } | Decl::Const { name, value, .. } => {
                let v = self.lower_expr(fb, value)?;
                let slot = fb.alloc_slot();
                fb.push_op(EirOpKind::Store(StoreOp::Slot(StoreSlot {
                    dest: slot,
                    value: v,
                    check_initialized: None,
                })));
                fb.bind_name(name.clone(), slot);
                Ok(Some(slot))
            }
            Decl::Function { .. } => Ok(None),
            Decl::Import { .. } | Decl::FromImport { .. } => {
                // Bootstrap: import binds a placeholder None (no multi-module runtime yet).
                Ok(None)
            }
            Decl::Export { item, .. } => self.lower_decl(fb, item),
        }
    }

    fn lower_block_stmts(
        &mut self,
        fb: &mut FuncBuilder,
        stmts: &[Stmt],
    ) -> Result<Option<SlotId>, CodegenError> {
        let mut last = None;
        for s in stmts {
            if fb.terminated {
                break;
            }
            last = self.lower_stmt(fb, s)?;
        }
        Ok(last)
    }

    fn lower_stmt(
        &mut self,
        fb: &mut FuncBuilder,
        stmt: &Stmt,
    ) -> Result<Option<SlotId>, CodegenError> {
        match stmt {
            Stmt::Expr { expr, .. } => Ok(Some(self.lower_expr(fb, expr)?)),
            Stmt::Return { value, .. } => {
                let slot = match value {
                    Some(e) => Some(self.lower_expr(fb, e)?),
                    None => None,
                };
                fb.finish_return(slot);
                Ok(slot)
            }
            Stmt::Assign { name, value, .. } => {
                let v = self.lower_expr(fb, value)?;
                let dest = fb
                    .lookup(name)
                    .ok_or_else(|| CodegenError::new(format!("unbound `{name}`")))?;
                fb.push_op(EirOpKind::Store(StoreOp::Slot(StoreSlot {
                    dest,
                    value: v,
                    check_initialized: None,
                })));
                Ok(Some(dest))
            }
            Stmt::IndexAssign { .. } | Stmt::AttrAssign { .. } | Stmt::Try { .. } => {
                Err(CodegenError::new(
                    "index/attr/try not supported in demo codegen; use script_eir_lower",
                ))
            }
            Stmt::AugAssign { name, op, value, .. } => {
                // Expand `x += e` to `x = x + e` for demo codegen only.
                let dest = fb
                    .lookup(name)
                    .ok_or_else(|| CodegenError::new(format!("unbound `{name}`")))?;
                let rhs = self.lower_expr(fb, value)?;
                let bin = match op {
                    script_parse::AugOp::Add => BinaryOperator::Add,
                    script_parse::AugOp::Sub => BinaryOperator::Subtract,
                    script_parse::AugOp::Mul => BinaryOperator::Multiply,
                    script_parse::AugOp::Div => BinaryOperator::Divide,
                    script_parse::AugOp::Rem => BinaryOperator::Modulo,
                };
                let tmp = fb.alloc_slot();
                fb.push_op(EirOpKind::Binary(BinaryOp {
                    dest: tmp,
                    op: bin,
                    left: dest,
                    right: rhs,
                    overflow_policy: None,
                }));
                fb.push_op(EirOpKind::Store(StoreOp::Slot(StoreSlot {
                    dest,
                    value: tmp,
                    check_initialized: None,
                })));
                Ok(Some(dest))
            }
            Stmt::If {
                cond,
                then_block,
                elifs,
                else_block,
                ..
            } => self.lower_if(fb, cond, then_block, elifs, else_block),
            Stmt::While { cond, body, .. } => self.lower_while(fb, cond, body),
            Stmt::For { .. } => Err(CodegenError::new(
                "for-loops not yet lowered in bootstrap codegen",
            )),
            Stmt::Break { .. } | Stmt::Continue { .. } => Err(CodegenError::new(
                "break/continue lowering requires loop context blocks (deferred)",
            )),
            Stmt::Raise { value, .. } => {
                // Bootstrap: evaluate then return as if error — not full raise path.
                let _ = self.lower_expr(fb, value)?;
                Err(CodegenError::new("raise not yet lowered to EIR Raise terminator"))
            }
            Stmt::Assert { cond, .. } => {
                // Evaluate condition; ignore failure path in bootstrap.
                Ok(Some(self.lower_expr(fb, cond)?))
            }
            Stmt::Decl(d) => self.lower_decl(fb, d),
        }
    }

    fn lower_if(
        &mut self,
        fb: &mut FuncBuilder,
        cond: &Expr,
        then_block: &Block,
        elifs: &[(Expr, Block)],
        else_block: &Option<Block>,
    ) -> Result<Option<SlotId>, CodegenError> {
        // Flatten elif into nested if for simplicity.
        if let Some((econd, ebody)) = elifs.first() {
            let rest_elifs = &elifs[1..];
            let nested_else = Some(Block {
                stmts: vec![Stmt::If {
                    cond: econd.clone(),
                    then_block: ebody.clone(),
                    elifs: rest_elifs.to_vec(),
                    else_block: else_block.clone(),
                    span: ebody.span,
                }],
                span: ebody.span,
            });
            return self.lower_if(fb, cond, then_block, &[], &nested_else);
        }

        let c = self.lower_expr(fb, cond)?;
        let then_id = fb.new_block_id();
        let else_id = fb.new_block_id();
        let merge_id = fb.new_block_id();

        fb.finish_branch(c, then_id, else_id);

        // then
        fb.start_block(then_id);
        let then_slot = self.lower_block_stmts(fb, &then_block.stmts)?;
        if !fb.terminated {
            fb.finish_jump(merge_id);
        }

        // else
        fb.start_block(else_id);
        let else_slot = if let Some(b) = else_block {
            self.lower_block_stmts(fb, &b.stmts)?
        } else {
            None
        };
        if !fb.terminated {
            fb.finish_jump(merge_id);
        }

        // merge
        fb.start_block(merge_id);
        // Pick a result slot: prefer then/else if both present — bootstrap uses then or else or None.
        Ok(then_slot.or(else_slot))
    }

    fn lower_while(
        &mut self,
        fb: &mut FuncBuilder,
        cond: &Expr,
        body: &Block,
    ) -> Result<Option<SlotId>, CodegenError> {
        let header = fb.new_block_id();
        let body_id = fb.new_block_id();
        let exit = fb.new_block_id();
        fb.finish_jump(header);

        fb.start_block(header);
        let c = self.lower_expr(fb, cond)?;
        fb.finish_branch(c, body_id, exit);

        fb.start_block(body_id);
        let _ = self.lower_block_stmts(fb, &body.stmts)?;
        if !fb.terminated {
            fb.finish_jump(header);
        }

        fb.start_block(exit);
        Ok(None)
    }

    fn lower_expr(&mut self, fb: &mut FuncBuilder, expr: &Expr) -> Result<SlotId, CodegenError> {
        match expr {
            Expr::Nil { .. } => {
                let s = fb.alloc_slot();
                let cid = self.intern_const(Value::None);
                fb.push_op(EirOpKind::Constant(ConstantOp {
                    dest: s,
                    constant: cid,
                }));
                Ok(s)
            }
            Expr::Bool { value, .. } => {
                let s = fb.alloc_slot();
                let cid = self.intern_const(Value::Bool(*value));
                fb.push_op(EirOpKind::Constant(ConstantOp {
                    dest: s,
                    constant: cid,
                }));
                Ok(s)
            }
            Expr::Int { value, .. } => {
                let n: i64 = value
                    .parse()
                    .map_err(|_| CodegenError::new(format!("bad int literal {value}")))?;
                let s = fb.alloc_slot();
                let cid = self.intern_const(Value::Int(n));
                fb.push_op(EirOpKind::Constant(ConstantOp {
                    dest: s,
                    constant: cid,
                }));
                Ok(s)
            }
            Expr::Float { value, .. } => {
                let n: f64 = value
                    .parse()
                    .map_err(|_| CodegenError::new(format!("bad float literal {value}")))?;
                let s = fb.alloc_slot();
                let cid = self.intern_const(Value::Float(n));
                fb.push_op(EirOpKind::Constant(ConstantOp {
                    dest: s,
                    constant: cid,
                }));
                Ok(s)
            }
            Expr::String { value, .. } => {
                let s = fb.alloc_slot();
                let cid = self.intern_const(Value::String(value.clone()));
                fb.push_op(EirOpKind::Constant(ConstantOp {
                    dest: s,
                    constant: cid,
                }));
                Ok(s)
            }
            Expr::Name { name, .. } => {
                if let Some(slot) = fb.lookup(name) {
                    let dest = fb.alloc_slot();
                    fb.push_op(EirOpKind::Load(LoadOp::Slot(LoadSlot {
                        dest,
                        source: slot,
                        require_initialized: true,
                    })));
                    return Ok(dest);
                }
                if let Some(cid) = self.fn_const.get(name).copied() {
                    let dest = fb.alloc_slot();
                    fb.push_op(EirOpKind::Constant(ConstantOp {
                        dest,
                        constant: cid,
                    }));
                    return Ok(dest);
                }
                Err(CodegenError::new(format!("unresolved name `{name}` in codegen")))
            }
            Expr::Call { callee, args, .. } => {
                let c = self.lower_expr(fb, callee)?;
                let mut arg_slots = vec![c];
                for a in args {
                    arg_slots.push(self.lower_expr(fb, a)?);
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
            Expr::Unary { op, expr, .. } => {
                let operand = self.lower_expr(fb, expr)?;
                let dest = fb.alloc_slot();
                let uop = match op {
                    AstUn::Neg => UnaryOperator::Minus,
                    AstUn::Not => UnaryOperator::Not,
                };
                fb.push_op(EirOpKind::Unary(UnaryOp {
                    dest,
                    op: uop,
                    operand,
                }));
                Ok(dest)
            }
            Expr::Binary {
                op, left, right, ..
            } => {
                let l = self.lower_expr(fb, left)?;
                let r = self.lower_expr(fb, right)?;
                let dest = fb.alloc_slot();
                // Logical and/or: bootstrap as binary non-short-circuit using Equal/etc not available —
                // use nested if would be better; for fib we only need arith + compare.
                match op {
                    AstBin::And | AstBin::Or => {
                        return Err(CodegenError::new(
                            "logical and/or short-circuit not yet lowered",
                        ));
                    }
                    _ => {}
                }
                let bop = map_bin(*op)?;
                fb.push_op(EirOpKind::Binary(BinaryOp {
                    dest,
                    op: bop,
                    left: l,
                    right: r,
                    overflow_policy: None,
                }));
                Ok(dest)
            }
            Expr::List { .. } => Err(CodegenError::new("list literals not yet lowered to EIR")),
            Expr::Map { .. } => Err(CodegenError::new("map literals not yet lowered to EIR")),
            Expr::Index { .. } | Expr::Attr { .. } => {
                Err(CodegenError::new("index/attr not supported in demo codegen"))
            }
        }
    }
}

fn map_bin(op: AstBin) -> Result<BinaryOperator, CodegenError> {
    Ok(match op {
        AstBin::Add => BinaryOperator::Add,
        AstBin::Sub => BinaryOperator::Subtract,
        AstBin::Mul => BinaryOperator::Multiply,
        AstBin::Div => BinaryOperator::Divide,
        AstBin::Rem => BinaryOperator::Modulo,
        AstBin::Eq => BinaryOperator::Equal,
        AstBin::NotEq => BinaryOperator::NotEqual,
        AstBin::Lt => BinaryOperator::Less,
        AstBin::LtEq => BinaryOperator::LessEqual,
        AstBin::Gt => BinaryOperator::Greater,
        AstBin::GtEq => BinaryOperator::GreaterEqual,
        AstBin::Is => BinaryOperator::Identity,
        AstBin::In => BinaryOperator::Contains,
        AstBin::And | AstBin::Or => {
            return Err(CodegenError::new("internal: and/or mapped as binary"));
        }
    })
}

/// Per-function EIR block builder.
struct FuncBuilder {
    eir_id: u32,
    function_id: u32,
    names: HashMap<String, SlotId>,
    next_slot: u32,
    blocks: Vec<EirBlock>,
    current_id: EirBlockId,
    current_ops: Vec<EirOp>,
    next_block: u32,
    terminated: bool,
}

impl FuncBuilder {
    fn new(eir_id: u32, function_id: u32) -> Self {
        Self {
            eir_id,
            function_id,
            names: HashMap::new(),
            next_slot: 0,
            blocks: Vec::new(),
            current_id: EirBlockId::new(0),
            current_ops: Vec::new(),
            next_block: 1,
            terminated: false,
        }
    }

    fn bind_name(&mut self, name: String, slot: SlotId) {
        self.names.insert(name, slot);
    }

    fn lookup(&self, name: &str) -> Option<SlotId> {
        self.names.get(name).copied()
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
        // If current block open without seal, something wrong — seal with halt return None.
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

// Need Clone on FnInfo - Block and such already Clone
impl Clone for FnInfo {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            params: self.params.clone(),
            body: self.body.clone(),
            eir_id: self.eir_id,
            function_id: self.function_id,
            object_id: self.object_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vm_core::control::ControlState;
    use vm_eval::Interpreter;

    fn run(src: &str) -> ControlState {
        let prog = compile_source(src).expect("compile");
        let mut interp = Interpreter::new();
        prog.install_callables(&mut interp.state_mut().callable_registry);
        interp.run_module(&prog.module, prog.entry)
    }

    #[test]
    fn compile_literal_return() {
        let r = run("1 + 2\n");
        assert_eq!(r, ControlState::Return(Some(Value::Int(3))));
    }

    #[test]
    fn compile_let_and_arith() {
        let r = run("let x = 10\nlet y = 5\nx - y\n");
        assert_eq!(r, ControlState::Return(Some(Value::Int(5))));
    }

    #[test]
    fn compile_if() {
        let r = run("if true:\n    1\nelse:\n    0\n");
        assert_eq!(r, ControlState::Return(Some(Value::Int(1))));
    }

    #[test]
    fn compile_fib_10() {
        let src = r#"
def fib(n):
    if n < 2:
        return n
    return fib(n - 1) + fib(n - 2)

fib(10)
"#;
        let r = run(src);
        assert_eq!(r, ControlState::Return(Some(Value::Int(55))));
    }

    #[test]
    fn compile_print_fib() {
        let src = r#"
def fib(n):
    if n < 2:
        return n
    return fib(n - 1) + fib(n - 2)

print(fib(10))
"#;
        let r = run(src);
        assert_eq!(r, ControlState::Return(Some(Value::Int(55))));
    }
}
