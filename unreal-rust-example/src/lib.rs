use bevy_app::App;
use bevy_ecs::prelude::*;
use unreal_module::UserModule;

// fn begin(query: Query<(Entity, &ActorComponent)>) {
//     // let mut alloc = ffi::RustAlloc::empty();
//     // unsafe {
//     //     (bindings().core_fns.get_all_uclasses)(&mut alloc);
//     // }
//     //
//     // let classes = unsafe {
//     //     std::slice::from_raw_parts(
//     //         alloc.ptr as *mut *mut UClassOpague,
//     //         alloc.size / std::mem::size_of::<*mut UClassOpague>(),
//     //     )
//     // };
//     // let ptrs = unreal_api::bindings::globals::CLASS_PTRS.wait();
//     // log::warn!("-----------------------------------------------------");
//     // for name in ptrs.name_to_ptr.keys() {
//     //     log::warn!("Class: {}", name);
//     // }
//
//     // let class = ARustActor::static_class();
//     //
//     // let mut hit_result = FHitResult::new();
//     //
//     // let mut str_alloc = StrRustAlloc::empty();
//     // unsafe {
//     //     (bindings().core_fns.get_class_name)(class as *const UClassOpague, &mut str_alloc);
//     // }
//     // log::info!("{}", str_alloc.into_string());
//     // for (_entity, actor) in query.iter() {
//     //     let mut actor = unsafe { actor.actor.0.cast::<AActor>().as_mut().unwrap() };
//     //
//     //     let v = actor.get_actor_location();
//     //
//     //     actor.set_actor_location(FVector { x: 0.0, y: 0.0, z: 0.0 }, false, &mut hit_result, true);
//     //     log::warn!("{} {} {}", v.x, v.y, v.z);
//     // }
//     // log::warn!("Class: {}", ptrs.name_to_ptr.keys().count());
// }
// fn update(query: Query<(Entity, &ActorComponent)>) {
//     // let mut hit_result = FHitResult::new();
//     // for (_entity, actor) in query.iter() {
//     //     let mut actor = unsafe { actor.actor.0.cast::<AActor>().as_mut().unwrap() };
//     //     let v = actor.get_actor_location();
//     //
//     //     actor.set_actor_location(
//     //         FVector {
//     //             x: 0.0,
//     //             y: 0.0,
//     //             z: 0.0,
//     //         },
//     //         true,
//     //         &mut hit_result,
//     //         false,
//     //     );
//     //     log::warn!("{} {} {}", v.x, v.y, v.z);
//     // }
//
//     // for &class in classes {
//     //     let mut str_alloc = StrRustAlloc::empty();
//     //     unsafe {
//     //         (bindings().core_fns.get_class_name)(class as *const UClassOpague, &mut str_alloc);
//     //     }
//     //
//     //     log::info!("{}", str_alloc.into_string());
//     // }
//
//     // unsafe {
//     //     alloc.free();
//     // }
//
//     // log::warn!("-----------------");
//     // UVolumetricCloudComponent::verify_layout();
//
//     // log::warn!(
//     //     "{} {} {} {}",
//     //     offset_of!(FTransform, rotation),
//     //     offset_of!(FTransform, translation),
//     //     offset_of!(FTransform, scale3_d),
//     //     size_of::<FTransform>()
//     // );
//
//     // let class = ARustActor::static_class();
//     // let mut str_alloc = StrRustAlloc::empty();
//     // unsafe {
//     //     (bindings().core_fns.get_class_name)(class as *const UClassOpague, &mut str_alloc);
//     // }
//
//     // log::info!("{}", str_alloc.into_string());
//     // for (_entity, actor) in query.iter() {
//     //     let actor = unsafe { actor.actor.0.cast::<AActor>().as_ref().unwrap() };
//     //
//     //     let v = actor.get_actor_location();
//     //     log::warn!("{} {} {}", v.x, v.y, v.z);
//     // }
//
//     // for (_entity, actor) in query.iter() {
//     //     let mut stack_alloc = StackAlloc::<24>::new();
//     //     let actor_ptr = unsafe { actor.actor.0 };
//     //     let actor = unsafe { actor.actor.0.cast::<AActor>().as_mut().unwrap() };
//     //     // let actor_class = unsafe { (bindings().actor_fns.get_class1)(&) };
//     //     let mut class_name = StrRustAlloc::empty();
//     //     unsafe { (bindings().core_fns.get_class_name)(actor_class, &mut class_name) };
//     //     // log::warn!("{}", class_name.into_string());
//     //     // let actor = unsafe { actor.actor.0.cast::<AActor>().as_ref().unwrap() };
//     //
//     //     // log::warn!("{} {} {}", actor.pivot_offset.x, actor.pivot_offset.y, actor.pivot_offset.z);
//     //     // log::warn!("{} {}", actor.net_update_frequency, actor.net_priority);
//     //     // let aactor_ptr = unsafe { actor.actor.0.cast::<Actor>().as_ref().unwrap() };
//     //     // log::info!(
//     //     //     "{} and {} {}",
//     //     //     std::mem::offset_of!(Actor, net_update_frequency),
//     //     //     std::mem::offset_of!(Actor, net_update_frequency2),
//     //     //     aactor_ptr.net_update_frequency.read()
//     //     // );
//     //     let fn_name = Utf8Str::from("K2_GetActorLocation");
//     //
//     //     let mut fn_ptr: *mut UFunctionOpague = std::ptr::null_mut();
//     //
//     //     // #[repr(C)]
//     //     // struct Params
//     //     // {
//     //     //     out: FVector
//     //     // }
//     //
//     //     unsafe {
//     //         //     let mut params = Params{
//     //         //         out: FVector {x: 0.0, y: 0.0, z: 0.0}
//     //         //     };
//     //         //
//     //         (bindings().core_fns.begin_trace)(c"MyGetActorLocation".as_ptr());
//     //         (bindings().core_fns.find_function_by_name)(actor_class, fn_name, &raw mut fn_ptr);
//     //         // (bindings().core_fns.initialize_values_in_param_buffer)(
//     //         //     fn_ptr,
//     //         //     stack_alloc.buffer_mut(),
//     //         // );
//     //         // (bindings().core_fns.process_event)(actor_ptr, fn_ptr, &mut params as *mut _ as *mut c_void);
//     //         (bindings().core_fns.process_event)(actor_ptr, fn_ptr, stack_alloc.buffer_mut());
//     //         let data = stack_alloc.buffer_mut().cast::<FVector>().read();
//     //         (bindings().core_fns.destroy_values_in_param_buffer)(fn_ptr, stack_alloc.buffer_mut());
//     //         //
//     //         //
//     //         // log::warn!("{:?}", params.out);
//     //
//     //         (bindings().core_fns.end_trace)();
//     //     }
//     // }
// }

pub struct UnrealEcs {
    app: App,
}

impl UnrealEcs {
    pub fn new() -> Self {
        let app = App::new();
        Self { app }
    }
}

impl Default for UnrealEcs {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Resource)]
pub struct Frame {
    pub dt: f32,
}

impl UserModule for UnrealEcs {
    fn initialize(&mut self) {

        self.app.world_mut().register_resource::<Frame>();
    }

    fn tick(&mut self, dt: f32) {
        log::warn!("Hello Foo");
        // self.app
        //     .world_mut()
        //     .resource_scope::<Frame, _>(|_, mut frame| {
        //         frame.dt += dt;
        //     })
    }

    fn begin_play(&mut self) {}
}
//
// impl InitUserModule for MyModule {
//     fn initialize() -> Self {
//         Self {}
//     }
// }
//
// impl UserModule for MyModule {
//     fn initialize(&self, module: &mut Module) {
//         // register_editor_components! {
//         //     CharacterSoundsComponent,
//         //     PlaySoundOnImpactComponent,
//         //     => module
//         // }
//         //
//         // register_components! {
//         //     CameraComponent,
//         //     => module
//         // };
//
//         module
//             .app
//             .add_systems(
//                 unreal_api::ecs::Startup,
//                 (
//                     // register_class_resource,
//                     // register_player_input,
//                     // register_hit_events,
//                     begin,
//                 )
//                     .chain(),
//             )
//             .add_systems(
//                 unreal_api::ecs::Update,
//                 (
//                     // spawn_class,
//                     // spawn_camera,
//                     // update_controller_view,
//                     // rotate_camera,
//                     // move_camera_up,
//                     // // update_camera,
//                     // toggle_camera,
//                     // play_sound_on_hit,
//                     update,
//                 )
//                     .chain(),
//             );
//     }
// }

unreal_module::implement_unreal_module!(UnrealEcs::new());
