// Fill out your copyright notice in the Description page of Project Settings.

#pragma once

#include "CoreMinimal.h"
#include "Kismet/BlueprintFunctionLibrary.h"
#include "UObject/Object.h"
#include "UObject/StrongObjectPtr.h"
#include "RustClassDef.generated.h"

class UClass;
class UEnum;
class URustType;
class UScriptStruct;

USTRUCT()
struct FRustPropertyDefinition
{
	GENERATED_BODY()

	int Offset = 0;
	FString Name;

	TStrongObjectPtr<URustType> Type;
};

UCLASS(Abstract, BlueprintType)
class RUSTPLUGIN_API URustType : public UObject
{
	GENERATED_BODY()
};

UCLASS(BlueprintType)
class RUSTPLUGIN_API URustType_Bool : public URustType
{
	GENERATED_BODY()
};

UCLASS(BlueprintType)
class RUSTPLUGIN_API URustType_Int8 : public URustType
{
	GENERATED_BODY()
};

UCLASS(BlueprintType)
class RUSTPLUGIN_API URustType_Int16 : public URustType
{
	GENERATED_BODY()
};

UCLASS(BlueprintType)
class RUSTPLUGIN_API URustType_Int32 : public URustType
{
	GENERATED_BODY()
};

UCLASS(BlueprintType)
class RUSTPLUGIN_API URustType_Int64 : public URustType
{
	GENERATED_BODY()
};

UCLASS(BlueprintType)
class RUSTPLUGIN_API URustType_UInt8 : public URustType
{
	GENERATED_BODY()
};

UCLASS(BlueprintType)
class RUSTPLUGIN_API URustType_UInt16 : public URustType
{
	GENERATED_BODY()
};

UCLASS(BlueprintType)
class RUSTPLUGIN_API URustType_UInt32 : public URustType
{
	GENERATED_BODY()
};

UCLASS(BlueprintType)
class RUSTPLUGIN_API URustType_UInt64 : public URustType
{
	GENERATED_BODY()
};

UCLASS(BlueprintType)
class RUSTPLUGIN_API URustType_Float : public URustType
{
	GENERATED_BODY()
};

UCLASS(BlueprintType)
class RUSTPLUGIN_API URustType_Double : public URustType
{
	GENERATED_BODY()
};

UCLASS(BlueprintType)
class RUSTPLUGIN_API URustType_FName : public URustType
{
	GENERATED_BODY()
};

UCLASS(BlueprintType)
class RUSTPLUGIN_API URustType_FString : public URustType
{
	GENERATED_BODY()
};

UCLASS(BlueprintType)
class RUSTPLUGIN_API URustType_FText : public URustType
{
	GENERATED_BODY()
};

UCLASS(BlueprintType)
class RUSTPLUGIN_API URustType_UObject : public URustType
{
	GENERATED_BODY()

public:
	UPROPERTY(BlueprintReadOnly)
	TObjectPtr<UClass> PropertyClass;
};

UCLASS(BlueprintType)
class RUSTPLUGIN_API URustType_UClass : public URustType
{
	GENERATED_BODY()

public:
	UPROPERTY(BlueprintReadOnly)
	TObjectPtr<UClass> MetaClass;
};

UCLASS(BlueprintType)
class RUSTPLUGIN_API URustType_SoftObject : public URustType
{
	GENERATED_BODY()

public:
	UPROPERTY(BlueprintReadOnly)
	TObjectPtr<UClass> PropertyClass;
};

UCLASS(BlueprintType)
class RUSTPLUGIN_API URustType_SoftClass : public URustType
{
	GENERATED_BODY()

public:
	UPROPERTY(BlueprintReadOnly)
	TObjectPtr<UClass> MetaClass;
};

UCLASS(BlueprintType)
class RUSTPLUGIN_API URustType_WeakObject : public URustType
{
	GENERATED_BODY()

public:
	UPROPERTY(BlueprintReadOnly)
	TObjectPtr<UClass> PropertyClass;
};

UCLASS(BlueprintType)
class RUSTPLUGIN_API URustType_LazyObject : public URustType
{
	GENERATED_BODY()

public:
	UPROPERTY(BlueprintReadOnly)
	TObjectPtr<UClass> PropertyClass;
};

UCLASS(BlueprintType)
class RUSTPLUGIN_API URustType_Struct : public URustType
{
	GENERATED_BODY()

public:
	UPROPERTY(BlueprintReadOnly)
	TObjectPtr<UScriptStruct> ScriptStruct;
};

UCLASS(BlueprintType)
class RUSTPLUGIN_API URustType_Enum : public URustType
{
	GENERATED_BODY()

public:
	UPROPERTY(BlueprintReadOnly)
	TObjectPtr<UEnum> Enum;
};

UCLASS(BlueprintType)
class RUSTPLUGIN_API URustType_Array : public URustType
{
	GENERATED_BODY()

public:
	UPROPERTY(BlueprintReadOnly)
	TObjectPtr<URustType> InnerType;
};

UCLASS(BlueprintType)
class RUSTPLUGIN_API URustType_Set : public URustType
{
	GENERATED_BODY()

public:
	UPROPERTY(BlueprintReadOnly)
	TObjectPtr<URustType> ElementType;
};

UCLASS(BlueprintType)
class RUSTPLUGIN_API URustType_Map : public URustType
{
	GENERATED_BODY()

public:
	UPROPERTY(BlueprintReadOnly)
	TObjectPtr<URustType> KeyType;

	UPROPERTY(BlueprintReadOnly)
	TObjectPtr<URustType> ValueType;
};

USTRUCT(BlueprintType)
struct FRustClassDef
{
	GENERATED_BODY()

	FString Name;
	int Size = 0;
	TArray<FRustPropertyDefinition> PropertyDefinitions;
};

UCLASS(meta=(Impl=FRustClassDef))
class URustExtension_RustClassDef : public UBlueprintFunctionLibrary
{
	GENERATED_BODY()

public:
	UFUNCTION(BlueprintCallable)
	static void AddProperty(FRustClassDef& Def, FString Name, int Offset, URustType* Type);

	UFUNCTION(BlueprintCallable)
	static URustType* CreateTypeBool();

	UFUNCTION(BlueprintCallable)
	static URustType* CreateTypeInt8();

	UFUNCTION(BlueprintCallable)
	static URustType* CreateTypeInt16();

	UFUNCTION(BlueprintCallable)
	static URustType* CreateTypeInt32();

	UFUNCTION(BlueprintCallable)
	static URustType* CreateTypeInt64();

	UFUNCTION(BlueprintCallable)
	static URustType* CreateTypeUInt8();

	UFUNCTION(BlueprintCallable)
	static URustType* CreateTypeUInt16();

	UFUNCTION(BlueprintCallable)
	static URustType* CreateTypeUInt32();

	UFUNCTION(BlueprintCallable)
	static URustType* CreateTypeUInt64();

	UFUNCTION(BlueprintCallable)
	static URustType* CreateTypeFloat();

	UFUNCTION(BlueprintCallable)
	static URustType* CreateTypeDouble();

	UFUNCTION(BlueprintCallable)
	static URustType* CreateTypeFName();

	UFUNCTION(BlueprintCallable)
	static URustType* CreateTypeFString();

	UFUNCTION(BlueprintCallable)
	static URustType* CreateTypeFText();

	UFUNCTION(BlueprintCallable)
	static URustType* CreateTypeUObject(UClass* PropertyClass);

	UFUNCTION(BlueprintCallable)
	static URustType* CreateTypeUClass(UClass* MetaClass);

	UFUNCTION(BlueprintCallable)
	static URustType* CreateTypeSoftObject(UClass* PropertyClass);

	UFUNCTION(BlueprintCallable)
	static URustType* CreateTypeSoftClass(UClass* MetaClass);

	UFUNCTION(BlueprintCallable)
	static URustType* CreateTypeWeakObject(UClass* PropertyClass);

	UFUNCTION(BlueprintCallable)
	static URustType* CreateTypeLazyObject(UClass* PropertyClass);

	UFUNCTION(BlueprintCallable)
	static URustType* CreateTypeStruct(UScriptStruct* ScriptStruct);

	UFUNCTION(BlueprintCallable)
	static URustType* CreateTypeEnum(UEnum* Enum);

	UFUNCTION(BlueprintCallable)
	static URustType* CreateTypeArray(URustType* InnerType);

	UFUNCTION(BlueprintCallable)
	static URustType* CreateTypeSet(URustType* ElementType);

	UFUNCTION(BlueprintCallable)
	static URustType* CreateTypeMap(URustType* KeyType, URustType* ValueType);

	UFUNCTION(BlueprintCallable)
	static void Register(const FRustClassDef& Def);

	UFUNCTION(BlueprintCallable)
	static FRustClassDef New(FString Name, int Size);
};
