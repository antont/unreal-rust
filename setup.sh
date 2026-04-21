#!/bin/bash
set -euo pipefail

WORKSPACE="$(cd "$(dirname "$0")" && pwd)"

echo "Symlinking RustPlugin into RustExample"
mkdir -p "$WORKSPACE/example/RustExample/Plugins"
if [ ! -e "$WORKSPACE/example/RustExample/Plugins/RustPlugin" ]; then
    ln -s ../../../RustPlugin "$WORKSPACE/example/RustExample/Plugins/RustPlugin"
fi

mkdir -p "$WORKSPACE/example/RustExample/Binaries"

# Build + deploy the loader dylib. The loader bakes its build-time cargo
# target dir into the binary so it can find the sibling host dylib at runtime
# even when cargo is configured to use a shared target dir.
echo "Building loader dylib"
cargo build --release -p unreal-rust-loader

LOADER_SRC=$(cargo metadata --format-version=1 --no-deps \
    | python3 -c 'import json,sys; m=json.load(sys.stdin); print(m["target_directory"])')/release/libunreal_rust_loader.dylib
LOADER_DST="$WORKSPACE/example/RustExample/Binaries/unreal_rust_loader.dylib"

echo "Deploying loader: $LOADER_SRC -> $LOADER_DST"
cp "$LOADER_SRC" "$LOADER_DST"

# Re-sign the copy so macOS doesn't kill the editor process on load.
if [[ "$OSTYPE" == "darwin"* ]]; then
    codesign --force --sign - "$LOADER_DST"
fi

echo "Setup complete."
echo "Build the host dylib next:  cargo build --release -p unreal-rust-host"
