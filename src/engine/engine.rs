use sdl_context::SdlContext;
use input::Input;
use resources::ImageResources;
use systems::{ColliderRendererSystem, RendererSystem};
use components::*;

pub struct Engine<'a> {
	// core
	pub sdl_context: SdlContext,
	pub input: Input,

	// resources
	pub image_resources: ImageResources,

	// systems
	pub renderer_system: RendererSystem,
	pub collider_renderer_system: ColliderRendererSystem,

	// components
	pub box_colliders: ComponentCollection<BoxCollider<'a>>,
	pub sprites: ComponentCollection<Sprite<'a>>,
	pub transforms: ComponentCollection<Transform>,
}

impl<'a> Engine<'a> {
	pub fn new(sdl_context: SdlContext) -> Engine<'a> {
		Engine {
			// resources
			image_resources: ImageResources::new(&sdl_context),

			// systems
			renderer_system: RendererSystem {},
			collider_renderer_system: ColliderRendererSystem {},

			// components
			box_colliders: ComponentCollection::new(),
			sprites: ComponentCollection::new(),
			transforms: ComponentCollection::new(),

			// core
			sdl_context: sdl_context,
			input: Input::new(),
		}
	}
}
