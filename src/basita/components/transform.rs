use math::Vector2;

use super::Component;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Transform {
	pub position: Vector2,
}

impl Transform {
	pub fn identity() -> Self {
		Transform {
			position: Vector2::default(),
		}
	}
}

impl Component for Transform {}

impl Default for Transform {
	fn default() -> Transform {
		Transform::identity()
	}
}
