use sdl2::mixer::Channel;

use sdl::{SdlContext, SdlStorage};
use specs::World;

use super::resources::{ChunkCommands, ChunkVariant};

pub fn mix<'a>(
	world: &mut World,
	sdl_context: &mut SdlContext,
	sdl_storage: &mut SdlStorage<'a>,
) {
	let mut chunk_commands = world.write_resource::<ChunkCommands>();

	for command in chunk_commands.commands.iter() {
		match command.variant {
			ChunkVariant::Play => {
				/*
				let chunk = sdl_storage.chunk_storage.at_mut(command.chunk_index);
				let channel = Channel::all().play(&chunk, 1).unwrap();
				channel.unregister_all_effects();
				chunk.channel = Some(channel);
				*/
			}
			ChunkVariant::Pan(left, right) => {
				/*
				let chunk = sdl_storage.chunk_storage.at_mut(command.chunk_index);
				let channel = Channel::all().play(&chunk, 1).unwrap();
				channel.unregister_all_effects();
				chunk.channel = Some(channel);
				*/
			}
		}
	}

	chunk_commands.commands.clear();
}
