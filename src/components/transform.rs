use components::Component;
use math::Vector2;

pub struct Transform {
	pub position: Vector2,
}

impl Transform {
	pub fn identity() -> Transform {
		Transform {
			position: Vector2::zero(),
		}
	}
}

impl Component for Transform {}
