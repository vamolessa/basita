use sdl2::mixer::Music;

use super::{SdlAssetStorage, SdlContext, SdlStorage};
use crate::core::assets::AssetLoadError;

pub type MusicStorage<'a> = SdlAssetStorage<Music<'a>>;

pub struct MusicLoader {}

impl MusicLoader {
	pub fn new(_sdl_context: &SdlContext) -> Self {
		MusicLoader {}
	}

	pub fn load<'a>(
		&'a self,
		path: &str,
		storage: &mut SdlStorage<'a>,
	) -> Result<usize, AssetLoadError> {
		match sdl2::mixer::Music::from_file(path) {
			Ok(chunk) => Ok(storage.music_storage.add(chunk)),
			Err(message) => Err(AssetLoadError::new(message)),
		}
	}
}
