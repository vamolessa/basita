extern crate basita;

use basita::specs::{DispatcherBuilder, World};

use basita::sdl::SdlContext;
use basita::renderer::systems::RenderSystem;

use basita::core::components::Transform;
use basita::renderer::components::Sprite;

pub fn main() {
	let sdl_context = SdlContext::new("platform maker", 400, 300);
	let mut world = World::new();

	let mut dispatcher = DispatcherBuilder::new()
		.add_thread_local(RenderSystem::new(&sdl_context))
		.build();

	world.register::<Transform>();
	world.register::<Sprite>();

	dispatcher.dispatch(&mut world.res);
}
