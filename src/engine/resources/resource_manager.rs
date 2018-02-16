use std::collections::HashMap;

#[derive(Copy, Clone)]
pub struct ResourceHandle {
	index: usize,
}

pub struct ResourceManager<'a, R, L>
where
	L: 'a + ResourceLoader<'a, R>,
{
	loader: &'a L,
	path_map: HashMap<String, ResourceHandle>,
	resources: Vec<R>,
}

impl<'a, R, L> ResourceManager<'a, R, L>
where
	L: ResourceLoader<'a, R>,
{
	pub fn new(loader: &'a L) -> Self {
		ResourceManager {
			loader: loader,
			path_map: HashMap::new(),
			resources: Vec::new(),
		}
	}

	pub fn load(&mut self, path: &String) -> Result<ResourceHandle, String>
	where
		L: ResourceLoader<'a, R>,
	{
		match self.path_map.get(path).cloned() {
			Some(handle) => Ok(handle),
			None => {
				let resource = self.loader.load(path)?;
				self.resources.push(resource);

				let handle = ResourceHandle {
					index: self.resources.len() - 1,
				};

				self.path_map.insert(path.clone(), handle);

				Ok(handle)
			}
		}
	}

	pub fn get(&self, handle: ResourceHandle) -> &R {
		&self.resources[handle.index]
	}
}

pub trait ResourceLoader<'l, R> {
	fn load(&'l self, path: &str) -> Result<R, String>;
}
