use sdl_context::SdlContext;
use input::Input;
use resources::ImageResources;
use systems::{ColliderRenderSystem, RenderSystem};
use components::*;

pub struct Engine<'a> {
	pub state: EngineState<'a>,
	pub systems: EngineSystems,
}

impl<'a> Engine<'a> {
	pub fn new(sdl_context: &'a SdlContext) -> Self {
		Engine {
			state: EngineState::new(sdl_context),
			systems: EngineSystems::new(),
		}
	}
}

pub struct EngineState<'a> {
	// core
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
	pub render_system: RenderSystem,
	pub collider_render_system: ColliderRenderSystem,
}

impl EngineSystems {
	pub fn new() -> Self {
		EngineSystems {
			render_system: RenderSystem {},
			collider_render_system: ColliderRenderSystem {},
		}
	}
}
