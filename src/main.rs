extern crate sdl2;

use std::path::Path;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::pixels::Color;

mod engine;
mod game;

use engine::*;
use sdl_context::SdlContext;

use math::Vector2;
use components::{BoxCollider, Transform};

//use game::*;
//use player_system::PlayerSystem;

pub fn main() {
	let sdl_context = SdlContext::new("platform maker", 400, 300);
	let mut engine = Engine::new(sdl_context);
	let mut engine_resources = EngineResources::new();

	let _player_image = engine::resources::load(
		&engine,
		&mut engine_resources,
		Path::new("resources/sprites/player.png"),
	);

	/*
	engine.transforms.add(Transform::default());

	let last_transform_index = engine.transforms.all.len() - 1;
	let transform = &mut engine.transforms.all[last_transform_index];
	transform.position = Vector2::new(10.0, 10.0);

	/*
	engine.sprites.add(Sprite {
		transform: transform,
		depth: 0,
		image: &player_image,
	});
	*/

	engine.box_colliders.add(BoxCollider {
		size: Vector2::from((32.0, 32.0)),
		offset: Vector2::default(),
	});
	*/

	//let mut _player_system = PlayerSystem::new(&mut engine.input);

	{
		let clear_color = Color::RGB(0, 0, 0);

		'main: loop {
			for event in engine.sdl_context.event_pump.poll_iter() {
				match event {
					Event::Quit { .. } => break 'main,
					_ => {
						if !engine.input.handle_event(event) {
							break 'main;
						}
					}
				}
			}

			engine.sdl_context.canvas.set_draw_color(clear_color);
			engine.sdl_context.canvas.clear();

			{
				//player_system.update(&input, transform);

				/*
				renderer_system.update(
					&mut engine.sdl_context,
					&engine_resources.image_resources,
					&mut engine.sprites,
				);
				collider_renderer_system.update(&mut engine.sdl_context, &engine.box_colliders);
				*/
			}

			engine.sdl_context.canvas.present();
			::std::thread::sleep(Duration::new(
				0,
				1_000_000_000u32 / engine.sdl_context.frames_per_second,
			));
		}
	}
}
