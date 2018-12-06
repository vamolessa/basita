use super::components::{BoxShape, Shape};
use crate::math::Vector2;

pub fn collide(
	a: Shape,
	a_position: Vector2,
	b: Shape,
	b_position: Vector2,
) -> Option<Vector2> {
	match a {
		Shape::Box(box_shape) => collide_box(box_shape, a_position, b, b_position),
	}
}

fn collide_box(
	a: BoxShape,
	a_position: Vector2,
	b: Shape,
	b_position: Vector2,
) -> Option<Vector2> {
	match b {
		Shape::Box(box_shape) => collide_box_box(a, a_position, box_shape, b_position),
	}
}

fn collide_box_box(
	a: BoxShape,
	a_position: Vector2,
	b: BoxShape,
	b_position: Vector2,
) -> Option<Vector2> {
	let a_min = a_position - a.half_size;
	let a_max = a_position + a.half_size;

	let b_min = b_position - b.half_size;
	let b_max = b_position + b.half_size;

	let min_to_max = b_max - a_min;
	let max_to_min = b_min - a_max;

	if min_to_max.x <= 0.0 || min_to_max.y <= 0.0 || max_to_min.x >= 0.0 || max_to_min.y >= 0.0 {
		return None;
	}

	let abs = min_to_max.x.abs();
	let mut min_abs = abs;

	let mut penetration = Vector2::new(min_to_max.x, 0.0);
	let abs = max_to_min.x.abs();
	if abs < min_abs {
		min_abs = abs;
		penetration.set(max_to_min.x, 0.0);
	}

	let abs = min_to_max.y.abs();
	if abs < min_abs {
		min_abs = abs;
		penetration.set(0.0, min_to_max.y);
	}

	let abs = max_to_min.y.abs();
	if abs < min_abs {
		penetration.set(0.0, max_to_min.y);
	}

	Some(penetration)
}
