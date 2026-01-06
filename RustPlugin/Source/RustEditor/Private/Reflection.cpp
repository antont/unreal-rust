// Fill out your copyright notice in the Description page of Project Settings.


#include "Reflection.h"

#include "GameplayTagsManager.h"

TMap<FString, FString> GetAllMetadata(UObject* Object)
{
	TMap<FString, FString> Result;

	if (UPackage* Package = Object->GetOutermost())
	{
		FMetaData& MetaData = Package->GetMetaData();
		// This returns a pointer to the internal map, or nullptr if none exists

		if (const TMap<FName, FString>* Map = MetaData.GetMapForObject(Object))
		{
			for (const auto& Pair : *Map)
			{
				Result.Add(Pair.Key.ToString(), Pair.Value);
			}
		}
	}

	// Don't care about these
	Result.Remove(TEXT("ModuleRelativePath"));
	Result.Remove(TEXT("Comment"));
	Result.Remove(TEXT("Tooltip"));

	return Result;
}

static TArray<FString> GetStructFlagStrings(uint32 Flags)
{
	TArray<FString> Result;

	// Note: We skip STRUCT_NoFlags (0) and composite masks (Inherit, ComputedFlags)
	// to only return the atomic flags set on the struct.

	if (Flags & STRUCT_Native) Result.Add(TEXT("Native"));
	if (Flags & STRUCT_IdenticalNative) Result.Add(TEXT("IdenticalNative"));
	if (Flags & STRUCT_HasInstancedReference) Result.Add(TEXT("HasInstancedReference"));
	if (Flags & STRUCT_NoExport) Result.Add(TEXT("NoExport"));
	if (Flags & STRUCT_Atomic) Result.Add(TEXT("Atomic"));
	if (Flags & STRUCT_Immutable) Result.Add(TEXT("Immutable"));
	if (Flags & STRUCT_AddStructReferencedObjects) Result.Add(TEXT("AddStructReferencedObjects"));
	if (Flags & STRUCT_RequiredAPI) Result.Add(TEXT("RequiredAPI"));
	if (Flags & STRUCT_NetSerializeNative) Result.Add(TEXT("NetSerializeNative"));
	if (Flags & STRUCT_SerializeNative) Result.Add(TEXT("SerializeNative"));
	if (Flags & STRUCT_CopyNative) Result.Add(TEXT("CopyNative"));
	if (Flags & STRUCT_IsPlainOldData) Result.Add(TEXT("IsPlainOldData"));
	if (Flags & STRUCT_NoDestructor) Result.Add(TEXT("NoDestructor"));
	if (Flags & STRUCT_ZeroConstructor) Result.Add(TEXT("ZeroConstructor"));
	if (Flags & STRUCT_ExportTextItemNative) Result.Add(TEXT("ExportTextItemNative"));
	if (Flags & STRUCT_ImportTextItemNative) Result.Add(TEXT("ImportTextItemNative"));
	if (Flags & STRUCT_PostSerializeNative) Result.Add(TEXT("PostSerializeNative"));
	if (Flags & STRUCT_SerializeFromMismatchedTag) Result.Add(TEXT("SerializeFromMismatchedTag"));
	if (Flags & STRUCT_NetDeltaSerializeNative) Result.Add(TEXT("NetDeltaSerializeNative"));
	if (Flags & STRUCT_PostScriptConstruct) Result.Add(TEXT("PostScriptConstruct"));
	if (Flags & STRUCT_NetSharedSerialization) Result.Add(TEXT("NetSharedSerialization"));
	if (Flags & STRUCT_Trashed) Result.Add(TEXT("Trashed"));
	if (Flags & STRUCT_NewerVersionExists) Result.Add(TEXT("NewerVersionExists"));
	if (Flags & STRUCT_CanEditChange) Result.Add(TEXT("CanEditChange"));
	if (Flags & STRUCT_Visitor) Result.Add(TEXT("Visitor"));

	return Result;
}

static TArray<FString> GetPropertyFlagStrings(uint64 Flags)
{
	TArray<FString> Result;

	if (Flags & CPF_Edit) Result.Add(TEXT("Edit"));
	if (Flags & CPF_ConstParm) Result.Add(TEXT("ConstParm"));
	if (Flags & CPF_BlueprintVisible) Result.Add(TEXT("BlueprintVisible"));
	if (Flags & CPF_ExportObject) Result.Add(TEXT("ExportObject"));
	if (Flags & CPF_BlueprintReadOnly) Result.Add(TEXT("BlueprintReadOnly"));
	if (Flags & CPF_Net) Result.Add(TEXT("Net"));
	if (Flags & CPF_EditFixedSize) Result.Add(TEXT("EditFixedSize"));
	if (Flags & CPF_Parm) Result.Add(TEXT("Parm"));
	if (Flags & CPF_OutParm) Result.Add(TEXT("OutParm"));
	if (Flags & CPF_ZeroConstructor) Result.Add(TEXT("ZeroConstructor"));
	if (Flags & CPF_ReturnParm) Result.Add(TEXT("ReturnParm"));
	if (Flags & CPF_DisableEditOnTemplate) Result.Add(TEXT("DisableEditOnTemplate"));
	if (Flags & CPF_NonNullable) Result.Add(TEXT("NonNullable"));
	if (Flags & CPF_Transient) Result.Add(TEXT("Transient"));
	if (Flags & CPF_Config) Result.Add(TEXT("Config"));
	if (Flags & CPF_RequiredParm) Result.Add(TEXT("RequiredParm"));
	if (Flags & CPF_DisableEditOnInstance) Result.Add(TEXT("DisableEditOnInstance"));
	if (Flags & CPF_EditConst) Result.Add(TEXT("EditConst"));
	if (Flags & CPF_GlobalConfig) Result.Add(TEXT("GlobalConfig"));
	if (Flags & CPF_InstancedReference) Result.Add(TEXT("InstancedReference"));
	if (Flags & CPF_ExperimentalExternalObjects) Result.Add(TEXT("ExperimentalExternalObjects"));
	if (Flags & CPF_DuplicateTransient) Result.Add(TEXT("DuplicateTransient"));
	if (Flags & CPF_SaveGame) Result.Add(TEXT("SaveGame"));
	if (Flags & CPF_NoClear) Result.Add(TEXT("NoClear"));
	if (Flags & CPF_Virtual) Result.Add(TEXT("Virtual"));
	if (Flags & CPF_ReferenceParm) Result.Add(TEXT("ReferenceParm"));
	if (Flags & CPF_BlueprintAssignable) Result.Add(TEXT("BlueprintAssignable"));
	if (Flags & CPF_Deprecated) Result.Add(TEXT("Deprecated"));
	if (Flags & CPF_IsPlainOldData) Result.Add(TEXT("IsPlainOldData"));
	if (Flags & CPF_RepSkip) Result.Add(TEXT("RepSkip"));
	if (Flags & CPF_RepNotify) Result.Add(TEXT("RepNotify"));
	if (Flags & CPF_Interp) Result.Add(TEXT("Interp"));
	if (Flags & CPF_NonTransactional) Result.Add(TEXT("NonTransactional"));
	if (Flags & CPF_EditorOnly) Result.Add(TEXT("EditorOnly"));
	if (Flags & CPF_NoDestructor) Result.Add(TEXT("NoDestructor"));
	if (Flags & CPF_AutoWeak) Result.Add(TEXT("AutoWeak"));
	if (Flags & CPF_ContainsInstancedReference) Result.Add(TEXT("ContainsInstancedReference"));
	if (Flags & CPF_AssetRegistrySearchable) Result.Add(TEXT("AssetRegistrySearchable"));
	if (Flags & CPF_SimpleDisplay) Result.Add(TEXT("SimpleDisplay"));
	if (Flags & CPF_AdvancedDisplay) Result.Add(TEXT("AdvancedDisplay"));
	if (Flags & CPF_Protected) Result.Add(TEXT("Protected"));
	if (Flags & CPF_BlueprintCallable) Result.Add(TEXT("BlueprintCallable"));
	if (Flags & CPF_BlueprintAuthorityOnly) Result.Add(TEXT("BlueprintAuthorityOnly"));
	if (Flags & CPF_TextExportTransient) Result.Add(TEXT("TextExportTransient"));
	if (Flags & CPF_NonPIEDuplicateTransient) Result.Add(TEXT("NonPIEDuplicateTransient"));
	if (Flags & CPF_ExposeOnSpawn) Result.Add(TEXT("ExposeOnSpawn"));
	if (Flags & CPF_PersistentInstance) Result.Add(TEXT("PersistentInstance"));
	if (Flags & CPF_UObjectWrapper) Result.Add(TEXT("UObjectWrapper"));
	if (Flags & CPF_HasGetValueTypeHash) Result.Add(TEXT("HasGetValueTypeHash"));
	if (Flags & CPF_NativeAccessSpecifierPublic) Result.Add(TEXT("NativeAccessSpecifierPublic"));
	if (Flags & CPF_NativeAccessSpecifierProtected) Result.Add(TEXT("NativeAccessSpecifierProtected"));
	if (Flags & CPF_NativeAccessSpecifierPrivate) Result.Add(TEXT("NativeAccessSpecifierPrivate"));
	if (Flags & CPF_SkipSerialization) Result.Add(TEXT("SkipSerialization"));
	if (Flags & CPF_TObjectPtr) Result.Add(TEXT("TObjectPtr"));
	if (Flags & CPF_ExperimentalOverridableLogic) Result.Add(TEXT("ExperimentalOverridableLogic"));
	if (Flags & CPF_ExperimentalAlwaysOverriden) Result.Add(TEXT("ExperimentalAlwaysOverriden"));
	if (Flags & CPF_ExperimentalNeverOverriden) Result.Add(TEXT("ExperimentalNeverOverriden"));
	if (Flags & CPF_AllowSelfReference) Result.Add(TEXT("AllowSelfReference"));

	return Result;
}

static TArray<FString> GetClassFlagStrings(uint32 Flags)
{
	TArray<FString> Result;

	if (Flags & CLASS_Abstract) Result.Add(TEXT("Abstract"));
	if (Flags & CLASS_DefaultConfig) Result.Add(TEXT("DefaultConfig"));
	if (Flags & CLASS_Config) Result.Add(TEXT("Config"));
	if (Flags & CLASS_Transient) Result.Add(TEXT("Transient"));
	if (Flags & CLASS_Optional) Result.Add(TEXT("Optional"));
	if (Flags & CLASS_MatchedSerializers) Result.Add(TEXT("MatchedSerializers"));
	if (Flags & CLASS_ProjectUserConfig) Result.Add(TEXT("ProjectUserConfig"));
	if (Flags & CLASS_Native) Result.Add(TEXT("Native"));
	if (Flags & CLASS_NotPlaceable) Result.Add(TEXT("NotPlaceable"));
	if (Flags & CLASS_PerObjectConfig) Result.Add(TEXT("PerObjectConfig"));
	if (Flags & CLASS_ReplicationDataIsSetUp) Result.Add(TEXT("ReplicationDataIsSetUp"));
	if (Flags & CLASS_EditInlineNew) Result.Add(TEXT("EditInlineNew"));
	if (Flags & CLASS_CollapseCategories) Result.Add(TEXT("CollapseCategories"));
	if (Flags & CLASS_Interface) Result.Add(TEXT("Interface"));
	if (Flags & CLASS_PerPlatformConfig) Result.Add(TEXT("PerPlatformConfig"));
	if (Flags & CLASS_Const) Result.Add(TEXT("Const"));
	if (Flags & CLASS_NeedsDeferredDependencyLoading) Result.Add(TEXT("NeedsDeferredDependencyLoading"));
	if (Flags & CLASS_CompiledFromBlueprint) Result.Add(TEXT("CompiledFromBlueprint"));
	if (Flags & CLASS_MinimalAPI) Result.Add(TEXT("MinimalAPI"));
	if (Flags & CLASS_RequiredAPI) Result.Add(TEXT("RequiredAPI"));
	if (Flags & CLASS_DefaultToInstanced) Result.Add(TEXT("DefaultToInstanced"));
	if (Flags & CLASS_TokenStreamAssembled) Result.Add(TEXT("TokenStreamAssembled"));
	if (Flags & CLASS_HasInstancedReference) Result.Add(TEXT("HasInstancedReference"));
	if (Flags & CLASS_Hidden) Result.Add(TEXT("Hidden"));
	if (Flags & CLASS_Deprecated) Result.Add(TEXT("Deprecated"));
	if (Flags & CLASS_HideDropDown) Result.Add(TEXT("HideDropDown"));
	if (Flags & CLASS_GlobalUserConfig) Result.Add(TEXT("GlobalUserConfig"));
	if (Flags & CLASS_Intrinsic) Result.Add(TEXT("Intrinsic"));
	if (Flags & CLASS_Constructed) Result.Add(TEXT("Constructed"));
	if (Flags & CLASS_ConfigDoNotCheckDefaults) Result.Add(TEXT("ConfigDoNotCheckDefaults"));
	if (Flags & CLASS_NewerVersionExists) Result.Add(TEXT("NewerVersionExists"));

	return Result;
}

static TArray<FString> GetFunctionFlagStrings(uint32 Flags)
{
	TArray<FString> Result;

	if (Flags & FUNC_Final) Result.Add(TEXT("Final"));
	if (Flags & FUNC_RequiredAPI) Result.Add(TEXT("RequiredAPI"));
	if (Flags & FUNC_BlueprintAuthorityOnly)Result.Add(TEXT("BlueprintAuthorityOnly"));
	if (Flags & FUNC_BlueprintCosmetic) Result.Add(TEXT("BlueprintCosmetic"));
	if (Flags & FUNC_Net) Result.Add(TEXT("Net"));
	if (Flags & FUNC_NetReliable) Result.Add(TEXT("NetReliable"));
	if (Flags & FUNC_NetRequest) Result.Add(TEXT("NetRequest"));
	if (Flags & FUNC_Exec) Result.Add(TEXT("Exec"));
	if (Flags & FUNC_Native) Result.Add(TEXT("Native"));
	if (Flags & FUNC_Event) Result.Add(TEXT("Event"));
	if (Flags & FUNC_NetResponse) Result.Add(TEXT("NetResponse"));
	if (Flags & FUNC_Static) Result.Add(TEXT("Static"));
	if (Flags & FUNC_NetMulticast) Result.Add(TEXT("NetMulticast"));
	if (Flags & FUNC_UbergraphFunction) Result.Add(TEXT("UbergraphFunction"));
	if (Flags & FUNC_MulticastDelegate) Result.Add(TEXT("MulticastDelegate"));
	if (Flags & FUNC_Public) Result.Add(TEXT("Public"));
	if (Flags & FUNC_Private) Result.Add(TEXT("Private"));
	if (Flags & FUNC_Protected) Result.Add(TEXT("Protected"));
	if (Flags & FUNC_Delegate) Result.Add(TEXT("Delegate"));
	if (Flags & FUNC_NetServer) Result.Add(TEXT("NetServer"));
	if (Flags & FUNC_HasOutParms) Result.Add(TEXT("HasOutParms"));
	if (Flags & FUNC_HasDefaults) Result.Add(TEXT("HasDefaults"));
	if (Flags & FUNC_NetClient) Result.Add(TEXT("NetClient"));
	if (Flags & FUNC_DLLImport) Result.Add(TEXT("DLLImport"));
	if (Flags & FUNC_BlueprintCallable) Result.Add(TEXT("BlueprintCallable"));
	if (Flags & FUNC_BlueprintEvent) Result.Add(TEXT("BlueprintEvent"));
	if (Flags & FUNC_BlueprintPure) Result.Add(TEXT("BlueprintPure"));
	if (Flags & FUNC_EditorOnly) Result.Add(TEXT("EditorOnly"));
	if (Flags & FUNC_Const) Result.Add(TEXT("Const"));
	if (Flags & FUNC_NetValidate) Result.Add(TEXT("NetValidate"));

	return Result;
}

TSharedPtr<FJsonObject> RustReflection_MapType::ToJson()
{
	auto Json = MakeShared<FJsonObject>();
	Json->SetStringField(TEXT("Kind"), TEXT("Map"));
	Json->SetObjectField(TEXT("KeyType"), KeyType->ToJson());
	Json->SetObjectField(TEXT("ValueType"), ValueType->ToJson());
	Json->SetNumberField(TEXT("ArrayDim"), ArrayDim);

	return Json;
}

TUniquePtr<RustReflection_ConcreteType> MakeConcreteType(FString Name, int32 ArrayDim, TOptional<FString> Hint)
{
	auto Type = MakeUnique<RustReflection_ConcreteType>();
	Type->TypeName = Name;
	Type->Hint = Hint;
	Type->ArrayDim = ArrayDim;

	return Type;
}

TUniquePtr<RustReflection_ContainerType> MakeContainerType(FString ContainerTypeName,
                                                           TUniquePtr<RustReflection_Type> InnerType, int32 ArrayDim)
{
	auto ContainerType = MakeUnique<RustReflection_ContainerType>();
	ContainerType->ContainerTypeName = ContainerTypeName;
	ContainerType->InnerType = MoveTemp(InnerType);
	ContainerType->ArrayDim = ArrayDim;

	return ContainerType;
}

TUniquePtr<RustReflection_Type> FromProperty(FProperty* Property)
{
	auto GetCppNameFromUClass = [](const UClass* Class) -> FString
	{
		return Class->GetPrefixCPP() + Class->GetName();
	};

	if (FArrayProperty* ArrayProperty = CastField<FArrayProperty>(Property))
	{
		return MakeContainerType(TEXT("TArray"), FromProperty(ArrayProperty->Inner), Property->ArrayDim);
	}

	if (auto* ClassProperty = CastField<FClassProperty>(Property))
	{
		return MakeContainerType(
			TEXT("TSubclassOf"),
			MakeConcreteType(GetCppNameFromUClass(ClassProperty->MetaClass), 1), Property->ArrayDim);
	}

	if (auto* ObjectProperty = CastField<FObjectProperty>(Property))
	{
		return MakeConcreteType(GetCppNameFromUClass(ObjectProperty->PropertyClass), Property->ArrayDim,
		                        TOptional<FString>(TEXT("UObject")));
	}

	if (auto* InnerProperty = CastField<FBoolProperty>(Property))
	{
		if (InnerProperty->IsNativeBool())
		{
			return MakeConcreteType(TEXT("bool"), Property->ArrayDim);
		}
		else
		{
			auto Bitfield = MakeUnique<RustReflection_Bitfield>();
			Bitfield->Offset = InnerProperty->GetByteOffset();
			Bitfield->FieldMask = InnerProperty->GetFieldMask();
			return Bitfield;
		}
	}

	if (auto* InnerProperty = CastField<FStrProperty>(Property))
	{
		return MakeConcreteType(TEXT("FString"), Property->ArrayDim);
	}

	if (auto* InnerProperty = CastField<FAnsiStrProperty>(Property))
	{
		return MakeConcreteType(TEXT("FAnsiString"), Property->ArrayDim);
	}

	if (auto* InnerProperty = CastField<FStructProperty>(Property))
	{
		return MakeConcreteType(InnerProperty->Struct->GetStructCPPName(), Property->ArrayDim);
	}

	if (auto* InnerProperty = CastField<FFieldPathProperty>(Property))
	{
		return MakeContainerType(
			TEXT("TFieldPath"),
			MakeConcreteType(FString::Printf(TEXT("F%s"), *InnerProperty->PropertyClass->GetName()),
			                 Property->ArrayDim), 1);
	}

	if (auto* InnerProperty = CastField<FSetProperty>(Property))
	{
		return MakeContainerType(
			TEXT("TSet"),
			FromProperty(InnerProperty->ElementProp), Property->ArrayDim);
	}

	if (auto* InnerProperty = CastField<FMapProperty>(Property))
	{
		auto MapType = MakeUnique<RustReflection_MapType>();
		MapType->KeyType = FromProperty(InnerProperty->KeyProp);
		MapType->ValueType = FromProperty(InnerProperty->ValueProp);

		return MapType;
	}

	if (auto* InnerProperty = CastField<FEnumProperty>(Property))
	{
		return MakeConcreteType(InnerProperty->GetEnum()->CppType, Property->ArrayDim);
	}

	if (auto* InnerProperty = CastField<FOptionalProperty>(Property))
	{
		return MakeContainerType(
			TEXT("TOptional"),
			FromProperty(InnerProperty->GetValueProperty()), Property->ArrayDim);
	}

	if (auto* InnerProperty = CastField<FNameProperty>(Property))
	{
		return MakeConcreteType(TEXT("FName"), Property->ArrayDim);
	}

	if (auto* InnerProperty = CastField<FTextProperty>(Property))
	{
		return MakeConcreteType(TEXT("FText"), Property->ArrayDim);
	}

	if (auto* InnerProperty = CastField<FUtf8StrProperty>(Property))
	{
		return MakeConcreteType(TEXT("FUtf8String"), Property->ArrayDim);
	}

	if (auto* InnerProperty = CastField<FLazyObjectProperty>(Property))
	{
		return MakeContainerType(
			TEXT("TLazyObjectPtr"),
			MakeConcreteType(GetCppNameFromUClass(InnerProperty->PropertyClass), 1), Property->ArrayDim);
	}

	if (auto* InnerProperty = CastField<FSoftObjectProperty>(Property))
	{
		return MakeContainerType(
			TEXT("TSoftObjectPtr"),
			MakeConcreteType(GetCppNameFromUClass(InnerProperty->PropertyClass), 1), Property->ArrayDim);
	}

	if (auto* InnerProperty = CastField<FSoftClassProperty>(Property))
	{
		return MakeContainerType(
			TEXT("TSoftClassPtr"),
			MakeConcreteType(GetCppNameFromUClass(InnerProperty->MetaClass), 1), Property->ArrayDim);
	}

	if (auto* InnerProperty = CastField<FInterfaceProperty>(Property))
	{
		FString InterfaceName;
		if (InnerProperty->InterfaceClass == UInterface::StaticClass())
		{
			return MakeConcreteType(TEXT("FScriptInterface"), Property->ArrayDim);
		}
		else
		{
			return MakeConcreteType(FString::Printf(TEXT("I%s"), *InnerProperty->InterfaceClass.GetName()),
			                        Property->ArrayDim,
			                        TOptional<FString>((TEXT("ScriptInterface"))));
		}
	}

	if (auto* InnerProperty = CastField<FWeakObjectProperty>(Property))
	{
		return MakeContainerType(
			TEXT("TWeakObjectPtr"),
			MakeConcreteType(GetCppNameFromUClass(InnerProperty->PropertyClass), 1), Property->ArrayDim);
	}

	if (auto* InnerProperty = CastField<FMulticastDelegateProperty>(Property))
	{
		auto OwningProperty = InnerProperty->GetOwnerStruct();
		auto SignatureName = InnerProperty->GetName();
		// SignatureName.RemoveFromEnd(TEXT("Delegate"));
		return MakeConcreteType(FString::Printf(TEXT("F%s_%s"), *OwningProperty->GetName(), *SignatureName),
		                        Property->ArrayDim);
	}

	if (auto* InnerProperty = CastField<FDelegateProperty>(Property))
	{
		auto OwningProperty = InnerProperty->GetOwnerStruct();
		auto SignatureName = InnerProperty->GetName();
		// SignatureName.RemoveFromEnd(TEXT("Delegate"));
		return MakeConcreteType(FString::Printf(TEXT("F%s_%s"), *OwningProperty->GetName(), *SignatureName),
		                        Property->ArrayDim);
	}

	if (auto* InnerProperty = CastField<FByteProperty>(Property))
	{
		if (InnerProperty->Enum != nullptr)
		{
			return MakeConcreteType(InnerProperty->Enum.GetName(), Property->ArrayDim);
		}

		return MakeConcreteType(TEXT("uint8"), Property->ArrayDim);
	}

	if (auto* InnerProperty = CastField<FNumericProperty>(Property))
	{
		return MakeConcreteType(InnerProperty->GetCPPType(nullptr, 0), Property->ArrayDim);
	};
	return MakeConcreteType(FString::Format(TEXT("GeneratorInvalid_ {0}"), {*Property->GetNameCPP()}),
	                        Property->ArrayDim);
}

FRustReflection_Property FRustReflection_Property::FromProperty(FProperty* Property)
{
	FRustReflection_Property PropertyReflection;
	PropertyReflection.Name = Property->GetName();
	PropertyReflection.Type = ::FromProperty(Property);
	PropertyReflection.PropertyFlags = GetPropertyFlagStrings(Property->GetPropertyFlags());
	PropertyReflection.Offset = Property->GetOffset_ForUFunction();
	PropertyReflection.Size = Property->GetSize();
	PropertyReflection.Alignment = Property->GetMinAlignment();

	auto DocText = Property->GetToolTipText();
	if (!DocText.IsEmpty())
	{
		PropertyReflection.Documentation = Property->GetToolTipText();
	}

	return PropertyReflection;
}

TSharedPtr<FJsonObject> RustReflection_ConcreteType::ToJson()
{
	auto Json = MakeShared<FJsonObject>();
	Json->SetStringField(TEXT("Kind"), TEXT("Concrete"));
	Json->SetNumberField(TEXT("ArrayDim"), ArrayDim);
	Json->SetStringField(TEXT("TypeName"), TypeName);
	if (Hint.IsSet())
	{
		Json->SetStringField(TEXT("UsageHint"), Hint.GetValue());
	}

	return Json;
}

TSharedPtr<FJsonObject> RustReflection_Bitfield::ToJson()
{
	auto Json = MakeShared<FJsonObject>();
	Json->SetStringField(TEXT("Kind"), TEXT("Bitfield"));
	Json->SetNumberField(TEXT("Offset"), Offset);
	Json->SetNumberField(TEXT("FieldMask"), FieldMask);
	Json->SetNumberField(TEXT("ArrayDim"), ArrayDim);
	return Json;
}

TSharedPtr<FJsonObject> RustReflection_ContainerType::ToJson()
{
	auto Json = MakeShared<FJsonObject>();
	Json->SetStringField(TEXT("Kind"), TEXT("Container"));
	Json->SetStringField(TEXT("ContainerTypeName"), ContainerTypeName);
	Json->SetObjectField(TEXT("InnerType"), InnerType->ToJson());
	Json->SetNumberField(TEXT("ArrayDim"), ArrayDim);

	return Json;
}

FRustReflection_Param FRustReflection_Param::FromProperty(FProperty* Property)
{
	FRustReflection_Param ParamReflection;


	ParamReflection.Property = FRustReflection_Property::FromProperty(Property);

	ParamReflection.bIsOut = Property->HasAnyPropertyFlags(CPF_OutParm);
	ParamReflection.bIsReturn = Property->HasAnyPropertyFlags(CPF_ReturnParm);
	ParamReflection.bIsRef = Property->HasAnyPropertyFlags(CPF_ReferenceParm);

	return ParamReflection;
}

FRustReflection_Function FRustReflection_Function::FromFunction(UFunction* Function)
{
	FRustReflection_Function ReflectionFunction;

	ReflectionFunction.Name = Function->GetName();
	ReflectionFunction.FunctionFlags = GetFunctionFlagStrings(Function->FunctionFlags);
	ReflectionFunction.Metadata = GetAllMetadata(Function);
	ReflectionFunction.ParamSize = Function->ParmsSize;

	for (TFieldIterator<FProperty> ParamIt(Function); ParamIt; ++ParamIt)
	{
		ReflectionFunction.Parameters.Add(FRustReflection_Param::FromProperty(*ParamIt));
	}

	auto DocText = Function->GetToolTipText();
	if (!DocText.IsEmpty())
	{
		ReflectionFunction.Documentation = Function->GetToolTipText();
	}

	return ReflectionFunction;
}

FRustReflection_UStruct FRustReflection_UStruct::FromStruct(UStruct* InStruct)
{
	// TODO: Change api to UScriptStruct
	UScriptStruct* Struct = Cast<UScriptStruct>(InStruct);
	check(Struct);

	FRustReflection_UStruct ReflectionStruct;
	// ReflectionStruct.StructFlags = GetClassFlagStrings(->ClassFlags);
	ReflectionStruct.StructName = Struct->GetPrefixCPP() + Struct->GetName();
	if (auto SuperStruct = Struct->GetSuperStruct())
	{
		ReflectionStruct.SuperStuctName = SuperStruct->GetPrefixCPP() + SuperStruct->GetName();
	}

	ReflectionStruct.StructFlags = GetStructFlagStrings(Struct->StructFlags);
	ReflectionStruct.Metadata = GetAllMetadata(Struct);
	ReflectionStruct.Package = Struct->GetOutermost()->GetPackage()->GetName();

	for (TFieldIterator<FProperty> ParamIt(Struct, EFieldIteratorFlags::ExcludeSuper); ParamIt; ++ParamIt)
	{
		ReflectionStruct.Properties.Add(FRustReflection_Property::FromProperty(*ParamIt));
	}

	auto DocText = Struct->GetToolTipText();
	if (!DocText.IsEmpty())
	{
		ReflectionStruct.Documentation = Struct->GetToolTipText();
	}

	ReflectionStruct.MinAlignment = Struct->MinAlignment;
	ReflectionStruct.PropertySizes = Struct->PropertiesSize;
	ReflectionStruct.Size = Struct->GetStructureSize();

	if (auto ScriptStruct = Cast<UScriptStruct>(Struct))
	{
		if (auto Ops = ScriptStruct->GetCppStructOps())
		{
			ReflectionStruct.bIsPlainOldData = Ops->IsPlainOldData();
		}
	}

	return ReflectionStruct;
}

FRustReflection_UClass FRustReflection_UClass::FromClass(UClass* Class)
{
	FRustReflection_UClass ClassReflection;

	FString Prefix = Class->GetPrefixCPP();
	ClassReflection.bIsInterface = Class->IsChildOf(UInterface::StaticClass()) && Class != UInterface::StaticClass();
	ClassReflection.Metadata = GetAllMetadata(Class);

	ClassReflection.ClassFlags = GetClassFlagStrings(Class->ClassFlags);
	ClassReflection.ClassName = Prefix + Class->GetName();

	ClassReflection.MinAlignment = Class->MinAlignment;
	ClassReflection.PropertySizes = Class->PropertiesSize;


	if (auto SuperClass = Class->GetSuperClass())
	{
		ClassReflection.SuperClassName = SuperClass->GetPrefixCPP() + SuperClass->GetName();
	}

	ClassReflection.Package = Class->GetOutermost()->GetPackage()->GetName();

	for (TFieldIterator<FProperty> ParamIt(Class, EFieldIteratorFlags::ExcludeSuper); ParamIt; ++ParamIt)
	{
		ClassReflection.Properties.Add(FRustReflection_Property::FromProperty(*ParamIt));
	}
	for (TFieldIterator<UFunction> ParamIt(Class, EFieldIteratorFlags::ExcludeSuper); ParamIt; ++ParamIt)
	{
		ClassReflection.Functions.Add(FRustReflection_Function::FromFunction(*ParamIt));
	}

	auto DocText = Class->GetToolTipText();
	if (!DocText.IsEmpty())
	{
		ClassReflection.Documentation = Class->GetToolTipText();
	}

	return ClassReflection;
}

FRustReflection_Enum FRustReflection_Enum::FromEnum(FRustReflection_EnumWithType& EnumWithType)
{
	FRustReflection_Enum ReflectionEnum;

	ReflectionEnum.Name = EnumWithType.Enum->GetName();
	ReflectionEnum.Type = MoveTemp(EnumWithType.Type);
	ReflectionEnum.Metadata = GetAllMetadata(EnumWithType.Enum);
	ReflectionEnum.Package = EnumWithType.Enum->GetOutermost()->GetPackage()->GetName();

	switch (EnumWithType.Enum->GetCppForm())
	{
	case UEnum::ECppForm::Regular:
		ReflectionEnum.Kind = FString(TEXT("Regular"));
		break;
	case UEnum::ECppForm::Namespaced:
		ReflectionEnum.Kind = FString(TEXT("Namespaced"));
		break;
	case UEnum::ECppForm::EnumClass:
		ReflectionEnum.Kind = FString(TEXT("EnumClass"));
		break;
	}

	for (int32 i = 0; i < EnumWithType.Enum->NumEnums(); ++i)
	{
		FRustReflection_EnumEntry Entry;


		FString EnumName = EnumWithType.Enum->GetNameStringByIndex(i);

		if (EnumName.Contains(TEXT("_MAX")))
		{
			continue;
		}

		// FString Left;
		// FString Right;
		//
		// if (EnumName.Split(TEXT("_"), &Left, &Right) && !Right.IsEmpty())
		// {
		// 	EnumName = *Right;
		// }
		//
		Entry.Name = EnumName;

		Entry.Value = EnumWithType.Enum->GetValueByIndex(i);

		FText Documentation = EnumWithType.Enum->GetToolTipTextByIndex(i);
		if (!Documentation.IsEmpty())
		{
			Entry.Documentation = Documentation;
		}

		ReflectionEnum.Entries.Add(Entry);
	}

	return ReflectionEnum;
}

TSharedPtr<FJsonObject> FRustReflection_Enum::ToJson()
{
	auto Json = MakeShared<FJsonObject>();
	Json->SetStringField(TEXT("Name"), Name);
	Json->SetObjectField(TEXT("Type"), Type->ToJson());
	Json->SetStringField(TEXT("Kind"), Kind);
	Json->SetStringField(TEXT("Package"), Package);

	TArray<TSharedPtr<FJsonValue>> JsonEntries;

	for (auto& Entry : Entries)
	{
		auto JsonEntry = MakeShared<FJsonObject>();
		JsonEntry->SetStringField(TEXT("Name"), Entry.Name);
		JsonEntry->SetNumberField(TEXT("Value"), Entry.Value);
		if (Entry.Documentation.IsSet())
		{
			JsonEntry->SetStringField(TEXT("Documentation"), Entry.Documentation.GetValue().ToString());
		}

		JsonEntries.Add(MakeShared<FJsonValueObject>(JsonEntry));
	}

	WriteMetadataField(Json, Metadata);
	Json->SetArrayField(TEXT("Entries"), JsonEntries);
	return Json;
}

TSharedPtr<FJsonObject> FRustReflection_Opague::ToJson()
{
	TSharedPtr<FJsonObject> Json = MakeShared<FJsonObject>();
	Json->SetStringField(TEXT("Name"), Name);
	Json->SetNumberField(TEXT("Alignment"), Alignment);
	Json->SetNumberField(TEXT("Size"), Size);
	return Json;
}

TSharedPtr<FJsonObject> FRustReflection_Delegate::ToJson()
{
	TSharedPtr<FJsonObject> Json = MakeShared<FJsonObject>();
	Json->SetStringField(TEXT("Name"), Name);
	Json->SetNumberField(TEXT("Size"), Size);
	Json->SetNumberField(TEXT("Alignment"), Alignment);
	Json->SetStringField(TEXT("Package"), Package);
	Json->SetStringField(TEXT("Namespace"), Namespace);
	Json->SetStringField(TEXT("Kind"), Kind);
	Json->SetObjectField(TEXT("Function"), Function.ToJson());
	return Json;
}

void WriteMetadataField(TSharedRef<FJsonObject> Json, TMap<FString, FString>& Metadata)
{
	TSharedPtr<FJsonObject> JsonMetadata = MakeShared<FJsonObject>();
	for (const auto& Elem : Metadata)
	{
		JsonMetadata->SetStringField(Elem.Key, Elem.Value);
	}

	Json->SetObjectField(TEXT("Metadata"), JsonMetadata);
}

TSharedPtr<FJsonObject> FRustReflection_Function::ToJson()
{
	auto Json = MakeShared<FJsonObject>();
	Json->SetStringField(TEXT("FunctionName"), Name);

	TArray<TSharedPtr<FJsonValue>> JsonFlags;
	for (const FString& Flag : FunctionFlags)
	{
		JsonFlags.Add(MakeShared<FJsonValueString>(Flag));
	}

	Json->SetArrayField(TEXT("Flags"), JsonFlags);

	Json->SetNumberField(TEXT("ParamSize"), ParamSize);

	TArray<TSharedPtr<FJsonValue>> JsonParams;
	for (auto& Param : Parameters)
	{
		JsonParams.Add(MakeShared<FJsonValueObject>(Param.ToJson()));
	}

	Json->SetArrayField(TEXT("Parameters"), JsonParams);

	WriteMetadataField(Json, Metadata);

	if (Documentation.IsSet())
	{
		Json->SetStringField(TEXT("Documentation"), Documentation.GetValue().ToString());
	}

	return Json;
}

TSharedPtr<FJsonObject> FRustReflection_Property::ToJson()
{
	auto Json = MakeShared<FJsonObject>();
	Json->SetStringField(TEXT("Name"), Name);
	Json->SetObjectField(TEXT("Type"), Type->ToJson());

	TArray<TSharedPtr<FJsonValue>> JsonFlags;
	for (const FString& Flag : PropertyFlags)
	{
		JsonFlags.Add(MakeShared<FJsonValueString>(Flag));
	}
	Json->SetArrayField(TEXT("Flags"), JsonFlags);

	if (Documentation.IsSet())
	{
		Json->SetStringField(TEXT("Documentation"), Documentation.GetValue().ToString());
	}

	Json->SetNumberField(TEXT("Offset"), Offset);
	Json->SetNumberField(TEXT("Size"), Size);
	Json->SetNumberField(TEXT("Alignment"), Alignment);

	return Json;
}

TSharedPtr<FJsonObject> FRustReflection_Param::ToJson()
{
	auto Json = MakeShared<FJsonObject>();

	Json->SetObjectField(TEXT("Property"), Property.ToJson());
	Json->SetBoolField(TEXT("IsOut"), bIsOut);
	Json->SetBoolField(TEXT("IsRef"), bIsRef);
	Json->SetBoolField(TEXT("IsReturn"), bIsReturn);
	return Json;
}

TSharedPtr<FJsonObject> FRustReflection_UStruct::ToJson()
{
	auto Json = MakeShared<FJsonObject>();

	Json->SetStringField(TEXT("StructName"), StructName);

	TArray<TSharedPtr<FJsonValue>> JsonFlags;
	for (const FString& Flag : StructFlags)
	{
		JsonFlags.Add(MakeShared<FJsonValueString>(Flag));
	}

	WriteMetadataField(Json, Metadata);
	Json->SetArrayField(TEXT("Flags"), JsonFlags);

	if (SuperStuctName.IsSet())
	{
		Json->SetStringField(TEXT("SuperStruct"), SuperStuctName.GetValue());
	}
	Json->SetStringField(TEXT("Package"), Package);

	TArray<TSharedPtr<FJsonValue>> JsonProperties;
	for (auto& Property : Properties)
	{
		JsonProperties.Add(MakeShared<FJsonValueObject>(Property.ToJson()));
	}
	FScriptArray Arr;
	Json->SetNumberField(TEXT("MinAlignment"), MinAlignment);
	Json->SetNumberField(TEXT("PropertySizes"), PropertySizes);
	Json->SetNumberField(TEXT("Size"), Size);
	Json->SetBoolField(TEXT("IsPlainOldData"), bIsPlainOldData);

	Json->SetArrayField(TEXT("Properties"), JsonProperties);

	if (Documentation.IsSet())
	{
		Json->SetStringField(TEXT("Documentation"), Documentation.GetValue().ToString());
	}

	return Json;
}

TSharedPtr<FJsonObject> FRustReflection_UClass::ToJson()
{
	auto Json = MakeShared<FJsonObject>();

	Json->SetStringField(TEXT("ClassName"), ClassName);

	TArray<TSharedPtr<FJsonValue>> JsonFlags;
	for (const FString& Flag : ClassFlags)
	{
		JsonFlags.Add(MakeShared<FJsonValueString>(Flag));
	}

	WriteMetadataField(Json, Metadata);
	Json->SetArrayField(TEXT("Flags"), JsonFlags);

	if (SuperClassName.IsSet())
	{
		Json->SetStringField(TEXT("SuperClass"), SuperClassName.GetValue());
	}

	Json->SetBoolField(TEXT("IsInterface"), bIsInterface);
	Json->SetStringField(TEXT("Package"), Package);

	Json->SetNumberField(TEXT("MinAlignment"), MinAlignment);
	Json->SetNumberField(TEXT("PropertySizes"), PropertySizes);

	TArray<TSharedPtr<FJsonValue>> JsonProperties;
	for (auto& Property : Properties)
	{
		JsonProperties.Add(MakeShared<FJsonValueObject>(Property.ToJson()));
	}

	Json->SetNumberField(TEXT("MinAlignment"), MinAlignment);
	Json->SetNumberField(TEXT("PropertySizes"), PropertySizes);

	Json->SetArrayField(TEXT("Properties"), JsonProperties);

	TArray<TSharedPtr<FJsonValue>> JsonFunctions;
	for (auto& Function : Functions)
	{
		JsonFunctions.Add(MakeShared<FJsonValueObject>(Function.ToJson()));
	}

	Json->SetArrayField(TEXT("Functions"), JsonFunctions);

	if (Documentation.IsSet())
	{
		Json->SetStringField(TEXT("Documentation"), Documentation.GetValue().ToString());
	}

	return Json;
}

void FRustReflection_Root::ExportToJson_Classes(TSharedPtr<FJsonObject> Json)
{
	TArray<TSharedPtr<FJsonValue>> JsonClasses;
	for (TObjectIterator<UClass> It; It; ++It)
	{
		if (!It->HasAnyClassFlags(CLASS_Native))
		{
			continue;
		}

		auto Class = FRustReflection_UClass::FromClass(*It);
		JsonClasses.Add(MakeShared<FJsonValueObject>(Class.ToJson()));
	}

	UE_LOG(LogTemp, Warning, TEXT("%i"), JsonClasses.Num());
	Json->SetArrayField(TEXT("Classes"), JsonClasses);
}

void FRustReflection_Root::ExportToJson_Structs(TSharedPtr<FJsonObject> Json)
{
	TArray<TSharedPtr<FJsonValue>> JsonStructs;
	for (TObjectIterator<UScriptStruct> It; It; ++It)
	{
		if (!It->IsNative())
		{
			continue;
		}

		auto Struct = FRustReflection_UStruct::FromStruct(*It);
		JsonStructs.Add(MakeShared<FJsonValueObject>(Struct.ToJson()));
	}
	Json->SetArrayField(TEXT("Structs"), JsonStructs);
}

static FProperty* FindPropertyRecursive(
	FProperty* Prop,
	TFunctionRef<bool(FProperty* /*Current*/)> Predicate)
{
	if (!Prop)
		return nullptr;

	if (Predicate(Prop))
		return Prop;

	if (FArrayProperty* Arr = CastField<FArrayProperty>(Prop))
		return FindPropertyRecursive(Arr->Inner, Predicate);

	if (FSetProperty* Set = CastField<FSetProperty>(Prop))
		return FindPropertyRecursive(Set->ElementProp, Predicate);

	if (FMapProperty* Map = CastField<FMapProperty>(Prop))
	{
		if (FProperty* Found = FindPropertyRecursive(Map->KeyProp, Predicate))
			return Found;

		if (FProperty* Found = FindPropertyRecursive(Map->ValueProp, Predicate))
			return Found;

		return nullptr;
	}

	if (FStructProperty* StructProp = CastField<FStructProperty>(Prop))
	{
		UStruct* Struct = StructProp->Struct;
		for (FProperty* Field = Struct->PropertyLink; Field; Field = Field->PropertyLinkNext)
		{
			if (FProperty* Found = FindPropertyRecursive(Field, Predicate))
				return Found;
		}
		return nullptr;
	}

	if (FDelegateProperty* Del = CastField<FDelegateProperty>(Prop))
	{
		if (UFunction* Sig = Del->SignatureFunction)
		{
			for (FProperty* P = Sig->PropertyLink; P; P = P->PropertyLinkNext)
			{
				if (FProperty* Found = FindPropertyRecursive(P, Predicate))
					return Found;
			}
		}
		return nullptr;
	}

	if (FMulticastDelegateProperty* Multi = CastField<FMulticastDelegateProperty>(Prop))
	{
		if (UFunction* Sig = Multi->SignatureFunction)
		{
			for (FProperty* P = Sig->PropertyLink; P; P = P->PropertyLinkNext)
			{
				if (FProperty* Found = FindPropertyRecursive(P, Predicate))
					return Found;
			}
		}
		return nullptr;
	}

	return nullptr;
}

void FRustReflection_Root::ExportToJson_Delegates(TSharedPtr<FJsonObject> Json)
{
	TSet<FString> DelegateNames;
	TArray<FRustReflection_Delegate> Delegates;
	for (TObjectIterator<UStruct> StructIter; StructIter; ++StructIter)
	{
		UStruct* Struct = *StructIter;

		for (TFieldIterator<FProperty> FieldIter(Struct, EFieldIteratorFlags::ExcludeSuper); FieldIter; ++
		     FieldIter)
		{
			FProperty* DelegateProperty = FindPropertyRecursive(*FieldIter, [](FProperty* P)
			{
				return P->IsA<FDelegateProperty>() || P->IsA<FMulticastDelegateProperty>();
			});

			if (DelegateProperty == nullptr)
			{
				continue;
			}

			if (auto InnerProperty = CastField<FMulticastDelegateProperty>(DelegateProperty))
			{
				FRustReflection_Delegate Delegate;
				Delegate.Name = FString::Format(TEXT("F{0}_{1}"), {
					                                *Struct->GetName(), InnerProperty->GetName()
				                                });
				if (DelegateNames.Contains(Delegate.Name))
				{
					continue;
				}
				Delegate.Package = InnerProperty->GetOutermost()->GetPackage()->GetName();
				Delegate.Size = InnerProperty->GetSize();
				Delegate.Alignment = InnerProperty->GetMinAlignment();
				Delegate.Namespace = Struct->GetName();
				Delegate.Kind = TEXT("Multicast");
				Delegate.Function = FRustReflection_Function::FromFunction(
					InnerProperty->SignatureFunction.Get());
				DelegateNames.Add(Delegate.Name);
				Delegates.Add(MoveTemp(Delegate));
			}

			if (auto* InnerProperty = CastField<FDelegateProperty>(DelegateProperty))
			{
				FRustReflection_Delegate Delegate;
				Delegate.Name = FString::Format(TEXT("F{0}_{1}"), {
					                                *Struct->GetName(), InnerProperty->GetName()
				                                });
				if (DelegateNames.Contains(Delegate.Name))
				{
					continue;
				}
				Delegate.Size = InnerProperty->GetSize();
				Delegate.Alignment = InnerProperty->GetMinAlignment();
				Delegate.Namespace = Struct->GetName();
				Delegate.Kind = TEXT("Single");
				Delegate.Package = InnerProperty->GetOutermost()->GetPackage()->GetName();
				Delegate.Function = FRustReflection_Function::FromFunction(
					InnerProperty->SignatureFunction.Get());
				if (DelegateNames.Contains(Delegate.Name))
				{
					continue;
				}
				DelegateNames.Add(Delegate.Name);
				Delegates.Add(MoveTemp(Delegate));
			}
		}
	};

	TArray<TSharedPtr<FJsonValue>> JsonDelegates;
	for (auto& Delegate : Delegates)
	{
		JsonDelegates.Add(MakeShared<FJsonValueObject>(Delegate.ToJson()));
	}

	Json->SetArrayField(TEXT("DelegateDefinitions"), JsonDelegates);
}

void FRustReflection_Root::ExportToJson_Enum(TSharedPtr<FJsonObject> Json)
{
	// This is really dumb. I don't have the size or type of the enum for a UEnum. So I need
	// to discover all the used enums which then allows me to get their size
	// I should maybe check that all found sizes are the same
	TSet<UEnum*> FoundEnums;
	TArray<FRustReflection_EnumWithType> EnumsWithType;
	for (TObjectIterator<UStruct> StructIter; StructIter; ++StructIter)
	{
		for (TFieldIterator<FProperty> FieldIter(*StructIter, EFieldIteratorFlags::ExcludeSuper); FieldIter; ++
		     FieldIter)
		{
			FProperty* FoundEnumProperty = FindPropertyRecursive(*FieldIter, [](FProperty* P)
			{
				if (P->IsA<FEnumProperty>())
				{
					return true;
				}

				if (auto* InnerProperty = CastField<FByteProperty>(P))
				{
					if (InnerProperty->Enum != nullptr)
					{
						return true;
					}
				}
				return false;
			});

			UEnum* Enum = nullptr;
			TUniquePtr<RustReflection_Type> Type;

			if (auto EnumProperty = CastField<FEnumProperty>(FoundEnumProperty))
			{
				Enum = EnumProperty->GetEnum();
				Type = FromProperty(EnumProperty->GetUnderlyingProperty());
			}

			if (auto* InnerProperty = CastField<FByteProperty>(FoundEnumProperty))
			{
				Enum = InnerProperty->Enum;
				Type = MakeConcreteType(TEXT("uint8"), 1);
			}

			if (Enum != nullptr && !FoundEnums.Contains(Enum))
			{
				FRustReflection_EnumWithType EnumWithType;
				EnumWithType.Enum = Enum;
				EnumWithType.Type = MoveTemp(Type);
				EnumsWithType.Add(MoveTemp(EnumWithType));
				FoundEnums.Add(Enum);
			}
		}
	}

	TArray<TSharedPtr<FJsonValue>> JsonEnums;
	for (auto& EnumWithType : EnumsWithType)
	{
		auto Enum = FRustReflection_Enum::FromEnum(EnumWithType);
		JsonEnums.Add(MakeShared<FJsonValueObject>(Enum.ToJson()));
	}

	Json->SetArrayField(TEXT("Enums"), JsonEnums);
}

void FRustReflection_Root::ExportToJson_GameplayTags(TSharedPtr<FJsonObject> Json)
{
	auto& Manager = UGameplayTagsManager::Get();
	FGameplayTagContainer AllTags;
	Manager.RequestAllGameplayTags(AllTags, true);

	TArray<TSharedPtr<FJsonValue>> JsonTagsArray;
	for (FGameplayTag Tag : AllTags)
	{
		auto JsonTagObject = MakeShared<FJsonObject>();
		JsonTagObject->SetStringField(TEXT("Tag"), Tag.ToString());

		JsonTagsArray.Add(MakeShared<FJsonValueObject>(JsonTagObject));
	}

	Json->SetArrayField(TEXT("Tags"), JsonTagsArray);
}

void FRustReflection_Root::ExportToJson_ConsoleVariables(TSharedPtr<FJsonObject> Json)
{
	// TArray<TSharedPtr<FJsonValue>> JsonVariablesArray;
	// IConsoleManager& ConsoleMgr = IConsoleManager::Get();
	// for (TObjectIterator<IConsoleObject> It; It; ++It)
	// {
	// 	IConsoleObject* Obj = *It;
	//        
	// 	// We only want Variables (CVars), not Commands (Funcs)
	// 	if (IConsoleVariable* CVar = Obj->AsVariable())
	// 	{
	// 		auto JsonObject = MakeShared<FJsonObject>();
	// 		Json->SetStringField(TEXT("Name"), CVar->GetString());
	// 		JsonVariablesArray.Add(MakeShared<FJsonValueObject>(JsonObject));
	// 	}
	// }
	// Json->SetArrayField(TEXT("ConsoleVariables"), JsonVariablesArray);
}

void FRustReflection_Root::ExportToJson_OpagueTypes(TSharedPtr<FJsonObject> Json)
{
	TArray<TSharedPtr<FJsonValue>> JsonArray;

	FRustReflection_Opague Text;
	Text.Name = TEXT("FText");
	Text.Alignment = alignof(FText);
	Text.Size = sizeof(FText);

	JsonArray.Add(MakeShared<FJsonValueObject>(Text.ToJson()));

	Json->SetArrayField(TEXT("OpagueDefinitions"), JsonArray);
}

FRustReflection_Root FRustReflection_Root::Collect()
{
	FRustReflection_Root Root;
	auto Json = MakeShared<FJsonObject>();

	TIME_SCOPE("ExportToJson_OpagueTypes", ExportToJson_OpagueTypes(Json));
	TIME_SCOPE("ExportToJson_Delegates", ExportToJson_Delegates(Json));
	TIME_SCOPE("ExportToJson_Enum", ExportToJson_Enum(Json));
	TIME_SCOPE("ExportToJson_ConsoleVariables", ExportToJson_ConsoleVariables(Json));
	TIME_SCOPE("ExportToJson_GameplayTags", ExportToJson_GameplayTags(Json));
	TIME_SCOPE("ExportToJson_Structs", ExportToJson_Structs(Json));
	TIME_SCOPE("ExportToJson_Classes", ExportToJson_Classes(Json));

	FString OutputString;

	TSharedRef<TJsonWriter<>> Writer = TJsonWriterFactory<>::Create(&OutputString);

	FJsonSerializer::Serialize(Json, Writer);

	FFileHelper::SaveStringToFile(OutputString, TEXT("C:\\Users\\maikk\\Documents\\unreal-rust\\api.json"),
	                              FFileHelper::EEncodingOptions::ForceUTF8);

	return Root;
}
