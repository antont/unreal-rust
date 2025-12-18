// Fill out your copyright notice in the Description page of Project Settings.


#include "Reflection.h"


// Sets default values
AReflection::AReflection()
{
	// Set this actor to call Tick() every frame.  You can turn this off to improve performance if you don't need it.
	PrimaryActorTick.bCanEverTick = true;
}

// Called when the game starts or when spawned
void AReflection::BeginPlay()
{
	Super::BeginPlay();
	
}

// Called every frame
void AReflection::Tick(float DeltaTime)
{
	Super::Tick(DeltaTime);
}

