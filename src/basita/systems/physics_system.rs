use super::super::{ContainsEngineEvents, ContainsEngineState};
use super::System;

use components::{Collider, ComponentHandle};
use math::Vector2;

pub struct PhysicsSystem;

impl<'a, S, E> System<S, E> for PhysicsSystem
where
	S: ContainsEngineState<'a>,
	E: ContainsEngineEvents<S, E>,
{
	fn init(_s: &mut S, e: &mut E) {
		let events = e.get_engine_events_mut();
		events
			.collision
			.on_dynamic_collision
			.subscribe(on_dynamic_collision);
		events
			.collision
			.on_static_collision
			.subscribe(on_static_collision);
	}

	fn update(s: &mut S, _e: &E) {
		let state = s.get_engine_state_mut();
		let physic_bodies = &mut state.physic_bodies;
		let transforms = &mut state.transforms;

		for &mut (_h, mut physic_body) in physic_bodies.iter_mut() {
			let mut transform = transforms.get_mut(&physic_body.transform);

			physic_body.velocity += physic_body.acceleration * state.delta_time;
			transform.position += physic_body.velocity * state.delta_time;
			physic_body.acceleration.set(0.0, 0.0);
		}
	}
}

fn on_dynamic_collision<'a, S, E>(
	s: &mut S,
	_e: &E,
	data: (
		ComponentHandle<Collider>,
		ComponentHandle<Collider>,
		Vector2,
	),
) where
	S: ContainsEngineState<'a>,
	E: ContainsEngineEvents<S, E>,
{
	let (ach, bch, penetration) = data;

	let (ah, bh, ath, bth, impulse, a_weight, b_weight) = {
		let state = s.get_engine_state_mut();

		let ac = state.colliders.get(&ach);
		let bc = state.colliders.get(&bch);
		let ah = ac.physic_body.unwrap();
		let bh = bc.physic_body.unwrap();
		let a = state.physic_bodies.get(&ah);
		let b = state.physic_bodies.get(&bh);

		let total_inverted_mass = a.inverted_mass + b.inverted_mass;
		if total_inverted_mass <= 0.0 {
			return;
		}

		let a_weight = a.inverted_mass / total_inverted_mass;
		let b_weight = b.inverted_mass / total_inverted_mass;

		let restitution = (a.bounciness * b.bounciness).sqrt();

		let impulse_magnitude =
			Vector2::dot(penetration, b.velocity - a.velocity) * (1.0 + restitution);
		let impulse = (penetration * impulse_magnitude) / penetration.sqr_magnitude();

		(
			ah,
			bh,
			a.transform,
			b.transform,
			impulse,
			a_weight,
			b_weight,
		)
	};

	s.get_engine_state_mut().physic_bodies.get_mut(&ah).velocity -= impulse * a_weight;
	s.get_engine_state_mut().transforms.get_mut(&ath).position -= penetration * a_weight;

	s.get_engine_state_mut().physic_bodies.get_mut(&bh).velocity += impulse * b_weight;
	s.get_engine_state_mut().transforms.get_mut(&bth).position += penetration * b_weight;
}

fn on_static_collision<'a, S, E>(
	s: &mut S,
	_e: &E,
	data: (
		ComponentHandle<Collider>,
		ComponentHandle<Collider>,
		Vector2,
	),
) where
	S: ContainsEngineState<'a>,
	E: ContainsEngineEvents<S, E>,
{
	let (_sch, dch, penetration) = data;

	let (dh, dth, impulse) = {
		let state = s.get_engine_state_mut();
		let dc = state.colliders.get(&dch);
		let dh = dc.physic_body.unwrap();
		let d = state.physic_bodies.get(&dh);

		let restitution = d.bounciness;

		let impulse_magnitude = Vector2::dot(penetration, -d.velocity) * (1.0 + restitution);
		let impulse = (penetration * impulse_magnitude) / penetration.sqr_magnitude();

		(dh, d.transform, impulse)
	};

	s.get_engine_state_mut().physic_bodies.get_mut(&dh).velocity += impulse;
	s.get_engine_state_mut().transforms.get_mut(&dth).position += penetration;
}
