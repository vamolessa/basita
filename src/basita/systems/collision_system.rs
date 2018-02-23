use super::super::{GameEvents, GameState};
use super::super::events::Event;
use super::System;

use components::{BoxShape, Collider, ComponentHandle, Shape, Transform};
use math::Vector2;

pub struct CollisionEvents<S, E>
where
	E: GameEvents<S, E>,
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

impl<'a, S, E> Default for CollisionEvents<S, E>
where
	S: GameState<'a>,
	E: GameEvents<S, E>,
{
	fn default() -> Self {
		CollisionEvents {
			on_dynamic_collision: Event::default(),
			on_static_collision: Event::default(),
		}
	}
}

pub struct CollisionSystem;

impl<'a, S, E> System<S, E> for CollisionSystem
where
	S: GameState<'a>,
	E: GameEvents<S, E>,
{
	fn update(s: &mut S, e: &E) {
		let total = s.get_engine_state().world.colliders.len();
		let events = e.get_engine_events();

		for i in 0..total {
			for j in (i + 1)..total {
				let (ai, bi, r, event) = {
					let world = &s.get_engine_state().world;
					let a = world.colliders.get_at(i);
					let b = world.colliders.get_at(j);

					let a_t = &world.transforms.get(&a.transform);
					let b_t = &world.transforms.get(&b.transform);

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
					let a = s.get_engine_state().world.colliders.get_handle(ai);
					let b = s.get_engine_state().world.colliders.get_handle(bi);

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
