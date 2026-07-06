#!/usr/bin/env pwsh
Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"
$root = Split-Path (Split-Path $PSScriptRoot -Parent) -Parent
$map = Join-Path $root "docs\agent-plan\local-reference-map.md"
if (-not (Test-Path $map)) {
    throw "Missing local reference map: $map"
}
$missing = @()
Get-Content $map | ForEach-Object {
    if ($_ -match '`ARCHITECTURE/([^`]+)`') {
        $rel = $matches[1] -replace '/', '\'
        $path = Join-Path $root "ARCHITECTURE\$rel"
        if (-not (Test-Path $path)) {
            $missing += $path
        }
    }
}
if ($missing.Count -gt 0) {
    $missing | ForEach-Object { Write-Error "Missing frozen spec path: $_" }
    exit 1
}
Write-Host "All mapped ARCHITECTURE paths exist."