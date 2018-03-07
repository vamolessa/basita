#![feature(vec_resize_default)]

// external
pub extern crate sdl2;

#[macro_use]
pub extern crate serde_derive;

pub extern crate serde;
pub extern crate serde_json;

// local
//pub mod systems;
pub mod components;
pub mod resources;
pub mod events;

pub mod assets;
pub mod input;
pub mod math;

pub mod sdl;

mod world;
pub use self::world::*;

mod entity;
pub use self::entity::*;

mod engine;
pub use self::engine::*;
