# Script VM · 唯一实现计划指导书

Document class: Non-normative unified implementation guidance  
Normative status: **Non-normative**  
Authority: Subordinate to frozen Phase 1–3 specifications and freeze declarations  
Created: 2026-07-16 11:56:36  
Status: **ACTIVE — sole forward-looking plan guide**

---

## 0. 地位与适用范围

### 0.1 唯一性声明

自本文件生效起，**面向未来实现的执行方向、阶段顺序、完成标准与 crate 处置**，以本文件为准。

下列文件 **不再单独充当“下一步做什么”的唯一入口**（仍保留历史与门禁工具价值）：

```text
IMPLEMENTATION-CODING-PLAN.md     → 归档为「Phase 3 bootstrap 已执行编码序列」
WORK-PACKAGE-INDEX.md 中 WP-00..19 → 归档为「Phase 3 bootstrap 工作包」
临时扩写的 WP-20..25 叙事         → 降级为「原型/实验记录」，见 §8
docs/IMPLEMENTATION-STATUS.md     → 仅作快照，必须服从本文件阶段定义
```

治理类文件 **继续有效**，且不定义产品阶段顺序：

```text
AGENT.md
AGENT-MASTER-PLAN.md
AGENT-OPERATING-PROTOCOL.md
GATE-CHECKLIST.md
HANDOFF-TEMPLATE.md
TRACEABILITY-MATRIX.md   （须随本计划扩行，见 §10）
RISK-REGISTER.md
```

### 0.2 权威顺序（不可颠倒）

```text
ARCHITECTURE 冻结规范（Phase 1 / 2 / 3 语义）
  > 各 PHASE-*-FREEZE.md
  > 本文件 UNIFIED-IMPLEMENTATION-GUIDANCE.md
  > AGENT-MASTER-PLAN / OPERATING-PROTOCOL / GATE-CHECKLIST
  > AGENT.md
  > WORK-PACKAGE-INDEX（按本文件维护的新 WP）
  > TRACEABILITY-MATRIX
  > PROGRESS.md / ISSUE.md
  > IMPLEMENTATION-STATUS / HANDOVER（快照与交接）
```

本文件 **不得** 修改语言、SIR、RuntimePlan、EIR 或 runtime 语义。  
若实现与冻结规范冲突：**规范赢**；缺口记 ISSUE / erratum，禁止 silent patch。

### 0.3 主副本

```text
Master:  PLAN/UNIFIED-IMPLEMENTATION-GUIDANCE.md
Mirror:  docs/agent-plan/UNIFIED-IMPLEMENTATION-GUIDANCE.md  （内容必须一致）
```

`PLAN/` 与 `docs/agent-plan/` 中其余计划文件：**以 PLAN/ 为 master**，`docs/agent-plan/` 为路由镜像（历史兼容）。  
冲突时以 `PLAN/` 与本文件为准。

---

## 1. 项目真实目标（两层，勿混称“完成”）

### 1.1 产品目标（`PROJECT-OVERVIEW`）

```text
source → lexer → parser → AST → semantic analysis
      → internal IR (SIR)
      → RuntimePlan + EIR（规范 lowering）
      → 校验 → 最小 VM 执行
```

v0 成功样例：

```python
def fib(n):
    if n < 2:
        return n
    return fib(n - 1) + fib(n - 2)

print(fib(10))
```

期望输出：`55`（含真实 `print` 语义时向 stdout 打印）。

### 1.2 Phase 3 bootstrap 目标（**已完成，归档**）

原 `IMPLEMENTATION-CODING-PLAN` Stage 0–14 / WP-00..19：

```text
工作区 + RuntimePlan/EIR 模型与校验 + runtime 基底
+ 最小 EIR 解释器 + 符合性/集成门禁
```

**判定：在 bootstrap 范围内已 CLOSED。**  
后续 **不得** 用“再开 WP-18/19”代替产品管线工作，除非明确 reopen 记录。

### 1.3 禁止的完成标准混用

| 说法 | 是否允许作为 Done |
|------|-------------------|
| “Phase 3 bootstrap 闭环” | ✅ 仅指 WP-00..19 |
| “规范 SIR→RuntimePlan lowering 完成” | ❌ 当前未完成 |
| “AST 直降 EIR 跑通 fib” | ❌ 仅 demo，**不得**标为规范路径完成 |
| “Phase 1 语言前端完成” | ❌ 须按本文件 §5 验收 |

---

## 2. 战略选择（已拍板）

```text
主航道：规范忠实产品管线（ARCHITECTURE 对齐）
优先序：先 Phase 1 实现 → 再 Phase 2 SIR → 再 Phase 3 规范 lowering → 再产品抛光
```

**明确采用「先 Phase 1」**：在 SIR 完整物质化与规范 lowering 之前，先把 **冻结语言表面** 的前端做扎实（词法、语法、AST、语义分析、诊断），作为后续一切 lowering 的唯一语义输入。

**不采用** 以 `script_codegen` 捷径替代规范 lowering 作为主路径。  
该捷径的处置见 §8。

---

## 3. 规范管道终态（所有 Track 的共同终点）

```text
┌────────────── Phase 1 实现（当前主战场）──────────────┐
│  source (UTF-8)                                        │
│    → lexer                                             │
│    → parser + AST                                      │
│    → semantic analysis（绑定/作用域/基础合同）           │
│    → 前端诊断（源位置）                                 │
└───────────────────────────┬────────────────────────────┘
                            ▼
┌────────────── Phase 2 实现 ───────────────────────────┐
│  analyzed AST → SIR (IrUnit + 规范表结构)               │
│    → sir_validate                                      │
└───────────────────────────┬────────────────────────────┘
                            ▼
┌────────────── Phase 3 规范 lowering ──────────────────┐
│  SIR → RuntimePlan + EIR                               │
│    依据：SIR-LOWERING-ROUND1 + CONTROL-LOWERING-ROUND2  │
│    + COVERAGE MATRIX + VALIDATION / CACHE              │
│    → 既有 vm_* 校验与解释器                             │
└───────────────────────────┬────────────────────────────┘
                            ▼
                    产品 v0：fib + print → 55
```

**Phase 3 VM 基底（已有）** 在终态中扮演 **执行与校验后端**，不是“再实现一遍语言”。

---

## 4. Track 划分

| Track ID | 名称 | 状态 | 说明 |
|----------|------|------|------|
| **T-P3B** | Phase 3 Bootstrap | **ARCHIVED / COMPLETE** | WP-00..19；EIR 解释器与 runtime 基底 |
| **T-P1** | Phase 1 Language Frontend | **ACTIVE** | 本文件当前唯一主推 Track |
| **T-P2** | Phase 2 SIR Materialization | BLOCKED on T-P1 gates | 完整 SIR，非玩具节点枚举即可交差 |
| **T-P3L** | Phase 3 Normative Lowering | BLOCKED on T-P2 | SIR→RuntimePlan/EIR，禁止 AST 直降冒充 |
| **T-DEMO** | Demo / Prototype Path | **QUARANTINED** | `script_codegen` 等；可保留测试对照，不进主验收 |
| **T-P3D** | Phase 3 Depth | DEFERRED | 生产 GC/JIT 等，服从 FREEZE deferred |

并行规则：

```text
默认 main-only 推进 T-P1。
未关闭 T-P1 阶段门禁前，不得将 T-P2/T-P3L 标 COMPLETE。
T-DEMO 变更不得写入“规范路径完成”类 PROGRESS 表述。
T-P3B 仅维护性修复与回归，不扩张语义。
```

---

## 5. Track T-P1 · Phase 1 语言前端（详细）

### 5.1 规范依据（必读顺序）

```text
1. ARCHITECTURE/01-phase-1/freeze/PHASE-1-FREEZE.md
2. ARCHITECTURE/01-phase-1/normative/PHASE-1-LANGUAGE-SPEC.md
3. ARCHITECTURE/01-phase-1/normative/PHASE-1-LANGUAGE-DESIGN.md
4. docs/frozen-specs/phase-1/INDEX.md
5. docs/agent-plan/local-reference-map.md 中 SPEC-P1-* 
```

别名：`SPEC-P1-FREEZE` / `SPEC-P1-LANG` / `SPEC-P1-DESIGN`。

### 5.2 T-P1 总完成标准

T-P1 标 **COMPLETE** 当且仅当：

```text
[P1-A] 词法：SPEC-P1 §3–§6 词法相关条可测对齐（含缩进栈、字面量、关键字边界）
[P1-B] 语法：模块级声明/语句/表达式的 AST 覆盖 v0 所需表面，并有拒绝用例
[P1-C] 语义：§2.1–2.2 及声明/赋值/作用域/循环控制等绑定规则可测
[P1-D] 诊断：错误带源位置（行/列或等价 span），可附着到前端错误类型
[P1-E] 差距表：SPEC-P1 相对实现的 GAP 清单受控（显式 DEFERRED 项，非 silent skip）
[P1-F] 输出契约：存在稳定的「经语义分析的模块」API，供 T-P2 消费（不暴露公共字节码）
```

**T-P1 不要求**：跑 VM、生成 EIR、完整 stdlib、生产性能。

### 5.3 T-P1 阶段（新编号，避免与 Stage 0–14 混淆）

使用前缀 **`L`**（Language）：

| 阶段 | 标题 | 目标 | 主要产出 |
|------|------|------|----------|
| **L0** | 计划与差距基线 | 本指导书生效；SPEC-P1 差距表 v0；crate 边界确认 | 本文件；`docs/phase-1/P1-GAP-MATRIX.md` |
| **L1** | 词法收敛 | 以 SPEC-P1 为准审计/加固 `script_lex` | 词法测试矩阵 ↔ §3–§6 |
| **L2** | 语法与 AST 扩展 | 按差距表扩展 `script_parse`（v0 表面优先） | AST + 解析正/负例 |
| **L3** | 语义分析收敛 | 扩展 `script_sema`（作用域、赋值、控制位置、导出标记等） | 语义正/负例 |
| **L4** | 前端诊断与 API 冻结 | span/诊断统一；`check_module`/`AnalyzedModule` 契约 | 可供 T-P2 的稳定入口 |
| **L5** | T-P1 验收 | P1-A..F 清单门禁 + 回归 | T-P1 COMPLETE 记录 |

### 5.4 v0 语言表面（T-P1 必须覆盖的最小集合）

对齐 `PROJECT-OVERVIEW` v0，**至少**：

```text
字面量：nil, bool, int, float, string
运算：算术、比较；逻辑 and/or/not（语义层至少校验操作数类型为 Bool）
绑定：let / const / 赋值规则（禁止隐式新绑定）
控制：if/elif/else, while, return
函数：def、位置参数、调用
模块：顶层声明与语句顺序
print：作为 prelude/host 名在语义层可解析（实现可在 T-P3L/host 落地 I/O）
源位置诊断：词法/语法/语义错误
```

**L2/L3 应按 SPEC 扩，但可分期 DEFER 的例子**（须写入 GAP 矩阵，不得假装已支持）：

```text
match / record / enum 完整形态
try/catch/finally / defer / use
from-import 全形态
类型注解合同的完整运行时检查（可先解析保留 AST）
完整字符串格式化生态
```

### 5.5 Crate 边界（T-P1）

| Crate | 职责 | 备注 |
|-------|------|------|
| `script_lex` | 源文本 → token | T-P1 主维护 |
| `script_parse` | token → AST | T-P1 主维护 |
| `script_sema` | AST → 绑定/语义诊断 | T-P1 主维护 |
| （可选）`script_front` | 统一 facade：`compile_front(source)` | L4 若需要再立，避免过早分层 |

**禁止** 在 T-P1 阶段把语义做进 `vm_runtime` / `vm_eval`。

### 5.6 T-P1 工作包编号（新序列）

自本文件起，**产品管线工作包** 使用：

```text
WP-L00  T-P1 计划生效与 P1-GAP-MATRIX
WP-L01  词法 SPEC 对齐与测试矩阵
WP-L02  语法/AST v0 表面
WP-L03  语义分析 v0 表面
WP-L04  前端诊断与 AnalyzedModule API
WP-L05  T-P1 验收与手交接
```

其后：

```text
WP-S00…  T-P2 SIR（Semantic IR）
WP-R00…  T-P3L 规范 lowering（RuntimePlan/EIR）
WP-V00…  产品 v0 集成（fib+print 规范路径）
```

**停止使用** WP-20..25 作为未来任务 ID（历史提交可保留；索引中标记 SUPERSEDED）。

### 5.7 T-P1 测试纪律

```text
每个 L 阶段：正例 + 负例（拒绝行为必须有测试）
引用 SPEC-P1 章节号写入测试模块文档注释或 MATRIX 行
禁止用“能 codegen 跑通”代替前端验收
```

建议清单文件：

```text
docs/phase-1/P1-GAP-MATRIX.md
docs/phase-1/P1-TEST-MATRIX.md   （可在 L1 建立）
```

---

## 6. 后续 Track 摘要（此刻不展开实现，仅锁顺序）

### 6.1 T-P2 · SIR（在 T-P1 COMPLETE 之后）

规范：

```text
SPEC-P2-FREEZE
SPEC-P2-IR / FRAMEWORK / DESIGN
PHASE-2-SIR-SEMANTICS-ROUND1..3
PHASE-2-SIR-INTEGRATION-ROUND4
```

完成要点：

```text
IrUnit 必选表齐备（可空表但结构在）
符号/作用域/绑定/节点与源映射可追溯
sir_validate 拒绝畸形单位
由 AnalyzedModule 物质化，而非手写玩具图
```

既有 `sir` / `script_lower`：**视为原型**，T-P2 开题时做 keep/rewrite 决策（默认倾向按规范重写表结构）。

### 6.2 T-P3L · 规范 Lowering（在 T-P2 之后）

规范：

```text
PHASE-3-SIR-LOWERING-ROUND1.md
PHASE-3-SIR-LOWERING-COVERAGE-MATRIX.md
PHASE-3-CONTROL-LOWERING-ROUND2.md
PHASE-3-RUNTIMEPLAN-SCHEMA-CLOSURE.md
PHASE-3-EIR-SCHEMA-CLOSURE.md
PHASE-3-VALIDATION-MATRIX.md
```

完成要点：

```text
主路径：SIR → RuntimePlan + EIR → validate → vm_eval
控制流 lowering 不得跳过 ROUND2 职责
禁止将 script_codegen 标为 T-P3L COMPLETE 证据
```

### 6.3 产品 v0 集成

```text
规范路径跑通 fib + print
host/print 语义按 SPEC-P1 与 host boundary
源位置诊断在错误路径上可见
```

---

## 7. 与旧计划文档的关系

| 旧文档 | 新角色 |
|--------|--------|
| `IMPLEMENTATION-CODING-PLAN.md` | **Archive**：T-P3B 已执行序列；不指导 T-P1+ |
| `WORK-PACKAGE-INDEX.md` WP-00..19 | **Archive COMPLETE** |
| `WORK-PACKAGE-INDEX.md` WP-20..25 | **SUPERSEDED** → 见 §8 原型映射 |
| `AGENT-MASTER-PLAN.md` | 继续：Agent 模型、门禁概念、别名表 |
| `GATE-CHECKLIST.md` | 继续：G0–G7；T-P1 任务默认 G0–G5+G7 |
| `TRACEABILITY-MATRIX.md` | 继续维护；新增 TR-L* / 归档 TR-P1-* 实验行 |

---

## 8. 现有代码处置（强制分类）

| 资产 | 分类 | 规则 |
|------|------|------|
| `vm_core` / `vm_runtime` / `vm_eval` / `vm_host` / `vm_diag` / `vm_tests` | **T-P3B 保留** | 回归必须绿；T-P1 不往里塞语言前端 |
| `script_lex` / `script_parse` / `script_sema` | **T-P1 主线资产** | 按 SPEC-P1 收敛，允许破坏性整理 |
| `sir` / `script_lower` | **原型（T-P2 候选）** | 不宣称 Phase 2 完成；T-P2 前仅最小维护 |
| `script_codegen` | **T-DEMO 隔离** | 可保留 `#[cfg(test)]` 或 `demo` 模块注释；**禁止**作为规范完成证据；新功能默认不扩展 |
| `sir_validate` | **T-P2 建设点** | 现状薄弱，T-P2 重点 |
| `vm_cli` | 脚手架 | v0 集成阶段再接线 |

### 8.1 对 WP-20..25 的官方定性

```text
WP-20..23  前端原型探索 — 有价值，纳入 T-P1 资产审计，编号废止
WP-24      薄 SIR 探索 — 非规范完成
WP-25      AST→EIR demo — T-DEMO；fib=55 仅证明解释器+捷径，不证明规范管线
```

---

## 9. 门禁（T-P1）

每个 WP-L*：

```text
G0  范围：不改冻结规范；不碰 T-DEMO 冒充主路径
G1  引用：SPEC-P1-* 具体章节
G2  依赖：仅允许已完成的前置 L 阶段
G3  设计：crate 边界符合 §5.5
G4  实现：代码与测试
G5  验证：cargo test 相关包；正/负例
G6  集成：T-P1 内 API 级即可；全 VM 集成留到 WP-V*
G7  交接：PROGRESS 追加；必要时 ISSUE；更新 P1-GAP-MATRIX
```

Hard stop（同 AGENT.md）：

```text
要求改冻结语义 / 公共字节码 / CPython ABI / 暴露 RuntimePlan·EIR 为公共 ABI
```

---

## 10. Traceability

### 10.1 新行命名

```text
TR-L-000 …    T-P1 语言前端
TR-S-000 …    T-P2 SIR
TR-R-000 …    T-P3L lowering
TR-V-000 …    产品 v0 集成
TR-GAP-*      显式缺口（保留）
```

历史 `TR-P1-00x`（实验期）：在矩阵中标注 `SUPERSEDED by TR-L-*` 或 `DEMO`，不删除历史。

### 10.2 WP-L00 必做

创建 `docs/phase-1/P1-GAP-MATRIX.md`，至少列：

```text
SPEC 章节 | 要求摘要 | 实现状态（YES/PARTIAL/NO） | 证据（测试/文件） | 计划阶段（L1–L5）
```

---

## 11. Agent 会话开工清单（强制）

每次实现会话：

```text
1. 读 AGENT.md
2. 读本文件 §0–§5（T-P1 进行中时）
3. 读 docs/IMPLEMENTATION-STATUS.md 快照
4. 读 P1-GAP-MATRIX（若已存在）
5. 确认当前 WP-L* 与 SPEC-P1 章节
6. 小步实现 → 测试 → PROGRESS 追加 → 按需 commit
7. 禁止扩展 script_codegen 作为“主进度”
```

---

## 12. 近期执行队列（唯一顺序）

```text
现在 → WP-L00  本指导书落地 + P1-GAP-MATRIX v0 + 状态文档改口
     → WP-L01  词法 SPEC 对齐
     → WP-L02  语法/AST v0
     → WP-L03  语义 v0
     → WP-L04  诊断与 AnalyzedModule API
     → WP-L05  T-P1 验收
然后 → T-P2（WP-S00…）
然后 → T-P3L（WP-R00…）
然后 → 产品 v0（WP-V00…）
```

**当前唯一“下一步”：WP-L00。**

---

## 13. 非目标（本指导书有效期内默认）

```text
以 demo codegen 替代规范 lowering
重开 Phase 3 规范设计
生产 GC / JIT
公共字节码 / 公共 IR ABI
CPython 兼容
为赶进度删除负例测试
同时宣称 T-P1 与 T-P3L 完成
```

---

## 14. 变更本指导书

允许：

```text
修正阶段边界、WP 列表、crate 处置、GAP 分期
澄清与冻结规范的引用
```

禁止：

```text
写入新的语言/IR/VM 语义
降低冻结边界（字节码、ABI、capability 等）
```

重大战略变更（例如改回“demo 优先主航道”）必须：

```text
1. 显式修订本节战略 §2
2. 追加 PROGRESS 条目
3. 更新 IMPLEMENTATION-STATUS
```

---

## 15. 合规摘要

Agent / 贡献者符合本指导书当且仅当：

```text
以本文件为唯一前向计划入口
T-P1 完成前不宣称产品管线或规范 lowering 完成
不把 script_codegen / WP-25 当作架构验收
引用 SPEC-P1 章节推进前端
保持 T-P3B 回归绿色
PROGRESS/ISSUE 纪律不变
```

---

## 16. 一句话

> **VM 底板已经有了；从现在起唯一主线是按冻结的 Phase 1 把语言前端做对，再进入 SIR 与规范 lowering——捷径可以留作对照，但不能再当计划本身。**
