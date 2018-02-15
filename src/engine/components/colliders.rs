use math::Vector2;

use super::Component;

pub struct BoxCollider {
	pub size: Vector2,
	pub offset: Vector2,
}

impl Component for BoxCollider {}
