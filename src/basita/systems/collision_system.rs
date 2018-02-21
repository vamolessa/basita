use super::super::{ContainsEngineEvents, ContainsEngineState};
use super::super::events::Event;
use super::System;

use components::{BoxShape, Collider, ComponentHandle, Shape, Transform};
use math::Vector2;

pub struct CollisionEvents<S, E>
where
	E: ContainsEngineEvents<S, E>,
{
	pub on_dynamic_collision: Event<
		S,
		E,
		(
			ComponentHandle<Collider>,
			ComponentHandle<Collider>,
			Vector2,
		),
	>,

	pub on_static_collision: Event<
		S,
		E,
		(
			ComponentHandle<Collider>,
			ComponentHandle<Collider>,
			Vector2,
		),
	>,
}

impl<S, E> CollisionEvents<S, E>
where
	E: ContainsEngineEvents<S, E>,
{
	pub fn new() -> Self {
		CollisionEvents {
			on_dynamic_collision: Event::new(),
			on_static_collision: Event::new(),
		}
	}
}

pub struct CollisionSystem;

impl<'a, S, E> System<S, E> for CollisionSystem
where
	S: ContainsEngineState<'a>,
	E: ContainsEngineEvents<S, E>,
{
	fn update(s: &mut S, e: &E) {
		let total = s.get_engine_state_mut().colliders.all.len();
		let events = e.get_engine_events();

		for i in 0..total {
			for j in i..total {
				let (ai, bi, r, event) = {
					let state = s.get_engine_state_mut();
					let a = &state.colliders.all[i];
					let b = &state.colliders.all[j];

					if !a.enabled || !b.enabled {
						continue;
					}

					let a_t = &state.transforms.get(a.transform);
					let b_t = &state.transforms.get(b.transform);

					if a.physic_body.is_some() {
						if b.physic_body.is_some() {
							let r = collide(a, a_t, b, b_t);
							(i, j, r, &events.collision.on_dynamic_collision)
						} else {
							let r = collide(a, a_t, b, b_t);
							(j, i, r, &events.collision.on_static_collision)
						}
					} else if b.physic_body.is_some() {
						let r = collide(a, a_t, b, b_t);
						(i, j, r, &events.collision.on_static_collision)
					} else {
						continue;
					}
				};

				if let Some(penetration) = r {
					let a = s.get_engine_state_mut().colliders.get_handle(ai);
					let b = s.get_engine_state_mut().colliders.get_handle(bi);

					println!("penetration! {:?}", penetration);
					event.raise(s, e, (a, b, penetration));
				}
			}
		}
	}
}

fn collide(
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
	a_position: Vector2,
	b: &Collider,
	b_transform: &Transform,
) -> Option<Vector2> {
	match b.shape {
		Shape::Box(box_shape) => {
			collide_box_box(a, a_position, &box_shape, b_transform.position + b.offset)
		}
	}
}

fn collide_box_box(
	a: &BoxShape,
	a_position: Vector2,
	b: &BoxShape,
	b_position: Vector2,
) -> Option<Vector2> {
	let a_min = a_position - a.half_size;
	let a_max = a_position + a.half_size;

	let b_min = b_position - b.half_size;
	let b_max = b_position + b.half_size;

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
		penetration.set(0.0, -max_to_min.y);
	}

	Some(penetration)
}
