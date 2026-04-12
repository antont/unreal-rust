// Fill out your copyright notice in the Description page of Project Settings.


#include "RustExtension.h"

FHitResult URustExtension_FHitResult::New()
{
	return FHitResult();
}

UObject* URustExtension_Core::NewObject(UObject* Outer, TSubclassOf<UObject> Class, FName Name)
{
	return ::NewObject<UObject>(Outer, Class.Get(), Name);
}

FName URustExtension_Core::FNameNone()
{
	return NAME_None;
}