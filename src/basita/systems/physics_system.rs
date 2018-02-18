use super::super::{ContainsEngineEvents, ContainsEngineState};
use super::collisions;

use components::{BoxCollider, ComponentCollection, ComponentHandle, PhysicBody, Transform};
use math::Vector2;

#[derive(Default)]
pub struct PhysicsSystemState {
	pub dynamic_physic_body_handles: Vec<ComponentHandle<PhysicBody>>,
}

impl PhysicsSystemState {}

pub fn update<'a, S, E>(s: &mut S, _e: &E)
where
	S: ContainsEngineState<'a, S>,
	E: ContainsEngineEvents<S, E>,
{
	let engine_state = s.get_engine_state_mut();
	let state = &mut engine_state.systems_state.physics_system;

	let physic_bodies = &mut engine_state.physic_bodies;
	let transforms = &mut engine_state.transforms;
	let box_colliders = &mut engine_state.box_colliders;

	for h in &state.dynamic_physic_body_handles {
		let mut dynamic_body = physic_bodies.get_mut(h.clone());
		let mut transform = transforms.get_mut(dynamic_body.transform);

		dynamic_body.velocity += dynamic_body.acceleration; //* delta_time;
		transform.position += dynamic_body.velocity; //* delta_time;
		dynamic_body.acceleration.set(0.0, 0.0);
	}

	for physic_body_handle in physic_bodies.iter() {
		let physic_body = physic_bodies.get(physic_body_handle);
		let o_transform = transforms.get_mut(physic_body.transform);
		let o_box_collider = box_colliders.get(physic_body.box_collider);

		for h in &state.dynamic_physic_body_handles {
			let dynamic_body_handle = h.clone();
			let dynamic_body = physic_bodies.get(dynamic_body_handle);

			if physic_body_handle != dynamic_body_handle {
				/*
				let d_transform = transforms.get_mut(dynamic_body.transform);
				let d_box_collider = box_colliders.get(dynamic_body.box_collider);

				collide_physic_bodies(
					physic_bodies,
					dynamic_body_handle,
					d_box_collider,
					d_transform,
					physic_body_handle,
					o_box_collider,
					o_transform,
				);
				*/
			}
		}
	}
}

fn collide_physic_bodies(
	a: &mut PhysicBody,
	a_box_collider: &BoxCollider,
	a_transform: &mut Transform,
	b: &mut PhysicBody,
	b_box_collider: &BoxCollider,
	b_transform: &mut Transform,
) {
	let total_inverted_mass = a.inverted_mass + b.inverted_mass;
	if total_inverted_mass <= 0.0 {
		return;
	}

	let a_weight = a.inverted_mass / total_inverted_mass;
	let b_weight = b.inverted_mass / total_inverted_mass;

	let restitution = (a.bounciness + b.bounciness).sqrt();

	// test if box colliders are enabled

	if let Some(penetration) =
		collisions::collide_box_box(a_box_collider, a_transform, b_box_collider, b_transform)
	{
		let impulse_magnitude =
			Vector2::dot(penetration, b.velocity - a.velocity) * (1.0 + restitution);
		let impulse = (penetration * impulse_magnitude) / penetration.sqr_magnitude();

		a.velocity -= impulse * a_weight;
		b.velocity += impulse * b_weight;

		a_transform.position -= penetration * a_weight;
		b_transform.position += penetration * b_weight;

		// notify collision
	}
}
