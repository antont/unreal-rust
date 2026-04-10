#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Actor.h"
#include "RustSimActivator.generated.h"

/**
 * Generic simulation activator actor.
 *
 * Place one instance in a level to start the Rust Mass Entity simulation.
 * Reads default configuration from Rust (via inventory registration) and
 * allows editor overrides via UPROPERTY fields.
 *
 * Replaces game-specific activator actors — game authors write only Rust code.
 */
UCLASS()
class RUSTPLUGIN_API ARustSimActivator : public AActor
{
	GENERATED_BODY()

public:
	/** If true, use the UPROPERTY values below instead of Rust-registered defaults. */
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Rust Simulation")
	bool bOverrideDefaults = false;

	/** Random seed for entity initialization. */
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Rust Simulation",
		meta = (EditCondition = "bOverrideDefaults"))
	int32 RandomSeed = 42;

	/** World-space simulation bounds. */
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Rust Simulation",
		meta = (EditCondition = "bOverrideDefaults"))
	FBox SimulationBounds = FBox(FVector(-500.0, -500.0, 0.0), FVector(500.0, 500.0, 100.0));

	/** Per-group entity count overrides (key = group name, value = count). */
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Rust Simulation",
		meta = (EditCondition = "bOverrideDefaults"))
	TMap<FString, int32> GroupCounts;

	virtual void BeginPlay() override;
};
