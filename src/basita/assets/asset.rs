use std::error::Error;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::fmt;
use std::any::TypeId;

pub trait Asset {}

#[derive(Serialize, Deserialize)]
pub struct AssetHandle<T: Asset> {
	index: usize,
	_phantom: PhantomData<T>,
}

impl<T: Asset> AssetHandle<T> {
	fn new(index: usize) -> Self {
		AssetHandle {
			index: index,
			_phantom: PhantomData,
		}
	}
}

impl<T: Asset> Default for AssetHandle<T> {
	fn default() -> Self {
		AssetHandle::new(0)
	}
}

impl<T: Asset> Clone for AssetHandle<T> {
	fn clone(&self) -> Self {
		AssetHandle::new(self.index)
	}
}

impl<T: Asset> Copy for AssetHandle<T> {}

impl<T: Asset> fmt::Debug for AssetHandle<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "AssetHandle [{}]", self.index )
	}
}

unsafe impl<T:Asset> Send for AssetHandle<T> {}
unsafe impl<T:Asset> Sync for AssetHandle<T> {}

pub trait AssetLoader<'a, T> {
	fn load(&'a self, path: &str) -> Result<T, AssetLoadError>;
}

pub struct AssetLoadError {
	message: String,
}

impl AssetLoadError {
	pub fn new(message: String) -> Self {
		AssetLoadError { message: message }
	}
}

impl Error for AssetLoadError {
	fn description(&self) -> &str {
		&self.message[..]
	}
}

impl fmt::Debug for AssetLoadError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.message.fmt(f)
	}
}

impl fmt::Display for AssetLoadError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.message.fmt(f)
	}
}

pub struct AssetCollection<T: Asset> {
	path_map: HashMap<String, AssetHandle<T>>,
	assets: Vec<T>,
}

impl<T: Asset> AssetCollection<T> {
	pub fn load<'a>(&mut self, path: &String, loader: &'a AssetLoader<'a, T>) -> AssetHandle<T> {
		match self.try_load(path, loader) {
			Ok(handle) => handle,
			Err(error) => panic!(
				"Could not load resource at '{}'. Error: '{}'",
				path,
				error.description()
			),
		}
	}

	pub fn try_load<'a>(
		&mut self,
		path: &String,
		loader: &'a AssetLoader<'a, T>,
	) -> Result<AssetHandle<T>, AssetLoadError> {
		match self.path_map.get(path).cloned() {
			Some(handle) => Ok(handle),
			None => {
				let handle = AssetHandle {
					index: self.assets.len(),
					_phantom: PhantomData,
				};

				self.path_map.insert(path.clone(), handle);

				let asset = loader.load(path)?;
				self.assets.push(asset);

				Ok(handle)
			}
		}
	}

	pub fn get(&self, handle: AssetHandle<T>) -> &T {
		&self.assets[handle.index]
	}

	pub fn get_mut(&mut self, handle: AssetHandle<T>) -> &mut T {
		&mut self.assets[handle.index]
	}
}

impl<T: Asset> Default for AssetCollection<T> {
	fn default() -> Self {
		AssetCollection {
			path_map: HashMap::default(),
			assets: Vec::default(),
		}
	}
}

