use std::rc::Rc;

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
	pub slots: EngineSlots,

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
			slots: EngineSlots::default(),

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

signal!(SomeEngineSignal, ());

#[derive(Default)]
pub struct EngineSlots {
	pub some_slot: Slot<SomeEngineSignal>,
}

fn test(_s: &System, _st: &mut EngineState, _d: &()) {}

struct A {}
impl A {
	pub fn f(_s: &System, _st: &mut EngineState, _d: &()) {
		let _a = _s as &A;
	}
	pub fn f2(&self, _st: &mut EngineState, _d: &()) {}
}
impl System for A {
	fn update(&self, _state: &mut EngineState) {}
}

impl System for EngineSystems {
	fn init(&self, state: &mut EngineState) {
		self.render_systems.init(state);
		self.collider_render_system.init(state);
		self.sdl_presenter_system.init(state);
		self.sdl_event_system.init(state);

		let mut some_event: Event<()> = Event::new();
		some_event.subscribe(|_system, _state, _data| println!("ADASDASDAS"));
		some_event.subscribe(test);
		some_event.subscribe(A::f);
		//some_event.raise(state, &());

		/*
		let callback = |state, data| self.collider_render_system.test(state, data);
		let boxed_callback = Box::new(callback);

		state
			.slots
			.some_slot
			.subscribe(Rc::clone(&(self.a as Box<SomeEngineSignal>)));
		let data = ();
		state.slots.some_slot.raise(&data);
		*/
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
