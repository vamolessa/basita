use SdlContext;
use input::Input;

use resources::*;
use components::*;
use systems::*;

pub trait ContainsEngineState<'a> {
	fn get_engine_state_mut(&mut self) -> &mut EngineState<'a>;
}

pub trait ContainsEngineEvents<S, E>
where
	E: ContainsEngineEvents<S, E>,
{
	fn get_engine_events(&self) -> &EngineEvents<S, E>;
	fn get_engine_events_mut(&mut self) -> &mut EngineEvents<S, E>;
}

pub struct EngineState<'a> {
	// core
	pub delta_time: f32,
	pub running: bool,
	pub sdl_context: &'a SdlContext,
	pub input: Input,
	pub systems_state: SystemsState,

	// resources
	pub image_resources: ImageResources<'a>,

	// components
	pub colliders: ComponentCollection<Collider>,
	pub sprites: ComponentCollection<Sprite<'a>>,
	pub transforms: ComponentCollection<Transform>,
	pub physic_bodies: ComponentCollection<PhysicBody>,
}

impl<'a> EngineState<'a> {
	pub fn new(sdl_context: &'a SdlContext) -> Self {
		EngineState {
			// core
			delta_time: 0.0,
			running: true,
			sdl_context: sdl_context,
			input: Input::new(),
			systems_state: SystemsState::default(),

			// resources
			image_resources: ImageResources::new(sdl_context),

			// components
			colliders: ComponentCollection::new(),
			sprites: ComponentCollection::new(),
			transforms: ComponentCollection::new(),
			physic_bodies: ComponentCollection::new(),
		}
	}
}

#[derive(Default)]
pub struct SystemsState {}

pub struct EngineEvents<S, E>
where
	E: ContainsEngineEvents<S, E>,
{
	pub collision: CollisionEvents<S, E>,
}

impl<S, E> EngineEvents<S, E>
where
	E: ContainsEngineEvents<S, E>,
{
	pub fn new() -> Self {
		EngineEvents {
			collision: CollisionEvents::new(),
		}
	}
}

impl<'a, S, E> SystemCollection<S, E>
where
	S: ContainsEngineState<'a>,
	E: ContainsEngineEvents<S, E>,
{
	pub fn add_default_systems(&mut self) {
		self.add_system::<PhysicsSystem>();
		self.add_system::<CollisionSystem>();
		self.add_system::<RenderSystem>();
		self.add_system::<ColliderRenderSystem>();
		self.add_system::<SdlPresenterSystem>();
		// frame start
		self.add_system::<SdlEventSystem>();
	}
}

pub fn play<'a, S, E>(mut state: S, mut events: E, systems: SystemCollection<S, E>)
where
	S: ContainsEngineState<'a>,
	E: ContainsEngineEvents<S, E>,
{
	systems.init(&mut state, &mut events);

	while state.get_engine_state_mut().running {
		systems.update(&mut state, &events);
	}
}
