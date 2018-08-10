extern crate basita;

use std::thread;
use std::time::Duration;

use basita::sdl2::event::Event;
use basita::specs::{DispatcherBuilder, World};

use basita::core::resources::Time;
use basita::input::Input;
use basita::physics::systems::{ColliderRenderSystem, PhysicsSystem};
use basita::renderer::systems::RenderSystem;
use basita::sdl::{SdlContext, SdlLoader, SdlStorage};
use basita::{core, input, physics, renderer};

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

	// Player
	world.register::<components::Player>();

	// DISPATCHER
	let mut dispatcher = DispatcherBuilder::new()
		// Engine
		.with(PhysicsSystem::with_iteration_count(2), "physics", &[])
		.with(RenderSystem, "render", &[])
		.with(ColliderRenderSystem, "collider_render", &["render"])
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

		{
			sdl_context
				.canvas
				.set_draw_color(::basita::sdl2::pixels::Color::RGB(0, 0, 0));
			sdl_context.canvas.clear();

			let mut fonts = world.write_resource::<::basita::renderer::resources::Fonts>();
			let font_handle = fonts.load(
				&(String::from("assets/fonts/consola.ttf"), 28),
				&sdl_loader,
				&mut sdl_storage,
			);
			let font_asset = fonts.get(font_handle);
			let font = sdl_storage.font_storage.at(font_asset.index);

			let surface = font
				.render("Hello Rust!")
				.blended(::basita::sdl2::pixels::Color::RGBA(255, 0, 0, 255))
				.unwrap();
			let texture = sdl_loader
				.texture_creator
				.create_texture_from_surface(&surface)
				.unwrap();
			let query = texture.query();
			let rect = ::basita::sdl2::rect::Rect::new(200, 0, query.width, query.height);
			sdl_context.canvas.copy(&texture, None, rect).unwrap();
		}

		renderer::render(&world, &mut sdl_context, &sdl_storage);
		world.maintain();

		thread::sleep(Duration::new(0, 1_000_000_000u32 / frames_per_second));

		let mut time = world.write_resource::<Time>();
		time.delta_time = 1.0 / frames_per_second as f32;
	}
}
