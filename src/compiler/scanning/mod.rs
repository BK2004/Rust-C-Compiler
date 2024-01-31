pub mod token;

use token::*;
use utf8_chars::BufReadCharsExt;

use std::fmt::Debug;
use std::fs::File;
use std::io::{BufReader, Result};

#[derive(Debug)]
pub struct Scanner {
	file: BufReader<File>,
	put_backs: Vec<char>,
	filename: String,
	line: usize,
}

impl Scanner {
	pub fn new(filename: String, file: BufReader<File>) -> Self {
		Self {
			file,
			filename,
			put_backs: [].to_vec(),
			line: 1,
		}
	}

	// Open file with given filename
	pub fn open_file(filename: String) -> Result<Self> {
		File::open(&filename)
			.map(|file| Self::new(filename, BufReader::new(file)))
			.map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error))
	}

	// Get next character in reader
	pub fn next_char(&mut self) -> Result<Option<char>> {
		// If there are any characters on put back, return the top
		if let Some(c) = self.put_backs.pop() {
			return Ok(Some(c));
		}

		let next = self.file.read_char()
			.map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error));

		if let Ok(Some(c)) = next {
			// If next is line break, add one to line counter
			if c == '\n' {
				self.line += 1;
			}

			return Ok(Some(c));
		}
		
		Ok(None)
	}

	pub fn file(&self) -> &BufReader<File> {
		&self.file
	}

	pub fn put_backs(&self) -> &[char] {
		&self.put_backs
	}

	pub fn filename(&self) -> &String {
		&self.filename
	}
}