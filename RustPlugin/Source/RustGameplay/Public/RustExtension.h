// Fill out your copyright notice in the Description page of Project Settings.

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
};

USTRUCT(BlueprintType)
struct FRustTypeInfo
{
	GENERATED_BODY()
	UPROPERTY(Blueprintable)
	int Offset;
	UPROPERTY(Blueprintable)
	FString Name;
};

USTRUCT(BlueprintType)
struct FRustType
{
	GENERATED_BODY()
	virtual ~FRustType() = default;
	FRustTypeInfo TypeInfo;
};


USTRUCT(BlueprintType)
struct FRustType_Numeric: public FRustType
{
	GENERATED_BODY()
};

USTRUCT(BlueprintType)
struct FRustTypeDef
{
	GENERATED_BODY()
	FRustTypeDef() = default;

	TArray<TUniquePtr<FRustType>> Types;
};

template<>
struct TStructOpsTypeTraits<FRustTypeDef> : public TStructOpsTypeTraitsBase2<FRustTypeDef>
{
	enum
	{
		WithCopy = false,
	};
};

UCLASS(meta=(Impl=FRustTypeDef))
class RUSTGAMEPLAY_API URustExtension_RustTypeDef : public UBlueprintFunctionLibrary
{
	GENERATED_BODY()
	UFUNCTION(BlueprintCallable)
	void AddNumeric(FRustTypeDef& Def, FRustTypeInfo TypeInfo);
};

