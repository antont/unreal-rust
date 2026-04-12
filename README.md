# 🦀 unreal-rust

Opinionated Rust integration for Unreal Engine

[![Build Status](https://github.com/MaikKlein/unreal-rust/workflows/CI/badge.svg)](https://github.com/MaikKlein/unreal-rust/actions?workflow=CI)
[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)
[![LICENSE](https://img.shields.io/badge/license-apache-blue.svg)](LICENSE-APACHE)

## About this fork

This is a fork of [MaikKlein/unreal-rust](https://github.com/MaikKlein/unreal-rust) that adds **zero-copy Mass Entity integration** and a **zero-C++ game authoring** workflow. Game developers write simulation logic, entity setup, and integration tests entirely in Rust — no C++ per-game code required.

### What this fork adds

- **Mass Entity bridge** — Rust systems operate directly on Unreal's MassEntity chunk memory with zero-copy access. Fragment layouts are validated at compile time across the FFI boundary.
- **Bevy-compatible systems** — A `bevy_mass` facade crate lets you write standard Bevy-style systems (`Query<&mut Position>`, `Res<DeltaTime>`) that compile against either pure Bevy for testing or Unreal MassEntity for production.
- **`#[mass_system]` macro** — Annotate a Rust function and it becomes a MassEntity processor. No C++ registration boilerplate.
- **`#[derive(MassFragment)]` macro** — Define fragment structs in Rust with auto-generated C++ headers and compile-time layout verification.
- **Auto-init from Rust** — Entity group counts, simulation bounds, spatial queries, and visualizer config are all registered from Rust via `inventory::submit!`. The C++ subsystem discovers and wires them up automatically.
- **Rust-authored UE automation tests** — Write integration tests in Rust using `assert!` macros. Tests run inside the UE editor with full Mass Entity and physics support, appearing in the UE test browser alongside C++ tests.
- **Rust hot-reload** — Rebuild the Rust dylib while PIE is running. The editor detects the change and reloads automatically.
- **macOS / UE 5.7 support**

### Example: Gatherers simulation

The included example is an ant colony simulation (~5000 entities) with movement, boundary reflection, food encounter detection via physics sweeps, and pickup/drop behavior. All game logic is in Rust across two crates: `gatherers-sim` (pure logic) and `gatherers-bevy-mass` (UE integration and fragment definitions).

## ☣️ Warning

`unreal-rust` is not ready to be used in a real project. `unreal-rust` is in a very early state and nothing more than a proof of concept right now. The API will change quite frequently. Things might crash, or not work as intended. The user experience will not be great.

I am releasing `unreal-rust` on github to develop it in the open.

## 🎯 Features

- **Simple opinionated bindings**: Easy access to core APIs like playing sounds, spawning actors, pathfinding, physics etc.
- **Developer friendly**: Fast iteration times with hot reloading in the editor and during live play. Panics are caught and do not crash the editor
- **Editor integration**: Add Rust components to actors in the editor, or access Rust components from Blueprint to drive animations.
- **Entity Component System**: unreal-rust is built on top of an ECS.
- **Built on top of `AActor`**: Most gameplay features like `GameMode`, `Characters`, `GameState`, `GAS` are not directly accessible in unreal-rust. Instead unreal-rust will provide optional alternatives. But you can still interact with most parts of the engine as Rust components can be accessed in Blueprint.
- **No engine modifications**: unreal-rust is only a `Plugin`, just drop it in your project. See [Supported versions](#supported-versions) for more information.
- **Samples**: The development of unreal-rust is heavily driven by samples.
- **Free**: Dual licensed under MIT/APACHE

## 🖥️ Supported Platforms

* 🐧 Linux
* 🪟 Windows

Potential future platforms: iOS, macOS, Android, html5/browser(possibly)

Consoles are unsupported. I simply can not offer console support as it is a closed off ecosystem. Nor do I have access to any developer kits myself.

## 💌 Blog posts / media

* [Announcing Unreal Rust](https://maikklein.github.io/unreal-rust-1/)
* [unreal-rust devlog](https://www.youtube.com/playlist?list=PLps1NSMUeqzicmTej83z-n1J383u1UVq1)


## 🚩 Known problems

- This is just a hobby project of mine that I work on outside of my normal work hours. I might be slow to respond to issues, questions, feature requests, or PR reviews.

## 🦮 Getting started

### Running the example

_I am aware that these are a lot of steps. I am sorry, I will try to simplify this in the future_

* Prerequisites:
* * [Unreal Engine 5](https://www.unrealengine.com/en-US/unreal-engine-5). For Linux users you can get it [here](https://www.unrealengine.com/en-US/linux)
* * Get [git lfs](https://git-lfs.github.com/), and run `git lfs install`
* * [ue4cli](https://docs.adamrehn.com/ue4cli/overview/introduction-to-ue4cli), You can get it with `pip3 install ue4cli`. This step is optional but I will use `ue4cli` in this guide.
* * [Rust](https://www.rust-lang.org/tools/install)

We start by cloning this repository 

```
git clone https://github.com/MaikKlein/unreal-rust
```

Next we clone the submodule. This will download the actual example with all the assets.

```
cd unreal-rust
git submodule update --init
```

Next we need to setup the example

- - Linux `sh setup.sh`
- - Windows `setup.bat`

This will symlink the `RustPlugin` into the unreal `example/RustExample/Plugin` folder.

Now we need to build the actual Rust code:

Simply run

```
cargo build --release
```

This will build the whole project. This also produces our dll that we are going to load into Unreal.

Copy the dll/so file into the project 

* Linux: `cp target/release/libunreal_rust_example.so example/RustExample/Binaries/rustplugin.so`
* Windows: `copy .\target\release\unreal_rust_example.dll .\example\RustExample\Binaries\rustplugin.dll`

Now we need to build the unreal example

```
cd example/RustExample
ue4 build Development Editor
```

Now you can run the example with `ue4 run`

## 🚀 Supported versions

- `5.0`

This project will always try to support the latest version.

- Latest version of Unreal
- Latest version of Rust
- Latest version of all dependencies

## 🤝 Alternatives

* [Unreal Angelscript](https://angelscript.hazelight.se/)
* [UnrealCLR](https://github.com/nxrighthere/UnrealCLR)

## 🥰 Thank you

* [kenney](https://kenney.nl/), [quaternius](https://www.patreon.com/quaternius)
* [bevy](https://bevyengine.org/)
