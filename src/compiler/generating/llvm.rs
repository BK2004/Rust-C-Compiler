#[derive(Debug, Clone)]
pub enum LLVMValue {
	VirtualRegister {
		val: u32,
		is_pointer: bool,
	},
	None
}

impl std::fmt::Display for LLVMValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		match self {
			LLVMValue::None => write!(f, "None"),
			LLVMValue::VirtualRegister{val: _, is_pointer: _} => write!(f, "VirtualRegister"),
		}
	}
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