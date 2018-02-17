use std::rc::Rc;

use super::super::EngineState;
use super::super::systems::System;

pub struct Event<D> {
	callbacks: Vec<fn(&System, &mut EngineState, &D) -> ()>,
}

impl<D> Event<D> {
	pub fn new() -> Self {
		Event {
			callbacks: Vec::new(),
		}
	}

	pub fn subscribe(&mut self, callback: fn(&System, &mut EngineState, &D) -> ()) {
		self.callbacks.push(callback);
	}

	pub fn raise(&self, state: &mut EngineState, data: &D) {
		/*
		for callback in &self.callbacks {
			callback(state, data);
		}
		*/
	}
}

pub trait Signal<C, D>
where
	C: ?Sized,
	D: ?Sized,
{
	fn handle(&self, data: &D);
}

pub struct Slot<S>
where
	S: ?Sized,
{
	listeners: Vec<Rc<Box<S>>>,
}

impl<S> Slot<S>
where
	S: ?Sized,
{
	pub fn new() -> Self {
		Slot {
			listeners: Vec::new(),
		}
	}

	pub fn subscribe(&mut self, listener: Rc<Box<S>>) {
		self.listeners.push(listener);
	}

	pub fn raise<C, D>(&self, data: &D)
	where
		C: ?Sized,
		S: Signal<C, D>,
	{
		for listener in &self.listeners {
			listener.handle(data);
		}
	}
}

impl<S> Default for Slot<S>
where
	S: ?Sized,
{
	fn default() -> Self {
		Slot::new()
	}
}

#[macro_export]
macro_rules! signal {
	($signal_name:ident, $signal_data:ty) => {
		pub trait $signal_name {
			fn handle(&self, data: &$signal_data);
		}

		impl<T> Signal<$signal_name, $signal_data> for T
		where
			T: $signal_name + ?Sized,
		{
			fn handle(&self, data: &$signal_data) {
				self.handle(data);
			}
		}
	}
}

/* EXAMPLE
signal!(OtherSignal, String);

pub trait MySignal {
	fn handle(&self, data: &String);
}

impl<T> Signal<MySignal, String> for T
where
	T: MySignal + ?Sized,
{
	fn handle(&self, data: &String) {
		self.handle(data);
	}
}

struct TestStruct {
	s: Slot<MySignal>,
}

struct MySystem {}

impl MySignal for MySystem {
	fn handle(&self, _data: &String) {}
}

fn example() {
	let mut a = TestStruct { s: Slot::new() };

	let b = MySystem {};

	a.s.subscribe(Rc::new(b));

	let data = String::from("asasdasd");
	a.s.raise(&data);
}
*/
