use std::mem;
use std::thread;
use std::time::{Duration, Instant};

use crate::game::GameContext;

type LazyEvaluation<T> = Box<for<'a> Fn(&mut GameContext<'a>, &mut T)>;

pub struct LazyEvaluations<T> {
	pub evaluations: Vec<LazyEvaluation<T>>,
	pub evaluations_backbuffer: Vec<LazyEvaluation<T>>,
}

impl<T> LazyEvaluations<T> {
	pub fn evaluate<'a, 'b>(&mut self, context: &mut GameContext<'a>, arg: &mut T) {
		let mut evaluations;
		{
			let evaluations_backbuffer = mem::replace(&mut self.evaluations_backbuffer, Vec::new());
			evaluations = mem::replace(&mut self.evaluations, evaluations_backbuffer);
		}

		for evaluation in &evaluations {
			(*evaluation)(context, arg);
		}

		{
			mem::replace(&mut self.evaluations_backbuffer, evaluations);
		}
	}

	pub fn add(&mut self, _evaluation: for<'a> fn(&mut GameContext<'a>, &mut T)) {}

	pub fn add2<F>(&mut self, evaluation: F)
	where
		F: 'static + for<'a> Fn(&mut GameContext<'a>, &mut T),
	{
		self.evaluations.push(Box::new(evaluation));
	}
}

impl<T> Default for LazyEvaluations<T> {
	fn default() -> Self {
		LazyEvaluations {
			evaluations: Vec::default(),
			evaluations_backbuffer: Vec::default(),
		}
	}
}

// Maybe implement this?
// https://gafferongames.com/post/fix_your_timestep/
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
