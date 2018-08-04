pub mod assets;
pub mod components;
pub mod resources;

pub fn init(world: &mut ::specs::World) {
	world.register::<components::Transform>();

	world.add_resource(resources::Time::default());
}
