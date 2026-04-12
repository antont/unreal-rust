#pragma once

#include "CoreMinimal.h"
#include "Bindings.h"
#include "CollisionShape.h"

class AActor;
class FRustPluginModule;

extern struct FLogCategoryRustVisualLog : public FLogCategory<ELogVerbosity::Log, ELogVerbosity::All>
{
	FORCEINLINE FLogCategoryRustVisualLog() : FLogCategory(TEXT("RustVisualLog"))
	{
	}
} RustVisualLog;;

struct FMassEntityManager;

UnrealBindings CreateBindings();
FRustPluginModule& GetRustModule();

/// Set the entity manager for Rust entity spawning. Call before any Rust spawn operations.
RUSTPLUGIN_API void RustMassSpawnSetEntityManager(FMassEntityManager* Manager);



FString ToFString(Utf8Str Str);
