pub trait Component {}

#[derive(Serialize, Deserialize)]
pub struct ComponentHandle<T: Component> {
	index: usize,
	_phantom: ::std::marker::PhantomData<T>,
}

impl<T: Component> ComponentHandle<T> {
	fn new(index: usize) -> Self {
		ComponentHandle {
			index: index,
			_phantom: ::std::marker::PhantomData,
		}
	}
}

impl<T: Component> Clone for ComponentHandle<T> {
	fn clone(&self) -> Self {
		ComponentHandle::new(self.index)
	}
}

impl<T: Component> Copy for ComponentHandle<T> {}

impl<T: Component> Default for ComponentHandle<T> {
	fn default() -> Self {
		ComponentHandle {
			index: Default::default(),
			_phantom: ::std::marker::PhantomData,
		}
	}
}

impl<T> ::std::fmt::Debug for ComponentHandle<T> {
	fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		write!(formatter, "{} [{}]", stringify!($name), self.index)
	}
}

pub struct ComponentCollection<T: Component> {
	pub all: Vec<T>,
}

impl<T: Component> ComponentCollection<T> {
	pub fn new() -> Self {
		ComponentCollection { all: Vec::new() }
	}

	pub fn add(&mut self, component: T) -> ComponentHandle<T> {
		self.all.push(component);
		ComponentHandle::new(self.all.len() - 1)
	}

	pub fn get(&self, handle: ComponentHandle<T>) -> &T {
		&self.all[handle.index]
	}

	pub fn get_mut(&mut self, handle: ComponentHandle<T>) -> &mut T {
		&mut self.all[handle.index]
	}

	pub fn get_handle(&self, index: usize) -> ComponentHandle<T> {
		ComponentHandle::new(index)
	}
}
