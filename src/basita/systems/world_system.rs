use std::error::Error;

use serde_json;

use super::System;
use super::super::{GameEvents, GameState};
use super::super::resources::WorldResourceHandle;

pub struct WorldSystem;

impl WorldSystem {
	pub fn save<'a, S>(s: &mut S) -> String
	where
		S: GameState<'a>,
	{
		let world = &s.get_engine_state().world;
		match serde_json::to_string_pretty(world) {
			Ok(json) => json,
			Err(e) => {
				panic!("Could not serialize World. Error: '{}'", e.description());
			}
		}
	}

	pub fn load<'a, S, E>(s: &mut S, e: &E, world_handle: WorldResourceHandle)
	where
		S: GameState<'a>,
		E: GameEvents<S, E>,
	{
		let world = {
			let world_resource = &s.get_engine_state().resources.worlds.get(world_handle);

			match serde_json::from_str(&world_resource.serialized_world[..]) {
				Ok(world) => world,
				Err(e) => {
					panic!("Could not deserialize World. Error: '{}'", e.description());
				}
			}
		};

		let world_events = &e.get_engine_events().world;

		s.get_engine_state_mut().world = world;
		world_events.on_deserialize.raise(s, e, ());
		world_events.on_load.raise(s, e, ());
	}
}

impl<'a, S, E> System<S, E> for WorldSystem
where
	S: GameState<'a>,
	E: GameEvents<S, E>,
{
	fn init(_s: &mut S, e: &mut E) {
		let events = e.get_engine_events_mut();
		events.world.on_deserialize.subscribe(on_deserialize);
	}

	fn update(_s: &mut S, _e: &E) {}
}

fn on_deserialize<'a, S, E>(s: &mut S, _e: &E, _data: ())
where
	S: GameState<'a>,
	E: GameEvents<S, E>,
{
	let state = s.get_engine_state_mut();
	state.world.init();
}
