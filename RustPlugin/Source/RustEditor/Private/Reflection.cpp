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

static TArray<FString> GetPropertyFlagStrings(uint64 Flags)
{
	TArray<FString> ActiveFlags;

	// -------------------------------------------------------------------
	// 1. EDIT & VISIBILITY SPECIFIERS (Mutually Exclusive Logic)
	// -------------------------------------------------------------------
	// In Unreal, 'Visible' is just 'Edit' turned on + 'EditConst' (Read Only).

	if (Flags & CPF_Edit)
	{
		// --- CASE A: VISIBLE (ReadOnly in Editor) ---
		if (Flags & CPF_EditConst)
		{
			if (Flags & CPF_DisableEditOnInstance)
			{
				ActiveFlags.Add(TEXT("VisibleDefaultsOnly"));
			}
			else if (Flags & CPF_DisableEditOnTemplate)
			{
				ActiveFlags.Add(TEXT("VisibleInstanceOnly"));
			}
			else
			{
				ActiveFlags.Add(TEXT("VisibleAnywhere"));
			}
		}
		// --- CASE B: EDIT (Read/Write in Editor) ---
		else
		{
			if (Flags & CPF_DisableEditOnInstance)
			{
				ActiveFlags.Add(TEXT("EditDefaultsOnly"));
			}
			else if (Flags & CPF_DisableEditOnTemplate)
			{
				ActiveFlags.Add(TEXT("EditInstanceOnly"));
			}
			else
			{
				ActiveFlags.Add(TEXT("EditAnywhere"));
			}
		}
	}

	// -------------------------------------------------------------------
	// 2. BLUEPRINT ACCESS (Composite Logic)
	// -------------------------------------------------------------------

	if (Flags & CPF_BlueprintVisible)
	{
		if (Flags & CPF_BlueprintReadOnly)
		{
			ActiveFlags.Add(TEXT("BlueprintReadOnly"));
		}
		else
		{
			ActiveFlags.Add(TEXT("BlueprintReadWrite"));
		}
	}

	// Note: BlueprintAssignable/Callable are separate from the Read/Write logic
	// They are specific to Delegates (Multicast or Single)
	if (Flags & CPF_BlueprintAssignable) ActiveFlags.Add(TEXT("BlueprintAssignable"));
	if (Flags & CPF_BlueprintCallable) ActiveFlags.Add(TEXT("BlueprintCallable"));
	if (Flags & CPF_BlueprintAuthorityOnly) ActiveFlags.Add(TEXT("BlueprintAuthorityOnly"));

	// -------------------------------------------------------------------
	// 3. NETWORKING
	// -------------------------------------------------------------------

	// If it has RepNotify, it implies replication, but usually we write the specific one.
	if (Flags & CPF_RepNotify)
	{
		ActiveFlags.Add(TEXT("ReplicatedUsing"));
		// Note: We don't add "Replicated" here because ReplicatedUsing implies it.
	}
	else if (Flags & CPF_Net)
	{
		ActiveFlags.Add(TEXT("Replicated"));
	}

	if (Flags & CPF_RepSkip) ActiveFlags.Add(TEXT("RepSkip")); // Advanced

	// -------------------------------------------------------------------
	// 4. PARAMETER FLAGS (If you are reflecting functions)
	// -------------------------------------------------------------------
	// You typically only see these on properties inside a UFunction

	if (Flags & CPF_Parm)
	{
		// "Return Value" is a parameter, but handled specially
		if (Flags & CPF_ReturnParm)
		{
			ActiveFlags.Add(TEXT("ReturnParm"));
		}
		else
		{
			// Standard Parameter
			if (Flags & CPF_ConstParm) ActiveFlags.Add(TEXT("Const"));
			if (Flags & CPF_OutParm) ActiveFlags.Add(TEXT("Out")); // Logic for TArray&, etc.
			if (Flags & CPF_ReferenceParm) ActiveFlags.Add(TEXT("Ref"));
		}
	}

	// -------------------------------------------------------------------
	// 5. STANDARD INDEPENDENT FLAGS
	// -------------------------------------------------------------------

	if (Flags & CPF_ExposeOnSpawn) ActiveFlags.Add(TEXT("ExposeOnSpawn"));
	if (Flags & CPF_Config) ActiveFlags.Add(TEXT("Config"));
	if (Flags & CPF_GlobalConfig) ActiveFlags.Add(TEXT("GlobalConfig"));
	if (Flags & CPF_Transient) ActiveFlags.Add(TEXT("Transient"));
	if (Flags & CPF_DuplicateTransient) ActiveFlags.Add(TEXT("DuplicateTransient"));
	if (Flags & CPF_TextExportTransient) ActiveFlags.Add(TEXT("TextExportTransient"));
	if (Flags & CPF_NonPIEDuplicateTransient) ActiveFlags.Add(TEXT("NonPIEDuplicateTransient"));
	if (Flags & CPF_SaveGame) ActiveFlags.Add(TEXT("SaveGame"));
	if (Flags & CPF_NoClear) ActiveFlags.Add(TEXT("NoClear"));
	if (Flags & CPF_Interp) ActiveFlags.Add(TEXT("Interp"));
	if (Flags & CPF_NonTransactional) ActiveFlags.Add(TEXT("NonTransactional"));
	if (Flags & CPF_EditorOnly) ActiveFlags.Add(TEXT("EditorOnly"));
	if (Flags & CPF_AdvancedDisplay) ActiveFlags.Add(TEXT("AdvancedDisplay"));
	if (Flags & CPF_SimpleDisplay) ActiveFlags.Add(TEXT("SimpleDisplay"));
	if (Flags & CPF_AssetRegistrySearchable) ActiveFlags.Add(TEXT("AssetRegistrySearchable"));

	// Instanced is tricky: 
	// - EditInlineNew (Class flag) allows creation.
	// - Instanced (Property flag) means the object is owned by this container.
	if (Flags & CPF_InstancedReference) ActiveFlags.Add(TEXT("Instanced"));
	if (Flags & CPF_ContainsInstancedReference) ActiveFlags.Add(TEXT("ContainsInstancedReference"));

	// -------------------------------------------------------------------
	// 6. DEPRECATION
	// -------------------------------------------------------------------
	if (Flags & CPF_Deprecated) ActiveFlags.Add(TEXT("Deprecated"));

	return ActiveFlags;
}

static TArray<FString> GetClassFlagStrings(uint32 Flags)
{
	TArray<FString> ActiveFlags;

	// --- Valid UCLASS() Specifiers ---

	// UCLASS(Abstract)
	if (Flags & CLASS_Abstract) ActiveFlags.Add(TEXT("Abstract"));

	// UCLASS(Transient)
	if (Flags & CLASS_Transient) ActiveFlags.Add(TEXT("Transient"));

	// UCLASS(Config=...) -> Sets CLASS_Config
	if (Flags & CLASS_Config) ActiveFlags.Add(TEXT("Config"));

	// UCLASS(DefaultConfig)
	if (Flags & CLASS_DefaultConfig) ActiveFlags.Add(TEXT("DefaultConfig"));

	// UCLASS(GlobalUserConfig)
	if (Flags & CLASS_GlobalUserConfig) ActiveFlags.Add(TEXT("GlobalUserConfig"));

	// UCLASS(ProjectUserConfig)
	if (Flags & CLASS_ProjectUserConfig) ActiveFlags.Add(TEXT("ProjectUserConfig"));

	// UCLASS(PerObjectConfig)
	if (Flags & CLASS_PerObjectConfig) ActiveFlags.Add(TEXT("PerObjectConfig"));

	// UCLASS(PerPlatformConfig)
	if (Flags & CLASS_PerPlatformConfig) ActiveFlags.Add(TEXT("PerPlatformConfig"));

	// UCLASS(ConfigDoNotCheckDefaults)
	if (Flags & CLASS_ConfigDoNotCheckDefaults) ActiveFlags.Add(TEXT("ConfigDoNotCheckDefaults"));

	// UCLASS(NotPlaceable)
	if (Flags & CLASS_NotPlaceable) ActiveFlags.Add(TEXT("NotPlaceable"));

	// UCLASS(EditInlineNew)
	if (Flags & CLASS_EditInlineNew) ActiveFlags.Add(TEXT("EditInlineNew"));

	// UCLASS(CollapseCategories)
	if (Flags & CLASS_CollapseCategories) ActiveFlags.Add(TEXT("CollapseCategories"));

	// UCLASS(Const)
	if (Flags & CLASS_Const) ActiveFlags.Add(TEXT("Const"));

	// UCLASS(MinimalAPI)
	if (Flags & CLASS_MinimalAPI) ActiveFlags.Add(TEXT("MinimalAPI"));

	// UCLASS(DefaultToInstanced)
	if (Flags & CLASS_DefaultToInstanced) ActiveFlags.Add(TEXT("DefaultToInstanced"));

	// UCLASS(Hidden)
	if (Flags & CLASS_Hidden) ActiveFlags.Add(TEXT("Hidden"));

	// UCLASS(Deprecated)
	if (Flags & CLASS_Deprecated) ActiveFlags.Add(TEXT("Deprecated"));

	// UCLASS(HideDropDown)
	if (Flags & CLASS_HideDropDown) ActiveFlags.Add(TEXT("HideDropDown"));

	return ActiveFlags;
}

static TArray<FString> GetFunctionFlagStrings(UFunction* Function)
{
	uint32 Flags = Function->FunctionFlags;

	TArray<FString> ActiveFlags;

	// -------------------------------------------------------------------
	// 1. BLUEPRINT LOGIC (The Composite Tricky Part)
	// -------------------------------------------------------------------

	// Logic for BlueprintPure vs BlueprintCallable
	if (Flags & FUNC_BlueprintPure)
	{
		ActiveFlags.Add(TEXT("BlueprintPure"));
	}
	else if (Flags & FUNC_BlueprintCallable)
	{
		ActiveFlags.Add(TEXT("BlueprintCallable"));
	}

	// Logic for BlueprintImplementableEvent vs BlueprintNativeEvent
	// Both are 'Events' and 'BlueprintEvents', but NativeEvent is also 'Native'.
	if (Flags & FUNC_Event)
	{
		// If it is an event, we check if it is implementable or native
		if (Flags & FUNC_BlueprintEvent)
		{
			if (Flags & FUNC_Native)
			{
				ActiveFlags.Add(TEXT("BlueprintNativeEvent"));
			}
			else
			{
				ActiveFlags.Add(TEXT("BlueprintImplementableEvent"));
			}
		}
	}

	// -------------------------------------------------------------------
	// 2. NETWORKING (RPCs)
	// -------------------------------------------------------------------

	if (Flags & FUNC_Net)
	{
		// Determine type of RPC
		if (Flags & FUNC_NetMulticast)
		{
			ActiveFlags.Add(TEXT("NetMulticast"));
		}
		else if (Flags & FUNC_NetServer)
		{
			ActiveFlags.Add(TEXT("Server"));
		}
		else if (Flags & FUNC_NetClient)
		{
			ActiveFlags.Add(TEXT("Client"));
		}

		// Reliability is a separate modifier for RPCs
		if (Flags & FUNC_NetReliable)
		{
			ActiveFlags.Add(TEXT("Reliable"));
		}
		else
		{
			ActiveFlags.Add(TEXT("Unreliable"));
		}
	}

	// -------------------------------------------------------------------
	// 3. EDITOR & TOOLS
	// -------------------------------------------------------------------

	if (Flags & FUNC_Exec) ActiveFlags.Add(TEXT("Exec"));

	// -------------------------------------------------------------------
	// 4. ADVANCED / MEMORY
	// -------------------------------------------------------------------

	if (Flags & FUNC_Static) ActiveFlags.Add(TEXT("Static"));
	if (Flags & FUNC_Const) ActiveFlags.Add(TEXT("Const"));

	// UFUNCTION(ServiceRequest) / UFUNCTION(ServiceResponse)
	if (Flags & FUNC_NetRequest) ActiveFlags.Add(TEXT("ServiceRequest"));
	if (Flags & FUNC_NetResponse) ActiveFlags.Add(TEXT("ServiceResponse"));

	if (Function->GetBoolMetaData(TEXT("CallInEditor")))
	{
		// You can manually add this string to your Flags array if you want 
		// it to appear alongside the others in your JSON.
		ActiveFlags.Add(TEXT("CallInEditor"));
	}

	return ActiveFlags;
}


TUniquePtr<RustReflection_Type> FromUClass(UClass* Class)
{
	return MakeContainerType(TEXT("TSubclassOf"), MakeConcreteType(Class->GetPrefixCPP() + Class->GetName()));
}

TSharedPtr<FJsonObject> RustReflection_MapType::ToJson()
{
	auto Json = MakeShared<FJsonObject>();
	Json->SetStringField(TEXT("Kind"), TEXT("Map"));
	Json->SetObjectField(TEXT("KeyType"), KeyType->ToJson());
	Json->SetObjectField(TEXT("ValueType"), ValueType->ToJson());

	return Json;
}

TUniquePtr<RustReflection_ConcreteType> MakeConcreteType(FString Name)
{
	auto Type = MakeUnique<RustReflection_ConcreteType>();
	Type->TypeName = Name;

	return Type;
}

TUniquePtr<RustReflection_ContainerType> MakeContainerType(FString ContainerTypeName,
                                                           TUniquePtr<RustReflection_Type> InnerType)
{
	auto ContainerType = MakeUnique<RustReflection_ContainerType>();
	ContainerType->ContainerTypeName = ContainerTypeName;
	ContainerType->InnerType = MoveTemp(InnerType);

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
		return MakeContainerType(TEXT("TArray"), FromProperty(ArrayProperty->Inner));
	}

	if (auto* ClassProperty = CastField<FClassProperty>(Property))
	{
		return MakeContainerType(
			TEXT("TSubclassOf"),
			MakeConcreteType(GetCppNameFromUClass(ClassProperty->MetaClass)));
	}

	if (auto* ObjectProperty = CastField<FObjectProperty>(Property))
	{
		return MakeConcreteType(GetCppNameFromUClass(ObjectProperty->PropertyClass));
	}

	if (auto* InnerProperty = CastField<FBoolProperty>(Property))
	{
		return MakeConcreteType(TEXT("bool"));
	}

	if (auto* InnerProperty = CastField<FStrProperty>(Property))
	{
		return MakeConcreteType(TEXT("FString"));
	}

	if (auto* InnerProperty = CastField<FStructProperty>(Property))
	{
		return MakeConcreteType(InnerProperty->Struct->GetStructCPPName());
	}

	if (auto* InnerProperty = CastField<FFieldPathProperty>(Property))
	{
		return MakeContainerType(
			TEXT("TFieldPath"),
			MakeConcreteType(FString::Printf(TEXT("F%s"), *InnerProperty->PropertyClass->GetName())));
	}

	if (auto* InnerProperty = CastField<FSetProperty>(Property))
	{
		return MakeContainerType(
			TEXT("TSet"),
			FromProperty(InnerProperty->ElementProp));
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
		return MakeConcreteType(InnerProperty->GetNameCPP());
	}

	if (auto* InnerProperty = CastField<FOptionalProperty>(Property))
	{
		return MakeContainerType(
			TEXT("TOptional"),
			FromProperty(InnerProperty->GetValueProperty()));
	}

	if (auto* InnerProperty = CastField<FNameProperty>(Property))
	{
		return MakeConcreteType(TEXT("FName"));
	}

	if (auto* InnerProperty = CastField<FTextProperty>(Property))
	{
		return MakeConcreteType(TEXT("FText"));
	}

	if (auto* InnerProperty = CastField<FUtf8StrProperty>(Property))
	{
		return MakeConcreteType(TEXT("FUtf8String"));
	}

	if (auto* InnerProperty = CastField<FLazyObjectProperty>(Property))
	{
		return MakeContainerType(
			TEXT("TLazyObjectPtr"),
			MakeConcreteType(GetCppNameFromUClass(InnerProperty->PropertyClass)));
	}

	if (auto* InnerProperty = CastField<FSoftObjectProperty>(Property))
	{
		return MakeContainerType(
			TEXT("TSoftObjectPtr"),
			MakeConcreteType(GetCppNameFromUClass(InnerProperty->PropertyClass)));
	}

	if (auto* InnerProperty = CastField<FSoftClassProperty>(Property))
	{
		return MakeContainerType(
			TEXT("TSoftClassPtr"),
			MakeConcreteType(GetCppNameFromUClass(InnerProperty->MetaClass)));
	}

	if (auto* InnerProperty = CastField<FInterfaceProperty>(Property))
	{
		return MakeConcreteType(FString::Printf(TEXT("I%s"), *InnerProperty->InterfaceClass.GetName()));
	}

	if (auto* InnerProperty = CastField<FWeakObjectProperty>(Property))
	{
		return MakeContainerType(
			TEXT("TWeakObjectPtr"),
			MakeConcreteType(GetCppNameFromUClass(InnerProperty->PropertyClass)));
	}

	if (auto* InnerProperty = CastField<FMulticastDelegateProperty>(Property))
	{
		auto SignatureName = InnerProperty->SignatureFunction.GetName();
		SignatureName.RemoveFromEnd(TEXT("__DelegateSignature"));

		return MakeConcreteType(FString::Printf(TEXT("F%s"), *SignatureName));
	}

	if (auto* InnerProperty = CastField<FDelegateProperty>(Property))
	{
		auto SignatureName = InnerProperty->SignatureFunction.GetName();
		SignatureName.RemoveFromEnd(TEXT("__DelegateSignature"));

		return MakeConcreteType(FString::Printf(TEXT("F%s"), *SignatureName));
	}

	if (auto* InnerProperty = CastField<FByteProperty>(Property))
	{
		if (InnerProperty->Enum != nullptr)
		{
			return MakeConcreteType(InnerProperty->Enum.GetName());
		}

		return MakeConcreteType(TEXT("uint8"));
	}

	if (auto* InnerProperty = CastField<FNumericProperty>(Property))
	{
		return MakeConcreteType(InnerProperty->GetCPPType(nullptr, 0));
	}

	return MakeConcreteType(TEXT("GeneratorInvalidType"));
}

FRustReflection_Property FRustReflection_Property::FromProperty(FProperty* Property)
{
	FRustReflection_Property PropertyReflection;
	PropertyReflection.Name = Property->GetName();
	PropertyReflection.Type = ::FromProperty(Property);
	PropertyReflection.PropertyFlags = GetPropertyFlagStrings(Property->GetPropertyFlags());
	PropertyReflection.Offset = Property->GetOffset_ForUFunction();

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
	Json->SetStringField(TEXT("TypeName"), TypeName);

	return Json;
}

TSharedPtr<FJsonObject> RustReflection_ContainerType::ToJson()
{
	auto Json = MakeShared<FJsonObject>();
	Json->SetStringField(TEXT("Kind"), TEXT("Container"));
	Json->SetStringField(TEXT("ContainerTypeName"), ContainerTypeName);
	Json->SetObjectField(TEXT("InnerType"), InnerType->ToJson());

	return Json;
}

FRustReflection_Param FRustReflection_Param::FromProperty(FProperty* Property)
{
	FRustReflection_Param ParamReflection;
	ParamReflection.Name = Property->GetName();
	ParamReflection.Type = ::FromProperty(Property);

	ParamReflection.bIsOut = Property->HasAnyPropertyFlags(CPF_OutParm);
	ParamReflection.bIsReturn = Property->HasAnyPropertyFlags(CPF_ReturnParm);
	ParamReflection.bIsRef = Property->HasAnyPropertyFlags(CPF_ReferenceParm);

	auto DocText = Property->GetToolTipText();
	if (!DocText.IsEmpty())
	{
		ParamReflection.Documentation = Property->GetToolTipText();
	}

	return ParamReflection;
}

FRustReflection_Function FRustReflection_Function::FromFunction(UFunction* Function)
{
	FRustReflection_Function ReflectionFunction;

	ReflectionFunction.Name = Function->GetName();
	ReflectionFunction.FunctionFlags = GetFunctionFlagStrings(Function);
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

FRustReflection_UStruct FRustReflection_UStruct::FromStruct(UStruct* Struct)
{
	FRustReflection_UStruct ReflectionStruct;
	// ReflectionStruct.StructFlags = GetClassFlagStrings(->ClassFlags);
	ReflectionStruct.StructName = Struct->GetPrefixCPP() + Struct->GetName();

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

	ClassReflection.ClassFlags = GetClassFlagStrings(Class->ClassFlags);
	ClassReflection.ClassName = Class->GetPrefixCPP() + Class->GetName();

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

		FString Left;
		FString Right;

		if (EnumName.Split(TEXT("_"), &Left, &Right) && !Right.IsEmpty())
		{
			EnumName = *Right;
		}

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
	Json->SetArrayField(TEXT("Entries"), JsonEntries);
	return Json;
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

	Json->SetNumberField(TEXT("ParamSize"), ParamSize);

	TArray<TSharedPtr<FJsonValue>> JsonParams;
	for (auto& Param : Parameters)
	{
		JsonParams.Add(MakeShared<FJsonValueObject>(Param.ToJson()));
	}

	Json->SetArrayField(TEXT("Parameters"), JsonParams);

	TSharedPtr<FJsonObject> JsonMetadata = MakeShared<FJsonObject>();
	for (const auto& Elem : Metadata)
	{
		JsonMetadata->SetStringField(Elem.Key, Elem.Value);
	}

	Json->SetObjectField(TEXT("Metadata"), JsonMetadata);

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

	return Json;
}

TSharedPtr<FJsonObject> FRustReflection_Param::ToJson()
{
	auto Json = MakeShared<FJsonObject>();

	Json->SetStringField(TEXT("Name"), Name);
	Json->SetObjectField(TEXT("Type"), Type->ToJson());
	if (Documentation.IsSet())
	{
		Json->SetStringField(TEXT("Documentation"), Documentation.GetValue().ToString());
	}
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

	Json->SetArrayField(TEXT("Flags"), JsonFlags);

	if (SuperClassName.IsSet())
	{
		Json->SetStringField(TEXT("SuperClass"), SuperClassName.GetValue());
	}
	Json->SetStringField(TEXT("Package"), Package);

	TArray<TSharedPtr<FJsonValue>> JsonProperties;
	for (auto& Property : Properties)
	{
		JsonProperties.Add(MakeShared<FJsonValueObject>(Property.ToJson()));
	}

	Json->SetNumberField(TEXT("MinAlignment"), MinAlignment);
	Json->SetNumberField(TEXT("PropertySizes"), PropertySizes);
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

	Json->SetArrayField(TEXT("Flags"), JsonFlags);

	if (SuperClassName.IsSet())
	{
		Json->SetStringField(TEXT("SuperClass"), SuperClassName.GetValue());
	}
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

void FRustReflection_Root::ExportToJson_Enum(TSharedPtr<FJsonObject> Json)
{
	// This is really dumb. I don't have the size or type of the enum for a UEnum. So I need
	// to discover all the used enums which then allows me to get their size
	// I should maybe check that all found sizes are the same
	TSet<UEnum*> FoundEnums;
	TArray<FRustReflection_EnumWithType> EnumsWithType;
	for (TObjectIterator<UStruct> StructIter; StructIter; ++StructIter)
	{
		for (TFieldIterator<FProperty> FieldIter(*StructIter); FieldIter; ++FieldIter)
		{
			UEnum* Enum = nullptr;
			TUniquePtr<RustReflection_Type> Type;
			if (auto EnumProperty = CastField<FEnumProperty>(*FieldIter))
			{
				Enum = EnumProperty->GetEnum();
				Type = FromProperty(EnumProperty->GetUnderlyingProperty());
			}
			if (auto* InnerProperty = CastField<FByteProperty>(*FieldIter))
			{
				Enum = InnerProperty->Enum;
				Type = MakeConcreteType(TEXT("uint8"));
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

FRustReflection_Root FRustReflection_Root::Collect()
{
	FRustReflection_Root Root;
	auto Json = MakeShared<FJsonObject>();

	ExportToJson_Enum(Json);
	ExportToJson_ConsoleVariables(Json);
	ExportToJson_GameplayTags(Json);
	ExportToJson_Structs(Json);
	ExportToJson_Classes(Json);

	FString OutputString;

	TSharedRef<TJsonWriter<>> Writer = TJsonWriterFactory<>::Create(&OutputString);

	FJsonSerializer::Serialize(Json, Writer);

	FFileHelper::SaveStringToFile(OutputString, TEXT("C:\\Users\\maikk\\Documents\\unreal-rust\\api.json"),
	                              FFileHelper::EEncodingOptions::ForceUTF8);

	return Root;
}
