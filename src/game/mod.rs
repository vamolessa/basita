pub mod components;
pub mod entities;
pub mod levels;
pub mod resources;
pub mod systems;

pub fn init(world: &mut basita::specs::World) {
	world.register::<components::Player>();
	world.add_resource(resources::GameFonts::default());
	world.add_resource(resources::GameSfxs::default());
}
