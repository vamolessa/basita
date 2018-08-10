use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::hash::Hash;
use std::marker::PhantomData;

pub trait Asset {
	type Id: fmt::Debug + Hash + Eq + Clone;
}

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

	pub fn is_valid(&self) -> bool {
		self.index != usize::max_value()
	}
}

impl<T: Asset> Default for AssetHandle<T> {
	fn default() -> Self {
		AssetHandle::new(usize::max_value())
	}
}

impl<T: Asset> Clone for AssetHandle<T> {
	fn clone(&self) -> Self {
		AssetHandle::new(self.index)
	}
}

impl<T: Asset> Copy for AssetHandle<T> {}

impl<T: Asset> PartialEq for AssetHandle<T> {
	fn eq(&self, other: &Self) -> bool {
		self.index == other.index
	}
}

impl<T: Asset> Eq for AssetHandle<T> {}

impl<T: Asset> fmt::Debug for AssetHandle<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "AssetHandle [{}]", self.index)
	}
}

unsafe impl<T: Asset> Send for AssetHandle<T> {}
unsafe impl<T: Asset> Sync for AssetHandle<T> {}

pub trait AssetLoader<'a, T: Asset> {
	type Storage;
	fn load(&'a self, id: &T::Id, storage: &mut Self::Storage) -> Result<T, AssetLoadError>;
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

pub struct AssetCollection<A: Asset> {
	cache_map: HashMap<A::Id, AssetHandle<A>>,
	assets: Vec<A>,
}

impl<A: Asset> AssetCollection<A> {
	pub fn load<'a, S>(
		&mut self,
		id: &A::Id,
		loader: &'a AssetLoader<'a, A, Storage = S>,
		storage: &mut S,
	) -> AssetHandle<A> {
		match self.try_load(id, loader, storage) {
			Ok(handle) => handle,
			Err(error) => panic!(
				"Could not load resource '{:?}'. Error: '{}'",
				id,
				error.description()
			),
		}
	}

	pub fn try_load<'a, S>(
		&mut self,
		id: &A::Id,
		loader: &'a AssetLoader<'a, A, Storage = S>,
		storage: &mut S,
	) -> Result<AssetHandle<A>, AssetLoadError> {
		match self.cache_map.get(id).cloned() {
			Some(handle) => Ok(handle),
			None => {
				let asset = loader.load(id, storage)?;
				let handle = self.add(asset);
				self.cache_map.insert(id.clone(), handle);

				Ok(handle)
			}
		}
	}

	pub fn add(&mut self, asset: A) -> AssetHandle<A> {
		let handle = AssetHandle {
			index: self.assets.len(),
			_phantom: PhantomData,
		};

		self.assets.push(asset);

		handle
	}

	pub fn get(&self, handle: AssetHandle<A>) -> &A {
		assert!(handle.is_valid());
		&self.assets[handle.index]
	}

	pub fn get_mut(&mut self, handle: AssetHandle<A>) -> &mut A {
		assert!(handle.is_valid());
		&mut self.assets[handle.index]
	}
}

impl<T: Asset> Default for AssetCollection<T> {
	fn default() -> Self {
		AssetCollection {
			cache_map: HashMap::default(),
			assets: Vec::default(),
		}
	}
}
