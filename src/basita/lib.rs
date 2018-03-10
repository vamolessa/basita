#![feature(vec_resize_default)]

// external
pub extern crate sdl2;

pub extern crate serde;
#[macro_use]
pub extern crate serde_derive;
pub extern crate serde_json;

pub extern crate fxhash;

#[macro_use]
pub extern crate mopa;

// local

pub mod systems;
pub mod entities;
pub mod components;
pub mod resources;

pub mod assets;
pub mod input;
pub mod math;

pub mod sdl;

mod world;
pub use self::world::*;

mod engine;
pub use self::engine::*;
