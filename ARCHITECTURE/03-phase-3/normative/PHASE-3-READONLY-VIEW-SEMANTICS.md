# Phase 3 · ReadOnlyView Semantics

Document class: Normative specification  
Normative status: This document defines canonical ReadOnlyView identity, equality, delegation, mutation, and optimization rules for Phase 3 VM specifications.

Created: 2026-06-29 09:31:28

---

## 0. Purpose

This document repairs second-stage audit item:

```text
S2-R2: Add canonical ReadOnlyView semantics.
```

It addresses residual finding:

```text
S2-M02: ReadOnlyView semantics still need a canonical document.
```

---

## 1. ReadOnlyView Boundary

`ReadOnlyView` is a language-visible runtime value that provides shallow read-only access to a target value.

It is not:

```text
deep immutable copy
frozen object
hashability wrapper
ownership transfer
GC lifetime boundary
security sandbox by itself
```

---

## 2. Construction

```text
readonly(value) -> ReadOnlyView
```

If `value` is already a ReadOnlyView, the VM MAY return the same view or create another view.

This choice MUST NOT change observable mutation restrictions.

---

## 3. Target

```text
ReadOnlyViewObj {
  target: Value
  source_span?: SourceSpanId
}
```

The target MUST be root-visible.

If target is heap-backed, ReadOnlyView traces the target.

---

## 4. Identity

A ReadOnlyView has its own runtime identity.

Canonical rule:

```text
readonly(x) is x == false
```

unless the value is an immutable immediate where the VM explicitly defines identity as value identity.

The VM MUST NOT optimize ReadOnlyView identity in a way that makes a mutable aggregate view identical to the original mutable aggregate.

---

## 5. Equality

Equality through ReadOnlyView delegates to target equality unless the language explicitly performs identity comparison.

```text
readonly(x) == x
```

MAY be true if target equality says so.

```text
readonly(x) is x
```

MUST be false for heap-backed mutable aggregates.

---

## 6. Shallow Read-Only Semantics

ReadOnlyView is shallow.

For aggregate target:

```text
view.field read delegates to target field read
view[index] read delegates to target index read
iteration if supported delegates to target iteration
```

Mutation through the view is forbidden.

Nested mutable objects reached through a read operation are not automatically wrapped unless an operation explicitly returns a readonly view.

---

## 7. Mutation Through View

The following MUST raise:

```text
ReadOnlyError
```

when target is reached through a ReadOnlyView:

```text
field write
index write
map insert/replace
list element write
resource state mutation if exposed
any mutating method call
```

Mutating methods accessed through ReadOnlyView MUST be rejected unless the method is explicitly marked non-mutating.

---

## 8. Mutation Through Original Object

ReadOnlyView does not freeze the original object.

If original object is still reachable, mutating the original object changes what the view reads.

Example semantic rule:

```text
let x = [1]
let v = readonly(x)
x[0] = 2
v[0] == 2
```

This is allowed.

---

## 9. Hashability

ReadOnlyView does not make a target hashable.

If target is non-hashable, ReadOnlyView over that target is non-hashable.

If target is hashable immutable value, the view MAY delegate hash/equality according to ValueKey rules only if doing so cannot violate hash stability.

---

## 10. Records

For record instances:

```text
ReadOnlyView(record).field
```

delegates to field read.

```text
ReadOnlyView(record).field = value
```

raises ReadOnlyError.

Fixed-shape record field indexing remains valid under ReadOnlyView if the receiver guard accounts for view unwrap.

---

## 11. Lists and Maps

List and Map reads through ReadOnlyView are allowed.

Writes through ReadOnlyView are forbidden.

Map key hashing/equality still follows ValueKey semantics.

ReadOnlyView over Map does not make map itself hashable.

---

## 12. Functions and Modules

ReadOnlyView over Function or Module is allowed only if the operation is semantically meaningful.

Calling a ReadOnlyView over a function MUST NOT bypass callability checks.

Module mutation through view is forbidden.

---

## 13. Resources

ReadOnlyView over Resource MUST NOT allow resource state mutation.

Closing a resource through a readonly view is a mutating operation and MUST raise ReadOnlyError unless future resource policy explicitly defines close as permitted through readonly handle.

---

## 14. Helper Behavior

Helpers MUST preserve ReadOnlyView rules.

```text
helper_get_attribute
helper_index_read
helper_slice_read
```

may unwrap readonly for read operations.

```text
helper_set_attribute
helper_index_write
helper_close_resource
mutating builtin/helper paths
```

MUST reject readonly mutation.

---

## 15. JIT Requirements

JIT fast paths may unwrap ReadOnlyView for reads only if guard includes:

```text
receiver is ReadOnlyView
target shape/type
operation is non-mutating
```

JIT MUST NOT remove readonly mutation checks.

Compiled mutation through ReadOnlyView MUST raise ReadOnlyError.

---

## 16. GC Requirements

ReadOnlyView target is a root edge.

Moving GC MUST update ReadOnlyView target reference.

---

## 17. Validation

Readonly validation/runtime MUST reject:

```text
mutating operation through ReadOnlyView
mutating method through ReadOnlyView
hashing readonly view over non-hashable target
JIT mutation path missing readonly guard/check
helper mutation path missing readonly check
```

---

## 18. Audit Tracking

This document completes:

```text
S2-R2
```

It addresses:

```text
S2-M02
```
