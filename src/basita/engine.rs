use SdlContext;
use input::Input;

use resources::*;
use components::*;
use systems::*;

pub trait GameState<'a> {
	fn get_engine_state(&self) -> &EngineState<'a>;
	fn get_engine_state_mut(&mut self) -> &mut EngineState<'a>;
}

pub trait GameEvents<S, E>
where
	E: GameEvents<S, E>,
{
	fn get_engine_events(&self) -> &EngineEvents<S, E>;
	fn get_engine_events_mut(&mut self) -> &mut EngineEvents<S, E>;
}

pub struct EngineState<'a> {
	pub delta_time: f32,
	pub running: bool,
	pub sdl_context: &'a SdlContext,
	pub input: Input,

	pub resources: EngineResources<'a>,
	pub systems: EngineSystemsState<'a>,
	pub world: EngineWorld<'a>,
}

impl<'a> EngineState<'a> {
	pub fn new(sdl_context: &'a SdlContext) -> Self {
		EngineState {
			delta_time: 0.0,
			running: true,
			sdl_context: sdl_context,
			input: Input::new(),

			resources: EngineResources::new(sdl_context),
			systems: EngineSystemsState::default(),
			world: EngineWorld::default(),
		}
	}
}

pub struct EngineResources<'a> {
	pub images: ImageResources<'a>,
}

impl<'a> EngineResources<'a> {
	pub fn new(sdl_context: &'a SdlContext) -> Self {
		EngineResources {
			images: ImageResources::new(sdl_context),
		}
	}
}

#[derive(Default)]
pub struct EngineSystemsState<'a> {
	pub render: RenderSystemState<'a>,
}

#[derive(Default)]
pub struct EngineWorld<'a> {
	pub colliders: ComponentCollection<Collider>,
	pub sprites: ComponentCollection<Sprite<'a>>,
	pub transforms: ComponentCollection<Transform>,
	pub physic_bodies: ComponentCollection<PhysicBody>,
}

pub struct EngineEvents<S, E>
where
	E: GameEvents<S, E>,
{
	pub collision: CollisionEvents<S, E>,
}

impl<S, E> EngineEvents<S, E>
where
	E: GameEvents<S, E>,
{
	pub fn new() -> Self {
		EngineEvents {
			collision: CollisionEvents::new(),
		}
	}
}

impl<'a, S, E> SystemCollection<S, E>
where
	S: GameState<'a>,
	E: GameEvents<S, E>,
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
	S: GameState<'a>,
	E: GameEvents<S, E>,
{
	systems.init(&mut state, &mut events);

	while state.get_engine_state_mut().running {
		systems.update(&mut state, &events);
	}
}
