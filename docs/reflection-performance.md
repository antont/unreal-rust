# Reflection System Performance

The unreal-rust reflection system has **no per-frame runtime cost** under normal operation. It uses a layered approach where most work happens at compile time or during one-time initialization.

## Compile time (zero cost)

- `ReflectStatic::TYPE` is a `const` value resolved at compile time.
- `#[derive(UClass)]` proc macro generates `__register_uclass()` functions and trait implementations as Rust source code. The generated field accessors use `match` statements on field indices for O(1) dispatch.
- The `inventory` crate collects all type registrations statically via linker sections.

## One-time initialization

- `register_all_uclasses()` runs once during `UserModule::initialize()`. It iterates inventory-collected registrations and calls FFI to register properties with Unreal.
- `ClassPtrDB` and `__FUNCTION_PTRS` resolve all UClass and UFunction pointers once at startup via the loader.

## Runtime (on-demand only)

- `ReflectDyn` trait methods (`name()`, `get_field_value()`, `number_of_fields()`, etc.) are trait object dispatches called only when something explicitly queries reflection — for example during serialization or editor inspection. They are **not** called per-frame.

## Code generation pipeline

The `unreal-api-generator` binary is a separate build-time tool that reads UE header dumps and generates:
- FFI binding structs and function pointer types (`unreal-ffi`)
- High-level Rust API wrappers (`unreal-api/src/bindings/`)
- Layout verification tests (11,000+ tests comparing Rust struct layouts to C++ layouts)

This runs before compilation, not at runtime. The generated code lives in the repo as checked-in source files.

## Key files

| File | Role |
|------|------|
| `unreal-reflect/src/registry.rs` | Hand-written `ReflectDyn`/`ReflectStatic` impls for primitive types |
| `unreal-api-derive/src/uclass.rs` | Proc macro generating registration and reflection for `#[derive(UClass)]` types |
| `unreal-api/src/registration.rs` | `register_all_uclasses()` init entry point |
| `unreal-api-generator/src/generator/mod.rs` | Build-time codegen from UE headers |
