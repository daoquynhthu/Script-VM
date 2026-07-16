#!/usr/bin/env pwsh
# WP-19 / Stage 14 G6 integration scan (local Windows).
# Mirrors scripts/integration/g6-scan.sh for CI.
Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$root = Resolve-Path (Join-Path $PSScriptRoot "..\..")
Set-Location $root

$fail = 0
function Fail([string]$msg) {
    Write-Host "FAIL: $msg" -ForegroundColor Red
    $script:fail++
}
function Pass([string]$msg) {
    Write-Host "PASS: $msg" -ForegroundColor Green
}

Write-Host "=== G6 integration scan (WP-19) ==="
Write-Host "root: $root"

# 1. Public bytecode / CPython ABI forbidden strings in crate sources
$crateSrc = Get-ChildItem -Path "crates" -Recurse -Include *.rs -File |
    Where-Object { $_.FullName -notmatch '\\target\\' }
$joined = $crateSrc | ForEach-Object { Get-Content -Raw $_.FullName } | Out-String

if ($joined -match 'CPython|cpython_abi|PyObject\s*\*') {
    Fail "CPython ABI symbols found in crate sources"
} else {
    Pass "no CPython ABI symbols in crate sources"
}

# Exposure markers only (reject_public_bytecode_cache_claim and error messages are allowed).
$exposureHits = Select-String -Path $crateSrc.FullName `
    -Pattern 'export_bytecode|PublicBytecodeAbi|publish_bytecode|pub\s+fn\s+emit_bytecode' `
    -ErrorAction SilentlyContinue
if ($exposureHits) {
    Fail "public bytecode exposure markers found: $($exposureHits[0].Path):$($exposureHits[0].LineNumber)"
} else {
    Pass "no public bytecode exposure markers"
}

# 2. Host boundary: raw FFI host pointer calls without wrapper markers
# (heuristic: forbid extern "C" in vm_runtime/vm_eval host paths)
$externC = Select-String -Path (Get-ChildItem crates/vm_runtime,crates/vm_eval,crates/vm_host -Recurse -Filter *.rs).FullName `
    -Pattern 'extern\s+"C"' -SimpleMatch:$false -ErrorAction SilentlyContinue
if ($externC) {
    Fail "extern `"C`" found under vm_runtime/vm_eval/vm_host (host boundary risk)"
} else {
    Pass "no extern `"C`" in runtime/eval/host crates"
}

# 3. Cache boundary: reject_public_bytecode_cache_claim must exist
if ($joined -notmatch 'reject_public_bytecode_cache_claim') {
    Fail "reject_public_bytecode_cache_claim missing"
} else {
    Pass "public-bytecode cache claim rejection present"
}

# 4. Capability checks present
if ($joined -notmatch 'CapabilityError|check_capability|requires.*capability') {
    Fail "capability gating symbols missing"
} else {
    Pass "capability gating symbols present"
}

# 5. Helper registry central dispatch present
if ($joined -notmatch 'dispatch_helper' -or $joined -notmatch 'RuntimeHelperRegistry') {
    Fail "central helper dispatch/registry missing"
} else {
    Pass "central helper registry + dispatch_helper present"
}

# 6. CI workflow exists
if (-not (Test-Path ".github/workflows/ci.yml")) {
    Fail "CI workflow missing"
} else {
    Pass "CI workflow present"
}

# 7. Conformance matrix present and marked complete for WP-18
$matrix = Get-Content "tests/MATRIX.md" -Raw
if ($matrix -notmatch 'WP status:\s*\*\*COMPLETE\*\*') {
    Fail "tests/MATRIX.md not marked COMPLETE"
} else {
    Pass "WP-18 matrix COMPLETE"
}

# 8. PROGRESS / ISSUE append-only headers
foreach ($f in @("PROGRESS.md", "ISSUE.md")) {
    if (-not (Test-Path $f)) { Fail "$f missing" }
    else { Pass "$f present" }
}

if ($fail -gt 0) {
    Write-Host "=== G6 scan FAILED ($fail) ===" -ForegroundColor Red
    exit 1
}
Write-Host "=== G6 scan PASSED ===" -ForegroundColor Green
exit 0
