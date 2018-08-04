pub mod systems;
pub mod components;
pub mod helpers;

pub fn init(world: &mut ::specs::World) {
	world.register::<components::Collider>();
	world.register::<components::PhysicBody>();
}
