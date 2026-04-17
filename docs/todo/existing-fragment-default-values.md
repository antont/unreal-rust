# Default-value asymmetry for `existing` fragments

`mass_fragment!` with `existing` flag creates Rust types that mirror C++ USTRUCTs (e.g. `FTransformFragment`). The Rust `Default` impl may not match C++'s default constructor values.

## Current state

In practice this hasn't caused issues because `existing` fragments are initialized from C++ (MassEntity spawns them with C++ defaults) and Rust only reads/writes them during simulation. The Rust `Default` is only used in standalone Bevy mode where the standalone init code sets explicit values.

## When to address

If `existing` fragment types start being spawned from Rust in UE mode (e.g. via a shared spawning API — see idiomatic-bevy-authoring.md item 17), the default values would matter and should be synchronized.
