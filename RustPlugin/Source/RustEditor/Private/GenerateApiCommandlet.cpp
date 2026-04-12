// Fill out your copyright notice in the Description page of Project Settings.


#include "GenerateApiCommandlet.h"

#include "Reflection.h"

int UGenerateApiCommandlet::Main(const FString& Params)
{
	FRustReflection_Root::Collect();
	return 0;
}
