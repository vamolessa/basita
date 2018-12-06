#![feature(vec_resize_default)]
#![feature(duration_as_u128)]

// external
pub use sdl2;
pub use specs;

//pub use serde_derive::{Deserialize, Serialize};
#[macro_use]
pub extern crate serde_derive;

// local
pub mod core;
pub mod gui;
pub mod input;
pub mod math;
pub mod mixer;
pub mod physics;
pub mod renderer;
pub mod sdl;
