use math::Vector2;

use super::{BoxCollider, Component, ComponentHandle, Transform};

#[derive(Clone, Copy, Debug, Default)]
pub struct PhysicBody {
	pub velocity: Vector2,
	pub acceleration: Vector2,

	pub inverted_mass: f32,
	pub bounciness: f32,

	pub transform: ComponentHandle<Transform>,
	pub box_collider: ComponentHandle<BoxCollider>,
}

impl Component for PhysicBody {}
