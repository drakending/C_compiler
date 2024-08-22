use crate::ast::{ASTExpression, GrammarVartype};

#[derive(Debug)]
pub enum ASTStatementKind{
    Expression(ASTExpression),
    Declaration(ASTDeclarationList),
}
#[derive(Debug)]
pub struct ASTStatement{
    pub(crate) kind:ASTStatementKind,
}

impl ASTStatement {
    pub fn new(kind: ASTStatementKind) -> Self {
        ASTStatement { kind }
    }
    pub fn expression(expr: ASTExpression) -> Self {
        ASTStatement::new(ASTStatementKind::Expression(expr))
    }
    pub fn declaration(declaration_list: ASTDeclarationList) -> Self {
        ASTStatement::new(ASTStatementKind::Declaration(declaration_list))
    }
}


#[derive(Debug)]
pub enum ASTDeclaration{
    VariableDeclareDirect(String),
    VariableDeclareWithInit(String,ASTExpression),
}

#[derive(Debug)]
pub struct ASTDeclarationList{
    pub(crate) vartype:GrammarVartype,
    pub(crate) declare_list:Vec<ASTDeclaration>
}

impl ASTDeclarationList{
    pub fn new(vartype:GrammarVartype) -> Self{
        Self{vartype,declare_list:Vec::new()}
    }
}


