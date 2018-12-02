use sdl2::mixer::Chunk;

use super::{SdlAssetStorage, SdlContext, SdlStorage};
use core::assets::AssetLoadError;

pub type ChunkStorage = SdlAssetStorage<Chunk>;

pub struct ChunkLoader {}

impl ChunkLoader {
	pub fn new(_sdl_context: &SdlContext) -> Self {
		ChunkLoader {}
	}

	pub fn load<'a>(
		&'a self,
		path: &str,
		storage: &mut SdlStorage<'a>,
	) -> Result<usize, AssetLoadError> {
		match sdl2::mixer::Chunk::from_file(path) {
			Ok(chunk) => Ok(storage.chunk_storage.add(chunk)),
			Err(message) => Err(AssetLoadError::new(message)),
		}
	}
}
