use std::iter::Iterator;

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

	pub fn iter(&self) -> SparseStorageIter<T> {
		SparseStorageIter {
			storage: self,
			index: 0
		}
	}
/*
	pub fn iter_mut(&mut self) -> SparseStorageIterMut<T> {
		SparseStorageIterMut {
			storage: self,
			index: 0
		}
	}
	*/
}

impl<T> Default for SparseStorage<T> {
	fn default() -> Self {
		SparseStorage {
			elements: Vec::default(),
		}
	}
}

pub struct SparseStorageIter<'a, T: 'a> {
	storage: &'a SparseStorage<T>,
	index : usize
}

impl<'a, T: 'a> Iterator for SparseStorageIter<'a, T> {
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item> {
		let len = self.storage.elements.len();

		while self.index < len {
			let i = self.index;
			self.index += 1;

			let e = self.storage.get(i);
			if e.is_some() {
				return e;
			}
		}

		None
	}
}

pub struct SparseStorageIterMut<'a, T: 'a> {
	vec: &'a mut Vec<Option<T>>,
	index : usize
}

impl<'a, T: 'a> Iterator for SparseStorageIterMut<'a, T> {
	type Item = &'a mut T;

	fn next(&mut self) -> Option<Self::Item> {
		let len = self.vec.len();

		while self.index < len {
			let i = self.index;
			self.index += 1;

			let e = self.vec[i].as_mut();
			if e.is_some() {
				return e;
			}
		}

		None
	}
}