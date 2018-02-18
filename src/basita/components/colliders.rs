use math::Vector2;

use super::{Component, ComponentHandle, PhysicBody, Transform};

#[derive(Clone, Copy, Debug, Default)]
pub struct Collider {
	pub shape: Shape,
	pub offset: Vector2,
	pub enabled: bool,
	pub is_trigger: bool,

	pub transform: ComponentHandle<Transform>,
	pub physic_body: Option<ComponentHandle<PhysicBody>>,
}

impl Component for Collider {}

#[derive(Clone, Copy, Debug)]
pub enum Shape {
	Box(BoxShape),
}

impl Default for Shape {
	fn default() -> Self {
		Shape::Box(BoxShape::default())
	}
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BoxShape {
	pub half_size: Vector2,
}
