pub mod collision {
	use components::BoxCollider;
	use math::Vector2;

	pub fn collide(
		a: &BoxCollider,
		a_pos: &Vector2,
		b: &BoxCollider,
		b_pos: &Vector2,
	) -> Option<Vector2> {
		None
	}
}
