use std::mem;

use sdl::{SdlLoader, SdlStorage};
use specs::World;

type LazyEvaluation = Box<for<'a, 'b> Fn(&mut World, &'a SdlLoader, &'b mut SdlStorage<'a>)>;

#[derive(Default)]
pub struct LazyEvaluations {
	pub evaluations: Vec<LazyEvaluation>,
	pub evaluations_backbuffer: Vec<LazyEvaluation>,
}

impl LazyEvaluations {
	pub fn evaluate<'a, 'b>(
		world: &mut World,
		sdl_loader: &'a SdlLoader,
		sdl_storage: &'b mut SdlStorage<'a>,
	) {
		let mut evaluations;
		{
			let mut this = world.write_resource::<Self>();
			let evaluations_backbuffer = mem::replace(&mut this.evaluations_backbuffer, Vec::new());
			evaluations = mem::replace(&mut this.evaluations, evaluations_backbuffer);
		}

		for evaluation in &evaluations {
			(*evaluation)(world, sdl_loader, sdl_storage);
		}
		evaluations.clear();

		{
			let mut this = world.write_resource::<Self>();
			mem::replace(&mut this.evaluations_backbuffer, evaluations);
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
