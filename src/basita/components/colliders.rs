use math::Vector2;

use super::{Component, ComponentHandle, PhysicBody, Transform};

#[derive(Clone, Copy, Debug, Default)]
pub struct BoxCollider {
	pub half_size: Vector2,
	pub offset: Vector2,

	pub transform: ComponentHandle<Transform>,
	pub physic_body: Option<ComponentHandle<PhysicBody>>,
}

impl Component for BoxCollider {}
