use std::mem;

use sdl::{SdlLoader, SdlStorage};
use specs::World;

#[derive(Default)]
pub struct LazyEvaluations {
	pub evaluations: Vec<Box<for<'a, 'b> Fn(&mut World, &'a SdlLoader, &'b mut SdlStorage<'a>)>>,
}

impl LazyEvaluations {
	pub fn evaluate<'a, 'b>(
		world: &mut World,
		sdl_loader: &'a SdlLoader,
		sdl_storage: &'b mut SdlStorage<'a>,
	) {
		let evaluations = mem::replace(&mut world.write_resource::<Self>().evaluations, Vec::new());
		for evaluation in &evaluations {
			(*evaluation)(world, sdl_loader, sdl_storage);
		}
	}

	pub fn add<F>(&mut self, evaluation: F)
	where
		F: 'static + for<'a, 'b> Fn(&mut World, &'a SdlLoader, &'b mut SdlStorage<'a>),
	{
		self.evaluations.push(Box::new(evaluation));
	}
}

unsafe impl Send for LazyEvaluations {}
unsafe impl Sync for LazyEvaluations {}

#[derive(Default)]
pub struct Time {
	pub delta_time: f32,
}
