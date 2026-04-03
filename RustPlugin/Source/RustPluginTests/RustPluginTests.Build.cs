using UnrealBuildTool;

public class RustPluginTests : ModuleRules
{
	public RustPluginTests(ReadOnlyTargetRules Target) : base(Target)
	{
		PCHUsage = PCHUsageMode.UseExplicitOrSharedPCHs;

		PublicDependencyModuleNames.AddRange(new string[]
		{
			"Core",
			"CoreUObject",
			"Engine",
			"UnrealEd",
			"RustPlugin",
		});
	}
}
