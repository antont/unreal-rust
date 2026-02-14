// Fill out your copyright notice in the Description page of Project Settings.

#pragma once

#include "CoreMinimal.h"
#include "Commandlets/Commandlet.h"
#include "GenerateApiCommandlet.generated.h"

/**
 * 
 */
UCLASS()
class RUSTEDITOR_API UGenerateApiCommandlet : public UCommandlet
{
	GENERATED_BODY()
public:
	virtual int Main(const FString& Params) override;
};
