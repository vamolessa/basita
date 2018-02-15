use math::Vector2;

use super::{Component, Transform};

pub struct BoxCollider<'a> {
	pub size: Vector2,
	pub offset: Vector2,
	pub transform: &'a Transform,
}

impl<'a> Component for BoxCollider<'a> {}
