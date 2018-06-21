extern crate basita;

use std::thread;
use std::time::Duration;

use basita::sdl2::event::Event;
use basita::sdl2::pixels::Color;
use basita::specs::{DispatcherBuilder, World};

use basita::sdl::{SdlContext, SdlStorage};

use basita::core::components::Transform;
use basita::core::resources::Time;
use basita::input::Input;

use basita::renderer::components::Sprite;
use basita::renderer::resources::{Images, Layers};
use basita::renderer::systems::RenderSystem;

use basita::physics::components::{Collider, PhysicBody};
use basita::physics::systems::{ColliderRenderSystem, PhysicsSystem};

mod game;
use game::*;

pub fn main() {
	let frames_per_second = 60;
	let clear_color = Color::RGB(0, 0, 0);

	let sdl_context = SdlContext::new("game", 400, 300);
	let sdl_storage = SdlStorage::default();

	// DECLARE RESOURCES
	let mut world = World::new();

	world.register::<Transform>();
	world.register::<Sprite>();
	world.register::<Collider>();
	world.register::<PhysicBody>();

	world.add_resource(Time::default());

	world.add_resource(Images::default());
	world.add_resource(Layers::default());
	world.add_resource(Input::default());

	// DISPATCHER
	let mut dispatcher = DispatcherBuilder::new()
		.with(PhysicsSystem::default(), "physics", &[])
		.with_thread_local(RenderSystem::new(&sdl_context, &sdl_storage))
		.with_thread_local(ColliderRenderSystem::new(&sdl_context))
		.build();

	// LOAD LEVEL
	levels::level1::load(&mut world, &sdl_context, &sdl_storage);

	// MAIN LOOP
	'main: loop {
		{
			let mut input = world.write_resource::<Input>();
			input.update();

			let mut event_pump = sdl_context.event_pump.borrow_mut();
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

		{
			let mut canvas = sdl_context.canvas.borrow_mut();

			canvas.set_draw_color(clear_color);
			canvas.clear();
		}

		dispatcher.dispatch(&mut world.res);

		{
			let mut canvas = sdl_context.canvas.borrow_mut();
			canvas.present();
		}

		world.maintain();

		thread::sleep(Duration::new(0, 1_000_000_000u32 / frames_per_second));
	}
}
