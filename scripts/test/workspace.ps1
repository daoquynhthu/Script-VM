#!/usr/bin/env pwsh
Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"
Push-Location (Split-Path (Split-Path $PSScriptRoot -Parent) -Parent)
try {
    cargo test --workspace
} finally {
    Pop-Location
}