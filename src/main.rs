extern crate sdl2;

use std::time::Duration;

use sdl2::event::Event;
use sdl2::pixels::Color;

mod engine;
mod game;

use engine::*;

use math::Vector2;
use components::{BoxCollider, Sprite, Transform};

//use game::*;
//use player_system::PlayerSystem;

pub fn main() {
	let sdl_context = SdlContext::new("platform maker", 400, 300);

	let mut engine = Engine::new(&sdl_context);

	let player_image = engine
		.image_resources
		.load(&String::from("resources/sprites/player.png"))
		.unwrap();

	engine.transforms.add(Transform::default());

	let last_transform_index = engine.transforms.all.len() - 1;
	let transform = &mut engine.transforms.all[last_transform_index];
	transform.position = Vector2::new(10.0, 10.0);

	engine.sprites.add(Sprite {
		depth: 0,
		image_resource: player_image,
	});

	engine.box_colliders.add(BoxCollider {
		size: Vector2::from((32.0, 32.0)),
		offset: Vector2::default(),
	});

	//let mut _player_system = PlayerSystem::new(&mut engine.input);

	{
		let clear_color = Color::RGB(0, 0, 0);

		'main: loop {
			{
				let mut event_pump = engine.sdl_context.event_pump.borrow_mut();
				for event in event_pump.poll_iter() {
					match event {
						Event::Quit { .. } => break 'main,
						_ => {
							if !engine.input.handle_event(event) {
								break 'main;
							}
						}
					}
				}
			}

			{
				let mut canvas = engine.sdl_context.canvas.borrow_mut();
				canvas.set_draw_color(clear_color);
				canvas.clear();
			}

			//player_system.update(&input, transform);

			engine.render_system.update(
				&engine.sdl_context,
				&engine.image_resources,
				&mut engine.sprites,
			);

			engine
				.collider_render_system
				.update(&engine.sdl_context, &engine.box_colliders);

			{
				let mut canvas = engine.sdl_context.canvas.borrow_mut();
				canvas.present();
				::std::thread::sleep(Duration::new(
					0,
					1_000_000_000u32 / engine.sdl_context.frames_per_second,
				));
			}
		}
	}
}
