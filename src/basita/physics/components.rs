use serde_derive::{Deserialize, Serialize};
use specs::{Component, VecStorage};

use crate::math::Vector2;

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PhysicBody {
	pub velocity: Vector2,
	pub acceleration: Vector2,
	pub inverted_mass: f32,
}

impl PhysicBody {
	pub fn with_inverted_mass(inverted_mass: f32) -> Self {
		PhysicBody {
			velocity: Vector2::zero(),
			acceleration: Vector2::zero(),
			inverted_mass: inverted_mass,
		}
	}
}

impl Component for PhysicBody {
	type Storage = VecStorage<Self>;
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Collider {
	pub shape: Shape,
	pub offset: Vector2,
	pub bounciness: f32,
}

impl Collider {
	pub fn new_box(half_size: Vector2) -> Self {
		Collider {
			shape: Shape::Box(BoxShape {
				half_size: half_size,
			}),
			offset: Vector2::zero(),
			bounciness: 0.0,
		}
	}
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
