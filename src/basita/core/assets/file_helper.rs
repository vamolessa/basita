use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn read_all_text(path: &str) -> Result<String, io::Error> {
	let mut file = File::open(path)?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;
	Ok(contents)
}

pub fn write_all_bytes(path: &str, bytes: &[u8]) -> Result<(), io::Error> {
	let mut file = File::create(path)?;
	file.write_all(bytes)
}

pub fn write_all_text(path: &str, text: &str) -> Result<(), io::Error> {
	write_all_bytes(path, text.as_bytes())
}
