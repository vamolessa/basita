use std::fmt;
use std::collections::HashMap;
use std::marker::PhantomData;

pub struct ResourceHandle<R> {
	index: usize,
	_phantom: PhantomData<R>,
}

impl<R> Clone for ResourceHandle<R> {
	fn clone(&self) -> Self {
		ResourceHandle {
			index: self.index,
			_phantom: self._phantom,
		}
	}
}

impl<R> Copy for ResourceHandle<R> {}

impl<R> Default for ResourceHandle<R> {
	fn default() -> Self {
		ResourceHandle {
			index: Default::default(),
			_phantom: PhantomData,
		}
	}
}

impl<R> fmt::Debug for ResourceHandle<R> {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!(formatter, "ResourceHandle [{}]", self.index)
	}
}

pub struct ResourceManager<'a, R, L>
where
	L: 'a + ResourceLoader<'a, R>,
{
	loader: &'a L,
	path_map: HashMap<String, ResourceHandle<R>>,
	resources: Vec<R>,
}

impl<'a, R, L> ResourceManager<'a, R, L>
where
	L: 'a + ResourceLoader<'a, R>,
{
	pub fn new(loader: &'a L) -> Self {
		ResourceManager {
			loader: loader,
			path_map: HashMap::new(),
			resources: Vec::new(),
		}
	}

	pub fn load(&mut self, path: &String) -> ResourceHandle<R>
	where
		L: 'a + ResourceLoader<'a, R>,
	{
		match self.try_load(path) {
			Ok(handle) => handle,
			Err(message) => panic!(
				"Could not load resource at '{}'. Error: '{}'",
				path, message
			),
		}
	}

	pub fn try_load(&mut self, path: &String) -> Result<ResourceHandle<R>, String>
	where
		L: 'a + ResourceLoader<'a, R>,
	{
		match self.path_map.get(path).cloned() {
			Some(handle) => Ok(handle),
			None => {
				let resource = self.loader.load(path)?;
				self.resources.push(resource);

				let handle = ResourceHandle {
					index: self.resources.len() - 1,
					_phantom: PhantomData,
				};

				self.path_map.insert(path.clone(), handle);

				Ok(handle)
			}
		}
	}

	pub fn get(&self, handle: ResourceHandle<R>) -> &R {
		&self.resources[handle.index]
	}
}

pub trait ResourceLoader<'a, R> {
	fn load(&'a self, path: &str) -> Result<R, String>;
}
