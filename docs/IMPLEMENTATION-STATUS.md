# Implementation Status Snapshot

Updated: 2026-07-16  
**Plan:** `PLAN/UNIFIED-IMPLEMENTATION-GUIDANCE.md`

---

## Position

| Track | Status |
|-------|--------|
| T-P1 | COMPLETE |
| T-P2 | S00–S02 COMPLETE |
| T-P3L | **R00–R07 COMPLETE** (bootstrap) |
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
lists, maps, xs[i], m[k], o.field (map-key)
and/or short-circuit, arith/compare
try / catch / finally (soft raise/return handlers)
```

### R05 note

`o.x` is **map string-key** sugar (`index_read/write`), not record field indices yet.

### R06 note

`try/catch/finally` uses **soft** pending-kind routing in EIR (kind 0 normal / 1 return / 2 raise).
Not full structured unwinding / PendingRaise frame model yet.

### R07 note

`for x in xs` lowers to `list_len` + index `while` (helper id 47). Map/string iteration deferred.
`helper_list_len` is a bootstrap registry extension (48 helpers).

---

## Next

- Record/enum construction + real attribute helpers  
- Map for-in (optional)  
- RuntimePlan-from-SIR metadata deepen  
- Harden try: nested finally identity, multi-catch guards, structured unwind fidelity  
