use super::assets::{Bgm, Sfx};
use core::assets::{AssetCollection, AssetHandle};

pub type Sfxs = AssetCollection<Sfx>;
pub type Bgms = AssetCollection<Bgm>;

pub enum SfxVariant {
	Play,
	Volume(u8),
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
			variant: SfxVariant::Volume(volume),
		})
	}

	pub fn add_pan(&mut self, sfx_handle: AssetHandle<Sfx>, left: u8, right: u8) {
		self.commands.push(SfxCommand {
			sfx_handle: sfx_handle,
			variant: SfxVariant::Pan(left, right),
		})
	}
}

#[derive(Default)]
pub struct MusicCommands {
	pub commands: Vec<()>,
}
