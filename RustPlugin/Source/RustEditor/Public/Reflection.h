// Fill out your copyright notice in the Description page of Project Settings.

#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Actor.h"
#include "Reflection.generated.h"

USTRUCT()
struct FRustReflection_Property
{
	GENERATED_BODY()
	UPROPERTY()
	FName Name;
    
};

USTRUCT()
struct FRustReflection_Function
{
	GENERATED_BODY()
	UPROPERTY()
	FName Name;
    
	UPROPERTY()
	TArray<FRustReflection_Property> Parameters;
};

USTRUCT()
struct FRustReflection_UObject
{
	GENERATED_BODY()
	UPROPERTY()
	FName Name;
    
	UPROPERTY()
	TArray<FRustReflection_Property> Properties;
	UPROPERTY()
	TArray<FRustReflection_Function> Functions;
};

UCLASS()
class RUSTEDITOR_API AReflection : public AActor
{
	GENERATED_BODY()

public:
	// Sets default values for this actor's properties
	AReflection();

protected:
	// Called when the game starts or when spawned
	virtual void BeginPlay() override;

public:
	// Called every frame
	virtual void Tick(float DeltaTime) override;
};
