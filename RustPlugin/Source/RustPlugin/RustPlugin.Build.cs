// Copyright Epic Games, Inc. All Rights Reserved.

using UnrealBuildTool;

public class RustPlugin : ModuleRules
{
	public RustPlugin(ReadOnlyTargetRules Target) : base(Target)
	{
		PCHUsage = ModuleRules.PCHUsageMode.UseExplicitOrSharedPCHs;
		
		PublicIncludePaths.AddRange(
			new string[] {
			}
		);
				
		
		PrivateIncludePaths.AddRange(
			new string[] {
				// ... add other private include paths required here ...
			}
		);
			
		
		PublicDependencyModuleNames.AddRange(
			new string[]
			{
				"Core", "GraphEditor",
				"MassEntity",
				"MassCommon",
				"MassMovement",
				"MassRepresentation",
				"MassLOD",
				"MassSpawner",
				"MassSimulation",
				"MassActors",
				"MassNavigation",
				"AIModule",
				"StructUtils",
			}
			);
			
		
		PrivateDependencyModuleNames.AddRange(
			new string[]
			{
				"Projects",
				"InputCore",
				"EditorFramework",
				"UnrealEd",
				"ToolMenus",
				"CoreUObject",
				"Engine",
				"Slate",
				"SlateCore",
				"BlueprintGraph",
				"GraphEditor",
				"KismetWidgets",
				"KismetCompiler",
				"PropertyEditor",
				"EditorWidgets",
				"ClassViewer",
				"EditorStyle",
				"Json",
				"EditorSubsystem"
			}
			);
		
		
		DynamicallyLoadedModuleNames.AddRange(
			new string[]
			{
				// ... add any modules that your module loads dynamically here ...
			}
			);
	}
}
