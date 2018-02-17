use math::Vector2;

use super::{Component, ComponentHandle, Transform};

pub struct BoxCollider {
	pub size: Vector2,
	pub offset: Vector2,

	pub transform: ComponentHandle<Transform>,
}

impl Component for BoxCollider {}
