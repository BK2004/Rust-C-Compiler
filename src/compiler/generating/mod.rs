pub mod writer;
pub mod llvm;

use crate::error::*;

use crate::parsing::ast::{ASTNode, FunctionParameter, Type};
use crate::parsing::Parser;
use crate::scanning::token::*;
use llvm::*;
use writer::Writer;

pub const TYPE_FORMATS: &[(&str, RegisterFormat)] = &[
	("bool", RegisterFormat::Boolean),
	("int", RegisterFormat::Integer),
];

#[derive(Debug)]
pub struct Generator {
	writer: Writer,
	next_register: u32,
	free_register_count: u32,
	label_count: u32,
	local_symbol_table: SymbolTable,
	global_symbol_table: SymbolTable,
}

impl Generator {
	pub fn new(writer: Writer) -> Self {
		Self {
			writer,
			next_register: 1,
			free_register_count: 0,
			label_count: 0,
			local_symbol_table: SymbolTable::new(64),
			global_symbol_table: SymbolTable::new(64),
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

	pub fn local_symbol_table(&self) -> &SymbolTable {
		&self.local_symbol_table
	}

	pub fn global_symbol_table(&self) -> &SymbolTable {
		&self.global_symbol_table
	}

	pub fn generate(&mut self, parser: &mut Parser) -> Result<()> {
		self.writer.write_preamble()?;

		// Allocate variable stack space and write to output
		while let Some(function) = parser.parse_global_statement()? {
			self.free_register_count = self.next_register - 1;

			self.ast_to_llvm(&function, None)?;
		}

		self.writer.write_postamble()?;

		Ok(())
	}

	// Claim next register value and update next register
	pub fn update_virtual_register(&mut self, amt: u32) -> u32 {
		self.next_register += amt;

		self.next_register - amt
	}

	// Update label count and return count prior to update
	pub fn update_label_count(&mut self, amt: u32) -> u32 {
		self.label_count += amt;

		self.label_count - amt
	}

	// Claim free register from available free registers
	pub fn claim_free_register(&mut self) -> u32 {
		self.free_register_count -= 1;

		self.free_register_count + 1
	}

	// Traverse AST and generate LLVM for the tree
	pub fn ast_to_llvm(&mut self, root: &ASTNode, expected_fmt: Option<RegisterFormat>) -> Result<LLVMValue> {
		match root {
			ASTNode::Literal(x) => Ok(self.generate_literal(x)?),
			ASTNode::Binary {token, left, right} => Ok(self.generate_binary(token, *(*left).clone(), *(*right).clone())?),
			ASTNode::Let { name, val_type, value } => Ok(self.generate_let(name, val_type, value)?),
			ASTNode::If { expr, block, else_block } => Ok(self.generate_if(expr, block, else_block, &expected_fmt)?),
			ASTNode::While { expr, block } => Ok(self.generate_while(expr, block, &expected_fmt)?),
			ASTNode::FunctionDefinition { name, parameters, body_block, return_type } => Ok(self.generate_function(name.to_owned(), parameters, body_block, return_type)?),
			ASTNode::Return { return_val } => Ok(self.generate_return(return_val, &expected_fmt)?),
			ASTNode::FunctionCall { name, args } => Ok(self.generate_function_call(name, args)?),
			ASTNode::Print { expr } => Ok(self.generate_print(expr)?),
			ASTNode::Dereference { child } => Ok(self.generate_deref(child)?),
			ASTNode::Reference { child } => Ok(self.generate_ref(child)?),
		}

	}

	// Generate literal value based on given type
	pub fn generate_literal(&mut self, literal: &Literal) -> Result<LLVMValue> {
		match literal {
			Literal::Integer(x) => Ok(LLVMValue::Constant(Constant::Integer(*x))),
			Literal::Identifier(i) => match i {
				Identifier::Symbol(x) => Ok(self.local_symbol_table.get(x)?.value().to_owned()),
				_ => Err(Error::TerminalTokenExpected { received_token: None, received_identifier: Some(i.clone()) })
			},
		}
	}

	// Generate binary statement given operation and left/right LLVMValues
	pub fn generate_binary(&mut self, token: &Token, left: ASTNode, right: ASTNode) -> Result<LLVMValue> {
		let left = self.ast_to_llvm(&left, None)?;
		let right = self.ast_to_llvm(&right, None)?;

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
		self.ensure_rvalue(&mut left)?;
		self.ensure_rvalue(&mut right)?;
		self.ensure_arithmetic_operands(&mut left, &mut right)?;
		let reg = self.update_virtual_register(1);
		self.writer.write_mul(&left, &right, reg)?;

		Ok(LLVMValue::VirtualRegister(VirtualRegister::new(reg.to_string(), RegisterFormat::Integer, true)))
	}

	// Generate LLVMValue for subtraction
	pub fn generate_sub(&mut self, mut left: LLVMValue, mut right: LLVMValue) -> Result<LLVMValue> {
		self.ensure_rvalue(&mut left)?;
		self.ensure_rvalue(&mut right)?;
		self.ensure_arithmetic_operands(&mut left, &mut right)?;
		let reg = self.update_virtual_register(1);
		self.writer.write_sub(&left, &right, reg)?;

		Ok(LLVMValue::VirtualRegister(VirtualRegister::new(reg.to_string(), RegisterFormat::Integer, true)))
	}

	// Generate LLVMValue for addition
	pub fn generate_add(&mut self, mut left: LLVMValue, mut right: LLVMValue) -> Result<LLVMValue> {
		self.ensure_rvalue(&mut left)?;
		self.ensure_rvalue(&mut right)?;
		self.ensure_arithmetic_operands(&mut left, &mut right)?;
		let reg = self.update_virtual_register(1);
		self.writer.write_add(&left, &right, reg)?;

		Ok(LLVMValue::VirtualRegister(VirtualRegister::new(reg.to_string(), RegisterFormat::Integer, true)))
	}

	// Generate LLVMValue for division
	pub fn generate_div(&mut self, mut left: LLVMValue, mut right: LLVMValue) -> Result<LLVMValue> {
		self.ensure_arithmetic_operands(&mut left, &mut right)?;
		let reg = self.update_virtual_register(1);
		self.writer.write_div(&left, &right, reg)?;

		Ok(LLVMValue::VirtualRegister(VirtualRegister::new(reg.to_string(), RegisterFormat::Integer, true)))
	}

	// Generate LLVMValue for assignment of left = right
	pub fn generate_assign(&mut self, left: LLVMValue, mut right: LLVMValue) -> Result<LLVMValue> {
		// Make right an operand, assign it to left, and return left for use again
		self.ensure_rvalue(&mut right)?;
		let mut new_left = left.clone();
		self.ensure_lvalue(&mut new_left)?;

		// Special case: left format is pointer, so check if pointee matches right
		if let RegisterFormat::Pointer { pointee } = left.format() {
			pointee.expect(right.format())?;
		} else {
			left.format().expect(right.format())?;
		}
		self.writer.write_store(&right, &new_left)?;
		Ok(left)
	}

	// Generate LLVMValue for comparison operator
	pub fn generate_comparison(&mut self, operator: Token, mut left: LLVMValue, mut right: LLVMValue) -> Result<LLVMValue> {
		// Make sure both sides are operands, compare them, and store the result as a boolean register
		self.ensure_comparison_operands(&mut left, &mut right, &operator)?;
		let pnemonic = operator.get_pnemonic();
		let reg = self.update_virtual_register(1);
		self.writer.write_cmp(&left, &right, reg, pnemonic)?;
		
		Ok(LLVMValue::VirtualRegister(VirtualRegister::new(reg.to_string(), RegisterFormat::Boolean, true)))
	}

	pub fn generate_let(&mut self, name: &String, val_type: &Option<Type>, value: &Option<Box<ASTNode>>) -> Result<LLVMValue> {
		if let Ok(_) = self.local_symbol_table.get(name) {
			return Err(Error::SymbolDeclared { name: name.to_owned() });
		}

		if let Some(val) = value {
			let mut assigned_llvm = self.ast_to_llvm(&val, None)?;
			self.ensure_rvalue(&mut assigned_llvm)?;
			// If val_type is not given, use implicit format
			let reg_fmt = match val_type {
				Some(v) => self.get_format_from_type(v)?,
				None => assigned_llvm.format(),
			};
			
			if !assigned_llvm.format().can_convert_to(&reg_fmt) {
				Err(Error::InvalidAssignment { received: assigned_llvm.format().to_owned(), expected: reg_fmt.clone() })?;
			}
			let (symbol, reg) = self.local_symbol_table.create_local(name, &reg_fmt);
			self.writer.write_local_alloc(&reg, &reg_fmt)?;
			self.writer.write_store(&assigned_llvm, &LLVMValue::VirtualRegister(reg))?;
			self.local_symbol_table.insert(symbol);
		} else {
			// No value assigned; if value type specified, assign that type; else, assign an int
			let reg_fmt = match val_type {
				Some(v) => self.get_format_from_type(v)?,
				None => RegisterFormat::Integer
			};
			let (symbol, reg) = self.local_symbol_table.create_local(name, &reg_fmt);
			self.writer.write_local_alloc(&reg, &reg_fmt)?;
			self.local_symbol_table.insert(symbol);
		}

		Ok(LLVMValue::None)
	}

	// Generate if statement
	pub fn generate_if(&mut self, expr: &ASTNode, block: &Vec<ASTNode>, else_block: &Option<Vec<ASTNode>>, expected_fmt: &Option<RegisterFormat>) -> Result<LLVMValue> {
		let mut expr_llvm = self.ast_to_llvm(expr, None)?;
		self.ensure_rvalue(&mut expr_llvm)?;
		self.coerce(&mut expr_llvm, RegisterFormat::Boolean)?;

		// Generate a label for if branch.
		// If else is present, generate an else label, emit conditional branch with body label and else label, and parse blocks
		// Else, emit conditional branch with body label and tail label, and parse only body block
		let body_label = Label::new(self.update_label_count(1));
		if let Some(else_block) = else_block {
			let else_label = Label::new(self.update_label_count(1));
			let tail_label = Label::new(self.update_label_count(1));

			self.writer.write_cond_branch(&expr_llvm, &body_label, &else_label)?;

			// Write body portion of if statement
			self.writer.write_label(&body_label)?;

			for block_statement in block {
				self.ast_to_llvm(block_statement, expected_fmt.to_owned())?;
			}

			self.writer.write_branch(&tail_label)?;
			
			// Write else portion
			self.writer.write_label(&else_label)?;
			for else_statement in else_block {
				self.ast_to_llvm(else_statement, expected_fmt.to_owned())?;
			}

			self.writer.write_branch(&tail_label)?;
			self.writer.write_label(&tail_label)?;
		} else {
			// No else statement
			let tail_label = Label::new(self.update_label_count(1));

			self.writer.write_cond_branch(&expr_llvm, &body_label, &tail_label)?;

			// Body portion
			self.writer.write_label(&body_label)?;

			for block_statement in block {
				self.ast_to_llvm(block_statement, None)?;
			}

			self.writer.write_branch(&tail_label)?;
			self.writer.write_label(&tail_label)?;
		}

		Ok(LLVMValue::None)
	}

	pub fn generate_while(&mut self, expr: &ASTNode, block: &Vec<ASTNode>, expected_fmt: &Option<RegisterFormat>) -> Result<LLVMValue> {
		let cond_label = Label::new(self.update_label_count(1));
		let body_label = Label::new(self.update_label_count(1));
		let tail_label = Label::new(self.update_label_count(1));

		self.writer.write_branch(&cond_label)?;
		self.writer.write_label(&cond_label)?;
		let mut expr_llvm = self.ast_to_llvm(expr, None)?;
		self.ensure_rvalue(&mut expr_llvm)?;
		expr_llvm.format().expect(RegisterFormat::Boolean)?;
		self.writer.write_cond_branch(&expr_llvm, &body_label, &tail_label)?;

		// Write body
		self.writer.write_label(&body_label)?;
		for body_statement in block {
			self.ast_to_llvm(body_statement, expected_fmt.to_owned())?;
		}
		self.writer.write_branch(&cond_label)?;

		// Tail
		self.writer.write_label(&tail_label)?;

		Ok(LLVMValue::None)
	}

	// Generate a function, including header and body
	pub fn generate_function(&mut self, name: String, parameters: &Vec<FunctionParameter>, body_block: &Vec<ASTNode>, return_type: &Type) -> Result<LLVMValue> {
		let return_fmt = self.get_format_from_type(return_type)?;
		let mut param_values: Vec<LLVMValue> = Vec::new();
		for (_, param) in parameters.iter().enumerate() {
			param_values.push(LLVMValue::VirtualRegister(VirtualRegister::new(param.name.to_owned(), self.get_format_from_type(&param.param_type)?, true)));
		}

		let signature = FunctionSignature::new(&param_values.iter().map(|p| p.format()).collect(), return_fmt.clone());

		// Write function header, convert args into locals, generate the block statements, and close function definition
		self.writer.write_function_header(&name, &param_values, &return_fmt)?;

		for (i, param) in param_values.iter().enumerate() {
			let arg_reg = VirtualRegister::new("arg.".to_owned() + &i.to_string(), param.format(), true);
			let (local_symbol, local_reg) = self.local_symbol_table.create_local(&parameters[i].name, &param.format());

			self.writer.write_local_alloc(&local_reg, &param.format())?;
			self.writer.write_store(&LLVMValue::VirtualRegister(arg_reg), &LLVMValue::VirtualRegister(VirtualRegister::new(parameters[i].name.to_owned(), local_symbol.value().format().to_pointer(), true)))?;
			self.local_symbol_table.insert(local_symbol);
		}

		// Add to symbol table before parsing body so recursive functions can exist
		let (func_symbol, _func_register) = self.global_symbol_table.create_function(&name, &signature);
		self.global_symbol_table.insert(func_symbol);

		for block_statement in body_block {
			self.ast_to_llvm(block_statement, Some(return_fmt.clone()))?;
		}

		self.writer.write_function_close()?;
		self.free_register_count = 0;
		self.next_register = 1;
		self.local_symbol_table.clear();

		Ok(LLVMValue::None)
	}

	// Generate a return statement
	pub fn generate_return(&mut self, expr: &Option<Box<ASTNode>>, expected_fmt: &Option<RegisterFormat>) -> Result<LLVMValue> {
		let mut val = match expr {
			Some(expr) => self.ast_to_llvm(expr, None)?,
			None => LLVMValue::VirtualRegister(VirtualRegister::new(self.update_virtual_register(0).to_string(), RegisterFormat::Void, true)),
		};
		self.ensure_rvalue(&mut val)?;
		if let Some(fmt) = expected_fmt {
			fmt.expect(val.format())?;
		}

		if let LLVMValue::None = val {
			return Err(Error::ExpressionExpected)
		}

		self.update_virtual_register(1);
		self.writer.write_ret(&val)?;

		Ok(LLVMValue::None)
	}

	// Generate a function call given name and args
	pub fn generate_function_call(&mut self, name: &String, args: &Vec<ASTNode>) -> Result<LLVMValue> {
		// Parse arguments
		let mut arg_vals: Vec<LLVMValue> = Vec::new();
		for node in args {
			arg_vals.push(self.ast_to_llvm(node, None)?);
		}

		for (_, arg) in arg_vals.iter_mut().enumerate() {
			self.ensure_rvalue(arg)?;
		}

		// Get function symbol and check that args match
		let func_symbol = self.global_symbol_table.get(name)?.clone();
		if let Symbol::Function { name, value } = func_symbol {
			// existing function call, check arg types
			if let RegisterFormat::Function { signature } = value.format() {
				// Guaranteed if symbol is function
				for (i, fmt) in signature.params().iter().enumerate() {
					if !fmt.can_convert_to(&arg_vals.get(i).map_or(RegisterFormat::Void, |res| res.format())) {
						return Err(Error::ArgumentMismatch { expected: signature, received: arg_vals })
					}
				}

				// Generate new numbered register for call result if not a void call
				let ret_reg_num = self.update_virtual_register(match signature.return_fmt() { RegisterFormat::Void => 0, _ => 1 });
				let ret_reg = LLVMValue::VirtualRegister(VirtualRegister::new(ret_reg_num.to_string(), signature.return_fmt().to_owned(), true));

				self.writer.write_function_call(&name, &arg_vals, &ret_reg)?;

				return Ok(ret_reg)
			} else {
				// this is just for Rust
				return Ok(LLVMValue::None)
			}
		} else {
			return Err(Error::ExpressionExpected)
		}
	}

	// Generate print statement
	pub fn generate_print(&mut self, expr: &ASTNode) -> Result<LLVMValue> {
		let mut val = self.ast_to_llvm(expr, None)?;
		self.ensure_rvalue(&mut val)?;
		
		if let LLVMValue::None = val {
			return Err(Error::ExpressionExpected)
		}

		self.update_virtual_register(1);
		self.writer.write_print(&val)?;

		Ok(LLVMValue::None)
	}

	// Generate instructions for dereferencing a node
	pub fn generate_deref(&mut self, node: &ASTNode) -> Result<LLVMValue> {
		// Get target to dereference
		let mut trg = self.ast_to_llvm(node, None)?;
		self.ensure_rvalue(&mut trg)?;
		if let RegisterFormat::Pointer { pointee } = trg.format() {
			let ret = LLVMValue::Indirect { pointee: Box::new(trg), referenced_fmt: (*pointee).clone() };

			Ok(ret)
		} else {
			Err(Error::InvalidDereference { received: trg.format() })
		}
	}

	// Generate instructions for creating a reference to a node
	pub fn generate_ref(&mut self, node: &ASTNode) -> Result<LLVMValue> {
		// Get target to reference
		let trg = self.ast_to_llvm(node, None)?;
		dbg!(&trg);
		if let LLVMValue::Indirect { pointee, .. } = trg {
			Ok(*pointee)
		} else {
			Err(Error::ExpectedLValue)
		}
	}

	pub fn load_numbered_register(&mut self, format: RegisterFormat, val: LLVMValue) -> Result<LLVMValue> {
		match val {
			LLVMValue::VirtualRegister(_) | LLVMValue::Indirect { .. } => {
				let reg = self.update_virtual_register(1);
				let reg_val = VirtualRegister::new(reg.to_string(), format, true);

				self.writer.write_load(&val.clone(), &reg_val)?;

				Ok(LLVMValue::VirtualRegister(reg_val))
			},
			_ => Err(Error::UnexpectedLLVMValue { expected: LLVMValue::VirtualRegister(VirtualRegister::new("0".to_string(), format, true)), received: val })
		}
	}

	// Verify that LLVMValues are able to be operated on by arithmetic
	pub fn ensure_arithmetic_operands(&mut self, mut left: &mut LLVMValue, mut right: &mut LLVMValue) -> Result<()> {
		self.ensure_rvalue(&mut left)?;
		self.ensure_rvalue(&mut right)?;

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
	pub fn ensure_comparison_operands(&mut self, mut left: &mut LLVMValue, mut right: &mut LLVMValue, op: &Token) -> Result<()> {
		self.ensure_rvalue(&mut left)?;
		self.ensure_rvalue(&mut right)?;

		let left_fmt = left.format();
		let right_fmt = right.format();

		if left_fmt.can_compare_to(&right_fmt, op) {
			Ok(())
		} else {
			Err(Error::InvalidComparisonOperands { left: left_fmt, right: right_fmt })
		}
	}

	// Ensure LLVMValue is an L-value
	pub fn ensure_lvalue(&mut self, node: &mut LLVMValue) -> Result<()> {
		match node {
			LLVMValue::Indirect { pointee, .. } => {
				*node = (**pointee).clone();
				Ok(())
			},
			_ => Err(Error::ExpectedLValue)
		}
	}

	// Ensure that given LLVM Value is in operatable form
	pub fn ensure_rvalue(&mut self, node: &mut LLVMValue) -> Result<()> {
		match node {
			LLVMValue::None => Err(Error::UnexpectedLLVMValue { expected: LLVMValue::Constant(Constant::Integer(3)), received: node.clone() }),
			LLVMValue::Indirect { pointee, referenced_fmt } => {
				*node = self.load_numbered_register(referenced_fmt.to_owned(), (**pointee).clone())?;
				Ok(())
			},
			_ => Ok(())
		}?;

		Ok(())
	}

	// Coerce LLVMValue to given format
	pub fn coerce(&mut self, value: &mut LLVMValue, new_fmt: RegisterFormat) -> Result<()> {
		let val_fmt = value.format();
		if !val_fmt.can_convert_to(&new_fmt) {
			return Err(Error::BadConversion { from: val_fmt, to: new_fmt })
		}

		// Guaranteed to be one of these pairs
		match (val_fmt, new_fmt) {
			(RegisterFormat::Pointer { .. }, RegisterFormat::Boolean) => {
				let reg = self.update_virtual_register(1);
				let new_val = LLVMValue::VirtualRegister(VirtualRegister::new(reg.to_string(), RegisterFormat::Boolean, true));

				self.writer.write_cmp(&value, &LLVMValue::Null, reg, "ne".to_string())?;
				*value = new_val;

				Ok(())
			},
			_ => {
				// The formats must be equal
				Ok(())
			}
		}
	}

	pub fn get_format_from_type(&mut self, source: &Type) -> Result<RegisterFormat> {
		let fmt = match source {
			Type::Named { type_name } => {
				TYPE_FORMATS.iter().find_map(|type_fmt| if type_name == type_fmt.0 { Some(type_fmt.1.clone()) } else { None }  )
			},
			Type::Pointer { pointee_type } => Some(RegisterFormat::Pointer { pointee: Box::new(self.get_format_from_type(pointee_type)?)}),
			Type::Void => Some(RegisterFormat::Void),
		};

		match fmt {
			Some(type_format) => Ok(type_format),
			None => Err(Error::TypeUnknown { received: source.to_owned() }),
		}
	}
}