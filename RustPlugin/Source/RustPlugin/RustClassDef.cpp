// Fill out your copyright notice in the Description page of Project Settings.

#include "RustClassDef.h"

#include "RustPlugin.h"
#include "RustUtils.h"

namespace
{
template <typename TRustType>
TRustType* CreateRustType()
{
	return NewObject<TRustType>(GetTransientPackage());
}

template <typename TRustType, typename TMetaType>
TRustType* CreateRustTypeWithMeta(TMetaType* MetaValue, TObjectPtr<TMetaType> TRustType::*MetaField)
{
	TRustType* Type = CreateRustType<TRustType>();
	Type->*MetaField = MetaValue;
	return Type;
}
}

void URustExtension_RustClassDef::AddProperty(FRustClassDef& Def, FString Name, int Offset, URustType* Type)
{
	check(Type);
	FRustPropertyDefinition Property;
	Property.Name = MoveTemp(Name);
	Property.Offset = Offset;
	Property.Type = TStrongObjectPtr(Type);
	Def.PropertyDefinitions.Add(MoveTemp(Property));
}

URustType* URustExtension_RustClassDef::CreateTypeBool()
{
	return CreateRustType<URustType_Bool>();
}

URustType* URustExtension_RustClassDef::CreateTypeInt8()
{
	return CreateRustType<URustType_Int8>();
}

URustType* URustExtension_RustClassDef::CreateTypeInt16()
{
	return CreateRustType<URustType_Int16>();
}

URustType* URustExtension_RustClassDef::CreateTypeInt32()
{
	return CreateRustType<URustType_Int32>();
}

URustType* URustExtension_RustClassDef::CreateTypeInt64()
{
	return CreateRustType<URustType_Int64>();
}

URustType* URustExtension_RustClassDef::CreateTypeUInt8()
{
	return CreateRustType<URustType_UInt8>();
}

URustType* URustExtension_RustClassDef::CreateTypeUInt16()
{
	return CreateRustType<URustType_UInt16>();
}

URustType* URustExtension_RustClassDef::CreateTypeUInt32()
{
	return CreateRustType<URustType_UInt32>();
}

URustType* URustExtension_RustClassDef::CreateTypeUInt64()
{
	return CreateRustType<URustType_UInt64>();
}

URustType* URustExtension_RustClassDef::CreateTypeFloat()
{
	return CreateRustType<URustType_Float>();
}

URustType* URustExtension_RustClassDef::CreateTypeDouble()
{
	return CreateRustType<URustType_Double>();
}

URustType* URustExtension_RustClassDef::CreateTypeFName()
{
	return CreateRustType<URustType_FName>();
}

URustType* URustExtension_RustClassDef::CreateTypeFString()
{
	return CreateRustType<URustType_FString>();
}

URustType* URustExtension_RustClassDef::CreateTypeFText()
{
	return CreateRustType<URustType_FText>();
}

URustType* URustExtension_RustClassDef::CreateTypeUObject(UClass* PropertyClass)
{
	check(PropertyClass);
	return CreateRustTypeWithMeta<URustType_UObject>(PropertyClass, &URustType_UObject::PropertyClass);
}

URustType* URustExtension_RustClassDef::CreateTypeUClass(UClass* MetaClass)
{
	check(MetaClass);
	return CreateRustTypeWithMeta<URustType_UClass>(MetaClass, &URustType_UClass::MetaClass);
}

URustType* URustExtension_RustClassDef::CreateTypeSoftObject(UClass* PropertyClass)
{
	check(PropertyClass);
	return CreateRustTypeWithMeta<URustType_SoftObject>(PropertyClass, &URustType_SoftObject::PropertyClass);
}

URustType* URustExtension_RustClassDef::CreateTypeSoftClass(UClass* MetaClass)
{
	check(MetaClass);
	return CreateRustTypeWithMeta<URustType_SoftClass>(MetaClass, &URustType_SoftClass::MetaClass);
}

URustType* URustExtension_RustClassDef::CreateTypeWeakObject(UClass* PropertyClass)
{
	check(PropertyClass);
	return CreateRustTypeWithMeta<URustType_WeakObject>(PropertyClass, &URustType_WeakObject::PropertyClass);
}

URustType* URustExtension_RustClassDef::CreateTypeLazyObject(UClass* PropertyClass)
{
	check(PropertyClass);
	return CreateRustTypeWithMeta<URustType_LazyObject>(PropertyClass, &URustType_LazyObject::PropertyClass);
}

URustType* URustExtension_RustClassDef::CreateTypeStruct(UScriptStruct* ScriptStruct)
{
	check(ScriptStruct);
	return CreateRustTypeWithMeta<URustType_Struct>(ScriptStruct, &URustType_Struct::ScriptStruct);
}

URustType* URustExtension_RustClassDef::CreateTypeEnum(UEnum* Enum)
{
	check(Enum);
	return CreateRustTypeWithMeta<URustType_Enum>(Enum, &URustType_Enum::Enum);
}

URustType* URustExtension_RustClassDef::CreateTypeArray(URustType* InnerType)
{
	check(InnerType);
	return CreateRustTypeWithMeta<URustType_Array>(InnerType, &URustType_Array::InnerType);
}

URustType* URustExtension_RustClassDef::CreateTypeSet(URustType* ElementType)
{
	check(ElementType);
	return CreateRustTypeWithMeta<URustType_Set>(ElementType, &URustType_Set::ElementType);
}

URustType* URustExtension_RustClassDef::CreateTypeMap(URustType* KeyType, URustType* ValueType)
{
	check(KeyType);
	check(ValueType);
	URustType_Map* Type = CreateRustType<URustType_Map>();
	Type->KeyType = KeyType;
	Type->ValueType = ValueType;
	return Type;
}

void URustExtension_RustClassDef::Register(const FRustClassDef& Def)
{
	GetRustModule().Plugin.Types.Add(Def.Name, Def);
}

FRustClassDef URustExtension_RustClassDef::New(FString Name, int Size)
{
	FRustClassDef Def;
	Def.Size = Size;
	Def.Name = MoveTemp(Name);
	return Def;
}
