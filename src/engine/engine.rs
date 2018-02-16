use sdl_context::SdlContext;
use input::Input;
use resources::ImageResources;
use systems::{ColliderRenderSystem, RenderSystem};
use components::*;

pub struct Engine<'a> {
	// core
	pub sdl_context: &'a SdlContext,
	pub input: Input,

	// resources
	pub image_resources: ImageResources<'a>,

	// systems
	pub render_system: RenderSystem,
	pub collider_render_system: ColliderRenderSystem,

	// components
	pub box_colliders: ComponentCollection<BoxCollider>,
	pub sprites: ComponentCollection<Sprite>,
	pub transforms: ComponentCollection<Transform>,
}

impl<'a> Engine<'a> {
	pub fn new(sdl_context: &'a SdlContext) -> Self {
		Engine {
			// core
			sdl_context: sdl_context,
			input: Input::new(),

			// resources
			image_resources: ImageResources::new(sdl_context),

			// systems
			render_system: RenderSystem {},
			collider_render_system: ColliderRenderSystem {},

			// components
			box_colliders: ComponentCollection::new(),
			sprites: ComponentCollection::new(),
			transforms: ComponentCollection::new(),
		}
	}
}

