use std::iter::{FilterMap,Iterator};
use std::slice::Iter;

#[derive(Serialize, Deserialize)]
pub struct SparseStorage<T> {
	elements: Vec<Option<T>>,
}

impl<T> SparseStorage<T> {
	pub fn add(&mut self, element: T) -> usize {
		let index = self.elements.len();
		self.elements.push(Some(element));
		index
	}

	/*
	// https://github.com/rust-lang/rust/issues/41758
	pub fn require_capacity(&mut self, capacity: usize) {
		if capacity > self.elements.len() {
			self.elements.resize_default(capacity);
		}
	}
	*/

	pub fn get(&self, index: usize) -> Option<&T> {
		self.elements[index].as_ref()
	}

	pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
		self.elements[index].as_mut()
	}

	pub fn remove(&mut self, index: usize) {
		self.elements[index] = None;
	}

	pub fn iter(&self) -> FilterMap<Iter<Option<T>>, T> {
		self.elements.iter().filter_map(|e:Option<T>| e )
	}
}

impl<T> Default for SparseStorage<T> {
	fn default() -> Self {
		SparseStorage {
			elements: Vec::default(),
		}
	}
}
