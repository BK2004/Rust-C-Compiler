use core::fmt;

#[derive(Debug, Clone)]
pub enum LLVMValue {
	VirtualRegister(VirtualRegister),
	Constant(Constant),
	None
}

impl std::fmt::Display for LLVMValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
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
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
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
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(f, "%{}", self.id())
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegisterFormat {
	Integer,
}

#[derive(Debug, Clone)]
pub struct LLVMStackEntry {
	register: LLVMValue,
	align_bytes: u32,
}

impl LLVMStackEntry {
	pub fn new(register: LLVMValue, align_bytes: u32) -> Self {
		Self {
			register,
			align_bytes,
		}
	}

	pub fn register(&self) -> &LLVMValue {
		&self.register
	}

	pub fn align_bytes(&self) -> u32 {
		self.align_bytes
	}
}