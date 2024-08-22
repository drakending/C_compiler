mod ast;
mod diagnostics;
mod text;

use std::cell::RefCell;
use std::rc::Rc;
use crate::ast::Ast;
use crate::ast::parser::Parser;
use crate::ast::evaluator::ASTEvaluator;


fn main() {
    let input = "int a = 5,b = a + 10; a = a+b; b = b + (a = a + b); a; ";
    let text = text::SourceText::new(input.to_string());
    let mut lexer = ast::lexer::Lexer::new(input);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token(){
        tokens.push(token);
    }
    println!("{:?}",tokens);
    let diagnostics_bag = Rc::new(RefCell::new(diagnostics::DiagnosticBag::new()));

    let mut ast: Ast = Ast::new();
    let mut parser = Parser::new(tokens,diagnostics_bag.clone());
    while let Some(stmt) = parser.next_statement(){
        ast.add_statement(stmt);
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
