use systems::System;

pub struct Test {}

pub trait Signal {
	fn on_signal(&mut self, arg: &Test);
}

pub struct Slot {
	//listeners: Vec<RefCell<Box<Signal>>>,
	//listener:
}
