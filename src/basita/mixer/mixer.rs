use sdl2::mixer::{Channel, Music, MAX_VOLUME};

use sdl::SdlStorage;
use specs::World;

use super::resources::{MusicCommand, MusicCommands, SfxCommands, SfxVariant, Sfxs};

pub fn mix<'a>(world: &mut World, sdl_storage: &mut SdlStorage<'a>) {
	let mut sfx_commands = world.write_resource::<SfxCommands>();
	let mut sfxs = world.write_resource::<Sfxs>();
	for command in sfx_commands.commands.iter() {
		match command.variant {
			SfxVariant::Play => {
				let sfx = sfxs.get_mut(command.sfx_handle);
				let chunk = sdl_storage.chunk_storage.at_mut(sfx.chunk_index);

				if let Ok(channel) = Channel::all().play(chunk, 0) {
					channel.unregister_all_effects().unwrap();
					channel.set_volume(MAX_VOLUME);
					sfx.channel = Some(channel);
				}
			}
			SfxVariant::Volume(volume) => {
				let sfx = sfxs.get_mut(command.sfx_handle);
				if let Some(channel) = sfx.channel {
					channel.set_volume(volume);
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

	let mut music_commands = world.write_resource::<MusicCommands>();
	for command in music_commands.commands.iter() {
		match *command {
			MusicCommand::Play(music_index, fade_in) => {
				let music = sdl_storage.music_storage.at(music_index);
				if let Ok(_) = music.fade_in(0, fade_in) {}
			}
			MusicCommand::Stop(fade_out) => if let Ok(_) = Music::fade_out(fade_out) {},
			MusicCommand::Volume(volume) => Music::set_volume(volume),
		}
	}
	music_commands.commands.clear();
}
