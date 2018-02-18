use math::Vector2;

use super::{Component, ComponentHandle, Transform};

#[derive(Clone, Copy, Debug, Default)]
pub struct BoxCollider {
	pub half_size: Vector2,
	pub offset: Vector2,

	pub transform: ComponentHandle<Transform>,
}

impl Component for BoxCollider {}
