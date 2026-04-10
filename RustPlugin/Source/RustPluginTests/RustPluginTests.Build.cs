using UnrealBuildTool;
using System.IO;

public class RustPluginTests : ModuleRules
{
	public RustPluginTests(ReadOnlyTargetRules Target) : base(Target)
	{
		PCHUsage = PCHUsageMode.UseExplicitOrSharedPCHs;

		// RustPlugin uses a flat source layout (no Public/Private split),
		// so we need to add its source directory explicitly.
		PrivateIncludePaths.Add(Path.Combine(ModuleDirectory, "..", "RustPlugin"));

		PrivateIncludePaths.Add(Path.Combine(ModuleDirectory, "..", "RustMassSpike"));
		PrivateIncludePaths.Add(Path.Combine(ModuleDirectory, "..", "RustPlugin", "Generated"));

		PublicDependencyModuleNames.AddRange(new string[]
		{
			"Core",
			"CoreUObject",
			"Engine",
			"UnrealEd",
			"RustPlugin",
			"MassEntity",
			"RustMassSpike",
		});
	}
}
