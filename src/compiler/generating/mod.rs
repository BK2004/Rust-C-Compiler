pub mod writer;
pub mod llvm;

use crate::error::*;

use crate::parsing::ast::ASTNode;
use crate::parsing::Parser;
use crate::scanning::token::*;
use llvm::*;
use writer::Writer;

#[derive(Debug)]
pub struct Generator {
	writer: Writer,
	loaded_registers: Vec<LLVMValue>,
	next_register: u32,
	free_register_count: u32,
}

impl Generator {
	pub fn new(writer: Writer) -> Self {
		Self {
			writer,
			next_register: 1,
			free_register_count: 0,
			loaded_registers: Vec::new(),
		}
	}

	pub fn from_filename(filename: String) -> Result<Self> {
		Writer::from_filename(filename)
			.map(|writer| Self::new(writer))
			.map_err(|cause| Error::FileOpenError { cause })
	}

	pub fn writer(&self) -> &Writer {
		&self.writer
	}

	pub fn next_virtual_register(&self) -> u32 {
		self.next_register
	}

	pub fn generate(&mut self, parser: &mut Parser) -> Result<()> {
		self.writer.write_preamble()?;

		// Allocate variable stack space and write to output
		while let Some(statement) = parser.parse_statement()? {
			self.free_register_count = self.next_register - 1;
			let alloc_list = self.determine_binary_expression_stack_allocation(&statement.1)?;
			self.allocate_stack(alloc_list)?;

			let mut root = self.ast_to_llvm(&statement.1)?;
			self.ensure_registers_loaded(&mut[&mut root])?;

			match statement.0 {
				Identifier::Print => {
					self.print_int(&root)?;
				},
				Identifier::Pascal => {
					self.pascal(self.interpret_ast(&statement.1)? as u32)?;
				}
			};
		}

		self.writer.write_postamble()?;

		Ok(())
	}

	// Claim next register value and update next register
	pub fn update_virtual_register(&mut self, amt: u32) -> u32 {
		self.next_register += amt;

		self.next_register - amt
	}

	// Claim free register from available free registers
	pub fn claim_free_register(&mut self) -> u32 {
		self.free_register_count -= 1;

		self.free_register_count + 1
	}

	// Determines stack allocations for expression
	pub fn determine_binary_expression_stack_allocation(&mut self, root: &ASTNode) -> Result<Vec<LLVMStackEntry>> {
		match root {
			ASTNode::Literal(Literal::Integer(_)) => {
				self.free_register_count += 1;
				Ok([LLVMStackEntry::new(LLVMValue::VirtualRegister(VirtualRegister::new(self.update_virtual_register(1).to_string(), RegisterFormat::Integer)), 4)].to_vec())
			},
			ASTNode::Binary{token: _, left, right} => {
				let mut left_allocs = self.determine_binary_expression_stack_allocation(&left)?;
				left_allocs.append(&mut self.determine_binary_expression_stack_allocation(&right)?);

				return Ok(left_allocs);
			}
		}
	}

	// Allocates stack space given list of stack entries
	pub fn allocate_stack(&mut self, entries: Vec<LLVMStackEntry>) -> Result<()> {
		for (_, entry) in entries.iter().enumerate() {
			self.writer.write_alloc(entry)?;
		}

		Ok(())
	}

	// Ensures registers in a list are loaded; if not, they have new registers loaded and references are updated
	pub fn ensure_registers_loaded(&mut self, registers: &mut[&mut LLVMValue]) -> Result<()> {
		for i in 0..registers.len() {
			if let LLVMValue::VirtualRegister(reg) = registers[i] {
				let mut loaded = false;

				for (__, loaded_reg) in self.loaded_registers.iter().enumerate() {
					if let LLVMValue::VirtualRegister(check) = loaded_reg {
						if reg.id() == check.id() {
							loaded = true;
						}
					}
				}

				if !loaded {
					// If not loaded, load a new register with old one
					let new_reg_id = self.update_virtual_register(1);
					;
					self.writer.write_load(reg.id().parse().map_err(|cause| Error::StringParseError { cause })?, new_reg_id)?;

					*registers[i] = LLVMValue::VirtualRegister(VirtualRegister::new(new_reg_id.to_string(), RegisterFormat::Integer));

					self.loaded_registers.push(registers[i].clone());
				}
			}
		}

		Ok(())
	}

	// Traverse AST and generate LLVM for the tree
	pub fn ast_to_llvm(&mut self, root: &ASTNode) -> Result<LLVMValue> {
		match root {
			ASTNode::Literal(x) => Ok(self.generate_literal(x)?),
			ASTNode::Binary {token, left, right} => {
				// Convert left and right children to llvm values
				let left_llvm = self.ast_to_llvm(&left)?;
				let right_llvm = self.ast_to_llvm(&right)?;

				return Ok(self.generate_binary(token, left_llvm, right_llvm)?);
			}
		}

	}

	// Print integer
	pub fn print_int(&mut self, reg: &LLVMValue) -> Result<()> {
		Ok(match reg {
			LLVMValue::VirtualRegister(reg) => {
				// Printing int returns value so register count needs to increase
				self.update_virtual_register(1);
				self.writer.print_int(reg.id().parse().map_err(|cause| Error::StringParseError { cause })?)
			},
			LLVMValue::Constant(Constant::Integer(x)) => self.writer.print_int(*x as u32),
			_ => Err(Error::UnexpectedLLVMValue { expected: LLVMValue::VirtualRegister(VirtualRegister::new("0".to_string(), RegisterFormat::Integer)), received: reg.clone() })
		}?)
	}

	// Calculate nth row of Pascal's triangle
	pub fn pascal(&mut self, n: u32) -> Result<()> {
		let mut res: u32 = 0;

		let mut n_fact = 1;
		for i in 1..n+1 {
			n_fact *= i;
		}

		for i in 0..n+1 {
			let mut r_fact = 1;
			for j in 1..i+1 {
				r_fact *= j;
			}

			let mut n_minus_r_fact = 1;
			for j in 1..(n-i+1) {
				n_minus_r_fact *= j;
			}

			res += n_fact / (r_fact * n_minus_r_fact);
		}

		let reg = self.update_virtual_register(1);
		self.writer.writeln(&format!("\t%{} = add i32 {}, 0", reg, res))?;
		self.print_int(&LLVMValue::VirtualRegister(VirtualRegister::new(reg.to_string(), RegisterFormat::Integer)))?;

		Ok(())
	}

	// Generate constant given literal
	pub fn generate_literal(&mut self, literal: &Literal) -> Result<LLVMValue> {
		let reg = self.claim_free_register();
		self.writer.write_literal(literal, reg)?;

		match literal {
			Literal::Integer(_) => Ok(LLVMValue::VirtualRegister(VirtualRegister::new(reg.to_string(), RegisterFormat::Integer))),
		}
	}

	// Generate binary statement given operation and left/right LLVMValues
	pub fn generate_binary(&mut self, token: &Token, mut left: LLVMValue, mut right: LLVMValue) -> Result<LLVMValue> {
		self.ensure_registers_loaded(&mut[&mut left, &mut right])?;
		let out = match token {
			Token::Asterisk => Ok(self.generate_mul(left, right)?),
			Token::Minus => Ok(self.generate_sub(left, right)?),
			Token::Plus => Ok(self.generate_add(left, right)?),
			Token::Slash => Ok(self.generate_div(left, right)?),
			_ => Err(Error::BinaryOperatorExpected { received: *token })
		}?;

		self.loaded_registers.push(out.clone());

		Ok(out)
	}

	// Generate LLVMValue for multiplication
	pub fn generate_mul(&mut self, left: LLVMValue, right: LLVMValue) -> Result<LLVMValue> {
		let reg = self.update_virtual_register(1);
		self.writer.write_mul(&left, &right, reg)?;

		Ok(LLVMValue::VirtualRegister(VirtualRegister::new(reg.to_string(), RegisterFormat::Integer)))
	}

	// Generate LLVMValue for subtraction
	pub fn generate_sub(&mut self, left: LLVMValue, right: LLVMValue) -> Result<LLVMValue> {
		let reg = self.update_virtual_register(1);
		self.writer.write_sub(&left, &right, reg)?;

		Ok(LLVMValue::VirtualRegister(VirtualRegister::new(reg.to_string(), RegisterFormat::Integer)))
	}

	// Generate LLVMValue for addition
	pub fn generate_add(&mut self, left: LLVMValue, right: LLVMValue) -> Result<LLVMValue> {
		let reg = self.update_virtual_register(1);
		self.writer.write_add(&left, &right, reg)?;

		Ok(LLVMValue::VirtualRegister(VirtualRegister::new(reg.to_string(), RegisterFormat::Integer)))
	}

	// Generate LLVMValue for division
	pub fn generate_div(&mut self, left: LLVMValue, right: LLVMValue) -> Result<LLVMValue> {
		let reg = self.update_virtual_register(1);
		self.writer.write_div(&left, &right, reg)?;

		Ok(LLVMValue::VirtualRegister(VirtualRegister::new(reg.to_string(), RegisterFormat::Integer)))
	}

	// Interpret an AST recursively
	pub fn interpret_ast(&self, node: &crate::parsing::ast::ASTNode) -> Result<i32> {
		match node {
			crate::parsing::ast::ASTNode::Literal(Literal::Integer(x)) => Ok(*x),
			crate::parsing::ast::ASTNode::Binary{token, left, right} => {
				let left_res = self.interpret_ast(&left)?;
				let right_res = self.interpret_ast(&right)?;
	
				return match token {
					Token::Asterisk => Ok(left_res * right_res),
					Token::Minus => Ok(left_res - right_res),
					Token::Plus => Ok(left_res + right_res),
					Token::Slash => Ok(left_res / right_res),
					_ => Err(Error::BinaryOperatorExpected { received: token.clone() })
				};
			}
		}
	}
}