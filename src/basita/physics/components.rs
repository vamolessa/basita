use specs::{Component, VecStorage};

use math::Vector2;

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PhysicBody {
	pub velocity: Vector2,
	pub acceleration: Vector2,

	pub inverted_mass: f32,
	pub bounciness: f32,
}

impl Component for PhysicBody {
	type Storage = VecStorage<Self>;
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Collider {
	pub shape: Shape,
	pub offset: Vector2,
	pub is_trigger: bool,
}

impl Component for Collider {
	type Storage = VecStorage<Self>;
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Shape {
	Box(BoxShape),
}

impl Default for Shape {
	fn default() -> Self {
		Shape::Box(BoxShape::default())
	}
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct BoxShape {
	pub half_size: Vector2,
}
