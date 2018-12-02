use std::boxed::FnBox;
use std::mem;

use sdl::{SdlLoader, SdlStorage};
use specs::World;

#[derive(Default)]
pub struct LazyEvaluations {
	pub evaluations: Vec<Box<for<'a> FnBox(&mut World, &SdlLoader, &mut SdlStorage<'a>)>>,
}

impl LazyEvaluations {
	pub fn evaluate<'a>(
		world: &mut World,
		sdl_loader: &SdlLoader,
		sdl_storage: &mut SdlStorage<'a>,
	) {
		let evaluations = mem::replace(&mut world.write_resource::<Self>().evaluations, Vec::new());
		for evaluation in evaluations {
			evaluation.call_box((world, sdl_loader, sdl_storage));
		}
	}

	pub fn add(
		&mut self,
		evaluation: Box<for<'a> FnBox(&mut World, &SdlLoader, &mut SdlStorage<'a>)>,
	) {
		self.evaluations.push(evaluation);
	}
}

unsafe impl Send for LazyEvaluations {}
unsafe impl Sync for LazyEvaluations {}

#[derive(Default)]
pub struct Time {
	pub delta_time: f32,
}
