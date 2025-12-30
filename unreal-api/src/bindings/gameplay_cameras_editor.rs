#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
pub use crate::bindings::prelude::*;
#[repr(C, align(8))]
pub struct FCameraNodeGraphSchemaAction_NewInterfaceParameterNode {
    pub parameter_definition: FCameraObjectInterfaceParameterDefinition,
}
#[repr(C, align(8))]
pub struct FCameraNodeGraphSchemaAction_AddInterfaceParameterNode {
    pub interface_parameter: UPtr<UCameraObjectInterfaceParameterBase>,
}
#[repr(C, align(8))]
pub struct FObjectTreeGraphSchemaAction_NewNode {
    pub object_outer: UPtr<UObject>,
    pub object_class: TSubclassOf<UObject>,
}
#[repr(C, align(8))]
pub struct FObjectTreeGraphSchemaAction_NewComment {}
pub struct UGameplayCameraActorFactory {}
pub struct UGameplayCameraRigActorFactory {}
pub struct UAssetDefinition_CameraAsset {}
pub struct UAssetDefinition_CameraRigAsset {}
pub struct UAssetDefinition_CameraRigProxyAsset {}
pub struct UAssetDefinition_CameraShakeAsset {}
pub struct UAssetDefinition_CameraVariableCollection {}
pub struct UCameraAssetEditor {
    pub camera_asset: UPtr<UCameraAsset>,
}
pub struct UCameraRigAssetEditor {
    pub camera_rig_asset: UPtr<UCameraRigAsset>,
}
pub struct UCameraRigProxyAssetEditor {
    pub camera_rig_proxy_asset: UPtr<UCameraRigProxyAsset>,
}
pub struct UCameraRigTransitionEditor {
    pub transition_owner: UPtr<UObject>,
}
pub struct UCameraShakeAssetEditor {
    pub camera_shake_asset: UPtr<UCameraShakeAsset>,
}
pub struct UCameraVariableCollectionEditor {}
pub struct UGameplayCamerasDebuggerMenuContext {}
pub struct UObjectTreeGraphNode {
    pub weak_object: TWeakObjectPtr<UObject>,
    pub self_pin_direction_override: EEdGraphPinDirection,
    pub b_override_self_pin_direction: bool,
}
pub struct UCameraNodeGraphNode {}
pub struct UObjectTreeGraphSchema {}
pub struct UCameraNodeGraphSchema {}
pub struct UCameraObjectInterfaceParameterGraphNode {}
pub struct UCameraRigCameraNodeGraphSchema {}
pub struct UCameraRigTransitionGraphSchemaBase {}
pub struct UCameraRigTransitionGraphSchema {}
pub struct UCameraShakeCameraNodeGraphSchema {}
pub struct UCameraSharedTransitionGraphSchema {}
pub struct UObjectTreeGraph {
    pub root_object_node: UPtr<UObjectTreeGraphNode>,
}
pub struct UObjectTreeGraphCommentNode {
    pub weak_object: TWeakObjectPtr<UObjectTreeGraphComment>,
}
pub struct UCameraAssetFactory {
    pub camera_director_class: TSubclassOf<UCameraDirector>,
}
pub struct UCameraRigAssetFactory {}
pub struct UCameraRigProxyAssetFactory {}
pub struct UCameraShakeAssetFactory {}
pub struct UCameraVariableCollectionFactory {}
pub struct UGameplayCamerasEditorSettings {
    pub camera_node_title_color: FLinearColor,
    pub camera_asset_title_color: FLinearColor,
    pub camera_rig_asset_title_color: FLinearColor,
    pub camera_shake_asset_title_color: FLinearColor,
    pub camera_rig_transition_title_color: FLinearColor,
    pub camera_rig_transition_condition_title_color: FLinearColor,
    pub camera_blend_node_title_color: FLinearColor,
    pub last_camera_asset_toolkit_mode_name: FName,
    pub b_enable_run_in_editor: bool,
}
pub struct UCameraAssetEditorMenuContext {}
pub struct UEdGraphSchema_CameraNodeK2 {}
pub struct UCameraRigAssetEditorMenuContext {}
pub struct UCameraShakeAssetEditorMenuContext {}
pub struct UCameraVariableCollectionEditorMenuContext {}
