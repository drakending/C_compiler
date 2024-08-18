mod ast;
mod diagnostics;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use crate::ast::Ast;
use crate::ast::parser::Parser;
use crate::ast::evaluator::ASTEvaluator;


fn main() {
    let input = "7 - ( 30 + 7 ) * 8 / 2";
    let mut lexer = ast::lexer::Lexer::new(input);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token(){
        tokens.push(token);
    }
    println!("{:?}",tokens);
    let diagnostics_bag = Rc::new(RefCell::new(diagnostics::DiagnosticBag::new()));

    let mut ast: Ast = Ast::new();
    let mut parser = Parser::new(tokens,diagnostics_bag);
    while let Some(stmt) = parser.next_statement(){
        ast.add_statement(stmt);
    }
    ast.visualize();
    let mut eval = ASTEvaluator::new();
    ast.visit(&mut eval);
    println!("{:?}",eval.last_value);
}
