use World;

pub struct Event<W, D>
where
	W: World,
{
	callbacks: Vec<fn(&mut W, &D) -> ()>,
}

impl<W, D> Event<W, D>
where
	W: World,
{
	pub fn subscribe(&mut self, callback: fn(&mut W, &D) -> ()) {
		self.callbacks.push(callback);
	}

	pub fn raise(&self, world: &mut W, data: &D) {
		for callback in &self.callbacks {
			callback(world, data);
		}
	}
}

impl<W, D> Default for Event<W, D>
where
	W: World,
{
	fn default() -> Self {
		Event {
			callbacks: Vec::new(),
		}
	}
}
