/*
use std::error::Error;

use super::{AssetHandle, AssetLoader, AssetManager};
use super::file_helper;

pub struct WorldAsset {
	pub serialized_world: String,
}

pub type WorldAssetHandle = AssetHandle<WorldAsset>;
pub type WorldAssets<'a> = AssetManager<'a, WorldAsset, ()>;

impl<'a> AssetLoader<'a, WorldAsset> for () {
	fn load(&'a self, path: &str) -> Result<WorldAsset, String> {
		let contents = file_helper::read_all_text(path).map_err(|e| String::from(e.description()))?;

		Ok(WorldAsset {
			serialized_world: contents,
		})
	}
}
*/