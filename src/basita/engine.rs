use std::rc::Rc;
use std::cell::RefCell;

use SdlContext;
use input::Input;

use resources::*;
use components::*;
use systems::*;
use events::*;

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

pub struct EngineSystems {
	pub render_systems: Rc<RenderSystem>,
	pub collider_render_system: Rc<ColliderRenderSystem>,
	pub sdl_presenter_system: Rc<SdlPresenterSystem>,
	pub sdl_event_system: Rc<SdlEventSystem>,
}

impl EngineSystems {
	pub fn new() -> Self {
		EngineSystems {
			render_systems: Rc::new(RenderSystem {}),
			collider_render_system: Rc::new(ColliderRenderSystem {}),
			sdl_presenter_system: Rc::new(SdlPresenterSystem {}),
			sdl_event_system: Rc::new(SdlEventSystem {}),
		}
	}
}

impl System for EngineSystems {
	fn init(&self, state: &mut EngineState) {
		self.render_systems.init(state);
		self.collider_render_system.init(state);
		self.sdl_presenter_system.init(state);
		self.sdl_event_system.init(state);
	}

	fn update(&self, state: &mut EngineState) {
		self.render_systems.update(state);
		self.collider_render_system.update(state);
		self.sdl_presenter_system.update(state);
		self.sdl_event_system.update(state);
	}
}

/*
pub fn add_default_systems(systems: &mut SystemCollection) {
	systems.add(RenderSystem {});
	systems.add(ColliderRenderSystem {});
	systems.add(SdlPresenterSystem {});
	systems.add(SdlEventSystem {});
}
*/
