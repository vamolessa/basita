use math::Vector2;

use super::Component;

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

impl Component for Transform {}
