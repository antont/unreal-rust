use std::ffi::c_void;

use bevy_ecs::{entity::Entity, prelude::World};
use glam::{Quat, Vec3};
use serde::de::Visitor;
use unreal_ffi as ffi;

#[derive(Copy, Clone, Debug)]
pub struct UClass {
    pub ptr: *mut ffi::UClassOpague,
}
unsafe impl Send for UClass {}
unsafe impl Sync for UClass {}

impl<'de> serde::Deserialize<'de> for UClass {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let ptr = deserializer.deserialize_str(StrVisitor)?;
        Ok(Self { ptr })
    }
}
struct StrVisitor;

impl<'de> Visitor<'de> for StrVisitor {
    type Value = *mut c_void;

    fn expecting(&self, _formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        todo!()
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let addr = str::parse::<usize>(v).unwrap();
        Ok(addr as *mut c_void)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct USound {
    pub ptr: *mut ffi::UObjectOpague,
}
unsafe impl Send for USound {}
unsafe impl Sync for USound {}

// TODO: Merge usound/uclass impl
impl<'de> serde::Deserialize<'de> for USound {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let ptr = deserializer.deserialize_str(StrVisitor)?;
        Ok(Self { ptr })
    }
}

pub enum ReflectValue {
    Float(f32),
    Vector3(Vec3),
    Bool(bool),
    Quat(Quat),
    UClass(UClass),
    USound(USound),
    Composite,
}

pub enum ReflectType {
    Float,
    Vector3,
    Bool,
    Quat,
    UClass,
    USound,
    Composite,
}

pub trait ReflectDyn: Send + Sync {
    fn name(&self) -> &'static str;
    fn number_of_fields(&self) -> u32 {
        0
    }
    fn get_field_name(&self, _idx: u32) -> Option<&'static str> {
        None
    }
    fn get_field_type(&self, _idx: u32) -> Option<ReflectType> {
        None
    }
    fn has_component(&self, _world: &World, _entity: Entity) -> bool {
        false
    }
    fn get_field_value(&self, _world: &World, _entity: Entity, _idx: u32) -> Option<ReflectValue> {
        None
    }
    fn get_value(&self) -> ReflectValue;
}

impl ReflectDyn for UClass {
    fn name(&self) -> &'static str {
        "UClass"
    }

    fn get_value(&self) -> ReflectValue {
        ReflectValue::UClass(*self)
    }
}
impl ReflectStatic for UClass {
    const TYPE: ReflectType = ReflectType::UClass;
}
impl ReflectDyn for USound {
    fn name(&self) -> &'static str {
        "USound"
    }

    fn get_value(&self) -> ReflectValue {
        ReflectValue::USound(*self)
    }
}

impl ReflectStatic for USound {
    const TYPE: ReflectType = ReflectType::USound;
}

impl ReflectDyn for Vec3 {
    fn name(&self) -> &'static str {
        "Vec3"
    }

    fn get_value(&self) -> ReflectValue {
        ReflectValue::Vector3(*self)
    }
}

impl ReflectStatic for Vec3 {
    const TYPE: ReflectType = ReflectType::Vector3;
}

impl ReflectDyn for Quat {
    fn name(&self) -> &'static str {
        "Quat"
    }

    fn get_value(&self) -> ReflectValue {
        ReflectValue::Quat(*self)
    }
}
impl ReflectStatic for Quat {
    const TYPE: ReflectType = ReflectType::Quat;
}

impl ReflectDyn for f32 {
    fn name(&self) -> &'static str {
        "f32"
    }

    fn get_value(&self) -> ReflectValue {
        ReflectValue::Float(*self)
    }
}

impl ReflectStatic for f32 {
    const TYPE: ReflectType = ReflectType::Float;
}
impl ReflectDyn for bool {
    fn name(&self) -> &'static str {
        "bool"
    }

    fn get_value(&self) -> ReflectValue {
        ReflectValue::Bool(*self)
    }
}

impl ReflectStatic for bool {
    const TYPE: ReflectType = ReflectType::Bool;
}

pub trait ReflectStatic {
    const TYPE: ReflectType;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3_reflect_name() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.name(), "Vec3");
    }

    #[test]
    fn vec3_reflect_value() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        match v.get_value() {
            ReflectValue::Vector3(val) => {
                assert_eq!(val.x, 1.0);
                assert_eq!(val.y, 2.0);
                assert_eq!(val.z, 3.0);
            }
            _ => panic!("expected Vector3"),
        }
    }

    #[test]
    fn quat_reflect() {
        let q = Quat::from_xyzw(0.0, 0.0, 0.0, 1.0);
        assert_eq!(q.name(), "Quat");
        assert!(matches!(q.get_value(), ReflectValue::Quat(_)));
    }

    #[test]
    fn f32_reflect() {
        let f: f32 = 42.0;
        assert_eq!(f.name(), "f32");
        match f.get_value() {
            ReflectValue::Float(val) => assert_eq!(val, 42.0),
            _ => panic!("expected Float"),
        }
    }

    #[test]
    fn bool_reflect() {
        assert_eq!(true.name(), "bool");
        match true.get_value() {
            ReflectValue::Bool(val) => assert!(val),
            _ => panic!("expected Bool"),
        }
    }

    #[test]
    fn uclass_reflect() {
        let c = UClass { ptr: std::ptr::null_mut() };
        assert_eq!(c.name(), "UClass");
        assert!(matches!(c.get_value(), ReflectValue::UClass(_)));
    }

    #[test]
    fn usound_reflect() {
        let s = USound { ptr: std::ptr::null_mut() };
        assert_eq!(s.name(), "USound");
        assert!(matches!(s.get_value(), ReflectValue::USound(_)));
    }

    #[test]
    fn reflect_static_types() {
        assert!(matches!(f32::TYPE, ReflectType::Float));
        assert!(matches!(bool::TYPE, ReflectType::Bool));
        assert!(matches!(Vec3::TYPE, ReflectType::Vector3));
        assert!(matches!(Quat::TYPE, ReflectType::Quat));
        assert!(matches!(UClass::TYPE, ReflectType::UClass));
        assert!(matches!(USound::TYPE, ReflectType::USound));
    }

    #[test]
    fn reflect_dyn_defaults() {
        let v = Vec3::ZERO;
        assert_eq!(v.number_of_fields(), 0);
        assert!(v.get_field_name(0).is_none());
        assert!(v.get_field_type(0).is_none());
    }
}
