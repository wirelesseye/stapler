use crate::{
    ast::{
        arg::Arg,
        decl::Decl,
        expr::{CallExpr, Expr, ExprKind, IdentExpr},
        ident::Ident,
        module_ast::ModuleAST,
        stmt::{DeclStmt, ExprStmt, ExternStmt, Stmt, StmtKind},
        types::{FuncType, RefType, Type, TypeKind},
    },
    decl_table::DeclTable,
};

pub struct Checker {
    def_table: DeclTable,
}

impl Checker {
    pub fn new() -> Self {
        Self {
            def_table: DeclTable::new(),
        }
    }

    pub fn check(&mut self, ast: &mut ModuleAST) {
        for stmt in &mut ast.stmts {
            self.check_stmt(stmt);
        }
    }

    fn check_stmt(&mut self, stmt: &mut Stmt) {
        match stmt.kind() {
            StmtKind::Decl => self.check_decl_stmt(stmt.cast_mut::<DeclStmt>()),
            StmtKind::Extern => self.check_extern_stmt(stmt.cast_mut::<ExternStmt>()),
            StmtKind::Expr => self.check_expr_stmt(stmt.cast_mut::<ExprStmt>()),
            _ => (),
        }
    }

    fn check_decl_stmt(&mut self, decl_stmt: &mut DeclStmt) {
        for decl in &mut decl_stmt.decls {
            self.check_decl(decl);
        }
    }

    fn check_extern_stmt(&mut self, extern_stmt: &mut ExternStmt) {
        for decl_stmt in &mut extern_stmt.decl_stmts {
            self.check_decl_stmt(decl_stmt);
        }
    }

    fn check_expr_stmt(&mut self, expr_stmt: &mut ExprStmt) {
        self.check_expr(&mut expr_stmt.expr);
    }

    // ==================================================

    fn check_decl(&mut self, decl: &mut Decl) {
        if let Some(value) = &mut decl.value {
            self.check_expr(value);

            if decl.r#type.is_none() && value.r#type().is_some() {
                decl.r#type = value.r#type().clone();
            }
        }

        let decl_id = self.def_table.push(&decl.name, decl.r#type.clone());
        decl.decl_id = Some(decl_id);
    }

    // ==================================================

    fn check_expr(&mut self, expr: &mut Expr) {
        match expr.kind() {
            ExprKind::Call => self.check_call_expr(expr.cast_mut::<CallExpr>()),
            ExprKind::Ident => self.check_ident_expr(expr.cast_mut::<IdentExpr>()),
            ExprKind::Member => (),
            _ => (),
        }
    }

    fn check_call_expr(&mut self, call_expr: &mut CallExpr) {
        self.check_expr(&mut call_expr.postfix_expr);
        for arg in &mut call_expr.args {
            self.check_arg(arg);
        }

        if call_expr.r#type.is_none() {
            let func_type = call_expr
                .postfix_expr
                .r#type()
                .as_ref()
                .unwrap()
                .cast::<FuncType>();
            call_expr.r#type = Some(func_type.return_type.clone());
        }
    }

    fn check_ident_expr(&mut self, ident_expr: &mut IdentExpr) {
        self.check_ident(&mut ident_expr.ident);

        let decl_entry = self.def_table.retrieve(&ident_expr.ident.name).unwrap();
        if ident_expr.r#type.is_none() && decl_entry.r#type.is_some() {
            ident_expr.r#type = decl_entry.r#type.clone();
        }
    }

    // ==================================================

    fn check_type(&mut self, r#type: &mut Type) {
        match r#type.kind() {
            TypeKind::Ref => self.check_ref_type(r#type.cast_mut::<RefType>()),
            _ => (),
        }
    }

    fn check_ref_type(&mut self, ref_type: &mut RefType) {
        self.check_expr(&mut ref_type.expr);
    }

    // ==================================================

    fn check_ident(&mut self, ident: &mut Ident) {
        let decl_entry = self.def_table.retrieve(&ident.name).unwrap();
        ident.decl_id = Some(decl_entry.decl_id);
    }

    fn check_arg(&mut self, arg: &mut Arg) {
        self.check_expr(&mut arg.expr);
    }
}
