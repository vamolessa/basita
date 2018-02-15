use sdl_context::SdlContext;
use input::Input;
use resources::ImageResources;
use systems::{ColliderRendererSystem, RendererSystem};
use components::*;

pub struct Engine {
	// core
	pub sdl_context: SdlContext,
	pub input: Input,

	// systems
	pub renderer_system: RendererSystem,
	pub collider_renderer_system: ColliderRendererSystem,

	// components
	pub box_colliders: ComponentCollection<BoxCollider>,
	pub sprites: ComponentCollection<Sprite>,
	pub transforms: ComponentCollection<Transform>,
}

impl Engine {
	pub fn new(sdl_context: SdlContext) -> Engine {
		Engine {
			// core
			sdl_context: sdl_context,
			input: Input::new(),

			// systems
			renderer_system: RendererSystem {},
			collider_renderer_system: ColliderRendererSystem {},

			// components
			box_colliders: ComponentCollection::new(),
			sprites: ComponentCollection::new(),
			transforms: ComponentCollection::new(),
		}
	}
}

pub struct EngineResources<'a> {
	pub image_resources: ImageResources<'a>,
}

impl<'a> EngineResources<'a> {
	pub fn new() -> EngineResources<'a> {
		EngineResources {
			image_resources: ImageResources::new(),
		}
	}
}
