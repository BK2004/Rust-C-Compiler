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
	next_register: u32,
	free_register_count: u32,
	symbol_table: SymbolTable,
}

impl Generator {
	pub fn new(writer: Writer) -> Self {
		Self {
			writer,
			next_register: 1,
			free_register_count: 0,
			symbol_table: SymbolTable::new(64),
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

			self.ast_to_llvm(&statement)?;
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

	// Traverse AST and generate LLVM for the tree
	pub fn ast_to_llvm(&mut self, root: &ASTNode) -> Result<LLVMValue> {
		match root {
			ASTNode::Literal(x) => Ok(self.generate_literal(x)?),
			ASTNode::Binary {token, left, right} => {
				// Convert left and right children to llvm values
				let left_llvm = self.ast_to_llvm(&left)?;
				let right_llvm = self.ast_to_llvm(&right)?;

				return Ok(self.generate_binary(token, left_llvm, right_llvm)?);
			},
			ASTNode::Let { name, value } => Ok(self.generate_let(name, value)?),
			ASTNode::Print { expr } => Ok(self.generate_print(expr)?),
		}

	}

	// Generate literal value based on given type
	pub fn generate_literal(&mut self, literal: &Literal) -> Result<LLVMValue> {
		match literal {
			Literal::Integer(x) => Ok(LLVMValue::Constant(Constant::Integer(*x))),
			Literal::Identifier(i) => match i {
				Identifier::Symbol(x) => Ok(LLVMValue::VirtualRegister(VirtualRegister::new(x.to_owned(), RegisterFormat::Identifier))),
				_ => Err(Error::TerminalTokenExpected { received_token: None, received_identifier: Some(i.clone()) })
			},
		}
	}

	// Generate binary statement given operation and left/right LLVMValues
	pub fn generate_binary(&mut self, token: &Token, mut left: LLVMValue, mut right: LLVMValue) -> Result<LLVMValue> {
		self.ensure_literals(&mut[&mut left, &mut right])?;

		let out = match token {
			Token::Asterisk => Ok(self.generate_mul(left, right)?),
			Token::Minus => Ok(self.generate_sub(left, right)?),
			Token::Plus => Ok(self.generate_add(left, right)?),
			Token::Slash => Ok(self.generate_div(left, right)?),
			_ => Err(Error::BinaryOperatorExpected { received: token.clone() })
		}?;

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

	pub fn generate_let(&mut self, name: &String, value: &Option<Box<ASTNode>>) -> Result<LLVMValue> {
		if let Ok(_) = self.symbol_table.get(name) {
			return Err(Error::SymbolDeclared { name: name.to_owned() });
		}

		// Write an allocate statement; if there is a value given, then store that value in the local variable
		let (symbol, reg) = self.symbol_table.create_local(name, &RegisterFormat::Integer);
		self.writer.write_local_alloc(&reg)?;

		if let Some(val) = value {
			let assigned_llvm = self.ast_to_llvm(&val)?;

			self.writer.write_store(&assigned_llvm, &LLVMValue::VirtualRegister(reg))?;
		}

		self.symbol_table.insert(symbol);

		Ok(LLVMValue::None)
	}

	// Generate print statement
	pub fn generate_print(&mut self, expr: &ASTNode) -> Result<LLVMValue> {
		let val = self.ast_to_llvm(expr)?;
		
		if let LLVMValue::None = val {
			return Err(Error::ExpressionExpected)
		}

		self.writer.write_print(&val)?;

		Ok(LLVMValue::None)
	}

	pub fn load_numbered_register(&mut self, val: LLVMValue) -> Result<LLVMValue> {
		if let LLVMValue::VirtualRegister(_) = &val {
			let reg = self.update_virtual_register(1);
			let reg_val = LLVMValue::VirtualRegister(VirtualRegister::new(reg.to_string(), RegisterFormat::Integer));

			self.writer.write_load(&val.clone(), &reg_val)?;

			Ok(reg_val)
		} else {
			Err(Error::UnexpectedLLVMValue { expected: LLVMValue::VirtualRegister(VirtualRegister::new("0".to_string(), RegisterFormat::Integer)), received: val })
		}
	}

	// Ensure that given LLVM Values are in operatable form
	pub fn ensure_literals(&mut self, nodes: &mut [&mut LLVMValue]) -> Result<()> {
		for (_, val) in nodes.iter_mut().enumerate() {
			match val {
				LLVMValue::None => Err(Error::UnexpectedLLVMValue { expected: LLVMValue::Constant(Constant::Integer(3)), received: val.clone() }),
				LLVMValue::VirtualRegister(r) => {
					match r.format() {
						RegisterFormat::Integer => Ok(()),
						RegisterFormat::Identifier => {
							**val = self.load_numbered_register(val.clone())?;

							Ok(())
						}
					}
				},
				_ => Ok(())
			}?;
		}

		Ok(())
	}

	// Interpret an AST recursively
	pub fn interpret_ast(&self, node: &crate::parsing::ast::ASTNode) -> Result<i32> {
		match node {
			ASTNode::Literal(Literal::Integer(x)) => Ok(*x),
			ASTNode::Binary{token, left, right} => {
				let left_res = self.interpret_ast(&left)?;
				let right_res = self.interpret_ast(&right)?;
	
				return match token {
					Token::Asterisk => Ok(left_res * right_res),
					Token::Minus => Ok(left_res - right_res),
					Token::Plus => Ok(left_res + right_res),
					Token::Slash => Ok(left_res / right_res),
					_ => Err(Error::BinaryOperatorExpected { received: token.clone() })
				};
			},
			_ => Err(Error::UnexpectedEOF { expected: Token::Semicolon }) // interpreter is solely for the purpose of binary expressions
		}
	}
}