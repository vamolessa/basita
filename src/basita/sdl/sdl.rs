use sdl2;
use sdl2::image::{INIT_JPG, INIT_PNG};
use sdl2::mixer::{AUDIO_S16LSB, DEFAULT_CHANNELS, INIT_FLAC, INIT_MOD, INIT_MP3, INIT_OGG};
use sdl2::render::Canvas;
use sdl2::video::Window;

use super::{
	ChunkLoader, ChunkStorage, FontLoader, FontStorage, MusicLoader, MusicStorage, TextureLoader,
	TextureStorage,
};

pub struct SdlContext {
	_sdl: sdl2::Sdl,

	pub canvas: Canvas<Window>,
	pub event_pump: sdl2::EventPump,
}

impl SdlContext {
	pub fn new(
		window_title: &str,
		window_width: u32,
		window_height: u32,
		simultaneous_audio_count: u8,
	) -> Self {
		let sdl = sdl2::init().unwrap();

		let video_subsystem = sdl.video().unwrap();
		let window = video_subsystem
			.window(window_title, window_width, window_height)
			.position_centered()
			.build()
			.unwrap();

		let canvas = window.into_canvas().build().unwrap();

		let _image_context = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();

		// Audio
		{
			let _audio = sdl.audio().unwrap();
			let frequency = 44_100;
			let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
			let channels = DEFAULT_CHANNELS; // Stereo
			let chunk_size = 1_024;
			sdl2::mixer::open_audio(frequency, format, channels, chunk_size).unwrap();
			let _mixer_context =
				sdl2::mixer::init(INIT_MP3 | INIT_FLAC | INIT_MOD | INIT_OGG).unwrap();
			sdl2::mixer::allocate_channels(simultaneous_audio_count as i32);
		}

		SdlContext {
			event_pump: sdl.event_pump().unwrap(),
			_sdl: sdl,
			canvas: canvas,
		}
	}
}

pub struct SdlLoader {
	pub texture_loader: TextureLoader,
	pub font_loader: FontLoader,
	pub chunk_loader: ChunkLoader,
	pub music_loader: MusicLoader,
}

impl SdlLoader {
	pub fn new(sdl_context: &SdlContext) -> Self {
		SdlLoader {
			texture_loader: TextureLoader::new(sdl_context),
			font_loader: FontLoader::new(),
			chunk_loader: ChunkLoader {},
			music_loader: MusicLoader {},
		}
	}
}

#[derive(Default)]
pub struct SdlStorage<'a> {
	pub texture_storage: TextureStorage<'a>,
	pub font_storage: FontStorage<'a, 'a>,
	pub chunk_storage: ChunkStorage,
	pub music_storage: MusicStorage<'a>,
}

pub struct SdlAssetStorage<A> {
	assets: Vec<A>,
}

impl<A> SdlAssetStorage<A> {
	pub fn add(&mut self, asset: A) -> usize {
		let index = self.assets.len();
		self.assets.push(asset);
		index
	}

	pub fn at(&self, index: usize) -> &A {
		&self.assets[index]
	}

	pub fn at_mut(&mut self, index: usize) -> &mut A {
		&mut self.assets[index]
	}
}

impl<A> Default for SdlAssetStorage<A> {
	fn default() -> Self {
		SdlAssetStorage {
			assets: Vec::default(),
		}
	}
}
