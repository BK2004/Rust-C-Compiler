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

	pub fn symbol_table(&self) -> &SymbolTable {
		&self.symbol_table
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
			ASTNode::Binary {token, left, right} => Ok(self.generate_binary(token, *(*left).clone(), *(*right).clone())?),
			ASTNode::Let { name, value } => Ok(self.generate_let(name, value)?),
			ASTNode::Print { expr } => Ok(self.generate_print(expr)?),
		}

	}

	// Generate literal value based on given type
	pub fn generate_literal(&mut self, literal: &Literal) -> Result<LLVMValue> {
		match literal {
			Literal::Integer(x) => Ok(LLVMValue::Constant(Constant::Integer(*x))),
			Literal::Identifier(i) => match i {
				Identifier::Symbol(x) => Ok(LLVMValue::VirtualRegister(VirtualRegister::from_identifier(x.to_owned(), i.to_owned(), self.symbol_table())?)),
				_ => Err(Error::TerminalTokenExpected { received_token: None, received_identifier: Some(i.clone()) })
			},
		}
	}

	// Generate binary statement given operation and left/right LLVMValues
	pub fn generate_binary(&mut self, token: &Token, left: ASTNode, right: ASTNode) -> Result<LLVMValue> {
		let left = self.ast_to_llvm(&left)?;
		let right = self.ast_to_llvm(&right)?;

		let out = match token {
			Token::Asterisk => Ok(self.generate_mul(left, right)?),
			Token::Minus => Ok(self.generate_sub(left, right)?),
			Token::Plus => Ok(self.generate_add(left, right)?),
			Token::Slash => Ok(self.generate_div(left, right)?),
			Token::Equals => Ok(self.generate_assign(left, right)?),
			_ => {
				// If token is a comparison operator, generate a comparison
				if token.is_comparison() {
					Ok(self.generate_comparison(token.to_owned(), left, right)?)
				} else {
					Err(Error::BinaryOperatorExpected { received: token.clone() })
				}
			}
		}?;

		Ok(out)
	}

	// Generate LLVMValue for multiplication
	pub fn generate_mul(&mut self, mut left: LLVMValue, mut right: LLVMValue) -> Result<LLVMValue> {
		self.ensure_literals(&mut[&mut left, &mut right])?;
		self.ensure_arithmetic_operands(&mut left, &mut right)?;
		let reg = self.update_virtual_register(1);
		self.writer.write_mul(&left, &right, reg)?;

		Ok(LLVMValue::VirtualRegister(VirtualRegister::new(reg.to_string(), RegisterFormat::Integer)))
	}

	// Generate LLVMValue for subtraction
	pub fn generate_sub(&mut self, mut left: LLVMValue, mut right: LLVMValue) -> Result<LLVMValue> {
		self.ensure_literals(&mut[&mut left, &mut right])?;
		self.ensure_arithmetic_operands(&mut left, &mut right)?;
		let reg = self.update_virtual_register(1);
		self.writer.write_sub(&left, &right, reg)?;

		Ok(LLVMValue::VirtualRegister(VirtualRegister::new(reg.to_string(), RegisterFormat::Integer)))
	}

	// Generate LLVMValue for addition
	pub fn generate_add(&mut self, mut left: LLVMValue, mut right: LLVMValue) -> Result<LLVMValue> {
		self.ensure_literals(&mut[&mut left, &mut right])?;
		self.ensure_arithmetic_operands(&mut left, &mut right)?;
		let reg = self.update_virtual_register(1);
		self.writer.write_add(&left, &right, reg)?;

		Ok(LLVMValue::VirtualRegister(VirtualRegister::new(reg.to_string(), RegisterFormat::Integer)))
	}

	// Generate LLVMValue for division
	pub fn generate_div(&mut self, mut left: LLVMValue, mut right: LLVMValue) -> Result<LLVMValue> {
		self.ensure_arithmetic_operands(&mut left, &mut right)?;
		let reg = self.update_virtual_register(1);
		self.writer.write_div(&left, &right, reg)?;

		Ok(LLVMValue::VirtualRegister(VirtualRegister::new(reg.to_string(), RegisterFormat::Integer)))
	}

	// Generate LLVMValue for assignment of left = right
	pub fn generate_assign(&mut self, left: LLVMValue, mut right: LLVMValue) -> Result<LLVMValue> {
		// Make right an operand, assign it to left, and return left for use again
		self.ensure_literals(&mut[&mut right])?;
		left.format().expect(&right.format())?;
		self.writer.write_store(&right, &left)?;
		Ok(left)
	}

	// Generate LLVMValue for comparison operator
	pub fn generate_comparison(&mut self, operator: Token, mut left: LLVMValue, mut right: LLVMValue) -> Result<LLVMValue> {
		// Make sure both sides are operands, compare them, and store the result as a boolean register
		self.ensure_comparison_operands(&mut left, &mut right, &operator)?;
		let pnemonic = operator.get_pnemonic();
		let reg = self.update_virtual_register(1);
		self.writer.write_cmp(&left, &right, reg, pnemonic)?;
		
		Ok(LLVMValue::VirtualRegister(VirtualRegister::new(reg.to_string(), RegisterFormat::Boolean)))
	}

	pub fn generate_let(&mut self, name: &String, value: &Option<Box<ASTNode>>) -> Result<LLVMValue> {
		if let Ok(_) = self.symbol_table.get(name) {
			return Err(Error::SymbolDeclared { name: name.to_owned() });
		}

		if let Some(val) = value {
			let assigned_llvm = self.ast_to_llvm(&val)?;
			let (symbol, reg) = self.symbol_table.create_local(name, &assigned_llvm.format());
			self.writer.write_local_alloc(&reg, &assigned_llvm.format())?;
			self.writer.write_store(&assigned_llvm, &LLVMValue::VirtualRegister(reg))?;
			self.symbol_table.insert(symbol);
		} else {
			// No value assigned; allocate an int
			let (symbol, reg) = self.symbol_table.create_local(name, &RegisterFormat::Integer);
			self.writer.write_local_alloc(&reg, &RegisterFormat::Integer)?;
			self.symbol_table.insert(symbol);
		}

		Ok(LLVMValue::None)
	}

	// Generate print statement
	pub fn generate_print(&mut self, expr: &ASTNode) -> Result<LLVMValue> {
		let mut val = self.ast_to_llvm(expr)?;
		self.ensure_literals(&mut[&mut val])?;
		
		if let LLVMValue::None = val {
			return Err(Error::ExpressionExpected)
		}

		self.update_virtual_register(1);
		self.writer.write_print(&val)?;

		Ok(LLVMValue::None)
	}

	pub fn load_numbered_register(&mut self, format: RegisterFormat, val: LLVMValue) -> Result<LLVMValue> {
		if let LLVMValue::VirtualRegister(_) = &val {
			let reg = self.update_virtual_register(1);
			let reg_val = VirtualRegister::new(reg.to_string(), format);

			self.writer.write_load(&val.clone(), &reg_val)?;

			Ok(LLVMValue::VirtualRegister(reg_val))
		} else {
			Err(Error::UnexpectedLLVMValue { expected: LLVMValue::VirtualRegister(VirtualRegister::new("0".to_string(), RegisterFormat::Integer)), received: val })
		}
	}

	// Verify that LLVMValues are able to be operated on by arithmetic
	pub fn ensure_arithmetic_operands(&mut self, left: &mut LLVMValue, right: &mut LLVMValue) -> Result<()> {
		self.ensure_literals(&mut[left, right])?;

		if let RegisterFormat::Integer = left.format() {
			if let RegisterFormat::Integer = right.format() {
				Ok(())
			} else {
				Err(Error::InvalidArithmeticOperand { received: right.format() })
			}
		} else {
			Err(Error::InvalidArithmeticOperand { received: left.format() })
		}
	}

	// Verify that left and right can be compared
	pub fn ensure_comparison_operands(&mut self, left: &mut LLVMValue, right: &mut LLVMValue, op: &Token) -> Result<()> {
		self.ensure_literals(&mut[left, right])?;

		let left_fmt = left.format();
		let right_fmt = right.format();

		if left_fmt.can_compare_to(&right_fmt, op) {
			Ok(())
		} else {
			Err(Error::InvalidComparisonOperands { left: left_fmt, right: right_fmt })
		}
	}

	// Ensure that given LLVM Values are in operatable form
	pub fn ensure_literals(&mut self, nodes: &mut [&mut LLVMValue]) -> Result<()> {
		for (_, val) in nodes.iter_mut().enumerate() {
			match val {
				LLVMValue::None => Err(Error::UnexpectedLLVMValue { expected: LLVMValue::Constant(Constant::Integer(3)), received: val.clone() }),
				LLVMValue::VirtualRegister(r) => {
					match r.format() {
						RegisterFormat::Identifier { id_type } => {
							**val = self.load_numbered_register(*id_type.to_owned(), val.clone())?;

							Ok(())
						}, 
						RegisterFormat::Pointer { pointee } => {
							**val = self.load_numbered_register(*pointee.clone(), val.clone())?;

							Ok(())
						},
						_ => Ok(()),
					}
				},
				_ => Ok(())
			}?;
		}

		Ok(())
	}

	// Interpret an AST recursively
	pub fn interpret_ast(&self, node: &crate::parsing::ast::ASTNode) -> Result<i64> {
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