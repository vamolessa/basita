use specs::{Entities, Entity, Join, Read, ReadStorage, System, WriteStorage};

use fxhash::FxHashMap;

use super::super::components::{Collider, PhysicBody};
use super::super::helpers::collide;
use crate::core::components::Transform;
use crate::core::resources::Time;
use crate::math::Vector2;

#[derive(Default)]
struct CollisionResponse {
	position_offset: Vector2,
	velocity_offset: Vector2,
}

pub struct PhysicsSystem {
	pub iteration_count: usize,
	collision_responses: FxHashMap<Entity, CollisionResponse>,
}

impl PhysicsSystem {
	pub fn with_iteration_count(iteration_count: usize) -> Self {
		PhysicsSystem {
			iteration_count: iteration_count,
			collision_responses: Default::default(),
		}
	}

	fn replace_response_if_bigger(&mut self, entity: Entity, response: CollisionResponse) {
		use std::collections::hash_map::Entry;

		match self.collision_responses.entry(entity) {
			Entry::Occupied(mut entry) => if response.position_offset.sqr_magnitude()
				> entry.get().position_offset.sqr_magnitude()
			{
				entry.insert(response);
			},
			Entry::Vacant(entry) => {
				entry.insert(response);
			}
		};
	}
}

impl<'a> System<'a> for PhysicsSystem {
	type SystemData = (
		Read<'a, Time>,
		Entities<'a>,
		ReadStorage<'a, Collider>,
		WriteStorage<'a, PhysicBody>,
		WriteStorage<'a, Transform>,
	);

	fn run(
		&mut self,
		(time, entities, colliders, mut physic_bodies, mut transforms): Self::SystemData,
	) {
		// integration
		for (physic_body, transform) in (&mut physic_bodies, &mut transforms).join() {
			physic_body.velocity += physic_body.acceleration * time.delta_time;
			transform.position += physic_body.velocity * time.delta_time;
			physic_body.acceleration.set(0.0, 0.0);
		}

		// check collisions
		for _iteration in 0..self.iteration_count {
			self.collision_responses.clear();

			for (ae, ac, at, ap) in (&*entities, &colliders, &transforms, &physic_bodies).join() {
				for (be, bc, bt, bp) in (&*entities, &colliders, &transforms, &physic_bodies).join()
				{
					if ae.id() <= be.id() {
						continue;
					}

					if let Some(penetration) = collide(
						ac.shape,
						at.position + ac.offset,
						bc.shape,
						bt.position + bc.offset,
					) {
						let restitution = (ac.bounciness * bc.bounciness).sqrt();
						let (ar, br) = get_dynamic_response(ap, bp, restitution, penetration);

						self.replace_response_if_bigger(ae, ar);
						self.replace_response_if_bigger(be, br);
					}
				}

				for (bc, bt, ()) in (&colliders, &transforms, !&physic_bodies).join() {
					if let Some(penetration) = collide(
						ac.shape,
						at.position + ac.offset,
						bc.shape,
						bt.position + bc.offset,
					) {
						let restitution = (ac.bounciness * bc.bounciness).sqrt();
						let ar = get_static_response(ap, restitution, penetration);

						self.replace_response_if_bigger(ae, ar);
					}
				}
			}

			if self.collision_responses.len() == 0 {
				break;
			}

			// respond to collisions
			for (entity, response) in &self.collision_responses {
				let transform = transforms.get_mut(*entity).unwrap();
				let physic_body = physic_bodies.get_mut(*entity).unwrap();

				transform.position += response.position_offset;
				physic_body.velocity += response.velocity_offset;
			}
		}
	}
}

fn get_dynamic_response(
	ap: &PhysicBody,
	bp: &PhysicBody,
	restitution: f32,
	penetration: Vector2,
) -> (CollisionResponse, CollisionResponse) {
	let total_inverted_mass = ap.inverted_mass + bp.inverted_mass;
	if total_inverted_mass <= 0.0 {
		return (CollisionResponse::default(), CollisionResponse::default());
	}

	let a_weight = ap.inverted_mass / total_inverted_mass;
	let b_weight = bp.inverted_mass / total_inverted_mass;

	let relative_velocity = bp.velocity - ap.velocity;

	let impulse = if Vector2::dot(relative_velocity, penetration) >= 0.0 {
		let impulse_magnitude =
			Vector2::dot(penetration, bp.velocity - ap.velocity) * (1.0 + restitution);
		(penetration * impulse_magnitude) / penetration.sqr_magnitude()
	} else {
		Vector2::zero()
	};

	let a_response = CollisionResponse {
		position_offset: penetration * (a_weight),
		velocity_offset: impulse * (a_weight),
	};

	let b_response = CollisionResponse {
		position_offset: penetration * (-b_weight),
		velocity_offset: impulse * (-b_weight),
	};

	(a_response, b_response)
}

fn get_static_response(
	p: &PhysicBody,
	restitution: f32,
	penetration: Vector2,
) -> CollisionResponse {
	let relative_velocity = -p.velocity;

	let impulse = if Vector2::dot(relative_velocity, penetration) >= 0.0 {
		let impulse_magnitude = Vector2::dot(penetration, relative_velocity) * (1.0 + restitution);
		(penetration * impulse_magnitude) / penetration.sqr_magnitude()
	} else {
		Vector2::zero()
	};

	CollisionResponse {
		position_offset: penetration,
		velocity_offset: impulse,
	}
}
