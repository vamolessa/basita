use std::mem;
use std::thread;
use std::time::{Duration, Instant};

use specs::World;

use crate::sdl::{SdlLoader, SdlStorage};

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

impl Time {
	pub fn sleep_rest_of_frame(&mut self, frames_per_second: u32, frame_start_instant: &Instant) {
		let frame_duration = frame_start_instant.elapsed();
		let max_frame_duration = Duration::new(0, 1_000_000_000u32 / frames_per_second);
		let sleep_time = max_frame_duration
			.checked_sub(frame_duration)
			.unwrap_or(Duration::new(0, 0));

		thread::sleep(sleep_time);

		self.delta_time = 1.0 / frames_per_second as f32;
	}
}
