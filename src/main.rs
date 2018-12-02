extern crate basita;

use std::thread;
use std::time::Duration;

use basita::sdl2::event::Event;
use basita::specs::{DispatcherBuilder, World};

use basita::core::resources::{LazyEvaluations, Time};
use basita::input::Input;
use basita::physics::systems::{ColliderRenderSystem, PhysicsSystem};
use basita::renderer::systems::RenderSystem;
use basita::sdl::{SdlContext, SdlLoader, SdlStorage};
use basita::{core, gui, input, physics, renderer};

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
	core::init(&mut world);
	input::init(&mut world);
	renderer::init(&mut world);
	physics::init(&mut world);
	gui::init(&mut world);

	// Player
	init(&mut world);

	// DISPATCHER
	let mut dispatcher = DispatcherBuilder::new()
		// Engine
		.with(PhysicsSystem::with_iteration_count(2), "physics", &[])
		.with(RenderSystem, "render", &[])
		.with(ColliderRenderSystem, "collider_render", &[])
		// Player
		.with(systems::PlayerSystem, "player", &[])
		.build();

	// LOAD LEVEL
	levels::level1::load(&mut world.write_resource::<LazyEvaluations>());

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

		LazyEvaluations::evaluate(&mut world, &sdl_loader, &mut sdl_storage);
		dispatcher.dispatch(&mut world.res);

		renderer::render(&mut world, &mut sdl_context, &mut sdl_storage);
		world.maintain();

		thread::sleep(Duration::new(0, 1_000_000_000u32 / frames_per_second));

		let mut time = world.write_resource::<Time>();
		time.delta_time = 1.0 / frames_per_second as f32;
	}
}
