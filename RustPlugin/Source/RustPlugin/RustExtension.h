// Fill out your copyright notice in the Description page of Project Settings.

#pragma once

#include "CoreMinimal.h"
#include "Kismet/BlueprintFunctionLibrary.h"
#include "RustExtension.generated.h"

/**
 * 
 */
UCLASS()
class RUSTPLUGIN_API URustExtension : public UBlueprintFunctionLibrary
{
	GENERATED_BODY()
};

UCLASS(meta=(Impl=FHitResult))
class RUSTPLUGIN_API URustExtension_FHitResult : public UBlueprintFunctionLibrary
{
	GENERATED_BODY()

	UFUNCTION(BlueprintCallable)
	static FHitResult New();
};
