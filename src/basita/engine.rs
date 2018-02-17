use SdlContext;
use input::Input;

use resources::*;
use components::*;
use systems::*;

pub struct Engine<'a> {
	pub state: EngineState<'a>,
	pub systems: EngineSystems<'a>,
}

impl<'a> Engine<'a> {
	pub fn new(sdl_context: &'a SdlContext) -> Self {
		Engine {
			state: EngineState::new(sdl_context),
			systems: EngineSystems::new(),
		}
	}

	pub fn play(&mut self) {
		for system in &self.systems.all {
			system.init(&mut self.state);
		}

		while self.state.running {
			for system in &self.systems.all {
				system.update(&mut self.state);
			}
		}
	}
}

pub struct EngineState<'a> {
	// core
	pub running: bool,
	pub sdl_context: &'a SdlContext,
	pub input: Input,

	// resources
	pub image_resources: ImageResources<'a>,

	// components
	pub box_colliders: ComponentCollection<BoxCollider>,
	pub sprites: ComponentCollection<Sprite<'a>>,
	pub transforms: ComponentCollection<Transform>,
}

impl<'a> EngineState<'a> {
	pub fn new(sdl_context: &'a SdlContext) -> Self {
		EngineState {
			// core
			running: true,
			sdl_context: sdl_context,
			input: Input::new(),

			// resources
			image_resources: ImageResources::new(sdl_context),

			// components
			box_colliders: ComponentCollection::new(),
			sprites: ComponentCollection::new(),
			transforms: ComponentCollection::new(),
		}
	}
}

pub struct EngineSystems<'a> {
	all: Vec<Box<System + 'a>>,
}

impl<'a> EngineSystems<'a> {
	pub fn new() -> Self {
		EngineSystems { all: Vec::new() }
	}

	pub fn add<T>(&mut self, system: T)
	where
		T: 'a + System,
	{
		self.all.push(Box::new(system));
	}

	pub fn add_defaults(&mut self) {
		self.add(RenderSystem {});
		self.add(ColliderRenderSystem {});
		self.add(SdlPresenterSystem {});
		self.add(SdlEventSystem {});
	}
}
