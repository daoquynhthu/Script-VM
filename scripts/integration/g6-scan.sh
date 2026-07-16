#!/usr/bin/env bash
# WP-19 / Stage 14 G6 integration scan (CI / Unix).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"
FAIL=0
fail() { echo "FAIL: $*" >&2; FAIL=$((FAIL + 1)); }
pass() { echo "PASS: $*"; }

echo "=== G6 integration scan (WP-19) ==="
echo "root: $ROOT"

SRC=$(find crates -name '*.rs' -not -path '*/target/*' -print0 | xargs -0 cat)

if echo "$SRC" | grep -Eiq 'CPython|cpython_abi|PyObject[[:space:]]*\*'; then
  fail "CPython ABI symbols found in crate sources"
else
  pass "no CPython ABI symbols in crate sources"
fi

# Exposure markers only (reject_public_bytecode_cache_claim and error messages are allowed).
if grep -REn 'export_bytecode|PublicBytecodeAbi|publish_bytecode|pub[[:space:]]+fn[[:space:]]+emit_bytecode' \
    crates --include='*.rs' >/dev/null 2>&1; then
  fail "public bytecode exposure markers found"
else
  pass "no public bytecode exposure markers"
fi

if grep -REn 'extern[[:space:]]+"C"' crates/vm_runtime crates/vm_eval crates/vm_host --include='*.rs' >/dev/null 2>&1; then
  fail 'extern "C" found under vm_runtime/vm_eval/vm_host'
else
  pass 'no extern "C" in runtime/eval/host crates'
fi

if ! echo "$SRC" | grep -q 'reject_public_bytecode_cache_claim'; then
  fail "reject_public_bytecode_cache_claim missing"
else
  pass "public-bytecode cache claim rejection present"
fi

if ! echo "$SRC" | grep -Eq 'CapabilityError|check_capability'; then
  fail "capability gating symbols missing"
else
  pass "capability gating symbols present"
fi

if ! echo "$SRC" | grep -q 'dispatch_helper' || ! echo "$SRC" | grep -q 'RuntimeHelperRegistry'; then
  fail "central helper dispatch/registry missing"
else
  pass "central helper registry + dispatch_helper present"
fi

if [[ ! -f .github/workflows/ci.yml ]]; then
  fail "CI workflow missing"
else
  pass "CI workflow present"
fi

if ! grep -q 'WP status: \*\*COMPLETE\*\*' tests/MATRIX.md; then
  fail "tests/MATRIX.md not marked COMPLETE"
else
  pass "WP-18 matrix COMPLETE"
fi

for f in PROGRESS.md ISSUE.md; do
  if [[ ! -f "$f" ]]; then fail "$f missing"; else pass "$f present"; fi
done

if [[ "$FAIL" -gt 0 ]]; then
  echo "=== G6 scan FAILED ($FAIL) ===" >&2
  exit 1
fi
echo "=== G6 scan PASSED ==="
exit 0
