@echo off
setlocal
cd /d "%~dp0"

echo Symlinking RustPlugin into RustExample
if not exist "example\RustExample\Plugins" mkdir "example\RustExample\Plugins"
REM If a file (not a directory/symlink-to-dir) is at the target, remove it so mklink
REM can create a proper directory symlink. Git may check out submodule-stored symlinks
REM as plain text files on Windows when symlink support isn't enabled.
if exist "example\RustExample\Plugins\RustPlugin" if not exist "example\RustExample\Plugins\RustPlugin\" (
    echo Removing non-directory placeholder at example\RustExample\Plugins\RustPlugin
    del /Q "example\RustExample\Plugins\RustPlugin"
)
if not exist "example\RustExample\Plugins\RustPlugin\" (
    REM Use a directory junction ^(/J^) rather than a symlink ^(/D^).
    REM Junctions don't require admin or Developer Mode on Windows; symlinks do.
    REM UE treats them identically for plugin discovery.
    REM /J requires an absolute target path.
    mklink /J "example\RustExample\Plugins\RustPlugin" "%~dp0RustPlugin"
    if errorlevel 1 (
        echo.
        echo mklink /J failed. Run from a Command Prompt with write access to this directory.
        exit /b 1
    )
)

if not exist "example\RustExample\Binaries" mkdir "example\RustExample\Binaries"

echo Building loader dll
cargo build --release -p unreal-rust-loader
if errorlevel 1 exit /b 1

set LOADER_SRC=target\release\unreal_rust_loader.dll
set LOADER_DST=example\RustExample\Binaries\unreal_rust_loader.dll

echo Deploying loader: %LOADER_SRC% -^> %LOADER_DST%
copy /Y "%LOADER_SRC%" "%LOADER_DST%" >nul
if errorlevel 1 exit /b 1

echo.
echo Setup complete.
echo Build the host dll next:  cargo build --release -p unreal-rust-host
