use core::components::Transform;
use math::Vector2;
use super::super::components::{BoxShape, Collider, Shape};

pub fn collide(
	a: &Collider,
	a_transform: &Transform,
	b: &Collider,
	b_transform: &Transform,
) -> Option<Vector2> {
	match a.shape {
		Shape::Box(box_shape) => {
			collide_box(&box_shape, a_transform.position + a.offset, b, b_transform)
		}
	}
}

fn collide_box(
	a: &BoxShape,
	a_center: Vector2,
	b: &Collider,
	b_transform: &Transform,
) -> Option<Vector2> {
	match b.shape {
		Shape::Box(box_shape) => {
			collide_box_box(a, a_center, &box_shape, b_transform.position + b.offset)
		}
	}
}

fn collide_box_box(
	a: &BoxShape,
	a_center: Vector2,
	b: &BoxShape,
	b_center: Vector2,
) -> Option<Vector2> {
	let a_min = a_center - a.half_size;
	let a_max = a_center + a.half_size;

	let b_min = b_center - b.half_size;
	let b_max = b_center + b.half_size;

	let min_to_max = b_min - a_max;
	let max_to_min = b_max - a_min;

	if min_to_max.x >= 0.0 || min_to_max.y >= 0.0 || max_to_min.x <= 0.0 || max_to_min.y <= 0.0 {
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
