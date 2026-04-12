#pragma once

#include "Modules/ModuleInterface.h"

class FRustPluginTestsModule : public IModuleInterface
{
public:
	virtual void StartupModule() override {}
	virtual void ShutdownModule() override {}
};
