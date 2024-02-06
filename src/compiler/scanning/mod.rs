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

	// Put character in put backs
	pub fn put_back(&mut self, c: char) -> () {
		self.put_backs.push(c);
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

	// Skip over all whitespace and return next char
	pub fn skip_whitespace(&mut self) -> Result<Option<char>> {
		while let Ok(Some(c)) = self.next_char() {
			if !c.is_whitespace() {
				self.put_back(c);
				break;
			}
		}

		self.next_char()
	}

	// Scan in next token and return result
	pub fn scan(&mut self) -> Result<Option<Token>> {
		let next = self.skip_whitespace()?;

		if let Some(mut c) = next {
			// Check if c is the start of a literal
			if c.is_numeric() {
				let num = self.scan_integer_literal(c)?;

				return Ok(Some(Token::Literal(Literal::Integer(num))));
			}

			// Check if c is start of an identifier
			if c.is_alphabetic() {
				let identifier = self.scan_identifier(c)?;
				println!("{identifier}");

				// Search for identifier in list of identifiers; if not found, error
				for (_, id) in IDENTIFIER_SYMBOLS.iter().enumerate() {
					if identifier.eq(id.0) {
						return Ok(Some(Token::Identifier(id.1)));
					}
				}

				return Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid identifier found."));
			}

			// Generate possible symbols that c represents
			let mut remaining_symbols: Vec<&(&str, Token)> = Vec::new();
			let mut curr: String = String::from(c);
			for (_, symbol) in TOKEN_SYMBOLS.iter().enumerate() {
				if symbol.0.chars().nth(0).unwrap() == c {
					remaining_symbols.push(symbol);
				}
			}

			while remaining_symbols.len() > 1 {
				// If character is alphanumeric/whitespace or EOF reached, stop reading symbols
				if let Some(next) = self.next_char()? {
					c = next;
				} else {
					break;
				}

				if c.is_alphanumeric() || c.is_whitespace() {
					self.put_back(c);
					break;
				} else {
					curr.push(c);

					// Remove symbols that don't match
					remaining_symbols.retain(|symbol| symbol.0.starts_with(&curr));
				}
			}

			// If possible symbols is empty, token is invalid
			if remaining_symbols.len() == 0 {
				Ok(Some(Token::Invalid))
			} else {
				for (_, symbol) in remaining_symbols.iter().enumerate() {
					if symbol.0 == curr {
						return Ok(Some(symbol.1.clone()));
					}
				}

				Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to recognize symbol."))
			}
		} else {
			Ok(Some(Token::EndOfFile))
		}
	}

	// Scan in integer literal
	pub fn scan_integer_literal(&mut self, mut c: char) -> Result<i32> {
		let mut res: i32 = 0;
		while c.is_numeric() {
			res = res * 10 + (c as i32 - ('0' as i32));
			
			match self.next_char()? {
				Some(next) => {c = next;},
				None => return Ok(res)
			}
		}

		self.put_back(c);

		Ok(res)
	}

	// Scan in identifier
	pub fn scan_identifier(&mut self, mut c: char) -> Result<String> {
		let mut res: String = String::from("");
		while c.is_alphanumeric() {
			res.insert(res.len(), c);

			match self.next_char()? {
				Some(next) => {c = next;},
				None => return Ok(res)
			}
		}

		self.put_back(c);

		Ok(res)
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