#pragma once

#include "Modules/ModuleInterface.h"

class FRustMassGatherersModule : public IModuleInterface
{
public:
	virtual void StartupModule() override {}
	virtual void ShutdownModule() override {}
};
