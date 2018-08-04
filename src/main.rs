extern crate basita;

use std::thread;
use std::time::Duration;

use basita::sdl2::event::Event;

use basita::specs::{DispatcherBuilder, World};

use basita::sdl::{SdlContext, SdlLoader, SdlStorage};

use basita::core::components::Transform;
use basita::core::resources::Time;
use basita::input::Input;

use basita::renderer;
use basita::renderer::components::Sprite;
use basita::renderer::resources::{Images, Layers};
use basita::renderer::systems::RenderSystem;

use basita::physics::components::{Collider, PhysicBody};
use basita::physics::systems::PhysicsSystem;
//use basita::physics::systems::{ColliderRenderSystem, PhysicsSystem};

mod game;
use game::*;

pub fn main() {
	let frames_per_second = 60;

	let mut sdl_context = SdlContext::new("game", 400, 300);
	let sdl_loader = SdlLoader::new(&sdl_context);
	let mut sdl_storage = SdlStorage::default();

	// DECLARE RESOURCES
	let mut world = World::new();

	// Engine
	world.register::<Transform>();
	world.register::<Sprite>();
	world.register::<Collider>();
	world.register::<PhysicBody>();

	world.add_resource(Time::default());

	world.add_resource(Images::default());
	world.add_resource(Layers::default());
	world.add_resource(Input::default());

	// Player
	world.register::<components::Player>();

	// DISPATCHER
	let mut dispatcher = DispatcherBuilder::new()
		// Engine
		.with(PhysicsSystem::default(), "physics", &[])
		.with(RenderSystem, "render", &[])
		//.with_thread_local(ColliderRenderSystem::new(&sdl_context))
		// Player
		.with(systems::PlayerSystem, "player", &[])
		.build();

	// LOAD LEVEL
	levels::level1::load(&mut world, &sdl_loader, &mut sdl_storage);

	// MAIN LOOP
	'main: loop {
		{
			let mut input = world.write_resource::<Input>();
			input.update();

			let event_pump = &mut sdl_context.event_pump;
			for event in event_pump.poll_iter() {
				match event {
					Event::Quit { .. } => {
						break 'main;
					}
					e => {
						input.handle_event(e);
					}
				};
			}
		}

		dispatcher.dispatch(&mut world.res);

		renderer::render(&world, &mut sdl_context, &sdl_storage);

		world.maintain();

		thread::sleep(Duration::new(0, 1_000_000_000u32 / frames_per_second));
	}
}
