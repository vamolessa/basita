use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use super::{ResourceHandle, ResourceLoader, ResourceManager};

pub struct WorldResource {
	pub serialized_world: String,
}

pub type WorldResourceHandle = ResourceHandle<WorldResource>;
pub type WorldResources<'a> = ResourceManager<'a, WorldResource, ()>;

impl<'a> ResourceLoader<'a, WorldResource> for () {
	fn load(&'a self, path: &str) -> Result<WorldResource, String> {
		let contents = read_all_text(path).map_err(|e| String::from(e.description()))?;

		Ok(WorldResource {
			serialized_world: contents,
		})
	}
}

fn read_all_text(path: &str) -> Result<String, io::Error> {
	let mut file = File::open(path)?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;
	Ok(contents)
}
