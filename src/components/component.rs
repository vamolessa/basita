use std::collections::BTreeSet;

pub trait Component {}

pub struct ComponentCollection<T>
where
	T: Component,
{
	pub all: BTreeSet<T>,
}

impl<T> ComponentCollection<T>
where
	T: Component + Ord,
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
