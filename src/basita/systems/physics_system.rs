use super::super::{GameEvents, GameState};
use super::System;

use components::{Collider, ComponentHandle};
use math::Vector2;

pub struct PhysicsSystem;

impl<'a, S, E> System<S, E> for PhysicsSystem
where
	S: GameState<'a>,
	E: GameEvents<S, E>,
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

		let delta_time = state.delta_time;
		let physic_bodies = &mut state.world.physic_bodies;
		let transforms = &mut state.world.transforms;

		for physic_body in physic_bodies.iter_mut() {
			let mut transform = transforms.get_mut(physic_body.transform);

			physic_body.velocity += physic_body.acceleration * delta_time;
			transform.position += physic_body.velocity * delta_time;
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
	S: GameState<'a>,
	E: GameEvents<S, E>,
{
	let (ach, bch, penetration) = data;

	let (ah, bh, ath, bth, impulse, a_weight, b_weight) = {
		let world = &s.get_engine_state().world;

		let ac = world.colliders.get(ach);
		let bc = world.colliders.get(bch);
		let ah = ac.physic_body.unwrap();
		let bh = bc.physic_body.unwrap();
		let a = world.physic_bodies.get(ah);
		let b = world.physic_bodies.get(bh);

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

	s.get_engine_state_mut()
		.world
		.physic_bodies
		.get_mut(ah)
		.velocity -= impulse * a_weight;
	s.get_engine_state_mut()
		.world
		.transforms
		.get_mut(ath)
		.position -= penetration * a_weight;

	s.get_engine_state_mut()
		.world
		.physic_bodies
		.get_mut(bh)
		.velocity += impulse * b_weight;
	s.get_engine_state_mut()
		.world
		.transforms
		.get_mut(bth)
		.position += penetration * b_weight;
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
	S: GameState<'a>,
	E: GameEvents<S, E>,
{
	let (_sch, dch, penetration) = data;

	let (dh, dth, impulse) = {
		let world = &s.get_engine_state().world;
		let dc = world.colliders.get(dch);
		let dh = dc.physic_body.unwrap();
		let d = world.physic_bodies.get(dh);

		let restitution = d.bounciness;

		let impulse_magnitude = Vector2::dot(penetration, -d.velocity) * (1.0 + restitution);
		let impulse = (penetration * impulse_magnitude) / penetration.sqr_magnitude();

		(dh, d.transform, impulse)
	};

	let world = &mut s.get_engine_state_mut().world;
	world.physic_bodies.get_mut(dh).velocity += impulse;
	world.transforms.get_mut(dth).position += penetration;
}
