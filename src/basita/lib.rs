#![feature(vec_resize_default)]

// external
pub use sdl2;
pub use serde_derive;
pub use specs;

// local
pub mod core;
//pub mod gui;
pub mod input;
pub mod math;
pub mod mixer;
//pub mod physics;
pub mod renderer;
pub mod sdl;

pub mod game;
