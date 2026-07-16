# HANDOVER.md

Document class: Agent onboarding and continuity guide  
Normative status: Non-normative (subordinate to frozen specs and `AGENT.md`)  
Audience: New Agent sessions taking over Script VM implementation  
Last updated: 2026-07-14 (WP-00..WP-19 COMPLETE for Phase 3 bootstrap; tip main)

---

## 0. Read This First

本仓库是 **Script VM Phase 1–3 冻结规范** 的 Rust 实现（Phase 3 bootstrap **最小 VM 候选**已闭环）。

新 Agent 上手顺序（强制）：

```text
1. AGENT.md
2. docs/IMPLEMENTATION-STATUS.md     — 实时快照（可重写）
3. docs/agent-plan/IMPLEMENTATION-CODING-PLAN.md
4. docs/agent-plan/WORK-PACKAGE-INDEX.md
5. docs/agent-plan/TRACEABILITY-MATRIX.md
6. PROGRESS.md 尾部 + ISSUE.md 尾部
7. 本文件 HANDOVER.md
```

**权威顺序**：

```text
Frozen Phase 1–3 specs > PHASE-3-FREEZE > Agent plan docs > AGENT.md
  > PROGRESS.md / ISSUE.md > HANDOVER / 临时笔记
```

禁止：改冻结规范、发明 VM 语义、暴露 RuntimePlan/EIR 为公共字节码 ABI、CPython ABI。

---

## 1. 当前状态（可信）

| 项 | 状态 |
|----|------|
| Coding stages 0–14 | **COMPLETE**（Phase 3 bootstrap 范围） |
| WP-00 .. WP-19 | **COMPLETE**（bootstrap / substrate 目标） |
| 有效 OPEN 阻塞审计 | **无** |
| 单元测试（约） | **368**（含 `vm_tests` 88） |
| CI | `check` + `test`×2 + `scripts/integration/g6-scan.sh` |
| G6 | **PASS**（见 `agent/gate-records/G0-G7-20260714-wp19-final.md`） |

**尚未实现（产品级，非本阶段失败）**：

```text
Phase 1 源语言前端流水线
生产 GC / JIT
完整工业语言一致性测试
```

---

## 2. 仓库结构（实现视角）

```text
crates/
  sir / sir_validate     — 源 IR 占位
  vm_core                — EIR、RuntimePlan、错误注册表、缓存键
  vm_runtime             — 堆、帧、调用、unwind、helper 分发（47）
  vm_eval                — EIR fast interpreter
  vm_diag / vm_host / vm_cli
  vm_tests               — WP-18 matrix + WP-19 integration (IG-*)

tests/MATRIX.md          — 符合性矩阵（WP-18 COMPLETE）
scripts/integration/     — G6 扫描 (g6-scan.ps1 / g6-scan.sh)
agent/gate-records/      — G6/G0-G7 与 WP-19 计划/签核
```

验证：

```powershell
cd D:\script
$env:RUSTFLAGS = "-D warnings"
cargo test --workspace
pwsh -File scripts/integration/g6-scan.ps1
```

---

## 3. 若继续开发

1. 先读 `docs/IMPLEMENTATION-STATUS.md` 与 `PROGRESS.md` 尾部。  
2. **不要**重开已 COMPLETE 的 WP-00..19，除非有 ISSUE 阻塞。  
3. 新能力：先加 TRACEABILITY 行 → 测试 → 实现 → PROGRESS 追加。  
4. 规范缺口：走 erratum 流程（`agent/gate-records/WP-19-post-freeze-erratum-policy.md`），禁止改冻结规范。  
5. Phase 1 语言工作：新开 WP / TR-GAP，勿塞进 Phase 3 冻结语义。

---

## 4. 关键交接文件

- `agent/gate-records/WP-19-handoff.md` — WP-19 正式 handoff  
- `agent/gate-records/G0-G7-20260714-wp19-final.md` — 全部门禁  
- `agent/gate-records/WP-19-release-candidate-criteria.md` — 候选标准  

---

## 5. 硬边界（重申）

```text
no public bytecode
RuntimePlan / EIR internal only
no CPython ABI
validation before execution
capability + host boundary gating
structured unwinding preserved
```
