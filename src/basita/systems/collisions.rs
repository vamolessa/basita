use components::{BoxCollider, Transform};
use math::Vector2;

pub fn collide_box_box(
	a: &BoxCollider,
	a_transform: &Transform,
	b: &BoxCollider,
	b_transform: &Transform,
) -> Option<Vector2> {
	let a_center = a_transform.position + a.offset;
	let b_center = b_transform.position + b.offset;

	let a_min = a_center - a.half_size;
	let a_max = a_center + a.half_size;

	let b_min = b_center - b.half_size;
	let b_max = b_center + b.half_size;

	let min_to_max = b_min - a_max;
	let max_to_min = b_max - a_min;

	if min_to_max.x < 0.0 || min_to_max.y < 0.0 || max_to_min.x > 0.0 || max_to_min.y > 0.0 {
		return None;
	}

	let abs = min_to_max.x.abs();
	let mut min_abs = abs;

	let mut penetration = Vector2::zero();

	let abs = max_to_min.x.abs();
	if abs < min_abs {
		min_abs = abs;
		penetration.set(max_to_min.x, 0.0);
	}

	let abs = min_to_max.y.abs();
	if abs < min_abs {
		min_abs = abs;
		penetration.set(0.0, -min_to_max.y);
	}

	let abs = max_to_min.y.abs();
	if abs < min_abs {
		min_abs = abs;
		penetration.set(0.0, -max_to_min.y);
	}

	None
}
