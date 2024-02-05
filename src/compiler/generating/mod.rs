pub mod writer;
pub mod llvm;

use std::io::Result;

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
}

impl Generator {
	pub fn new(writer: Writer) -> Self {
		Self {
			writer,
			next_register: 1,
			free_register_count: 0
		}
	}

	pub fn from_filename(filename: String) -> Result<Self> {
		Writer::from_filename(filename)
			.and_then(|writer| Ok(Self::new(writer)))
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
		let alloc_list = self.determine_binary_expression_stack_allocation(&parser.parse_binary_operation(0)?)?;
		self.allocate_stack(alloc_list)?;

		self.writer.write_postamble()?;

		Ok(())
	}

	// Claim next register value and update next register
	pub fn claim_virtual_register(&mut self) -> u32 {
		self.next_register += 1;

		self.next_register - 1
	}

	// Determines stack allocations for expression
	pub fn determine_binary_expression_stack_allocation(&mut self, root: &ASTNode) -> Result<Vec<LLVMStackEntry>> {
		match root {
			ASTNode::Literal(Literal::Integer(x)) => Ok([LLVMStackEntry::new(LLVMValue::VirtualRegister(self.claim_virtual_register()), 4)].to_vec()),
			ASTNode::Binary{token, left, right} => {
				let mut left_allocs = self.determine_binary_expression_stack_allocation(&left)?;
				left_allocs.append(&mut self.determine_binary_expression_stack_allocation(&right)?);

				return Ok(left_allocs);
			}
		}
	}

	// Allocates stack space given list of stack entries
	pub fn allocate_stack(&mut self, entries: Vec<LLVMStackEntry>) -> Result<()> {
		for (i, entry) in entries.iter().enumerate() {
			self.writer.write_alloc(entry)?;
		}

		Ok(())
	}

	// Traverse AST and generate LLVM for the tree
	// pub fn ast_to_llvm(&mut self, root: &ASTNode) -> Result<LLVMValue> {
		

	// }
}