use std::fs::File;
use std::io::Write;

use crate::error::*;
use crate::generating::llvm::LLVMValue;

use super::{Constant, RegisterFormat, VirtualRegister};

#[derive(Debug)]
pub struct Writer {
	filename: String,
	target: File,
}

impl Writer {
	pub fn new(filename: String, target: File) -> Self {
		Self {
			filename,
			target,
		}
	}

	pub fn from_filename(filename: String) -> std::io::Result<Self> {
		File::create(&filename)
			.map(|file| Ok(Self::new(filename.clone(), file)))
			.map_err(|cause| cause)?
	}

	pub fn write_preamble(&mut self) -> Result<()> {
		self.write(
&format!("; ModuleID = '{0}'
source_filename = \"{0}\"
target datalayout = \"e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128\"
target triple = \"x86_64-pc-linux-gnu\"

@print_int_fstring = private unnamed_addr constant [4 x i8] c\"%d\\0A\\00\", align 1

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i32 @main() #0 {{
", self.filename
		))?;

		Ok(())
	}

	pub fn write_postamble(&mut self) -> Result<()> {
		self.write(
&format!("\tret i32 0
}}

declare i32 @printf(i8*, ...) #1

attributes #0 = {{ noinline nounwind optnone uwtable \"frame-pointer\"=\"all\" \"min-legal-vector-width\"=\"0\" \"no-trapping-math\"=\"true\" \"stack-protector-buffer-size\"=\"8\" \"target-cpu\"=\"x86-64\" \"target-features\"=\"+cx8,+fxsr,+mmx,+sse,+sse2,+x87\" \"tune-cpu\"=\"generic\" }}
attributes #1 = {{ \"frame-pointer\"=\"all\" \"no-trapping-math\"=\"true\" \"stack-protector-buffer-size\"=\"8\" \"target-cpu\"=\"x86-64\" \"target-features\"=\"+cx8,+fxsr,+mmx,+sse,+sse2,+x87\" \"tune-cpu\"=\"generic\" }}

!llvm.module.flags = !{{!0, !1, !2, !3, !4}}
!llvm.ident = !{{!5}}

!0 = !{{i32 1, !\"wchar_size\", i32 4}}
!1 = !{{i32 7, !\"PIC Level\", i32 2}}
!2 = !{{i32 7, !\"PIE Level\", i32 2}}
!3 = !{{i32 7, !\"uwtable\", i32 1}}
!4 = !{{i32 7, !\"frame-pointer\", i32 2}}
!5 = !{{!\"ICD compiler\"}}")
		)
	}

	// Allocate space for local variable
	pub fn write_local_alloc(&mut self, register: &VirtualRegister, format: &RegisterFormat) -> Result<()> {
		self.writeln(&format!("\t{register} = alloca {}", format.format_type()))
	}

	// Load src register into target
	pub fn write_load(&mut self, src: &LLVMValue, trg: &VirtualRegister) -> Result<()> {
		self.writeln(&format!("\t{trg} = load {}, {} {src}", trg.reg_type(), src.val_type()))
	}

	// Store value in register
	pub fn write_store(&mut self, src: &LLVMValue, trg: &LLVMValue) -> Result<()> {
		self.writeln(&format!("\tstore {} {src}, {} {trg}", src.val_type(), trg.val_type()))
	}

	// Write a multiplication to the LLVM file
	pub fn write_mul(&mut self, left: &LLVMValue, right: &LLVMValue, reg: u32) -> Result<()> {
		let l_val: String = match left {
			LLVMValue::VirtualRegister(l) => Ok(l.to_string()),
			LLVMValue::Constant(Constant::Integer(x)) => Ok(x.to_string()),
			_ => Err(Error::UnexpectedLLVMValue { expected: LLVMValue::VirtualRegister(VirtualRegister::new("0".to_string(), RegisterFormat::Integer)), received: left.clone() })
		}?;

		let r_val: String = match right {
			LLVMValue::VirtualRegister(l) => Ok(l.to_string()),
			LLVMValue::Constant(Constant::Integer(x)) => Ok(x.to_string()),
			_ => Err(Error::UnexpectedLLVMValue { expected: LLVMValue::VirtualRegister(VirtualRegister::new("0".to_string(), RegisterFormat::Integer)), received: left.clone() })
		}?;

		self.writeln(&format!("\t%{reg} = mul nsw {} {l_val}, {r_val}", left.val_type()))
	}

	// Write a subtraction operation to the LLVM file
	pub fn write_sub(&mut self, left: &LLVMValue, right: &LLVMValue, reg: u32) -> Result<()> {
		let l_val: String = match left {
			LLVMValue::VirtualRegister(l) => Ok(l.to_string()),
			LLVMValue::Constant(Constant::Integer(x)) => Ok(x.to_string()),
			_ => Err(Error::UnexpectedLLVMValue { expected: LLVMValue::VirtualRegister(VirtualRegister::new("0".to_string(), RegisterFormat::Integer)), received: left.clone() })
		}?;

		let r_val: String = match right {
			LLVMValue::VirtualRegister(l) => Ok(l.to_string()),
			LLVMValue::Constant(Constant::Integer(x)) => Ok(x.to_string()),
			_ => Err(Error::UnexpectedLLVMValue { expected: LLVMValue::VirtualRegister(VirtualRegister::new("0".to_string(), RegisterFormat::Integer)), received: left.clone() })
		}?;

		self.writeln(&format!("\t%{reg} = sub nsw {} {l_val}, {r_val}", left.val_type()))
	}

	// Write an addition operation to the LLVM file
	pub fn write_add(&mut self, left: &LLVMValue, right: &LLVMValue, reg: u32) -> Result<()> {
		let l_val: String = match left {
			LLVMValue::VirtualRegister(l) => Ok(l.to_string()),
			LLVMValue::Constant(Constant::Integer(x)) => Ok(x.to_string()),
			_ => Err(Error::UnexpectedLLVMValue { expected: LLVMValue::VirtualRegister(VirtualRegister::new("0".to_string(), RegisterFormat::Integer)), received: left.clone() })
		}?;

		let r_val: String = match right {
			LLVMValue::VirtualRegister(l) => Ok(l.to_string()),
			LLVMValue::Constant(Constant::Integer(x)) => Ok(x.to_string()),
			_ => Err(Error::UnexpectedLLVMValue { expected: LLVMValue::VirtualRegister(VirtualRegister::new("0".to_string(), RegisterFormat::Integer)), received: left.clone() })
		}?;

		self.writeln(&format!("\t%{reg} = add nsw {} {l_val}, {r_val}", left.val_type()))
	}

	// Write an addition operation to the LLVM file
	pub fn write_div(&mut self, left: &LLVMValue, right: &LLVMValue, reg: u32) -> Result<()> {
		let l_val: String = match left {
			LLVMValue::VirtualRegister(l) => Ok(l.to_string()),
			LLVMValue::Constant(Constant::Integer(x)) => Ok(x.to_string()),
			_ => Err(Error::UnexpectedLLVMValue { expected: LLVMValue::VirtualRegister(VirtualRegister::new("0".to_string(), RegisterFormat::Integer)), received: left.clone() })
		}?;

		let r_val: String = match right {
			LLVMValue::VirtualRegister(l) => Ok(l.to_string()),
			LLVMValue::Constant(Constant::Integer(x)) => Ok(x.to_string()),
			_ => Err(Error::UnexpectedLLVMValue { expected: LLVMValue::VirtualRegister(VirtualRegister::new("0".to_string(), RegisterFormat::Integer)), received: left.clone() })
		}?;

		self.writeln(&format!("\t%{reg} = udiv {} {l_val}, {r_val}", left.val_type()))
	}

	// Print integer (i32)
	pub fn write_print(&mut self, val: &LLVMValue) -> Result<()> {
		self.writeln(&format!("\tcall i32(i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @print_int_fstring, i32 0, i32 0), {} {val})", val.val_type()))
	}

	pub fn write(&mut self, msg: &str) -> Result<()> {
		self.target.write(msg.as_bytes())
			.map(|_| Ok(()))
			.map_err(|cause| Error::FileWriteError { cause })?
	}

	pub fn writeln(&mut self, msg: &str) -> Result<()> {
		self.target.write((msg.to_owned() + "\n").as_bytes())
			.map(|_| Ok(()))
			.map_err(|cause| Error::FileWriteError { cause })?
	}

}