use std::collections::HashMap;
use std::marker::PhantomData;
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct ResourceHandle<T> {
	index: usize,
	_phantom: PhantomData<T>,
}

impl<T> ResourceHandle<T> {
	fn new(index: usize) -> Self {
		ResourceHandle {
			index: index,
			_phantom: PhantomData,
		}
	}
}

impl<T> Default for ResourceHandle<T> {
	fn default() -> Self {
		ResourceHandle::new(0)
	}
}

impl<T> Clone for ResourceHandle<T> {
	fn clone(&self) -> Self {
		ResourceHandle::new(self.index)
	}
}

impl<T> Copy for ResourceHandle<T> {}

impl<T> fmt::Debug for ResourceHandle<T> {
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
		let mut resources = Vec::new();

		resources.push(match loader.create_nil_resource() {
			Ok(resource) => resource,
			Err(error) => panic!("Could not create nil resource. Error: '{}'", error),
		});

		ResourceManager {
			loader: loader,
			path_map: HashMap::new(),
			resources: resources,
		}
	}

	pub fn load(&mut self, path: &String) -> ResourceHandle<R>
	where
		L: 'a + ResourceLoader<'a, R>,
	{
		match self.try_load(path) {
			Ok(handle) => handle,
			Err(error) => panic!("Could not load resource at '{}'. Error: '{}'", path, error),
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

	fn create_nil_resource(&'a self) -> Result<R, String>;
}
