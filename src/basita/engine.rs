use SdlContext;
use input::Input;

use resources::*;
use components::*;
use systems::*;
use events::*;

pub trait ContainsEngineState<'a, S>
where
	S: ContainsEngineState<'a, S>,
{
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
	pub running: bool,
	pub sdl_context: &'a SdlContext,
	pub input: Input,
	pub systems_state: SystemsState,

	// resources
	pub image_resources: ImageResources<'a>,

	// components
	pub box_colliders: ComponentCollection<BoxCollider>,
	pub sprites: ComponentCollection<Sprite<'a>>,
	pub transforms: ComponentCollection<Transform>,
	pub physic_bodies: ComponentCollection<PhysicBody>,
}

impl<'a> EngineState<'a> {
	pub fn new(sdl_context: &'a SdlContext) -> Self {
		EngineState {
			// core
			running: true,
			sdl_context: sdl_context,
			input: Input::new(),
			systems_state: SystemsState::default(),

			// resources
			image_resources: ImageResources::new(sdl_context),

			// components
			box_colliders: ComponentCollection::new(),
			sprites: ComponentCollection::new(),
			transforms: ComponentCollection::new(),
			physic_bodies: ComponentCollection::new(),
		}
	}
}

#[derive(Default)]
pub struct SystemsState {
	pub physics_system: physics_system::PhysicsSystemState,
}

pub struct EngineEvents<S, E>
where
	E: ContainsEngineEvents<S, E>,
{
	pub collision: collision_system::CollisionEvents<S, E>,
	pub _some_event: Event<S, E, String>,
}

impl<S, E> EngineEvents<S, E>
where
	E: ContainsEngineEvents<S, E>,
{
	pub fn new() -> Self {
		EngineEvents {
			collision: collision_system::CollisionEvents::new(),
			_some_event: Event::default(),
		}
	}
}

impl<'a, S, E> SystemCollection<S, E>
where
	S: ContainsEngineState<'a, S>,
	E: ContainsEngineEvents<S, E>,
{
	pub fn add_default_systems(&mut self) {
		self.add_system(None, physics_system::update);
		self.add_system(None, collision_system::update);
		self.add_system(None, render_system::update);
		self.add_system(None, collider_render_system::update);
		self.add_system(None, sdl_presenter_system::update);
		// frame start
		self.add_system(None, sdl_event_system::update);
	}
}

pub fn play<'a, S, E>(mut state: S, mut events: E, systems: SystemCollection<S, E>)
where
	S: ContainsEngineState<'a, S>,
	E: ContainsEngineEvents<S, E>,
{
	systems.init(&mut state, &mut events);

	while state.get_engine_state_mut().running {
		systems.update(&mut state, &events);
	}
}
