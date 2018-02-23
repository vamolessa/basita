use basita::sdl2::keyboard::Keycode;

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
		let player_transform = state.world.transforms.add(Transform::identity());
		let player_physic_body = state.world.physic_bodies.add(PhysicBody {
			velocity: Vector2::zero(),
			acceleration: Vector2::zero(),

			inverted_mass: 1.0,
			bounciness: 1.0,

			transform: player_transform,
		});

		PlayerSystemData {
			left_button: state.input.new_button(Keycode::Left),
			right_button: state.input.new_button(Keycode::Right),
			up_button: state.input.new_button(Keycode::Up),
			down_button: state.input.new_button(Keycode::Down),
			jump_button: state.input.new_button(Keycode::Space),

			player_transform_handle: player_transform,
			player_physic_body_handle: player_physic_body,
		}
	}
}

pub struct PlayerSystem;

impl<'a> System<GameState<'a>, GameEvents<'a>> for PlayerSystem {
	fn init(state: &mut GameState, _events: &mut GameEvents) {
		let player_image = state
			.engine
			.resources
			.images
			.load(&String::from("resources/sprites/player.png"));

		{
			let transform = state
				.engine
				.world
				.transforms
				.get_mut(&state.player_system_data.player_transform_handle);
			transform.position = Vector2::new(200.0, 150.0);

			let sprite = state.engine.world.sprites.add(Sprite {
				depth: 0,
				image_resource: player_image,
				transform: state.player_system_data.player_transform_handle,
			});

			state.engine.systems.render.add_sprite(sprite);
		}

		state.engine.world.colliders.add(Collider {
			shape: Shape::Box(BoxShape {
				half_size: Vector2::new(16.0, 16.0),
			}),
			offset: Vector2::zero(),
			is_trigger: false,

			transform: state.player_system_data.player_transform_handle,
			physic_body: Some(state.player_system_data.player_physic_body_handle),
		});

		let ground_transform = state
			.engine
			.world
			.transforms
			.add(Transform::new(Vector2::new(200.0, 200.0)));

		state.engine.world.colliders.add(Collider {
			shape: Shape::Box(BoxShape {
				half_size: Vector2::new(100.0, 5.0),
			}),
			offset: Vector2::zero(),
			is_trigger: false,

			transform: ground_transform,
			physic_body: None,
		});
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
