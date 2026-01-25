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

UnrealBindings CreateBindings();
FRustPluginModule& GetRustModule();



FString ToFString(Utf8Str Str);
