# Implementation Status Snapshot

Updated: 2026-07-16  
**Plan:** `PLAN/UNIFIED-IMPLEMENTATION-GUIDANCE.md`

---

## Position

| Track | Status |
|-------|--------|
| T-P1 | COMPLETE |
| T-P2 | S00–S02 COMPLETE |
| T-P3L | **R00–R05 COMPLETE** (bootstrap) |
| CLI | `script-vm` |

---

## Pipeline

```text
source → SIR → EIR → Interpreter
```

### Language surface (bootstrap, via SIR path)

```text
let/const/def, if/while, for[list lit], break/continue
raise/assert, print (stdout)
lists, maps, xs[i], m[k], o.field (map-key)
and/or short-circuit, arith/compare
```

### R05 note

`o.x` is **map string-key** sugar (`index_read/write`), not record field indices yet.

---

## Next

- Record/enum construction + real attribute helpers  
- General for over values  
- finally / structured unwind  
