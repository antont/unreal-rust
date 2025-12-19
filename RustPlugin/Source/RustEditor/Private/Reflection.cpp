// Fill out your copyright notice in the Description page of Project Settings.


#include "Reflection.h"

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

FRustReflection_Type FRustReflection_Type::FromProperty(FProperty* Property)
{
	if (FArrayProperty* ArrayProperty = CastField<FArrayProperty>(Property))
	{
		FRustReflection_Type Type;
		Type.ContainerType = TEXT("TArray");
		Type.Type = ArrayProperty->Inner->GetNameCPP();
		return Type;
	}

	if (auto* ClassProperty = CastField<FClassProperty>(Property))
	{
		FRustReflection_Type Type;
		Type.Type = ClassProperty->MetaClass->GetPrefixCPP() + ClassProperty->MetaClass->GetName();
		Type.ContainerType = TEXT("TSubclassOf");
		return Type;
	}

	if (auto* ObjectProperty = CastField<FObjectProperty>(Property))
	{
		FRustReflection_Type Type;
		Type.Type = ObjectProperty->PropertyClass->GetPrefixCPP() + ObjectProperty->PropertyClass.GetName();
		return Type;
	}


	if (auto* InnerProperty = CastField<FIntProperty>(Property))
	{
		FRustReflection_Type Type;
		Type.Type = TEXT("int");
		return Type;
	}

	if (auto* InnerProperty = CastField<FBoolProperty>(Property))
	{
		FRustReflection_Type Type;
		Type.Type = TEXT("bool");
		return Type;
	}

	if (auto* InnerProperty = CastField<FStrProperty>(Property))
	{
		FRustReflection_Type Type;
		Type.Type = TEXT("FString");
		return Type;
	}

	if (auto* InnerProperty = CastField<FFloatProperty>(Property))
	{
		FRustReflection_Type Type;
		Type.Type = TEXT("float");
		return Type;
	}

	if (auto* InnerProperty = CastField<FStructProperty>(Property))
	{
		FRustReflection_Type Type;
		Type.Type = InnerProperty->Struct->GetPrefixCPP() + InnerProperty->Struct.GetName();
		return Type;
	}
	if (auto* InnerProperty = CastField<FFieldPathProperty>(Property))
	{
		FRustReflection_Type Type;
		// TODO: does everything need an F?
		Type.Type = FString::Printf(TEXT("F%s"), *InnerProperty->PropertyClass->GetName());
		return Type;
	}

	if (auto* InnerProperty = CastField<FSetProperty>(Property))
	{
		FRustReflection_Type Type;
		Type.Type = InnerProperty->ElementProp->GetNameCPP();
		Type.ContainerType = TEXT("TSet");

		return Type;
	}

	if (auto* InnerProperty = CastField<FMapProperty>(Property))
	{
		FRustReflection_Type Type;
		Type.Type = InnerProperty->ValueProp->GetNameCPP();
		Type.KeyType = InnerProperty->KeyProp->GetNameCPP();
		Type.ContainerType = TEXT("TMap");

		return Type;
	}

	if (auto* InnerProperty = CastField<FEnumProperty>(Property))
	{
		FRustReflection_Type Type;
		Type.Type = InnerProperty->GetNameCPP();

		return Type;
	}

	if (auto* InnerProperty = CastField<FOptionalProperty>(Property))
	{
		FRustReflection_Type Type;
		Type.Type = InnerProperty->GetValueProperty()->GetCPPType();
		Type.ContainerType = TEXT("TOptional");

		return Type;
	}

	if (auto* InnerProperty = CastField<FNameProperty>(Property))
	{
		FRustReflection_Type Type;
		Type.Type = TEXT("FName");

		return Type;
	}

	if (auto* InnerProperty = CastField<FTextProperty>(Property))
	{
		FRustReflection_Type Type;
		Type.Type = TEXT("FText");

		return Type;
	}

	if (auto* InnerProperty = CastField<FInterfaceProperty>(Property))
	{
		FRustReflection_Type Type;
		Type.Type = FString::Printf(TEXT("I%s"), *InnerProperty->InterfaceClass.GetName());

		return Type;
	}

	if (auto* InnerProperty = CastField<FByteProperty>(Property))
	{
		if (InnerProperty->Enum != nullptr)
		{
			FRustReflection_Type Type;
			Type.Type = InnerProperty->Enum.GetName();
			Type.ContainerType = TEXT("TEnumAsByte");
			return Type;
		}
		else
		{
			FRustReflection_Type Type;
			Type.Type = TEXT("uint8");

			return Type;
		}
	}

	// TODO
	FRustReflection_Type Type;
	Type.Type = TEXT("GeneratorUnknown");

	return Type;
}

FRustReflection_Property FRustReflection_Property::FromProperty(FProperty* Property)
{
	FRustReflection_Property PropertyReflection;
	PropertyReflection.Name = Property->GetFName();
	PropertyReflection.Type = FRustReflection_Type::FromProperty(Property);
	PropertyReflection.PropertyFlags = GetPropertyFlagStrings(Property->GetPropertyFlags());

	auto DocText = Property->GetToolTipText();
	if (!DocText.IsEmpty())
	{
		PropertyReflection.Documentation = Property->GetToolTipText();
	}

	return PropertyReflection;
}

FRustReflection_Param FRustReflection_Param::FromProperty(FProperty* Property)
{
	FRustReflection_Param ParamReflection;
	ParamReflection.Name = Property->GetFName();
	ParamReflection.Type = FRustReflection_Type::FromProperty(Property);

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

	ReflectionFunction.Name = Function->GetFName();
	ReflectionFunction.FunctionFlags = GetFunctionFlagStrings(Function);
	ReflectionFunction.Metadata = GetAllMetadata(Function);

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

FRustReflection_UClass FRustReflection_UClass::FromClass(UClass* Class)
{
	FRustReflection_UClass ClassReflection;

	ClassReflection.ClassFlags = GetClassFlagStrings(Class->ClassFlags);
	ClassReflection.ClassName = Class->GetPrefixCPP() + Class->GetName();
	if (auto SuperClass = Class->GetSuperClass())
	{
		ClassReflection.SuperClassName = SuperClass->GetPrefixCPP() + SuperClass->GetName();
	}

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

FRustReflection_Enum FRustReflection_Enum::FromEnum(UEnum* Enum)
{
	FRustReflection_Enum ReflectionEnum;

	ReflectionEnum.Name = Enum->GetName();

	for (int32 i = 0; i < Enum->NumEnums(); ++i)
	{
		FRustReflection_EnumEntry Entry;
		FString EnumName = Enum->GetNameStringByIndex(i);

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

		Entry.Value = Enum->GetValueByIndex(i);

		FText Documentation = Enum->GetToolTipTextByIndex(i);
		if (!Documentation.IsEmpty())
		{
			Entry.Documentation = Documentation;
		}

		ReflectionEnum.Entries.Add(Entry);
	}

	return ReflectionEnum;
}

FRustReflection_Root FRustReflection_Root::Collect()
{
	FRustReflection_Root Root;

	for (TObjectIterator<UEnum> It; It; ++It)
	{
		Root.Enums.Add(FRustReflection_Enum::FromEnum(*It));
	}

	for (TObjectIterator<UClass> It; It; ++It)
	{
		Root.Classes.Add(FRustReflection_UClass::FromClass(*It));
	}

	return Root;
}
