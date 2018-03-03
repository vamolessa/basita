use std::error::Error;

use super::{Asset, AssetLoader, AssetLoadError};
use super::file_helper;

pub struct WorldAsset {
	pub serialized_world: String,
}

impl Asset for WorldAsset {}

impl<'a> AssetLoader<'a, WorldAsset> for () {
	fn load(&'a self, path: &str) -> Result<WorldAsset, AssetLoadError> {
		let contents = file_helper::read_all_text(path).map_err(|e| AssetLoadError::new(String::from(e.description())))?;

		Ok(WorldAsset {
			serialized_world: contents,
		})
	}
}