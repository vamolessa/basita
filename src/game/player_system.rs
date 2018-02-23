use basita::sdl2::keyboard::Keycode;

use basita::serialization;
use basita::EngineState;
use basita::systems::System;
use basita::input::Button;
use basita::math::Vector2;
use basita::components::*;

use game::*;

pub struct PlayerSystemData {
	pub left_button: Button,
	pub right_button: Button,
	pub up_button: Button,
	pub down_button: Button,
	pub jump_button: Button,

	pub player_transform_handle: ComponentHandle<Transform>,
	pub player_physic_body_handle: ComponentHandle<PhysicBody>,
}

impl PlayerSystemData {
	pub fn new(state: &mut EngineState) -> Self {
		PlayerSystemData {
			left_button: state.input.new_button(Keycode::Left),
			right_button: state.input.new_button(Keycode::Right),
			up_button: state.input.new_button(Keycode::Up),
			down_button: state.input.new_button(Keycode::Down),
			jump_button: state.input.new_button(Keycode::Space),

			player_transform_handle: ComponentHandle::default(),
			player_physic_body_handle: ComponentHandle::default(),
		}
	}
}

pub struct PlayerSystem;

impl<'a> System<GameState<'a>, GameEvents<'a>> for PlayerSystem {
	fn init(state: &mut GameState<'a>, events: &mut GameEvents<'a>) {
		let player_image = state
			.engine
			.resources
			.images
			.load(&String::from("resources/sprites/player.png"));

		let world1_handle = state
			.engine
			.resources
			.worlds
			.load(&String::from("resources/worlds/world1.json"));

		serialization::deserialize_world(state, events, world1_handle);

		{
			let sprite = state
				.engine
				.world
				.sprites
				.get_mut(&ComponentHandle::default());
			sprite.image_resource = player_image;
		}

		/*
		use std::fs::File;
		use std::io::prelude::*;

		let json = serialization::seserialize_world(state);
		let mut file = File::create("resources/worlds/world1.json").unwrap();
		file.write_all(json.as_bytes()).unwrap();
		*/
	}

	fn update(state: &mut GameState, _events: &GameEvents) {
		let player_physic_body = state
			.engine
			.world
			.physic_bodies
			.get_mut(&state.player_system_data.player_physic_body_handle);

		player_physic_body.acceleration += Vector2::new(0.0, 10.0);

		if state
			.engine
			.input
			.is_pressed(state.player_system_data.left_button)
		{
			player_physic_body.velocity.x = 1.0;
		} else if state
			.engine
			.input
			.is_pressed(state.player_system_data.right_button)
		{
			player_physic_body.velocity.x = 1.0;
		} else {
			player_physic_body.velocity.x = 0.0;
		}

		if state
			.engine
			.input
			.is_pressed(state.player_system_data.up_button)
		{
			player_physic_body.velocity.y = 1.0;
		}
		if state
			.engine
			.input
			.is_pressed(state.player_system_data.down_button)
		{
			player_physic_body.velocity.y = 1.0;
		}

		/*
		if state
			.engine
			.input
			.was_pressed(state.player_system_data.jump_button)
		{
			player_physic_body.position.y -= 10.0;
		}

		if state
			.engine
			.input
			.was_released(state.player_system_data.jump_button)
		{
			player_physic_body.position.y += 10.0;
		}
		*/
	}
}
