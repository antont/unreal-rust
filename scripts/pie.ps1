# Open the Unreal Editor with the GatherersBevyMass level loaded, ready for PIE testing.
# Usage: scripts\pie.ps1 [-Map <map-path>] [-Editor <path-to-UnrealEditor.exe>]
# Default map: /Game/Gatherers/GatherersBevyMass
# Default editor: C:\Program Files\Epic Games\UE_5.7\Engine\Binaries\Win64\UnrealEditor.exe
#                 (override with $env:UE_EDITOR or -Editor)
#
# Run from PowerShell directly, or use scripts\pie.cmd as a wrapper that bypasses
# ExecutionPolicy. Do NOT run from git-bash: MSYS auto-converts the /Game/... map
# path into a Windows path and UE won't find the asset.

param(
    [string]$Map = "/Game/Gatherers/GatherersBevyMass",
    [string]$Editor = $(if ($env:UE_EDITOR) { $env:UE_EDITOR } else { "C:\Program Files\Epic Games\UE_5.7\Engine\Binaries\Win64\UnrealEditor.exe" })
)

$ErrorActionPreference = "Stop"

$RepoRoot = Split-Path -Parent $PSScriptRoot
$Project = Join-Path $RepoRoot "example\RustExample\RustExample.uproject"

if (-not (Test-Path $Editor)) {
    Write-Error "UnrealEditor not found at: $Editor`nSet `$env:UE_EDITOR or pass -Editor <path>."
}
if (-not (Test-Path $Project)) {
    Write-Error "Project not found at: $Project"
}

$LogDir = Join-Path $env:TEMP "unreal-rust"
New-Item -ItemType Directory -Force -Path $LogDir | Out-Null
$LogFile = Join-Path $LogDir "ue_pie.log"

Write-Host "Editor:  $Editor"
Write-Host "Project: $Project"
Write-Host "Map:     $Map"
Write-Host "Log:     $LogFile"

# UE writes harmless warnings to stderr (Chromium usagestats etc). With
# ErrorActionPreference=Stop + 2>&1, PowerShell treats each stderr line as a
# terminating error. Reset to Continue for the native invocation.
$ErrorActionPreference = "Continue"
& $Editor $Project $Map 2>&1 | Tee-Object -FilePath $LogFile
