use serde_derive::{Deserialize, Serialize};
use specs::{Component, VecStorage};

use crate::math::Vector2;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Transform {
	pub position: Vector2,
}

impl Transform {
	pub fn identity() -> Self {
		Transform {
			position: Vector2::zero(),
		}
	}

	pub fn new(position: Vector2) -> Self {
		Transform { position: position }
	}
}

impl Default for Transform {
	fn default() -> Self {
		Transform::identity()
	}
}

impl Component for Transform {
	type Storage = VecStorage<Self>;
}
