// Force-include the generated vivarium fragment header so UHT picks up the
// USTRUCTs. UHT scans .h files referenced from .cpp files in the module; the
// generated header on its own wouldn't be parsed without a translation unit
// pulling it in. (Mirror of how RustMassPostMovementProcessor.cpp pulls in
// GatherersFragments.gen.h for the gatherers sim.)
#include "Generated/VivariumFragments.gen.h"
