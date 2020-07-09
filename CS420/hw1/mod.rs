use std::ops::Deref;
use std::convert::TryFrom;
use core::cmp::Ordering;
use core::fmt;
use core::mem;
use std::collections::HashMap;
use std::result::Result;

use failure::Fail;
use lang_c::ast::*;

use crate::ir::DtypeError;
use crate::write_base::WriteString;
use crate::*;

use itertools::izip;

#[derive(Default)]
#[allow(dead_code)]
pub struct Irgen {
    decls: HashMap<String, ir::Declaration>,
    typedefs: HashMap<String, ir::Dtype>,
}

#[derive(Debug, PartialEq)]
pub struct IrgenError {
    pub code: String,
    pub message: IrgenErrorMessage,
}

impl fmt::Display for IrgenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error code: {}", self.code)
    }
}

#[derive(Debug, PartialEq, Fail)]
pub enum IrgenErrorMessage {
    #[fail(display = "{}", message)]
    Misc { message: String },
    #[fail(
        display = "called object `{:?}` is not a function or a function pointer",
        callee
    )]
    NeedFunctionOrFunctionPointer { callee: ir::Operand },
    #[fail(display = "redefinition of `{}`", name)]
    Redefinition { name: String },
    #[fail(
        display = "`{}` conflicts prototype's dtype, `{}`",
        dtype, prototype_dtype
    )]
    ConflictingDtype {
        dtype: ir::Dtype,
        prototype_dtype: ir::Dtype,
    },
    #[fail(display = "Invalid dtype: `{}`", dtype_error)]
    InvalidDtype { dtype_error: DtypeError },
    #[fail(display = "l-value required as {}", message)]
    RequireLvalue { message: String },
}

impl Translate<TranslationUnit> for Irgen {
    type Target = ir::TranslationUnit;
    type Error = IrgenError;

    fn translate(&mut self, _unit: &TranslationUnit) -> Result<Self::Target, Self::Error> {
        for ext_decl in &_unit.0 {
            match ext_decl.node {
                ExternalDeclaration::Declaration(ref var) => {
                    self.add_declaration(&var.node)?;
                }
                ExternalDeclaration::StaticAssert(_) => {
                    panic!("ExternalDeclaration::StaticAssert");
                }
                ExternalDeclaration::FunctionDefinition(ref func) => {
                    self.add_function_definition(&func.node)?;
                }
            }
        }

        let decls = mem::replace(&mut self.decls, HashMap::new());
        let structs = todo!("structs");
        Ok(Self::Target { decls, structs })
    }
}

enum ConstantInitializer {
    Constant(ir::Constant),
    List(Vec<ConstantInitializer>),
}

pub enum ConstantDeclaration {
    Variable {
        dtype: ir::Dtype,
        initializer: Option<ConstantInitializer>,
    },
    Function {
        signature: ir::FunctionSignature,
        definition: Option<FunctionDefinition>,
    },
}

impl TryFrom<&Initializer> for ConstantInitializer {
    type Error = IrgenError;

    fn try_from(initializer: &Initializer) -> Result<Self, Self::Error>  {
        match initializer {
            Initializer::Expression(expr) => {
                let expr_node = expr.deref().node;
                let const_from_expr = ir::Constant::try_from(&expr_node).map_err(|_| {
                    IrgenError {
                        code: initializer.write_string(),
                        message: IrgenErrorMessage::Misc {
                            message: "the initializer cannot be converted to a constant at compile time".to_string(),
                        }
                    }
                })?;
                Ok(ConstantInitializer::Constant(const_from_expr))
            }
            Initializer::List(list) => {
                let list: Result<Vec<_>,_> = list
                    .into_iter()
                    .map(|item| {
                        assert!(item.node.designation.is_empty());
                        let item_init: Initializer = item.node.initializer.deref().node;
                        ConstantInitializer::try_from(&item_init)
                    })
                    .collect();
                Ok(Self::List(list?))
            }
        }
    }
}

impl Irgen {
    fn add_declaration(&mut self, source: &Declaration) -> Result<(), IrgenError> {
        let (base_dtype, is_typedef) =
            ir::Dtype::try_from_ast_declaration_specifiers(&source.specifiers).map_err(|e| {
                IrgenError {
                    code: source.write_string(),
                    message: IrgenErrorMessage::InvalidDtype { dtype_error: e },
                }
            })?;

        // typedef int tt;
        // tt *a;
        for init_decl in &source.declarators {
            let declarator = &init_decl.node.declarator.node;
            // get the named version of the declarator
            let name = name_of_declarator(declarator);
            // resolve the pointer types: dtype is going to be `pointer to tt`
            let dtype = base_dtype
                .clone()
                .with_ast_declarator(declarator)
                .map_err(|e| IrgenError {
                    code: source.write_string(),
                    message: IrgenErrorMessage::InvalidDtype { dtype_error: e },
                })?;
            // resolve the typedefs: dtype is going to be `pointer to int`
            let dtype = dtype
                .resolve_typedefs(&self.typedefs, structs)
                .map_err(|e| IrgenError {
                    code: source.write_string(),
                    message: IrgenErrorMessage::InvalidDtype { dtype_error: e },
                })?;

            // ***** typedef case
            // typedef int tt;
            if is_typedef {
                let prev_dtype = self
                    .typedefs
                    .entry(name.clone())
                    .or_insert_with(|| dtype.clone());

                if prev_dtype != &dtype {
                    return Err(IrgenError {
                        code: source.write_string(),
                        message: IrgenErrorMessage::ConflictingDtype {
                            dtype,
                            prototype_dtype: prev_dtype.clone(),
                        },
                    });
                }
            } else {
                // ***** declaration case
                let mut decl =
                    ir::Declaration::try_from(dtype.clone()).map_err(|e| IrgenError {
                        code: source.write_string(),
                        message: IrgenErrorMessage::InvalidDtype { dtype_error: e },
                    })?;

                // checks for the initializer (optional)
                let const_decl = if let Some(initializer) = init_decl.node.initializer.as_ref() {
                    // convert the initializer to a constant value
                    let value = ConstantInitializer::try_from(&(initializer.node))?;

                    match decl {
                        ir::Declaration::Variable {
                            initializer: var_initializer,
                            dtype: dtype,
                        } => {
                            if var_initializer.is_some() {
                                // redefinition!
                                return Err(IrgenError {
                                    code: source.write_string(),
                                    message: IrgenErrorMessage::Redefinition { name },
                                });
                            }

                            // TODO: implicit type casting

                            ConstantDeclaration::Variable {
                                dtype,
                                initializer: Some(value)
                            }
                        }
                        ir::Declaration::Function { .. } => {
                            return Err(IrgenError {
                                code: source.write_string(),
                                message: IrgenErrorMessage::Misc {
                                    message: "a function cannot be initialized".to_string(),
                                },
                            })
                        }
                    }
                } else {
                    
                }
                self.add_decl(&name, decl)?;
            }
        }
        Ok(())
    }

    fn add_decl(&mut self, name: &str, decl: ir::Declaration) -> Result<(), IrgenError> {
        let old_decl = some_or!(
            self.decls.insert(name.to_string(), decl.clone()),
            return Ok(())
        );

        if !old_decl.is_compatible(&decl) {
            return Err(IrgenError {
                code: name.to_string(),
                message: IrgenErrorMessage::ConflictingDtype {
                    dtype: old_decl.dtype(),
                    prototype_dtype: decl.dtype(),
                }
            });
        }

        Ok(())
    }
}


