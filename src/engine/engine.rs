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
	pub render_system: RendererSystem,
	pub collider_render_system: ColliderRendererSystem,

	// components
	pub box_colliders: ComponentCollection<BoxCollider>,
	pub sprites: ComponentCollection<Sprite>,
	pub transforms: ComponentCollection<Transform>,
}

impl Engine {
	pub fn new(sdl_context: SdlContext) -> Self {
		Engine {
			// core
			sdl_context: sdl_context,
			input: Input::new(),

			// systems
			render_system: RendererSystem {},
			collider_render_system: ColliderRendererSystem {},

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
