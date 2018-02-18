use super::super::{ContainsEngineEvents, ContainsEngineState};
use super::super::events::Event;

use components::{BoxCollider, ComponentHandle, Transform};
use math::Vector2;

pub struct CollisionEvents<S, E>
where
	E: ContainsEngineEvents<S, E>,
{
	pub on_collision: Event<
		S,
		E,
		(
			ComponentHandle<BoxCollider>,
			ComponentHandle<BoxCollider>,
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
			on_collision: Event::new(),
		}
	}
}

pub fn update<'a, S, E>(s: &mut S, e: &E)
where
	S: ContainsEngineState<'a, S>,
	E: ContainsEngineEvents<S, E>,
{
	let total = s.get_engine_state_mut().box_colliders.all.len();

	for i in 0..total {
		for j in i..total {
			if let Some(penetration) = collide_colliders(s, e, i, j) {
				let (a, b) = {
					let state = s.get_engine_state_mut();
					let a = state.box_colliders.get_handle(i);
					let b = state.box_colliders.get_handle(j);

					(a, b)
				};

				let events = e.get_engine_events();
				events
					.collision
					.on_collision
					.raise(s, e, &(a, b, penetration));
			}
		}
	}
}

fn collide_colliders<'a, S, E>(s: &mut S, _e: &E, i: usize, j: usize) -> Option<Vector2>
where
	S: ContainsEngineState<'a, S>,
	E: ContainsEngineEvents<S, E>,
{
	let state = s.get_engine_state_mut();
	let a = &state.box_colliders.all[i];
	let b = &state.box_colliders.all[j];

	if a.physic_body.is_some() && b.physic_body.is_some() {
		let a_t = &state.transforms.all[i];
		let b_t = &state.transforms.all[j];

		return collide_box_box(a, a_t, b, b_t);
	}

	None
}

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
		penetration.set(0.0, -max_to_min.y);
	}

	Some(penetration)
}
