use specs::{Join, Entities, Entity, ReadStorage, System, WriteStorage, Fetch};

use core::components::Transform;
use core::resources::Time;
use math::Vector2;
use super::super::helpers::collide;
use super::super::components::{Collider, PhysicBody};

#[derive(Default)]
struct CollisionResponse {
	position_offset: Vector2,
	velocity_offset: Vector2,
}

pub struct PhysicsSystem {
	collision_responses: Vec<(Entity, CollisionResponse)>,
}

impl<'a> System<'a> for PhysicsSystem {
	type SystemData = (
		Fetch<'a, Time>,
		Entities<'a>,
		ReadStorage<'a, Collider>,
		WriteStorage<'a, PhysicBody>,
		WriteStorage<'a, Transform>,
	);

	fn run(&mut self, (time, entities, colliders, mut physic_bodies, mut transforms): Self::SystemData) {
		// integration
		for (physic_body, transform) in (&mut physic_bodies, &mut transforms).join() {
			physic_body.velocity += physic_body.acceleration * time.delta_time;
			transform.position += physic_body.velocity * time.delta_time;
			physic_body.acceleration.set(0.0, 0.0);
		}

		self.collision_responses.clear();

		// check collisions
		for (ae, ac, at, ap) in (&*entities, &colliders, &transforms, &physic_bodies).join() {
			for (be, bc, bt, bp) in (&*entities, &colliders, &transforms, &physic_bodies).join() {
				if ae.id() <= be.id() {
					continue;
				}

				if let Some(penetration) = collide(ac, at, bc, bt) {
					let (ar, br) = get_dynamic_response(ap, bp, penetration);

					self.collision_responses.push((ae, ar));
					self.collision_responses.push((be, br));
				}
			}

			for (bc, bt, ()) in (&colliders, &transforms, !&physic_bodies).join() {
				if let Some(penetration) = collide(ac, at, bc, bt) {
					let ar = get_static_response(ap, penetration);
					self.collision_responses.push((ae, ar));
				}
			}
		}

		// respond to collisions
		for &(ref entity, ref response) in &self.collision_responses {
			let transform = transforms.get_mut(*entity).unwrap();
			let physic_body = physic_bodies.get_mut(*entity).unwrap();

			transform.position += response.position_offset;
			physic_body.velocity += response.velocity_offset;
		}
	}
}

fn get_dynamic_response(
	ap: &PhysicBody,
	bp: &PhysicBody,
	penetration: Vector2,
) -> (CollisionResponse, CollisionResponse) {
	let total_inverted_mass = ap.inverted_mass + bp.inverted_mass;
	if total_inverted_mass <= 0.0 {
		return (CollisionResponse::default(), CollisionResponse::default());
	}

	let a_weight = ap.inverted_mass / total_inverted_mass;
	let b_weight = bp.inverted_mass / total_inverted_mass;

	let restitution = (ap.bounciness * bp.bounciness).sqrt();

	let impulse_magnitude =
		Vector2::dot(penetration, bp.velocity - ap.velocity) * (1.0 + restitution);
	let impulse = (penetration * impulse_magnitude) / penetration.sqr_magnitude();

	let a_response = CollisionResponse {
		position_offset: penetration * (-a_weight),
		velocity_offset: impulse * (-a_weight),
	};

	let b_response = CollisionResponse {
		position_offset: penetration * (b_weight),
		velocity_offset: impulse * (b_weight),
	};

	(a_response, b_response)
}

fn get_static_response(p: &PhysicBody, penetration: Vector2) -> CollisionResponse {
	let restitution = p.bounciness;

	let impulse_magnitude = Vector2::dot(penetration, -p.velocity) * (1.0 + restitution);
	let impulse = (penetration * impulse_magnitude) / penetration.sqr_magnitude();

	CollisionResponse {
		position_offset: -penetration,
		velocity_offset: -impulse,
	}
}
