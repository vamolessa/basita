use math::Vector2;

use super::Component;

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PhysicBody {
	pub velocity: Vector2,
	pub acceleration: Vector2,

	pub inverted_mass: f32,
	pub bounciness: f32,
}

impl Component for PhysicBody {}
