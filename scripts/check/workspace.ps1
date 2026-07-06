#!/usr/bin/env pwsh
Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"
Push-Location (Split-Path (Split-Path $PSScriptRoot -Parent) -Parent)
try {
    cargo metadata --no-deps | Out-Null
    cargo check --workspace
} finally {
    Pop-Location
}