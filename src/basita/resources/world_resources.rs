use std::error::Error;

use super::{ResourceHandle, ResourceLoader, ResourceManager};
use super::file_helper;

pub struct WorldResource {
	pub serialized_world: String,
}

pub type WorldResourceHandle = ResourceHandle<WorldResource>;
pub type WorldResources<'a> = ResourceManager<'a, WorldResource, ()>;

impl<'a> ResourceLoader<'a, WorldResource> for () {
	fn load(&'a self, path: &str) -> Result<WorldResource, String> {
		let contents = file_helper::read_all_text(path).map_err(|e| String::from(e.description()))?;

		Ok(WorldResource {
			serialized_world: contents,
		})
	}
}
