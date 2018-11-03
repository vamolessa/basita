pub mod assets;
pub mod components;
pub mod resources;
pub mod systems;

mod renderer;
pub use self::renderer::*;

pub fn init(world: &mut specs::World) {
	world.register::<components::Camera>();
	world.register::<components::Sprite>();

	world.add_resource(resources::Images::default());
	world.add_resource(resources::ImageLoadRequests::default());

	world.add_resource(resources::Fonts::default());
	world.add_resource(resources::FontLoadRequests::default());

	world.add_resource(resources::RenderCommands::default());
}
