use SdlContext;
use input::Input;

use resources::*;
use components::*;
use systems::*;
use events::*;

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

#[derive(Default)]
pub struct EngineEvents<'a> {
	pub some_event: Event<EngineState<'a>, EngineEvents<'a>, String>,
}

impl<'a> SystemCollection<EngineState<'a>, EngineEvents<'a>> {
	pub fn add_default_systems(&mut self) {
		self.add_system(None, render_system::update);
		self.add_system(None, collider_render_system::update);
		self.add_system(None, sdl_presenter_system::update);
		self.add_system(None, sdl_event_system::update);
	}
}

/*
pub fn play<'a>(mut state: EngineState<'a>, systems: Rc<SystemCollection<'a>>) {
	for system in &systems.all {
		system.init(&mut state);
	}

	while state.running {
		for system in &systems.all {
			system.update(&mut state);
		}
	}
}
*/
