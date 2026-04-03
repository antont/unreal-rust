#pragma once

#include "MassEntityTypes.h"
#include "RustBobFragment.generated.h"

// Must match Rust BobFragment layout exactly:
// position_x: f64, position_y: f64, position_z: f64, velocity_z: f64, time: f32, speed: f32
// Total user data: 40 bytes (4 doubles + 2 floats)

USTRUCT()
struct FBobFragment : public FMassFragment
{
	GENERATED_BODY()

	// TODO: fields not yet defined — static_assert below should fail
};

// Layout verification: user data portion must be 40 bytes (4 doubles + 2 floats)
static_assert(sizeof(FBobFragment) - sizeof(FMassFragment) == 40, "FBobFragment user data must be exactly 40 bytes to match Rust BobFragment");
