use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::hash::Hash;
use std::marker::PhantomData;

pub trait Asset: Sync + Send + 'static {
	type Id: fmt::Debug + Hash + Eq + Clone + Sync + Send;
}

#[derive(Serialize, Deserialize)]
pub struct AssetHandle<A: Asset> {
	index: usize,
	_phantom: PhantomData<A>,
}

impl<A: Asset> AssetHandle<A> {
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

impl<A: Asset> Default for AssetHandle<A> {
	fn default() -> Self {
		AssetHandle::new(usize::max_value())
	}
}

impl<A: Asset> Clone for AssetHandle<A> {
	fn clone(&self) -> Self {
		AssetHandle::new(self.index)
	}
}

impl<A: Asset> Copy for AssetHandle<A> {}

impl<A: Asset> PartialEq for AssetHandle<A> {
	fn eq(&self, other: &Self) -> bool {
		self.index == other.index
	}
}

impl<A: Asset> Eq for AssetHandle<A> {}

impl<A: Asset> fmt::Debug for AssetHandle<A> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "AssetHandle [{}]", self.index)
	}
}

unsafe impl<A: Asset> Send for AssetHandle<A> {}
unsafe impl<A: Asset> Sync for AssetHandle<A> {}

pub trait AssetLoader<'a, A: Asset> {
	type Storage;
	fn load(&'a self, id: &A::Id, storage: &mut Self::Storage) -> Result<A, AssetLoadError>;
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

	pub fn asset_iter<'a>(&'a self) -> impl Iterator<Item = &'a A> {
		self.assets.iter()
	}

	pub fn asset_iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut A> {
		self.assets.iter_mut()
	}
}

impl<A: Asset> Default for AssetCollection<A> {
	fn default() -> Self {
		AssetCollection {
			cache_map: HashMap::default(),
			assets: Vec::default(),
		}
	}
}
