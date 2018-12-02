use std::time::Duration;

use sdl2::mixer::MAX_VOLUME;

use super::assets::{Bgm, Sfx};
use core::assets::{AssetCollection, AssetHandle};

pub type Sfxs = AssetCollection<Sfx>;
pub type Bgms = AssetCollection<Bgm>;

pub enum SfxVariant {
	Play,
	Volume(i32),
	Pan(u8, u8),
}

pub struct SfxCommand {
	pub sfx_handle: AssetHandle<Sfx>,
	pub variant: SfxVariant,
}

#[derive(Default)]
pub struct SfxCommands {
	pub commands: Vec<SfxCommand>,
}

impl SfxCommands {
	pub fn add_play(&mut self, sfx_handle: AssetHandle<Sfx>) {
		self.commands.push(SfxCommand {
			sfx_handle: sfx_handle,
			variant: SfxVariant::Play,
		});
	}

	pub fn add_volume(&mut self, sfx_handle: AssetHandle<Sfx>, volume: u8) {
		self.commands.push(SfxCommand {
			sfx_handle: sfx_handle,
			variant: SfxVariant::Volume(to_volume(volume)),
		})
	}

	pub fn add_pan(&mut self, sfx_handle: AssetHandle<Sfx>, left: u8, right: u8) {
		self.commands.push(SfxCommand {
			sfx_handle: sfx_handle,
			variant: SfxVariant::Pan(left, right),
		})
	}
}

pub enum MusicCommand {
	Play(usize, i32),
	Stop(i32),
	Volume(i32),
}

#[derive(Default)]
pub struct MusicCommands {
	pub commands: Vec<MusicCommand>,
}

impl MusicCommands {
	pub fn add_play(&mut self, music_index: usize, fade_in: Duration) {
		self.commands
			.push(MusicCommand::Play(music_index, fade_in.as_millis() as i32));
	}

	pub fn add_stop(&mut self, fade_out: Duration) {
		self.commands
			.push(MusicCommand::Stop(fade_out.as_millis() as i32));
	}

	pub fn add_volume(&mut self, volume: u8) {
		self.commands.push(MusicCommand::Volume(to_volume(volume)));
	}
}

fn to_volume(volume: u8) -> i32 {
	(volume as i32) * MAX_VOLUME / (u8::max_value() as i32)
}
