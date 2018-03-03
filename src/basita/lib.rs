// external
pub extern crate sdl2;

#[macro_use]
pub extern crate serde_derive;

pub extern crate serde;
pub extern crate serde_json;

// internal
//pub mod systems;
pub mod components;
pub mod events;

pub mod assets;
pub mod input;
pub mod math;

pub mod sdl;

mod engine;
pub use self::engine::*;

// modules

mod world;
pub use self::world::*;

pub mod renderer;
pub mod new_render_system;
