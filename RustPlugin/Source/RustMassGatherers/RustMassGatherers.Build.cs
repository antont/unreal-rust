using UnrealBuildTool;
using System.IO;

public class RustMassGatherers : ModuleRules
{
	public RustMassGatherers(ReadOnlyTargetRules Target) : base(Target)
	{
		PCHUsage = PCHUsageMode.UseExplicitOrSharedPCHs;

		PrivateIncludePaths.Add(Path.Combine(ModuleDirectory, "..", "RustPlugin"));

		PublicDependencyModuleNames.AddRange(new string[]
		{
			"Core",
			"CoreUObject",
			"Engine",
			"MassEntity",
			"MassCommon",
			"StructUtils",
			"RustPlugin",
		});
	}
}
