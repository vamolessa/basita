use std::collections::BTreeSet;

pub trait Component: Ord {}

pub struct ComponentCollection<T>
where
	T: Component,
{
	pub all: BTreeSet<T>,
}

impl<T> ComponentCollection<T>
where
	T: Component,
{
	pub fn new() -> ComponentCollection<T> {
		ComponentCollection {
			all: BTreeSet::new(),
		}
	}

	pub fn add(&mut self, component: T) {
		self.all.insert(component);
	}
}
