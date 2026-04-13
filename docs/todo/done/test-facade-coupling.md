# Reduce test coupling to generated facade struct names — DONE

Tests no longer reference `__FQ_`-prefixed generated struct names. The systems tests in `gatherers-bevy-mass/src/systems.rs` now test through message-based interfaces (constructing `AntFoodHit`, `FoodMutation` messages) rather than constructing internal macro-generated facade structs.
