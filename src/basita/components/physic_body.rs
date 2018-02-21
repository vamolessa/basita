use math::Vector2;

use super::{Collider, Component, ComponentHandle, Transform};

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct PhysicBody {
	pub velocity: Vector2,
	pub acceleration: Vector2,

	pub inverted_mass: f32,
	pub bounciness: f32,

	pub transform: ComponentHandle<Transform>,
	pub collider: ComponentHandle<Collider>,
}

impl Component for PhysicBody {}
