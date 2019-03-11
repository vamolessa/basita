use std::mem;
use std::thread;
use std::time::{Duration, Instant};

use crate::game::GameContext;

type LazyEvaluation<G> = Box<for<'a> Fn(&mut GameContext<'a>, &mut G)>;

pub struct LazyEvaluations<G> {
	pub evaluations: Vec<LazyEvaluation<G>>,
	pub evaluations_backbuffer: Vec<LazyEvaluation<G>>,
}

impl<G> LazyEvaluations<G> {
	pub fn evaluate<'a, 'b>(&mut self, context: &mut GameContext<'a>, game: &mut G) {
		let mut evaluations;
		{
			let evaluations_backbuffer = mem::replace(&mut self.evaluations_backbuffer, Vec::new());
			evaluations = mem::replace(&mut self.evaluations, evaluations_backbuffer);
		}

		for evaluation in &evaluations {
			(*evaluation)(context, game);
		}

		{
			mem::replace(&mut self.evaluations_backbuffer, evaluations);
		}
	}

	pub fn add<F>(&mut self, evaluation: F)
	where
		F: 'static + for<'a> Fn(&mut GameContext<'a>, &mut G),
	{
		self.evaluations.push(Box::new(evaluation));
	}
}

impl<G> Default for LazyEvaluations<G> {
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
