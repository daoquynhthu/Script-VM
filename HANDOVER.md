# HANDOVER.md

Document class: Agent onboarding and continuity guide  
Normative status: Non-normative (subordinate to frozen specs and `AGENT.md`)  
Audience: New Agent sessions taking over Script VM implementation  
Last updated: 2026-07-07 (after remediation pass 4b, commit `29eeb9c`)

---

## 0. Read This First

本仓库是 **Script VM Phase 1–3 冻结规范** 的 Rust 实现，不是可随意改语义的 demo 工程。

新 Agent 上手顺序（强制）：

```text
1. AGENT.md                          — 工作流、PROGRESS/ISSUE 规则、硬边界
2. docs/agent-plan/README.md         — Agent 计划包入口
3. docs/agent-plan/IMPLEMENTATION-CODING-PLAN.md — 具体编码阶段顺序
4. docs/agent-plan/WORK-PACKAGE-INDEX.md         — 选择当前 WP
5. docs/agent-plan/TRACEABILITY-MATRIX.md        — 规范引用与测试义务
6. docs/agent-plan/GATE-CHECKLIST.md             — G0–G7 门禁
7. PROGRESS.md 尾部 + ISSUE.md 尾部              — 当前进度与开放审计项
8. 本文件 HANDOVER.md                              — 连续性摘要
```

**权威顺序**（不可颠倒）：

```text
Frozen Phase 1–3 normative specifications
  > PHASE-3-FREEZE.md
  > Agent implementation plan documents (docs/agent-plan/)
  > AGENT.md
  > PROGRESS.md / ISSUE.md
  > HANDOVER.md / 临时笔记
```

禁止：修改冻结规范、发明 VM 语义、暴露 RuntimePlan/EIR 为公共 ABI、堆 demo 绕过测试与门禁。

---

## 1. 仓库结构（实现视角）

```text
D:\script\
  AGENT.md                 — Agent 工程契约（每次会话必读）
  HANDOVER.md              — 本文件
  PROGRESS.md              — 仅追加实现进度（禁止改写历史）
  ISSUE.md                 — 仅追加审计发现（禁止改写历史）

  ARCHITECTURE/03-phase-3/ — 冻结规范原文（normative + implementation-plans）
  docs/agent-plan/         — Agent 计划包（编码顺序、WP、门禁、风险）

  crates/
    sir / sir_validate     — 源 IR（后续阶段）
    vm_core                — EIR、RuntimePlan、错误注册表、缓存键
    vm_runtime             — 堆、帧/槽、调用、unwind、helper 注册表与分发
    vm_eval                — EIR fast interpreter（Stage 12）
    vm_diag                — 诊断与 source span
    vm_host                — Host 边界
    vm_tests               — 跨 crate 集成烟测
    vm_cli                 — CLI 壳（非当前重点）

  tests/                   — Stage 0 脚手架（conformance/negative/… 目录已建，矩阵未填）
```

Workspace 根：`Cargo.toml`。验证命令：

```powershell
cd D:\script
cargo check --workspace
cargo test --workspace
```

当前基线（2026-07-07）：workspace **195** 个单元测试，**0 failed**  
（`sir:1 + vm_core:38 + vm_diag:3 + vm_eval:11 + vm_host:6 + vm_runtime:132 + vm_tests:4`）

---

## 2. 实现阶段与当前位置

依据 `IMPLEMENTATION-CODING-PLAN.md`：

| 阶段 | 内容 | 状态 |
|------|------|------|
| Stage 0–11 | 核心 crate、调用、GC 元数据、缓存 | 已落地（见 PROGRESS 历史） |
| Stage 12 | EIR fast interpreter 最小执行路径 | 已落地（`cdecc82`） |
| Stage 13 | Conformance / regression 矩阵 | **未开始**（`tests/conformance` 等仅 `.gitkeep`） |
| Remediation passes 1–4 | 审计缺口修补 + H1 helper | 已落地（见 §3） |

**编码计划当前“下一步”族**：继续 WP-07 helper 里程碑 **H2**（access/construction/numeric），或按 WORK-PACKAGE-INDEX 选择 Stage 13 / WP-18 相关工作包——须先过 G0–G1 再动代码。

---

## 3. 近期 Remediation 提交链（连续性）

按时间顺序，`main` 上最近提交：

| Commit | 范围 |
|--------|------|
| `2d4006f` | Pass 1：Stage 0 `tests/` 脚手架 + WP-06 EIR 负测（Shape/Field/Case/CallSite/AccessSite/Deopt） |
| `67c21d2` | Pass 2：WP-10 嵌套 unwind 测试 |
| `4d404ab` | Pass 3：WP-08/09 冻结 `SlotState` 四模式 + write barrier |
| `7821236` | Pass 4：WP-07 Milestone **H1** helper dispatch |
| `29eeb9c` | Pass 4b：六个 H1 helper 经 `dispatch_helper` 的专项测试 |

### Pass 3 — 槽语义（`vm_runtime/src/frame.rs` 等）

- `SlotState`：`Uninitialized` / `Value` / `Cell(BindingCellRef)` / `RuntimeInternal(RuntimeValue)`
- 不可变 cell 写 → `ReadOnlyError`（非 TypeError）
- `write_cell`：`TypeContractChecker` + `WriteBarrierHook`
- **未做**：LoadCell/StoreCell interpreter op（留给 WP-17）

### Pass 4 — H1 helper（`vm_runtime/src/helpers/`）

- `h1.rs`：六个 helper 实现体
- `dispatch.rs`：`HelperDispatchEnv` + `dispatch_helper` 中央边界
- 已派发 ID：`0,1,2,6,7,8,29`（alloc / barrier / construct_error / type / callable / hashable / perform_unwind）
- `vm_eval` interpreter：`InterpreterState` 含 `heap`、`callable_registry`、`type_checker`、`write_barrier`
- 未派发 helper → `InvalidHelperError`（负测使用 id `15` = `helper_get_attribute`）

**测试入口（必须通过真实 API，禁止在测试里重写实现）：**

```powershell
cargo test -p vm_runtime helpers::dispatch
cargo test -p vm_runtime helpers::h1
cargo test -p vm_eval interpreter::
```

---

## 4. 新 Agent 会话标准流程

摘自 `AGENT.md` §7、§8、§22，压缩为可执行清单：

```text
□ 读 AGENT.md + 本 HANDOVER + PROGRESS/ISSUE 尾部
□ 在 WORK-PACKAGE-INDEX 选定一个 WP（单 bounded 任务）
□ TRACEABILITY-MATRIX 查规范引用与测试义务
□ RISK-REGISTER 扫 BLOCKER/MAJOR
□ GATE-CHECKLIST 做 G0/G1（实现任务再做 G4–G7）
□ 只改该 WP 范围内文件；跑针对性测试
□ 有文件变更 → 仅追加 PROGRESS.md
□ 有审计发现 → 仅追加 ISSUE.md（与 PROGRESS 分离）
□ 会话结束 → docs/agent-plan/HANDOFF-TEMPLATE.md 格式交班
```

### Git 范围纪律（Remediation 教训）

- **一次 pass = 一次 bounded commit**；不要把多 pass 的 PROGRESS/ISSUE 捆进同一提交。
- Pass N 开始前，Pass N-1 必须已 commit；`git diff --name-only` 应只显示当前 pass 文件。
- 不要顺手改 `Cargo.lock` 除非该 pass 明确引入依赖；无关改动 `git restore Cargo.lock`。
- `PROGRESS.md` Changed Files 必须与 `git diff --name-only` 一致，禁止叙事与 git 不符。

---

## 5. 关键代码地图

| 领域 | 路径 | 冻结规范入口 |
|------|------|----------------|
| EIR 校验 | `crates/vm_core/src/eir/validate.rs` | `PHASE-3-EIR-SCHEMA-CLOSURE.md` |
| 帧/槽 | `crates/vm_runtime/src/frame.rs` | `PHASE-3-EIR-OPERATION-SEMANTICS-ROUND1.md` §2 |
| Binding cell | `crates/vm_runtime/src/binding_cell.rs` | `PHASE-3-FAST-INTERPRETER-DATA-STRUCTURES.md` §7 |
| Unwind | `crates/vm_runtime/src/unwind/` | `PHASE-3-STRUCTURED-UNWINDING-ALGORITHM.md` |
| Helper 注册表 | `crates/vm_runtime/src/helpers/canonical.rs` | `PHASE-3-RUNTIME-HELPER-REGISTRY.md` §3 |
| Helper 分发 | `crates/vm_runtime/src/helpers/dispatch.rs` | `PHASE-3-RUNTIME-HELPER-IMPLEMENTATION-PLAN.md` §20 |
| H1 实现 | `crates/vm_runtime/src/helpers/h1.rs` | §20.2 Milestone H1 |
| Interpreter | `crates/vm_eval/src/interpreter/` | Stage 12 + helper bridge |

Helper ID 与 `canonical.rs` 中 `ROWS` 表顺序一致（0-based）。新增 dispatch 时：

1. 在 family 模块实现 `helper_*`
2. 在 `dispatch.rs` 注册路由
3. **`dispatch_helper` 专项测试**（计划要求，非仅 `h1::` 直调）
4. 如需 interpreter 路径：fixture + `interpreter::` 集成测

---

## 6. 开放项与已知缺口

### ISSUE.md（仍 OPEN / 已接受）

| ID | 摘要 | 处理建议 |
|----|------|----------|
| ISSUE-009 | ReadOnlyView identity 未测 | 留给 WP-13；勿在 helper pass 混做 |
| ISSUE-010 | 双 region-stack 表示 | Status: **ACCEPTED** bootstrap 分裂 |
| 20260701-* 等早期 OPEN | 部分为历史审计占位 | 动手前读 Evidence，避免重复劳动 |

Remediation 已 RESOLVED：ISSUE-005–008（EIR 负测、unwind 嵌套、SlotState）。

### 明确 Non-goals（当前轮次）

- 一次实现全部 47 helpers（按 H0→H1→H2… 里程碑推进）
- LoadCell/StoreCell interpreter ops（WP-17）
- Stage 13 全矩阵 / `tests/conformance` 内容填充（WP-18）
- 标记 WP COMPLETE 而无 G6 集成审查
- CPython ABI / 公共 native 对象布局

---

## 7. 推荐下一 bounded 任务

**默认推荐：Pass 5 — WP-07 Milestone H2**

依据 `PHASE-3-RUNTIME-HELPER-IMPLEMENTATION-PLAN.md` §20.3：

```text
get/set attribute, index read/write, slice read
construct record/enum/map, numeric binary, compare, display
```

实施前：

1. 确认 H1 无回归：`cargo test -p vm_runtime helpers::dispatch`
2. 新建 `helpers/h2.rs`（或按 family 分子模块），扩展 `dispatch_helper`
3. 每个 helper：`dispatch_helper` 测试 + 至少一个 interpreter 或集成路径（若里程碑要求）
4. 未实现 ID 保持 `InvalidHelperError` 负测诚实

**备选：Stage 13 / WP-18** — 在 `tests/conformance` 填首条真实矩阵用例（需单独 WP 与 G0 范围）。

---

## 8. 测试与证据纪律

- 负测必须覆盖 **拒绝路径**（EIR 校验、undispatched helper、immutable cell、非 hashable key 等）。
- 测试必须调用 **已发货 API**（`dispatch_helper`、`SlotArray::write_cell`、`Interpreter::run_module`），不得在测试内复制生产逻辑。
- 目标验证：改哪包测哪包；全仓库 `cargo test --workspace` 在 pass 结束前至少跑两次。
- Goal harness / 审计若要求 scratch 日志：完整 `cargo` stdout，禁止手写摘要冒充。

---

## 9. 硬停止条件（必须上报 ISSUE 并停止）

```text
需要修改冻结规范语义
要求公共 bytecode / 公共 IR ABI
引入 CPython ABI 兼容
暴露 RuntimePlan/EIR 为对外字节码
绕过 host boundary 或 capability
无法保持 structured unwinding
缺少规范引用且无法归类为 erratum/gap
G0 或 G1 失败
```

---

## 10. 交班模板速查

完整格式见 `docs/agent-plan/HANDOFF-TEMPLATE.md`。每次会话结束至少包含：

```text
Work Package:
Changed files:
Frozen spec references:
Gate results (G0–G7):
Tests run / added / not run:
PROGRESS.md entry: yes/no
ISSUE.md entry: yes/no
Risks:
Next bounded action:
```

---

## 11. 联系上下文的历史说明

- 仓库 **不是** 从空脚手架开始；Stage 12 interpreter 与大量 runtime 子系统已存在。
- 近期工作重点是 **Remediation**：在已有实现上补测试、补规范对齐、按里程碑扩展 helper dispatch，而非重写 VM。
- `PROGRESS.md` 中 Pass 3 的 `01:00` 条目写“staged only”已被后续 `4d404ab` commit 取代；以 git 与 `02:00`/`02:30` 条目为准。

---

## 12. 快速命令参考

```powershell
# 状态
git log --oneline -10
git status
git diff --name-only

# 针对性测试
cargo test -p vm_runtime frame::
cargo test -p vm_runtime helpers::dispatch
cargo test -p vm_runtime unwind::
cargo test -p vm_core eir
cargo test -p vm_eval interpreter::

# 全量
cargo check --workspace
cargo test --workspace
```

新 Agent 读完 §0–§4 后即可选取 §7 中的 bounded 任务开工；语义疑问一律回冻结规范，不得凭实现便利发明行为。