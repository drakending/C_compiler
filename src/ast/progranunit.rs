use crate::ast::expression::ASTBinaryExpression;
use crate::ast::{GrammarFunctiontype, GrammarVartype};
use crate::ast::statement::*;

#[derive(Debug)]
pub enum ASTProgramunitKind{
    Function(ASTFunction),
    Declaration(ASTDeclarationList),
}
#[derive(Debug)]
pub struct ASTProgramunit{
    pub(crate) kind:ASTProgramunitKind,
}
impl ASTProgramunit{
    pub fn function(function:ASTFunction) -> Self{
        ASTProgramunit{kind:ASTProgramunitKind::Function(function)}
    }
    pub fn declaration(declaration_list: ASTDeclarationList) -> Self{
        ASTProgramunit{kind:ASTProgramunitKind::Declaration(declaration_list)}
    }
}


#[derive(Debug,Clone)]
pub struct ASTFunction{
    pub(crate) function_type:GrammarFunctiontype,
    pub(crate) name:String,
    pub(crate) statements:Vec<ASTStatement>,
    pub(crate) params:Vec<ASTFunctonParam>,
}

impl ASTFunction{
    pub fn new(function_type:GrammarFunctiontype,params:Vec<ASTFunctonParam>) -> Self{
        Self{
            function_type,
            name:String::new(),
            statements:Vec::new(),
            params,
        }
    }
    
}

#[derive(Debug,Clone)]
pub struct ASTFunctonParam{
    pub (crate) param_type:GrammarVartype,
    pub (crate) name:String,
}
impl ASTFunctonParam{
    pub fn new(param_type:GrammarVartype,name:String) -> Self {
        Self {
            param_type,
            name,
        }
    }
}