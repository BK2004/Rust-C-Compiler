use core::fmt;
use crate::error::{Error, Result};

#[derive(Debug, Clone)]
pub enum LLVMValue {
	VirtualRegister(VirtualRegister),
	Constant(Constant),
	None
}

impl std::fmt::Display for LLVMValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
		match self {
			LLVMValue::None => write!(f, "None"),
			LLVMValue::VirtualRegister(vr) => write!(f, "{vr}"),
			LLVMValue::Constant(c) => write!(f, "{c}"),
		}
	}
}

#[derive(Debug, Clone)]
pub enum Constant {
	Integer(i32),
}

impl fmt::Display for Constant {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Constant::Integer(x) => write!(f, "{x}"),
		}
	}
}

#[derive(Debug, Clone)]
pub struct VirtualRegister {
	id: String,
	format: RegisterFormat,
}

impl VirtualRegister {
	pub fn new(id: String, format: RegisterFormat) -> Self {
		Self {
			id,
			format,
		}
	}

	pub fn id(&self) -> &str {
		&self.id
	}

	pub fn format(&self) -> &RegisterFormat {
		&self.format
	}
}

impl fmt::Display for VirtualRegister {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "%{}", self.id())
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegisterFormat {
	Integer,
	Identifier,
}

#[derive(Debug, Clone)]
pub enum Symbol {
	Local {
		name: String,
		value: LLVMValue,
	},
}

impl Symbol {
	pub fn name(&self) -> &str {
		match self {
			Symbol::Local { name, .. } => name.as_str(),
		}
	}
}

#[derive(Debug, Clone)]
pub struct SymbolTableNode {
	symbol: Symbol,
	next: Option<Box<SymbolTableNode>>,
}

impl SymbolTableNode {
	pub fn new(symbol: Symbol, next: Option<Box<SymbolTableNode>>) -> Self {
		Self {
			symbol,
			next
		}
	}

	pub fn symbol(&self) -> &Symbol {
		&self.symbol
	}

	pub fn next(&self) -> &Option<Box<SymbolTableNode>> {
		&self.next
	}
}

#[derive(Debug)]
pub struct SymbolTable {
	buckets: Vec<Option<Box<SymbolTableNode>>>,
}

impl SymbolTable {
	pub fn new(capacity: usize) -> Self {
		let mut buckets = Vec::new();
		buckets.resize(capacity, None);
		
		Self {
			buckets,
		}
	}

	pub fn len(&self) -> usize {
		self.buckets.len()
	}

	pub fn insert(&mut self, symbol: Symbol) {
		let hash = self.hash(symbol.name());

		let curr_node = &mut self.buckets[hash];
		let new_symbol = SymbolTableNode::new(symbol, curr_node.take());

		*curr_node = Some(Box::new(new_symbol));
	}

	pub fn get_mut(&mut self, name: &str) -> Result<&mut Symbol> {
		let hash = self.hash(name);

		let mut curr = &mut self.buckets[hash];
		while let Some(c) = curr {
			if name.eq(c.symbol().name()) {
				return Ok(&mut c.symbol);
			}

			curr = &mut c.next;
		}

		Err(Error::SymbolUndefined { name: name.to_owned() })
	}

	pub fn get(&mut self, name: &str) -> Result<&Symbol> {
		let hash = self.hash(name);

		let mut curr = &self.buckets[hash];
		while let Some(c) = curr {
			if name.eq(c.symbol().name()) {
				return Ok(c.symbol());
			}

			curr = c.next();
		}

		Err(Error::SymbolUndefined { name: name.to_owned() })
	}

	pub fn remove(&mut self, name: &str)  {
		let hash = self.hash(name);

		let mut curr = &mut self.buckets[hash];
		while curr.is_some() {
			if curr.as_ref().unwrap().symbol().name().eq(name) {
				// curr is target, so this is the first element; just make next the first element
				let next = curr.as_mut().unwrap().next.take();
				*curr = next;

				return ();
			} else if curr.as_ref().unwrap().next().is_none() {
				return ();
			} else if curr.as_ref().unwrap().next().as_ref().unwrap().symbol().name().eq(name) {
				let next = curr.as_mut().unwrap().next.as_mut().unwrap().next.take();
				*curr = next;

				return ();
			} else {
				curr = &mut curr.as_mut().unwrap().next;
			}
		}
	}

	pub fn hash(&self, name: &str) -> usize {
		let len = self.len() as u64;
		let prime: u64 = 67;
		let mut pow: u64 = 1;

		let mut hash: u64 = 0;
		for (_, c) in name.chars().enumerate() {
			hash = (hash + (c as u64 % len) * pow) % len;
			pow = (pow * prime) % len;
		}

		return hash as usize;
	}

	pub fn create_local(&self, name: &String, format: &RegisterFormat) -> (Symbol, VirtualRegister) {
		let reg = VirtualRegister::new(name.to_owned(), format.clone());
		let symbol = Symbol::Local {
			name: name.to_owned(),
			value: LLVMValue::VirtualRegister(reg.clone()),
		};

		(symbol, reg)
	}
}