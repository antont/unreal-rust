@echo off
REM Wrapper that runs pie.ps1 with ExecutionPolicy Bypass for the invocation only,
REM so users don't have to change their machine-wide policy.
REM All args are forwarded to the PowerShell script.
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0pie.ps1" %*
