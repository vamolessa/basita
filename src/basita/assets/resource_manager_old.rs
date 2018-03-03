use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use std::borrow::Borrow;

pub struct ResourceManager<'a, K, R, L>
where
	K: Hash + Eq,
	L: 'a + ResourceLoader<'a, R>,
{
	loader: &'a L,
	cache: HashMap<K, Rc<R>>,
}

impl<'a, K, R, L> ResourceManager<'a, K, R, L>
where
	K: Hash + Eq,
	L: ResourceLoader<'a, R>,
{
	pub fn new(loader: &'a L) -> Self {
		ResourceManager {
			cache: HashMap::new(),
			loader: loader,
		}
	}

	// Generics magic to allow a HashMap to use String as a key
	// while allowing it to use &str for gets
	pub fn load<D>(&mut self, details: &D) -> Result<Rc<R>, String>
	where
		L: ResourceLoader<'a, R, Args = D>,
		D: Eq + Hash + ?Sized,
		K: Borrow<D> + for<'b> From<&'b D>,
	{
		self.cache.get(details).cloned().map_or_else(
			|| {
				let resource = Rc::new(self.loader.load(details)?);
				self.cache.insert(details.into(), resource.clone());
				Ok(resource)
			},
			Ok,
		)
	}
}

pub trait ResourceLoader<'a, R> {
	type Args: ?Sized;
	fn load(&'a self, data: &Self::Args) -> Result<R, String>;
}
