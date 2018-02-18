use super::super::{ContainsEngineEvents, ContainsEngineState};

use components::{BoxCollider, ComponentHandle, PhysicBody, Transform};
use math::Vector2;

pub fn init<'a, S, E>(_s: &mut S, e: &mut E)
where
	S: ContainsEngineState<'a, S>,
	E: ContainsEngineEvents<S, E>,
{
	let events = e.get_engine_events_mut();
	events.collision.on_collision.subscribe(on_collision);
}

pub fn update<'a, S, E>(s: &mut S, _e: &E)
where
	S: ContainsEngineState<'a, S>,
	E: ContainsEngineEvents<S, E>,
{
	let state = s.get_engine_state_mut();
	let physic_bodies = &mut state.physic_bodies;
	let transforms = &mut state.transforms;

	for physic_body in &mut physic_bodies.all {
		let mut transform = transforms.get_mut(physic_body.transform);

		physic_body.velocity += physic_body.acceleration * state.delta_time;
		transform.position += physic_body.velocity * state.delta_time;
		physic_body.acceleration.set(0.0, 0.0);
	}
}

fn on_collision<'a, S, E>(
	_s: &mut S,
	_e: &E,
	_data: &(
		ComponentHandle<BoxCollider>,
		ComponentHandle<BoxCollider>,
		Vector2,
	),
) where
	S: ContainsEngineState<'a, S>,
	E: ContainsEngineEvents<S, E>,
{
}

fn collide_physic_bodies(
	a: &mut PhysicBody,
	a_box_collider: &BoxCollider,
	a_transform: &mut Transform,
	b: &mut PhysicBody,
	b_box_collider: &BoxCollider,
	b_transform: &mut Transform,
	penetration: Vector2,
) {
	let total_inverted_mass = a.inverted_mass + b.inverted_mass;
	if total_inverted_mass <= 0.0 {
		return;
	}

	let a_weight = a.inverted_mass / total_inverted_mass;
	let b_weight = b.inverted_mass / total_inverted_mass;

	let restitution = (a.bounciness + b.bounciness).sqrt();

	// test if box colliders are enabled

	let impulse_magnitude =
		Vector2::dot(penetration, b.velocity - a.velocity) * (1.0 + restitution);
	let impulse = (penetration * impulse_magnitude) / penetration.sqr_magnitude();

	a.velocity -= impulse * a_weight;
	b.velocity += impulse * b_weight;

	a_transform.position -= penetration * a_weight;
	b_transform.position += penetration * b_weight;

	// notify collision
}
