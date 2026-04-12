#pragma once

#include "Modules/ModuleInterface.h"

class FRustMassSpikeModule : public IModuleInterface
{
public:
	virtual void StartupModule() override {}
	virtual void ShutdownModule() override {}
};
