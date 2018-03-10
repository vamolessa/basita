use std::any::TypeId;

use mopa;

pub trait Resource: mopa::Any {}

mopafy!(Resource);

pub fn get_resource_id<T: Resource>() -> TypeId {
	TypeId::of::<T>()
}
