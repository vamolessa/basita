use sdl2::mixer::{Channel, MAX_VOLUME};

use sdl::SdlStorage;
use specs::World;

use super::resources::{SfxCommands, SfxVariant, Sfxs};

pub fn mix<'a>(world: &mut World, sdl_storage: &mut SdlStorage<'a>) {
	let mut sfx_commands = world.write_resource::<SfxCommands>();
	let mut sfxs = world.write_resource::<Sfxs>();

	for command in sfx_commands.commands.iter() {
		match command.variant {
			SfxVariant::Play => {
				let sfx = sfxs.get_mut(command.sfx_handle);
				let chunk = sdl_storage.chunk_storage.at_mut(sfx.chunk_index);

				if let Ok(channel) = Channel::all().play(&chunk, 1) {
					//channel.unregister_all_effects().unwrap();
					//channel.set_volume(MAX_VOLUME);
					sfx.channel = Some(channel);

					std::thread::sleep(std::time::Duration::new(5, 0));
				}
			}
			SfxVariant::Volume(volume) => {
				let sfx = sfxs.get_mut(command.sfx_handle);
				if let Some(channel) = sfx.channel {
					channel.set_volume((volume as i32) * MAX_VOLUME / (u8::max_value() as i32));
				}
			}
			SfxVariant::Pan(left, right) => {
				let sfx = sfxs.get_mut(command.sfx_handle);
				if let Some(channel) = sfx.channel {
					channel.set_panning(left, right).unwrap();
				}
			}
		}
	}

	sfx_commands.commands.clear();
}
