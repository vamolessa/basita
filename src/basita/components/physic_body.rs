use math::Vector2;

use super::{Component, ComponentHandle, Transform};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PhysicBody {
	pub velocity: Vector2,
	pub acceleration: Vector2,

	pub inverted_mass: f32,
	pub bounciness: f32,

	pub transform: ComponentHandle<Transform>,
}

impl Component for PhysicBody {}
