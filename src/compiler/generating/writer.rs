use std::fs::File;
use std::io::{Result, Write};

use crate::generating::llvm::{LLVMStackEntry, LLVMValue};
use crate::scanning::token::Literal;

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

	pub fn from_filename(filename: String) -> Result<Self> {
		File::create(&filename)
			.and_then(|file| Ok(Self::new(filename.clone(), file)))
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

	// Write an allocation to the LLVM file (only implemented for i32 as of now)
	pub fn write_alloc(&mut self, stack_entry: &LLVMStackEntry) -> Result<()> {
		if let LLVMValue::VirtualRegister(reg) = stack_entry.register() {
			self.writeln(&format!("\t%{} = alloca i32, align {}", reg, stack_entry.align_bytes()))
		} else {
			Err(std::io::Error::new(std::io::ErrorKind::Other, "VirtualRegister expected."))
		}
	}

	// Write a literal to the LLVM file (store it at given reg)
	pub fn write_literal(&mut self, literal: &Literal, reg: u32) -> Result<()> {
		match literal {
			Literal::Integer(x) => self.writeln(&format!("\tstore i32 {}, i32* %{}", x, reg))?
		}

		Ok(())
	}

	// Write a load statement to the LLVM file (Move stored i32 pointer to i32)
	pub fn write_load(&mut self, old_reg: u32, new_reg: u32) -> Result<()> {
		self.writeln(&format!("\t%{} = load i32, i32* %{}", new_reg, old_reg))?;

		Ok(())
	}

	// Write a multiplication to the LLVM file
	pub fn write_mul(&mut self, left: &LLVMValue, right: &LLVMValue, reg: u32) -> Result<()> {
		match left {
			LLVMValue::VirtualRegister(l) => match right {
				LLVMValue::VirtualRegister(r) => Ok(self.writeln(&format!("\t%{} = mul nsw i32 %{}, %{}", reg, l, r))?),
				_ => Err(std::io::Error::new(std::io::ErrorKind::Other, "Expected virtual register"))
			},
			_ => Err(std::io::Error::new(std::io::ErrorKind::Other, "Expected virtual register"))
		}
	}

	// Write a subtraction operation to the LLVM file
	pub fn write_sub(&mut self, left: &LLVMValue, right: &LLVMValue, reg: u32) -> Result<()> {
		match left {
			LLVMValue::VirtualRegister(l) => match right {
				LLVMValue::VirtualRegister(r) => Ok(self.writeln(&format!("\t%{} = sub nsw i32 %{}, %{}", reg, l, r))?),
				_ => Err(std::io::Error::new(std::io::ErrorKind::Other, "Expected virtual register"))
			},
			_ => Err(std::io::Error::new(std::io::ErrorKind::Other, "Expected virtual register"))
		}
	}

	// Write an addition operation to the LLVM file
	pub fn write_add(&mut self, left: &LLVMValue, right: &LLVMValue, reg: u32) -> Result<()> {
		match left {
			LLVMValue::VirtualRegister(l) => match right {
				LLVMValue::VirtualRegister(r) => Ok(self.writeln(&format!("\t%{} = add nsw i32 %{}, %{}", reg, l, r))?),
				_ => Err(std::io::Error::new(std::io::ErrorKind::Other, "Expected virtual register"))
			},
			_ => Err(std::io::Error::new(std::io::ErrorKind::Other, "Expected virtual register"))
		}
	}

	// Write an addition operation to the LLVM file
	pub fn write_div(&mut self, left: &LLVMValue, right: &LLVMValue, reg: u32) -> Result<()> {
		match left {
			LLVMValue::VirtualRegister(l) => match right {
				LLVMValue::VirtualRegister(r) => Ok(self.writeln(&format!("\t%{} = udiv i32 %{}, %{}", reg, l, r))?),
				_ => Err(std::io::Error::new(std::io::ErrorKind::Other, "Expected virtual register"))
			},
			_ => Err(std::io::Error::new(std::io::ErrorKind::Other, "Expected virtual register"))
		}
	}

	// Print integer (i32)
	pub fn print_int(&mut self, reg: u32) -> Result<()> {
		self.writeln(&format!("\tcall i32(i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @print_int_fstring, i32 0, i32 0), i32 %{})", reg))
	}

	pub fn write(&mut self, msg: &str) -> Result<()> {
		self.target.write(msg.as_bytes())
			.map(|_| Ok(()))
			.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?
	}

	pub fn writeln(&mut self, msg: &str) -> Result<()> {
		self.target.write((msg.to_owned() + "\n").as_bytes())
			.map(|_| Ok(()))
			.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?
	}

}