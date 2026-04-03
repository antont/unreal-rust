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

	double PositionX = 0.0;
	double PositionY = 0.0;
	double PositionZ = 0.0;
	double VelocityZ = 0.0;
	float Time = 0.0f;
	float Speed = 2.0f;
};

// Layout verification against Rust BobFragment (40 bytes: 4 doubles + 2 floats).
// FMassFragment is an empty base class (1 byte, EBO applies), so fields start at offset 0.
// MassEntity's GetMutableFragmentView returns data starting at the fragment address,
// which is the same as &PositionX due to EBO — matching the Rust #[repr(C)] layout.
static_assert(offsetof(FBobFragment, PositionX) == 0, "PositionX must be at offset 0 (EBO)");
static_assert(offsetof(FBobFragment, PositionY) == 8, "PositionY at offset 8");
static_assert(offsetof(FBobFragment, PositionZ) == 16, "PositionZ at offset 16");
static_assert(offsetof(FBobFragment, VelocityZ) == 24, "VelocityZ at offset 24");
static_assert(offsetof(FBobFragment, Time) == 32, "Time at offset 32");
static_assert(offsetof(FBobFragment, Speed) == 36, "Speed at offset 36");
