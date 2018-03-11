use specs::{Component, VecStorage};

use math::Vector2;

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
