use crate::codegen::symbol::ScopedSymbolTable;

use super::typed_ast;
use crate::ast;

pub fn infer_types_pass(program: ast::Program) -> typed_ast::Program {
    let mut new_program = typed_ast::Program(vec![]);

    let mut names: ScopedSymbolTable<typed_ast::Type> = ScopedSymbolTable::new();

    for stmt in program.0 {
        new_program.0.push(stmt.to_typed(&mut names));
    }

    new_program
}

trait ToTyped {
    type Typed;

    fn to_typed(self, names: &mut ScopedSymbolTable<typed_ast::Type>) -> Self::Typed;
}

impl ToTyped for ast::TopLevelStmt {
    type Typed = typed_ast::TopLevelStmt;

    fn to_typed(self, names: &mut ScopedSymbolTable<typed_ast::Type>) -> Self::Typed {
        use ast::TopLevelStmt;

        match self {
            TopLevelStmt::FunctionDef(def) => {
                typed_ast::TopLevelStmt::FunctionDef(def.to_typed(names))
            }
            TopLevelStmt::ExternDef(def) => typed_ast::TopLevelStmt::ExternDef(def.to_typed(names)),
            TopLevelStmt::ConstDef(def) => typed_ast::TopLevelStmt::ConstDef(def.to_typed(names)),
        }
    }
}

impl ToTyped for ast::FunctionDef {
    type Typed = typed_ast::FunctionDef;

    fn to_typed(self, names: &mut ScopedSymbolTable<typed_ast::Type>) -> Self::Typed {
        let name = self.name;
        let params = self
            .params
            .into_iter()
            .map(|binding| binding.to_typed(names))
            .collect();
        let return_type = self
            .return_type
            .map(|t| t.to_typed(names))
            .unwrap_or(typed_ast::Type::NoneType);

        let body = self.body.to_typed(names);

        names.add_symbol(name.clone(), return_type.clone()).unwrap();

        typed_ast::FunctionDef {
            name,
            params,
            body,
            return_type,
        }
    }
}

impl ToTyped for ast::ExternDef {
    type Typed = typed_ast::ExternDef;

    fn to_typed(self, names: &mut ScopedSymbolTable<typed_ast::Type>) -> Self::Typed {
        let return_type = self
            .return_type
            .map(|t| t.to_typed(names))
            .unwrap_or(typed_ast::Type::NoneType);

        names
            .add_symbol(self.name.clone(), return_type.clone())
            .unwrap();

        typed_ast::ExternDef {
            name: self.name,
            params: self
                .params
                .into_iter()
                .map(|binding| binding.to_typed(names))
                .collect(),
            return_type: return_type,
        }
    }
}

impl ToTyped for ast::GlobalConstDef {
    type Typed = typed_ast::GlobalConstDef;

    fn to_typed(self, names: &mut ScopedSymbolTable<typed_ast::Type>) -> Self::Typed {
        let new_binding = self.binding.to_typed(names);
        names
            .add_symbol(new_binding.name.clone(), new_binding.ty.clone())
            .unwrap();
        typed_ast::GlobalConstDef {
            binding: new_binding,
            value: self.value.to_typed(names),
        }
    }
}

impl ToTyped for ast::Stmt {
    type Typed = typed_ast::Stmt;

    fn to_typed(self, names: &mut ScopedSymbolTable<typed_ast::Type>) -> Self::Typed {
        use ast::Stmt;
        match self {
            Stmt::BlockStmt(stmts) => typed_ast::Stmt::BlockStmt(stmts.to_typed(names)),
            Stmt::ExprStmt(expr) => typed_ast::Stmt::ExprStmt(expr.to_typed(names)),
            Stmt::ConstDef(def) => typed_ast::Stmt::ConstDef(def.to_typed(names)),
            Stmt::VarDef(def) => typed_ast::Stmt::VarDef(def.to_typed(names)),
            Stmt::ReturnStmt(expr) => typed_ast::Stmt::ReturnStmt(expr.to_typed(names)),
            Stmt::WhileStmt(stmt) => typed_ast::Stmt::WhileStmt(stmt.to_typed(names)),
            Stmt::IfStmt(stmt) => typed_ast::Stmt::IfStmt(stmt.to_typed(names)),
        }
    }
}

impl ToTyped for ast::BlockStmt {
    type Typed = typed_ast::BlockStmt;

    fn to_typed(self, names: &mut ScopedSymbolTable<typed_ast::Type>) -> Self::Typed {
        names.push_scope();
        let new_inners = self
            .0
            .into_iter()
            .map(|stmt| stmt.to_typed(names))
            .collect();
        names.pop_scope().unwrap();
        typed_ast::BlockStmt { inners: new_inners }
    }
}

impl ToTyped for ast::IfStmt {
    type Typed = typed_ast::IfStmt;

    fn to_typed(self, names: &mut ScopedSymbolTable<typed_ast::Type>) -> Self::Typed {
        typed_ast::IfStmt {
            condition: self.condition.to_typed(names),
            body: self.body.to_typed(names),
        }
    }
}

impl ToTyped for ast::WhileStmt {
    type Typed = typed_ast::WhileStmt;

    fn to_typed(self, names: &mut ScopedSymbolTable<typed_ast::Type>) -> Self::Typed {
        typed_ast::WhileStmt {
            condition: self.condition.to_typed(names),
            body: self.body.to_typed(names),
        }
    }
}

impl ToTyped for ast::ConstDef {
    type Typed = typed_ast::ConstDef;

    fn to_typed(self, names: &mut ScopedSymbolTable<typed_ast::Type>) -> Self::Typed {
        let new_binding = self.binding.to_typed(names);

        names
            .add_symbol(new_binding.name.clone(), new_binding.ty.clone())
            .unwrap();

        typed_ast::ConstDef {
            binding: new_binding,
            value: self.value.to_typed(names),
        }
    }
}

impl ToTyped for ast::VarDef {
    type Typed = typed_ast::VarDef;

    fn to_typed(self, names: &mut ScopedSymbolTable<typed_ast::Type>) -> Self::Typed {
        let new_binding = self.binding.to_typed(names);
        names
            .add_symbol(new_binding.name.clone(), new_binding.ty.clone())
            .unwrap();
        typed_ast::VarDef {
            binding: new_binding,
            value: self.value.to_typed(names),
        }
    }
}

impl ToTyped for ast::Expr {
    type Typed = typed_ast::Expr;

    fn to_typed(self, names: &mut ScopedSymbolTable<typed_ast::Type>) -> Self::Typed {
        use ast::Expr;
        match self {
            Expr::Array(items, len) => {
                // TODO: Make sure items all have same type
                let new_items: Vec<typed_ast::Expr> =
                    items.into_iter().map(|item| item.to_typed(names)).collect();
                typed_ast::Expr {
                    ty: new_items[0].ty.clone(),
                    val: typed_ast::ExprInner::Array(typed_ast::ArrayExpr {
                        items: new_items,
                        len,
                    }),
                }
            }
            Expr::Binary(lhs, op, rhs) => {
                let new_lhs = lhs.to_typed(names);
                let new_rhs = rhs.to_typed(names);
                let new_op = op.to_typed(names);
                typed_ast::Expr {
                    ty: new_lhs.ty.clone(),
                    val: typed_ast::ExprInner::Binary(typed_ast::BinaryExpr {
                        lhs: Box::new(new_lhs),
                        op: new_op,
                        rhs: Box::new(new_rhs),
                    }),
                }
            }
            Expr::Cast(original, to_type) => {
                let new_type = to_type.to_typed(names);
                typed_ast::Expr {
                    ty: new_type.clone(),
                    val: typed_ast::ExprInner::Cast(typed_ast::CastExpr {
                        original: Box::new(original.to_typed(names)),
                        to_type: new_type,
                    }),
                }
            }
            Expr::FunctionCall(call) => {
                // TODO: Handle error better
                let new_name = call.name.to_typed(names);
                let return_type = new_name.ty.clone();
                typed_ast::Expr {
                    ty: return_type,
                    val: typed_ast::ExprInner::FunctionCall(typed_ast::FunctionCall {
                        name: Box::new(new_name),
                        args: call.args.into_iter().map(|a| a.to_typed(names)).collect(),
                    }),
                }
            }
            Expr::Ident(ident) => {
                // TODO: Handle error better
                let ty = names
                    .get_symbol(&ident)
                    .expect("Could not find ident")
                    .clone();
                typed_ast::Expr {
                    ty: ty.clone(),
                    val: typed_ast::ExprInner::Ident(ident),
                }
            }
            Expr::IndexExpr(name, index) => {
                // TODO: Handle error better
                let new_name = name.to_typed(names);
                use typed_ast::Type;
                let inner_type = match &new_name.ty {
                    Type::Array(ty, _len) => *ty.clone(),
                    _ => panic!("Index into non-array type"),
                };
                typed_ast::Expr {
                    ty: inner_type,
                    val: typed_ast::ExprInner::IndexExpr(typed_ast::IndexExpr {
                        name: Box::new(new_name),
                        index: Box::new(index.to_typed(names)),
                    }),
                }
            }
            Expr::Literal(lit) => {
                let new_lit = lit.to_typed(names);
                typed_ast::Expr {
                    ty: new_lit.ty.clone(),
                    val: typed_ast::ExprInner::Literal(new_lit),
                }
            }
            Expr::Unary(op, data) => {
                use ast::UnaryOp;
                let new_data = data.to_typed(names);
                let expr_ty = match op {
                    UnaryOp::Reference => typed_ast::Type::Ref(Box::new(new_data.ty.clone())),
                    _ => new_data.ty.clone(),
                };
                typed_ast::Expr {
                    ty: expr_ty,
                    val: typed_ast::ExprInner::Unary(typed_ast::UnaryExpr {
                        data: Box::new(new_data),
                        op: op.to_typed(names),
                    }),
                }
            }
        }
    }
}

impl ToTyped for ast::Literal {
    type Typed = typed_ast::Literal;

    fn to_typed(self, _names: &mut ScopedSymbolTable<typed_ast::Type>) -> Self::Typed {
        use ast::Literal;
        match self {
            Literal::Str(val) => typed_ast::Literal {
                ty: typed_ast::Type::Str,
                val: typed_ast::LiteralInner::Str(val),
            },
            Literal::Bool(val) => typed_ast::Literal {
                ty: typed_ast::Type::Bool,
                val: typed_ast::LiteralInner::Bool(val),
            },
            Literal::Int(val) => typed_ast::Literal {
                ty: typed_ast::Type::Int,
                val: typed_ast::LiteralInner::Int(val),
            },
            Literal::UInt(val) => typed_ast::Literal {
                ty: typed_ast::Type::UInt,
                val: typed_ast::LiteralInner::UInt(val),
            },
            Literal::Int8(val) => typed_ast::Literal {
                ty: typed_ast::Type::Int8,
                val: typed_ast::LiteralInner::Int8(val),
            },
            Literal::Int16(val) => typed_ast::Literal {
                ty: typed_ast::Type::Int16,
                val: typed_ast::LiteralInner::Int16(val),
            },
            Literal::Int32(val) => typed_ast::Literal {
                ty: typed_ast::Type::Int32,
                val: typed_ast::LiteralInner::Int32(val),
            },
            Literal::Int64(val) => typed_ast::Literal {
                ty: typed_ast::Type::Int64,
                val: typed_ast::LiteralInner::Int64(val),
            },
            Literal::UInt8(val) => typed_ast::Literal {
                ty: typed_ast::Type::UInt8,
                val: typed_ast::LiteralInner::UInt8(val),
            },
            Literal::UInt16(val) => typed_ast::Literal {
                ty: typed_ast::Type::UInt16,
                val: typed_ast::LiteralInner::UInt16(val),
            },
            Literal::UInt32(val) => typed_ast::Literal {
                ty: typed_ast::Type::UInt32,
                val: typed_ast::LiteralInner::UInt32(val),
            },
            Literal::UInt64(val) => typed_ast::Literal {
                ty: typed_ast::Type::UInt64,
                val: typed_ast::LiteralInner::UInt64(val),
            },
        }
    }
}

// impl ToTyped for ast::FunctionCall {
//     type Typed = typed_ast::FunctionCall;
//
//     fn to_typed(self, names: &mut ScopedSymbolTable<typed_ast::Type>) -> Self::Typed {
//         todo!();
//     }
// }

impl ToTyped for ast::BinOp {
    type Typed = typed_ast::BinOp;

    fn to_typed(self, _names: &mut ScopedSymbolTable<typed_ast::Type>) -> Self::Typed {
        use ast::BinOp;
        match self {
            BinOp::Plus => typed_ast::BinOp::Plus,
            BinOp::Minus => typed_ast::BinOp::Minus,
            BinOp::Times => typed_ast::BinOp::Times,
            BinOp::Divide => typed_ast::BinOp::Divide,
            BinOp::LogicalAnd => typed_ast::BinOp::LogicalAnd,
            BinOp::LogicalOr => typed_ast::BinOp::LogicalOr,
        }
    }
}

impl ToTyped for ast::UnaryOp {
    type Typed = typed_ast::UnaryOp;

    fn to_typed(self, _names: &mut ScopedSymbolTable<typed_ast::Type>) -> Self::Typed {
        use ast::UnaryOp;
        match self {
            UnaryOp::Minus => typed_ast::UnaryOp::Minus,
            UnaryOp::Not => typed_ast::UnaryOp::Not,
            UnaryOp::Reference => typed_ast::UnaryOp::Reference,
        }
    }
}

impl ToTyped for ast::TypeBinding {
    type Typed = typed_ast::TypeBinding;

    fn to_typed(self, names: &mut ScopedSymbolTable<typed_ast::Type>) -> Self::Typed {
        typed_ast::TypeBinding {
            name: self.name,
            ty: self.ty.to_typed(names),
        }
    }
}

impl ToTyped for ast::Type {
    type Typed = typed_ast::Type;

    fn to_typed(self, names: &mut ScopedSymbolTable<typed_ast::Type>) -> Self::Typed {
        use ast::Type;
        match self {
            Type::Array(inner, len) => typed_ast::Type::Array(Box::new(inner.to_typed(names)), len),
            Type::Ref(inner) => typed_ast::Type::Ref(Box::new(inner.to_typed(names))),
            Type::Bool => typed_ast::Type::Bool,
            Type::Char => typed_ast::Type::Char,
            Type::Str => typed_ast::Type::Str,
            Type::Int => typed_ast::Type::Int,
            Type::Int8 => typed_ast::Type::Int8,
            Type::Int16 => typed_ast::Type::Int16,
            Type::Int32 => typed_ast::Type::Int32,
            Type::Int64 => typed_ast::Type::Int64,
            Type::UInt => typed_ast::Type::UInt,
            Type::UInt8 => typed_ast::Type::UInt8,
            Type::UInt16 => typed_ast::Type::UInt16,
            Type::UInt32 => typed_ast::Type::UInt32,
            Type::UInt64 => typed_ast::Type::UInt64,
        }
    }
}
