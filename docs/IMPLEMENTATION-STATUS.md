# Implementation Status Snapshot

Updated: 2026-07-16  
**Plan:** `PLAN/UNIFIED-IMPLEMENTATION-GUIDANCE.md`

---

## Position

| Track | Status |
|-------|--------|
| T-P1 | COMPLETE |
| T-P2 | S00–S02 COMPLETE |
| T-P3L | **R00–R08 COMPLETE** (bootstrap) |
| CLI | `script-vm` |

---

## Pipeline

```text
source → SIR → EIR → Interpreter
```

### Language surface (bootstrap, via SIR path)

```text
let/const/def, if/while, for-in[list values], break/continue
raise/assert, print (stdout)
lists, maps, xs[i], m[k], o.field (map-key sugar)
record types: record Point: field x; Point(x = 1); p.x / mutable write
and/or short-circuit, arith/compare
try / catch / finally (soft raise/return handlers)
```

### Notes

- **R05:** map-key `o.x` when receiver is a map (`field_index = None`).
- **R06:** soft try handlers (pending kind 0/1/2).
- **R07:** `for` via `list_len` + index while; helper registry 48.
- **R08:** records use `construct_record` + `get_attribute`/`set_attribute` by field index.
  Named constructor args only. No methods / field defaults / type annotations yet.

---

## Next

- Enum definition + cases  
- Record methods / field defaults  
- RuntimePlan-from-SIR deepen  
- Harden try structured unwind  
