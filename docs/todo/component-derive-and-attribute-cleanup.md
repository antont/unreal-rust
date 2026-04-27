# Evaluate collapsing `MassFragment` into `Component`

Fallout from item #12 of the idiomatic-Bevy pass (see
`docs/todo/done/idiomatic-bevy-authoring.md` §12). Game code now uses
`#[derive(Component, MassFragment, ...)]` + `#[repr(C)]`.

(The sibling cleanup — deleting the unused `#[component]` attribute
macro + re-exports — has landed; see git history.)

## The remaining design question

The original plan wanted `#[derive(Component, Clone, Copy, Debug)]`
alone to be enough for a chunk-backed fragment. We diverged because
`Component` in game code resolves to `bevy_ecs::prelude::Component`
(re-exported through `bevy_mass::prelude`) — if we extended our local
`Component` derive to also emit `MassFragment` + `ChunkBacked`, we'd
have to shadow Bevy's built-in derive, which risks silently dropping
Bevy ECS behavior (hooks, relationships, storage-type attributes,
required-components metadata, etc.).

The cost of the current shape is one extra token per fragment struct
(`MassFragment`). The benefit is that game code uses Bevy's real
`Component` derive unchanged.

## When it would be worth revisiting

- Bevy's `Component` derive exposes a stable extension hook (a
  `#[component(on_add = …)]`-style mechanism that can be layered on
  without shadowing the derive), so we can register MassFragment /
  ChunkBacked metadata from a separate derive that runs alongside
  `#[derive(Component)]`.
- Or: we decide the Unreal path is worth a custom `Component` derive
  that re-implements the Bevy semantics we actually use (storage,
  mutability, hooks). That's a much larger surface to own, and only
  pays off if `MassFragment` tokens become load-bearing elsewhere.

Until one of those triggers, the explicit `MassFragment` opt-in is
the right trade — it's visibly the "this lives in chunk memory"
marker, which is information worth keeping in the type declaration.

## Acceptance

Design question, not a task — revisit only if Bevy's derive gains the
extension hook above, or if the `MassFragment` token starts carrying
more semantic load than today.
