use std::error::Error;

use serde_json;

use super::super::{GameEvents, GameState};
use super::super::resources::WorldResourceHandle;

pub fn seserialize_world<'a, S>(s: &mut S) -> String
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

pub fn deserialize_world<'a, S, E>(s: &mut S, e: &E, handle: WorldResourceHandle)
where
	S: GameState<'a>,
	E: GameEvents<S, E>,
{
	let world = {
		let world_resource = &s.get_engine_state().resources.worlds.get(handle);

		match serde_json::from_str(&world_resource.serialized_world[..]) {
			Ok(world) => world,
			Err(e) => {
				panic!("Could not deserialize World. Error: '{}'", e.description());
			}
		}
	};

	s.get_engine_state_mut().world = world;
	e.get_engine_events().world.on_load.raise(s, e, ());
}
