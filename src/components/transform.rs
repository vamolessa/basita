use components::Component;
use math::Vector2;

pub struct Transform {
	pub position: Vector2,
}

impl Transform {}

impl Component for Transform {}

impl Default for Transform {
	fn default() -> Transform {
		Transform {
			position: Vector2::default(),
		}
	}
}
