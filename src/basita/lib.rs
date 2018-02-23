// external
pub extern crate sdl2;

#[macro_use]
pub extern crate serde_derive;

pub extern crate serde;
pub extern crate serde_json;

pub extern crate uuid;

// internal
pub mod serialization;

pub mod systems;
pub mod components;
pub mod events;

pub mod resources;
pub mod input;
pub mod math;

mod sdl_context;
pub use self::sdl_context::*;

mod engine;
pub use self::engine::*;
