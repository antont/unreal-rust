using UnrealBuildTool;
using System.IO;

public class RustMassSpike : ModuleRules
{
	public RustMassSpike(ReadOnlyTargetRules Target) : base(Target)
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
			"RustPlugin",
		});
	}
}
