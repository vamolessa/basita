use sdl2::mixer::Channel;

use crate::core::assets::{Asset, AssetLoadError, AssetLoader};
use crate::sdl::{SdlLoader, SdlStorage};

pub struct Sfx {
	pub chunk_index: usize,
	pub channel: Option<Channel>,
}

impl Asset for Sfx {
	type Id = String;
}

impl<'a> AssetLoader<'a, Sfx> for SdlLoader {
	type Storage = SdlStorage<'a>;

	fn load(
		&'a self,
		path: &<Sfx as Asset>::Id,
		storage: &mut Self::Storage,
	) -> Result<Sfx, AssetLoadError> {
		self.chunk_loader
			.load(path, storage)
			.map(|chunk_index| Sfx {
				chunk_index: chunk_index,
				channel: None,
			})
	}
}

pub struct Bgm {
	pub music_index: usize,
}

impl Asset for Bgm {
	type Id = String;
}

impl<'a> AssetLoader<'a, Bgm> for SdlLoader {
	type Storage = SdlStorage<'a>;

	fn load(
		&'a self,
		path: &<Bgm as Asset>::Id,
		storage: &mut Self::Storage,
	) -> Result<Bgm, AssetLoadError> {
		self.music_loader
			.load(path, storage)
			.map(|music_index| Bgm {
				music_index: music_index,
			})
	}
}
