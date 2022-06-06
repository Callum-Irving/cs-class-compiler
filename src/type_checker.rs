use crate::ast;

pub fn infer_types(mut program: ast::Program) -> ast::Program {
    for stmt in program.0.iter_mut() {
        match stmt {
            ast::TopLevelStmt::FunctionDef(def) => infer_function_types(def),
            _ => (),
        }
    }

    program
}

fn infer_function_types(def: &mut ast::FunctionDef) {
    for stmt in def.body.0.iter_mut() {
        infer_stmt_types(stmt);
    }
}

fn infer_stmt_types(stmt: &mut ast::Stmt) {
    match stmt {
        ast::Stmt::VarDef(def) => {
            let ty = &def.binding.ty;
            if def.value.ty.is_none() {
                def.value.ty = Some(ty.clone());
            }
            infer_expr_types(&mut def.value);
        }
        ast::Stmt::BlockStmt(block) => {
            for stmt in block.0.iter_mut() {
                infer_stmt_types(stmt);
            }
        }
        ast::Stmt::ExprStmt(expr) => {
            infer_expr_types(expr);
        }
        _ => todo!(),
    }
}

fn infer_expr_types(expr: &mut ast::Expr) {
    match &mut expr.val {
        ast::ExprInner::Literal(lit) => {
            if lit.ty.is_none() {
                lit.ty = expr.ty.clone();
            }
        }
        ast::ExprInner::FunctionCall(_call) => (),
        _ => todo!(),
    }
}
