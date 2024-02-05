#[derive(Debug, Clone)]
pub enum LLVMValue {
	VirtualRegister(u32),
	None
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