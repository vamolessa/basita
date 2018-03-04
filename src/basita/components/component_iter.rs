use std::iter::Iterator;

use super::{Component, ComponentCollection};

/*
pub trait ComponentJoin {
	type Type;
	type Collection;

	fn get(collections: &Self::Collection, entity_id: usize) -> Self::Type;
}

pub struct ComponentIter<T: ComponentJoin> {
	entity_id: usize,
	entity_count: usize,
	collections: T::Collection,
}

impl<T: ComponentJoin> ComponentIter<T> {
	pub fn new(entity_count: usize, collections: T::Collection) -> Self {
		ComponentIter {
			entity_id: 0,
			entity_count: entity_count,
			collections: collections,
		}
	}
}

impl<T: ComponentJoin> Iterator for ComponentIter<T> {
	type Item = T::Type;

	fn next(&mut self) -> Option<T::Type> {
		if self.entity_id < self.entity_count {
			let id = self.entity_id;
			self.entity_id += 1;

			Some(T::get(&self.collections, id))
		} else {
			None
		}
	}
}

impl<T> ComponentJoin for ComponentCollection<T> {
	type Type = T;
	type Collection = ComponentCollection<T>;

	fn get(collections: &Self::Collection, entity_id: usize) -> Self::Type {

	}
}
*/

pub struct ComponentIter<'a, T: 'a + Component> {
	collections: (&'a ComponentCollection<T>,),
}
