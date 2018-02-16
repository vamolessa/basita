extern crate sdl2;

mod engine;
mod game;

use engine::*;
use engine::systems::System;

use math::Vector2;
use components::{BoxCollider, Sprite, Transform};

use game::*;
use game::player_system::PlayerSystem;

pub fn main() {
	let sdl_context = SdlContext::new("platform maker", 400, 300);

	let mut engine = Engine::new(&sdl_context);

	let player_image = engine
		.state
		.image_resources
		.load(&String::from("resources/sprites/player.png"))
		.unwrap();

	engine.state.transforms.add(Transform::default());

	let _last_transform_index = engine.state.transforms.all.len() - 1;
	//let transform = &mut state.transforms.all[_last_transform_index];
	//transform.position = Vector2::new(10.0, 10.0);

	engine.state.sprites.add(Sprite {
		depth: 0,
		image_resource: player_image,
	});

	engine.state.box_colliders.add(BoxCollider {
		size: Vector2::from((32.0, 32.0)),
		offset: Vector2::default(),
	});

	let game = Game {
		player_system: PlayerSystem::new(&mut engine.state.input),
	};

	loop {
		if !engine.systems.sdl_event_system.update(&mut engine.state) {
			break;
		}

		//game.player_system.update(&state.input, transform);

		engine.systems.render_system.update(&mut engine.state);
		engine
			.systems
			.collider_render_system
			.update(&mut engine.state);

		engine
			.systems
			.sdl_presenter_system
			.update(&mut engine.state);
	}
}
