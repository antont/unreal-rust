#include "RustUtils.h"
#include "Modules/ModuleManager.h"
#include "RustPlugin.h"

UnrealBindings CreateBindings()
{
	CoreFns core_fns = {};
	core_fns.get_all_uclasses = &GetAllUClasses;
	core_fns.get_class_name = &GetClassName;
	core_fns.get_cdo_from_class = &GetCDOFromClass;
	core_fns.find_function_by_name = &FindFunctionByName;
	core_fns.initialize_values_in_param_buffer = &InitializeValuesInParamBuffer;
	core_fns.destroy_values_in_param_buffer = &DestroyValuesInParamBuffer;
	core_fns.process_event = &ProcessEventFromRust;
	core_fns.end_trace = &EndTrace;
	core_fns.begin_trace = &BeginTrace;

	UnrealBindings b = {};
	b.core_fns = core_fns;
	b.log = &Log;
	return b;
}

FRustPluginModule& GetRustModule()
{
	return FModuleManager::LoadModuleChecked<FRustPluginModule>(TEXT("RustPlugin"));
}

FString ToFString(Utf8Str Str)
{
	if (Str.len == 0)
	{
		return FString();
	}

	return FString(Str.len, StringCast<TCHAR>(reinterpret_cast<const UTF8CHAR*>(Str.ptr), Str.len).Get());
}
