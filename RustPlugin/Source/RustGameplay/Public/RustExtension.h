#pragma once

#include "CoreMinimal.h"
#include "Kismet/BlueprintFunctionLibrary.h"
#include "RustExtension.generated.h"

/**
 * 
 */
UCLASS()
class RUSTGAMEPLAY_API URustExtension : public UBlueprintFunctionLibrary
{
	GENERATED_BODY()
};

UCLASS(meta=(Impl=FHitResult))
class RUSTGAMEPLAY_API URustExtension_FHitResult : public UBlueprintFunctionLibrary
{
	GENERATED_BODY()

	UFUNCTION(BlueprintCallable)
	static FHitResult New();
};

UCLASS()
class RUSTGAMEPLAY_API URustExtension_Core : public UBlueprintFunctionLibrary
{
	GENERATED_BODY()

	UFUNCTION(BlueprintCallable)
	static UObject* NewObject(UObject* Outer, TSubclassOf<UObject> Class, FName Name);
	
	UFUNCTION(BlueprintCallable)
	static FName FNameNone();
	
	// UFUNCTION(BlueprintCallable)
	// static void RegisterRustClassDef(FString TypeName, const FRustClassDef& Def);
};
