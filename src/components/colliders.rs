use math::Vector2;

use super::Component;

pub struct BoxCollider {
	size: Vector2,
	offset: Vector2,
}

impl Component for BoxCollider {}
