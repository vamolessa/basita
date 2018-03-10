use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};

use fxhash::FxHashMap;

use entities::Entities;
use components::{Component, ComponentCollection};
use assets::{Asset, AssetCollection};
use resources::Resource;

pub struct World {
	entities: Entities,
	components: FxHashMap<TypeId, RefCell<Box<Any>>>,
	assets: FxHashMap<TypeId, RefCell<Box<Any>>>,
	resources: FxHashMap<TypeId, RefCell<Box<Any>>>,
}

impl World {
	pub fn new() -> Self {
		World {
			entities: Entities::default(),
			components: FxHashMap::default(),
			assets: FxHashMap::default(),
			resources: FxHashMap::default(),
		}
	}

	pub fn register_component<T: 'static + Component>(&mut self) {
		let component_collection = ComponentCollection::<T>::default();

		self.components.insert(
			TypeId::of::<ComponentCollection<T>>(),
			RefCell::from(Box::from(component_collection)),
		);
	}

	pub fn register_asset<T: 'static + Asset>(&mut self) {
		let asset_collection = AssetCollection::<T>::default();

		self.assets.insert(
			TypeId::of::<AssetCollection<T>>(),
			RefCell::from(Box::from(asset_collection)),
		);
	}

	pub fn register_resource<T: 'static + Resource>(&mut self) {
		self.resources
			.insert(TypeId::of::<T>(), RefCell::from(Box::from(T::default())));
	}

	pub fn entities(&self) -> &Entities {
		&self.entities
	}

	pub fn components<T: 'static + Component>(&self) -> Ref<ComponentCollection<T>> {
		try_get(&self.components)
	}

	pub fn components_mut<T: 'static + Component>(&self) -> RefMut<ComponentCollection<T>> {
		try_get_mut(&self.components)
	}

	pub fn assets<T: 'static + Asset>(&self) -> Ref<AssetCollection<T>> {
		try_get(&self.assets)
	}

	pub fn assets_mut<T: 'static + Asset>(&self) -> RefMut<AssetCollection<T>> {
		try_get_mut(&self.assets)
	}

	pub fn resource<T: 'static + Resource>(&self) -> Ref<T> {
		try_get(&self.resources)
	}

	pub fn resource_mut<T: 'static + Resource>(&self) -> RefMut<T> {
		try_get_mut(&self.resources)
	}
}

fn try_get<T: Any>(map: &FxHashMap<TypeId, RefCell<Box<Any>>>) -> Ref<T> {
	Ref::map(
		match map.get(&TypeId::of::<T>()) {
			Some(e) => e,
			None => panic!("Type not registered {:?}", TypeId::of::<T>()),
		}.borrow(),
		|a| a.downcast_ref::<T>().unwrap(),
	)
}

fn try_get_mut<T: Any>(map: &FxHashMap<TypeId, RefCell<Box<Any>>>) -> RefMut<T> {
	RefMut::map(
		match map.get(&TypeId::of::<T>()) {
			Some(e) => e,
			None => panic!("Type not registered {:?}", TypeId::of::<T>()),
		}.borrow_mut(),
		|a| a.downcast_mut::<T>().unwrap(),
	)
}
