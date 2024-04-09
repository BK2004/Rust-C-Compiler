use core::fmt;
use crate::error::{Error, Result};

use super::{Identifier, Token};

#[derive(Debug, Clone)]
pub enum LLVMValue {
	VirtualRegister(VirtualRegister),
	Indirect { pointee: Box<LLVMValue>, referenced_fmt: RegisterFormat },
	Constant(Constant),
	None,
	Null
}

impl LLVMValue {
	pub fn val_type(&self) -> String {
		match self {
			LLVMValue::None => String::from("none"),
			LLVMValue::Null => String::from("null"),
			LLVMValue::Constant(c) => c.const_type(),
			LLVMValue::VirtualRegister(v) => v.reg_type(),
			LLVMValue::Indirect { pointee, referenced_fmt } => format!("{}", referenced_fmt.format_type()),
		}
	}

	pub fn format(&self) -> RegisterFormat {
		match self {
			LLVMValue::None => RegisterFormat::Void,
			LLVMValue::Null => RegisterFormat::Null,
			LLVMValue::Constant(c) => c.format(),
			LLVMValue::VirtualRegister(r) => r.format().clone(),
			LLVMValue::Indirect { pointee, referenced_fmt } => pointee.format(),
		}
	}
}

impl std::fmt::Display for LLVMValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
		match self {
			LLVMValue::None => write!(f, "None"),
			LLVMValue::Null => write!(f, "null"),
			LLVMValue::VirtualRegister(vr) => write!(f, "{vr}"),
			LLVMValue::Constant(c) => write!(f, "{c}"),
			LLVMValue::Indirect { pointee, referenced_fmt } => write!(f, "{pointee}"),
		}
	}
}

#[derive(Debug, Clone)]
pub enum Constant {
	Integer(i64),
}

impl Constant {
	pub fn const_type(&self) -> String {
		match self {
			Constant::Integer(_) => String::from("i64"),
		}
	}

	pub fn format(&self) -> RegisterFormat {
		match self {
			Constant::Integer(_) => RegisterFormat::Integer,
		}
	}
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
	is_local: bool,
}

impl VirtualRegister {
	pub fn new(id: String, format: RegisterFormat, is_local: bool) -> Self {
		Self {
			id,
			format,
			is_local,
		}
	}

	pub fn from_identifier(id: String, identifier: Identifier, is_local: bool, symbol_table: &SymbolTable) -> Result<Self> {
		Ok(Self {
			id,
			format: match &identifier {
				Identifier::Symbol(s) => {
					RegisterFormat::Identifier {
						id_type: Box::new(symbol_table.get(&s)?.value().format())
					}
				},
				_ => Err(Error::InvalidIdentifier { expected: [Identifier::Symbol("".to_string())].to_vec(), received: identifier })?
			},
			is_local,
		})
	}

	pub fn id(&self) -> &str {
		&self.id
	}

	pub fn format(&self) -> &RegisterFormat {
		&self.format
	}

	pub fn is_local(&self) -> bool {
		self.is_local
	}

	pub fn reg_type(&self) -> String {
		self.format.format_type()
	}
}

impl fmt::Display for VirtualRegister {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.is_local() {
			write!(f, "%{}", self.id())
		} else {
			write!(f, "@{}", self.id())
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionSignature {
	params: Vec<RegisterFormat>,
	return_fmt: Box<RegisterFormat>,
}

impl FunctionSignature {
	pub fn new(params: &Vec<RegisterFormat>, return_fmt: RegisterFormat) -> Self {
		Self {
			params: params.clone(),
			return_fmt: Box::new(return_fmt),
		}
	}

	pub fn params(&self) -> &Vec<RegisterFormat> {
		&self.params
	}

	pub fn return_fmt(&self) -> &RegisterFormat {
		&self.return_fmt
	}
}

impl fmt::Display for FunctionSignature {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut out = String::from("function(");
		for (i, param) in self.params.iter().enumerate() {
			if i < self.params.len() - 1 {
				out = format!("{out}{param}, ");
			} else {
				out = format!("{out}{param}");
			}
		}

		write!(f, "{out}")
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegisterFormat {
	Void,
	Null,
	Integer,
	Boolean,
	Identifier {
		id_type: Box<RegisterFormat>,
	},
	Pointer {
		pointee: Box<RegisterFormat>,
	},
	Function {
		signature: FunctionSignature,
	}
}

impl RegisterFormat {
	pub fn to_pointer(&self) -> RegisterFormat {
		RegisterFormat::Pointer { pointee: Box::new(self.clone()) }
	}

	pub fn can_compare_to(&self, other: &RegisterFormat, op: &Token) -> bool {
		match (self, op, other) {
			(RegisterFormat::Integer, _, RegisterFormat::Integer) => true,
			_ => false,
		}
	}

	pub fn can_convert_to(&self, other: &RegisterFormat) -> bool {
		match (self, other) {
			(RegisterFormat::Integer, RegisterFormat::Integer) => true,
			(RegisterFormat::Boolean, RegisterFormat::Boolean) => true,
			(RegisterFormat::Pointer { .. }, RegisterFormat::Boolean) => true,
			(RegisterFormat::Pointer { pointee: self_pointee }, RegisterFormat::Pointer { pointee: other_pointee }) => self_pointee.can_convert_to(&other_pointee),
			_ => false,
		}
	}

	pub fn format_type(&self) -> String {
		match self {
			RegisterFormat::Void => String::from("void"),
			RegisterFormat::Identifier { id_type } => String::from(format!("{}*", id_type.format_type())),
			RegisterFormat::Integer => String::from("i64"),
			RegisterFormat::Boolean => String::from("i1"),
			RegisterFormat::Pointer { pointee } => String::from(format!("{}*", pointee.format_type())),
			RegisterFormat::Function { .. } => String::from("function"),
			RegisterFormat::Null => String::from("null"),
		}
	}

	pub fn expect(&self, other: RegisterFormat) -> Result<()> {
		if self.to_owned().to_string() == other.to_string() {
			Ok(())
		} else {
			Err(Error::UnexpectedFormat { received: other.to_owned(), expected: self.to_owned() })
		}
	}
}

impl fmt::Display for RegisterFormat {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			RegisterFormat::Void => write!(f, "void"),
			RegisterFormat::Boolean => write!(f, "bool"),
			RegisterFormat::Integer => write!(f, "int"),
			RegisterFormat::Pointer { pointee } => write!(f, "{pointee}*"),
			RegisterFormat::Identifier { id_type } => write!(f, "{id_type}"),
			RegisterFormat::Function { .. } => write!(f, "function"),
			RegisterFormat::Null => write!(f, "null"),
		}
	}
}

#[derive(Debug, Clone)]
pub enum Symbol {
	Local {
		name: String,
		value: LLVMValue,
	},
	Function {
		name: String,
		value: LLVMValue,
	}
}

impl Symbol {
	pub fn name(&self) -> &str {
		match self {
			Symbol::Local { name, .. } => name.as_str(),
			Symbol::Function { name, .. } => name.as_str(),
		}
	}

	pub fn value(&self) -> &LLVMValue {
		match self {
			Symbol::Local { value, .. } => value,
			Symbol::Function { value, .. } => value,
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

	pub fn get(&self, name: &str) -> Result<&Symbol> {
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

	pub fn clear(&mut self) {
		for i in 0..self.len() {
			while self.buckets[i].is_some() {
				let next = self.buckets[i].as_mut().unwrap().next.take();
				self.buckets[i] = next;
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
		let pointer = VirtualRegister::new(name.to_owned(), format.to_pointer(), true);
		let value = LLVMValue::Indirect {
			pointee: Box::new(LLVMValue::VirtualRegister(pointer.clone())),
			referenced_fmt: format.clone(),
		};
		let symbol = Symbol::Local {
			name: name.to_owned(),
			value,
		};

		(symbol, pointer)
	}

	pub fn create_function(&self, name: &String, signature: &FunctionSignature) -> (Symbol, VirtualRegister) {
		let reg = VirtualRegister::new(name.to_owned(), RegisterFormat::Function { signature: signature.to_owned() }, false);
		let symbol = Symbol::Function {
			name: name.to_owned(),
			value: LLVMValue::VirtualRegister(reg.clone()),
		};

		(symbol, reg)
	}
}

pub struct Label {
	id: String,
}

impl Label {
	pub fn new(id: u32) -> Self {
		Self {
			id: format!("label.{id}")
		}
	}
}

impl fmt::Display for Label {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.id)
	}
}