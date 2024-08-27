mod ast;
mod diagnostics;
mod text;

use std::cell::RefCell;
use std::rc::Rc;
use std::fs::File;
use std::io::{self,Read};
use crate::ast::Ast;
use crate::ast::parser::Parser;
use crate::ast::evaluator::ASTEvaluator;

fn main() {
    let mut file = File::open("test.c").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input);
    let file_str:&str = &input.clone();
    let text = text::SourceText::new(input);
    let mut lexer = ast::lexer::Lexer::new(file_str);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token(){
        tokens.push(token);
    }
    println!("{:?}",tokens);
    let diagnostics_bag = Rc::new(RefCell::new(diagnostics::DiagnosticBag::new()));

    let mut ast: Ast = Ast::new();
    let mut parser = Parser::new(tokens,diagnostics_bag.clone());
    while let Some(prog) = parser.next_program_unit(){
        ast.add_program_unit(prog);
    }
    let diagnositcs_binding = diagnostics_bag.borrow();
    if diagnositcs_binding.diagnostics.len()>0{
        let diagnostics_printer = diagnostics::printer::DiagnosticsPrinter::new(
          &text,
          &diagnositcs_binding.diagnostics
        );
        diagnostics_printer.print();
        return;
    }
    else {
        ast.visualize();

    }
    let mut eval = ASTEvaluator::new();
    ast.visit(&mut eval);
    println!("{:?}",eval.last_value);

}
