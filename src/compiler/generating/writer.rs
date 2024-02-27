use std::fs::File;
use std::io::Write;

use crate::error::*;
use crate::generating::llvm::LLVMValue;

use super::{Constant, FunctionSignature, Label, RegisterFormat, VirtualRegister};

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

", self.filename
		))?;

		Ok(())
	}

	pub fn write_postamble(&mut self) -> Result<()> {
		self.write(
&format!("declare i32 @printf(i8*, ...) #1

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
			_ => Err(Error::UnexpectedLLVMValue { expected: LLVMValue::VirtualRegister(VirtualRegister::new("0".to_string(), RegisterFormat::Integer, true)), received: left.clone() })
		}?;

		let r_val: String = match right {
			LLVMValue::VirtualRegister(l) => Ok(l.to_string()),
			LLVMValue::Constant(Constant::Integer(x)) => Ok(x.to_string()),
			_ => Err(Error::UnexpectedLLVMValue { expected: LLVMValue::VirtualRegister(VirtualRegister::new("0".to_string(), RegisterFormat::Integer, true)), received: left.clone() })
		}?;

		self.writeln(&format!("\t%{reg} = mul nsw {} {l_val}, {r_val}", left.val_type()))
	}

	// Write a subtraction operation to the LLVM file
	pub fn write_sub(&mut self, left: &LLVMValue, right: &LLVMValue, reg: u32) -> Result<()> {
		let l_val: String = match left {
			LLVMValue::VirtualRegister(l) => Ok(l.to_string()),
			LLVMValue::Constant(Constant::Integer(x)) => Ok(x.to_string()),
			_ => Err(Error::UnexpectedLLVMValue { expected: LLVMValue::VirtualRegister(VirtualRegister::new("0".to_string(), RegisterFormat::Integer, true)), received: left.clone() })
		}?;

		let r_val: String = match right {
			LLVMValue::VirtualRegister(l) => Ok(l.to_string()),
			LLVMValue::Constant(Constant::Integer(x)) => Ok(x.to_string()),
			_ => Err(Error::UnexpectedLLVMValue { expected: LLVMValue::VirtualRegister(VirtualRegister::new("0".to_string(), RegisterFormat::Integer, true)), received: left.clone() })
		}?;

		self.writeln(&format!("\t%{reg} = sub nsw {} {l_val}, {r_val}", left.val_type()))
	}

	// Write an addition operation to the LLVM file
	pub fn write_add(&mut self, left: &LLVMValue, right: &LLVMValue, reg: u32) -> Result<()> {
		let l_val: String = match left {
			LLVMValue::VirtualRegister(l) => Ok(l.to_string()),
			LLVMValue::Constant(Constant::Integer(x)) => Ok(x.to_string()),
			_ => Err(Error::UnexpectedLLVMValue { expected: LLVMValue::VirtualRegister(VirtualRegister::new("0".to_string(), RegisterFormat::Integer, true)), received: left.clone() })
		}?;

		let r_val: String = match right {
			LLVMValue::VirtualRegister(l) => Ok(l.to_string()),
			LLVMValue::Constant(Constant::Integer(x)) => Ok(x.to_string()),
			_ => Err(Error::UnexpectedLLVMValue { expected: LLVMValue::VirtualRegister(VirtualRegister::new("0".to_string(), RegisterFormat::Integer, true)), received: left.clone() })
		}?;

		self.writeln(&format!("\t%{reg} = add nsw {} {l_val}, {r_val}", left.val_type()))
	}

	// Write an addition operation to the LLVM file
	pub fn write_div(&mut self, left: &LLVMValue, right: &LLVMValue, reg: u32) -> Result<()> {
		let l_val: String = match left {
			LLVMValue::VirtualRegister(l) => Ok(l.to_string()),
			LLVMValue::Constant(Constant::Integer(x)) => Ok(x.to_string()),
			_ => Err(Error::UnexpectedLLVMValue { expected: LLVMValue::VirtualRegister(VirtualRegister::new("0".to_string(), RegisterFormat::Integer, true)), received: left.clone() })
		}?;

		let r_val: String = match right {
			LLVMValue::VirtualRegister(l) => Ok(l.to_string()),
			LLVMValue::Constant(Constant::Integer(x)) => Ok(x.to_string()),
			_ => Err(Error::UnexpectedLLVMValue { expected: LLVMValue::VirtualRegister(VirtualRegister::new("0".to_string(), RegisterFormat::Integer, true)), received: left.clone() })
		}?;

		self.writeln(&format!("\t%{reg} = udiv {} {l_val}, {r_val}", left.val_type()))
	}

	// Compare left and right via 'op'
	pub fn write_cmp(&mut self, left: &LLVMValue, right: &LLVMValue, reg: u32, op: String) -> Result<()> {
		self.writeln(&format!("\t%{reg} = icmp {op} {} {left}, {right}", left.val_type()))
	}

	// Write given label to output
	pub fn write_label(&mut self, label: &Label) -> Result<()> {
		self.writeln(&format!("{label}:"))
	}

	// Write a conditional branch statement, true label, and false label
	pub fn write_cond_branch(&mut self, condition: &LLVMValue, t_label: &Label, f_label: &Label) -> Result<()> {
		self.writeln(&format!("\tbr {cond_type} {condition}, label %{t_label}, label %{f_label}", cond_type=condition.val_type()))
	}

	// Write a direct branch to a label
	pub fn write_branch(&mut self, label: &Label) -> Result<()> {
		self.writeln(&format!("\tbr label %{label}"))
	}

	// Write function header
	pub fn write_function_header(&mut self, name: &str, param_values: &Vec<LLVMValue>, return_fmt: &RegisterFormat) -> Result<()> {
		self.write(&format!("define dso_local {return_type} @{name}(", return_type=return_fmt.format_type()))?;

		for (i, param) in param_values.iter().enumerate() {
			self.write(&format!("{param_type} %arg.{i}{comma}", param_type=param.val_type(), comma={if i < param_values.len() - 1 { "," } else { "" }}))?;
		}

		self.writeln(") #0 {")
	}

	// Write function close
	pub fn write_function_close(&mut self) -> Result<()> {
		self.writeln("}")?;
		self.writeln("")
	}

	// Write function call and put res in trg;
	pub fn write_function_call(&mut self, name: &String, arg_vals: &Vec<LLVMValue>, ret_reg: &LLVMValue) -> Result<()> {
		self.write(&format!("\t{ret_reg} = call {ret_type} @{name}(", ret_type=ret_reg.val_type()))?;

		for (i, arg) in arg_vals.iter().enumerate() {
			self.write(&format!("{arg_type} {arg}", arg_type=arg.val_type()))?;
			if i < arg_vals.len() - 1 {
				self.write(",")?;
			}
		}

		self.writeln(")")
	}

	// Write a ret statement
	pub fn write_ret(&mut self, val: &LLVMValue) -> Result<()> {
		self.writeln(&format!("\tret {val_type} {val}", val_type=val.val_type()))
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