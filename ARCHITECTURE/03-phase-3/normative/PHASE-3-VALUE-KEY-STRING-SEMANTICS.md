# Phase 3 · Value Key and String Runtime Semantics

Document class: Normative specification  
Normative status: This document defines canonical map key/hash/equality semantics and string runtime constraints for Phase 3 VM specifications.

Created: 2026-06-29 09:26:37

---

## Normative Interpretation

This document is interpreted under `PHASE-3-NORMATIVE-KEYWORDS-GLOSSARY.md`.

Unmarked planning-style words such as `bootstrap`, `recommended`, `staged`, `first implementation`, `later`, `plan`, or `milestone` do not create implementation-plan status inside this normative document. They are interpreted as one of:

```text
MUST / MUST NOT / SHOULD / MAY
BOOTSTRAP allowance
RECOMMENDED implementation option
DEFERRED design area
NON-NORMATIVE NOTE
```

according to their local context and the normative keyword policy.

If this document conflicts with a later canonical repair document, the later canonical repair document owns the repaired term or schema.



## 0. Purpose

This document repairs audit item:

```text
R12: Define map key/hash/equality and string runtime constraints.
```

It addresses major findings:

```text
M-06: Map key/hash/equality semantics need closure.
M-07: String model needs sharper runtime constraints.
```

---

## 1. ValueKey

Map keys are represented by canonical `ValueKey`.

```text
ValueKey =
  | BoolKey(Bool)
  | IntKey(IntCanonical)
  | FloatKey(FloatCanonical)
  | StringKey(StringCanonical)
  | EnumKey(EnumIdentity, CaseIndex, PayloadKey?)
  | NilKey
```

Phase 3 minimal VM MUST NOT permit mutable aggregate values as map keys.

Forbidden as map keys:

```text
List
Map
RecordInstance
ReadOnlyView over mutable aggregate
Function
BuiltinFunction
Module
Resource
Error
HostObjectWrapper
```

unless a future explicit hash protocol is added.

---

## 2. Hashability Rule

A value is hashable only if:

```text
its equality is stable
its hash is stable
it cannot be mutated in a way that changes equality/hash while stored as key
```

Readonly view does not automatically make an underlying mutable object hashable.

---

## 3. Int Keys

Int keys use mathematical integer value.

If implementation uses checked i64, key canonicalization is i64 value.

If implementation uses arbitrary precision, key canonicalization is arbitrary-precision integer value.

---

## 4. Float Keys

Float keys are allowed only for finite Float values.

NaN keys are forbidden in Phase 3 minimal VM.

Infinity handling follows Float runtime policy; if Infinity is supported as ordinary Float, it MAY be allowed as FloatKey only if equality/hash are stable. Serializable values still require finite Float.

### 4.1 Negative Zero

`-0.0` and `0.0` MUST compare equal as Float values unless FloatPolicy later defines stricter distinction.

If they compare equal, they MUST hash to the same FloatKey.

---

## 5. String Keys

String keys use string scalar sequence equality.

String identity/interning MUST NOT affect key equality.

Two strings with the same scalar sequence are the same StringKey.

---

## 6. Enum Keys

Enum values MAY be hashable only if:

```text
enum identity is nominal and stable
case identity is stable
payload is absent or payload is itself hashable
```

If payload contains non-hashable value, enum value is non-hashable.

---

## 7. Equality and Hash Consistency

For all hashable keys:

```text
a == b implies hash(a) == hash(b)
```

Hash collision is allowed.

Hash collision MUST NOT imply equality.

---

## 8. Map Duplicate Key Rule

When constructing a map:

```text
later value replaces earlier value
first insertion position is preserved
```

Example semantic sequence:

```text
insert k -> position p
insert k again -> update value at p
```

Iteration order uses first insertion position.

---

## 9. Map Iteration

Map iteration yields keys in insertion order.

For `for` over Map:

```text
iteration value = key
order = insertion order
```

---

## 10. String Runtime Model

A String is an immutable Unicode scalar sequence.

The VM may store strings as UTF-8, UTF-16, rope, interned object, or other internal representation.

Internal representation is not public ABI.

---

## 11. String Length

`len(String)` returns the number of Unicode scalar values.

It does not return bytes.

If future language revision changes length semantics, this document must be reopened.

---

## 12. String Indexing

String indexing is not core Phase 3.

A source program MUST NOT assume:

```text
s[i] returns character
s[i] returns byte
s[i] returns scalar
```

String indexing must be rejected unless a later feature explicitly enables it.

---

## 13. String Slicing

String slicing is core if slice operation is available for strings.

Slice bounds are scalar indices.

Rules:

```text
slice is half-open [start, end)
start and end must be Int
negative bounds raise IndexError
out-of-range bounds raise IndexError
start > end raises IndexError
slice result preserves scalar sequence
```

The VM MUST NOT split invalid internal encoding boundaries.

If internal representation is UTF-8, scalar index to byte offset mapping is VM-internal.

---

## 14. String Constants and Interning

The VM MAY intern string constants.

Interning MUST NOT change:

```text
equality
hashing
display
serialization
source-visible behavior
```

Identity of strings is implementation-defined unless the language later defines string identity semantics.

JIT MUST NOT rely on string object identity for equality unless guarded by canonical interning policy.

---

## 15. Display Semantics

`helper_display` converts values to display strings.

Display conversion does not create implicit coercion for:

```text
binary operators
type contracts
map keys
pattern matching
```

String display of a String value returns its scalar sequence or escaped diagnostic representation depending on call site.

---

## 16. Format Strings

Format strings evaluate embedded expressions left-to-right.

Each embedded value is converted through display semantics.

Display failures raise language error at the embedded expression source span if possible.

---

## 17. Validation

Validation/runtime MUST reject:

```text
non-hashable map key
NaN map key
mutable aggregate as map key
string indexing in core Phase 3
negative string slice bound
out-of-range string slice bound
string slice start > end
hash/equality mismatch for key type
```

---

## 18. Audit Tracking

This document completes:

```text
R12
```

It addresses:

```text
M-06
M-07
```

It supports:

```text
R13
```
