use std::any::Any;

use unique_type_id::UniqueTypeId;

use Entities;
use components::{Component, ComponentCollection};
use assets::{Asset, AssetCollection};

pub trait World {
	fn entities(&self) -> &Entities;
	fn entities_mut(&mut self) -> &mut Entities;

	fn assets<T: Asset>(&self) -> &AssetCollection<T>;
	fn assets_mut<T: Asset>(&mut self) -> &mut AssetCollection<T>;

	fn components<T: Component>(&self) -> &ComponentCollection<T>;
	fn components_mut<T: Component>(&mut self) -> &mut ComponentCollection<T>;

	fn resource<T>(&self) -> &T;
	fn resource_mut<T>(&mut self) -> &mut T;
}

pub trait WorldForComponent<T: Component> {
	fn get(&self) -> &ComponentCollection<T>;
	fn get_mut(&mut self) -> &mut ComponentCollection<T>;
}

pub trait WorldForAsset<T: Asset> {
	fn get(&self) -> &AssetCollection<T>;
	fn get_mut(&mut self) -> &mut AssetCollection<T>;
}

pub trait WorldForResource<T> {
	fn get(&self) -> &T;
	fn get_mut(&mut self) -> &mut T;
}

pub struct WorldNew {
	resources: Vec<Box<Any>>,
}

impl WorldNew {
	pub fn new() -> Self {
		WorldNew {
			resources: Vec::new(),
		}
	}

	pub fn register<T: 'static + UniqueTypeId + Default>(&mut self) {
		let index = T::id().0 as usize;

		let resource_count = self.resources.len();
		if index >= resource_count {
			self.resources.reserve(index + 1 - resource_count);
			unsafe { self.resources.set_len(index + 1) };
		}

		let resource = Box::from(T::default());
		self.resources[index] = resource;
	}

	pub fn resource<T: 'static + UniqueTypeId>(&self) -> &T {
		let index = T::id().0 as usize;
		&self.resources[index].downcast_ref().unwrap()
	}
}
