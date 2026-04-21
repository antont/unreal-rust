// Copyright Epic Games, Inc. All Rights Reserved.

using UnrealBuildTool;

public class RustPlugin : ModuleRules
{
	public RustPlugin(ReadOnlyTargetRules Target) : base(Target)
	{
		PCHUsage = ModuleRules.PCHUsageMode.UseExplicitOrSharedPCHs;

		// Diagnostic: validate FTransformFragment / RepFrag / ISMShared rotations
		// every frame inside URustMassBevySubsystem::Tick(), before the
		// PostPhysics broadcast. Logs an error with first failing entity/buffer
		// instead of crashing in UInstancedStaticMeshComponent::UpdateInstanceTransform.
		// Default off — opt in locally by flipping to 1 when investigating a
		// suspected rotation corruption issue. The full-Tick scan walks every
		// entity in every group + up to 32 ISM shared-data descriptors × both
		// Current/Prev buffers, which is measurable (~2% of tick at 1k ants,
		// scales linearly) and pollutes perf measurements on this branch.
		PublicDefinitions.Add("RUST_MASS_VALIDATE_ROTATIONS=0");

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
