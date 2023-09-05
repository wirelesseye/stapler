use crate::{
    ast::{
        arg::Arg,
        decl::Decl,
        expr::{CallExpr, Expr, ExprKind, IdentExpr, CompositeExpr},
        ident::Ident,
        module_ast::ModuleAST,
        param::Param,
        stmt::{DeclStmt, ExprStmt, ExternStmt, Stmt, StmtKind, TypeStmt},
        types::{ArrayType, FuncType, RefType, Type, TypeKind, CompositeType},
    },
    symbol_table::SymbolTable,
};

pub struct Checker {
    symbol_table: SymbolTable,
}

impl Checker {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn check(&mut self, ast: &mut ModuleAST) {
        self.check_module(ast);
    }

    // ==================================================

    fn check_module(&mut self, module: &mut ModuleAST) {
        for stmt in &mut module.stmts {
            self.check_stmt(stmt);
        }
    }

    fn check_stmt(&mut self, stmt: &mut Stmt) {
        match stmt.kind() {
            StmtKind::Decl => self.check_decl_stmt(stmt.cast_mut::<DeclStmt>()),
            StmtKind::Extern => self.check_extern_stmt(stmt.cast_mut::<ExternStmt>()),
            StmtKind::Expr => self.check_expr_stmt(stmt.cast_mut::<ExprStmt>()),
            StmtKind::Type => self.check_type_stmt(stmt.cast_mut::<TypeStmt>()),
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
        self.check_expr(&mut expr_stmt.expr, &None);
    }

    fn check_type_stmt(&mut self, type_stmt: &mut TypeStmt) {
        self.check_type(&mut type_stmt.r#type);
        let type_id = self.symbol_table.push_type(&type_stmt.ident.name);
        type_stmt.ident.symbol_id = Some(type_id);
    }

    // ==================================================

    fn check_decl(&mut self, decl: &mut Decl) {
        if let Some(r#type) = &mut decl.r#type {
            self.check_type(r#type);
        }

        if let Some(value) = &mut decl.value {
            self.check_expr(value, &decl.r#type);
            if decl.r#type.is_none() && value.r#type().is_some() {
                decl.r#type = value.r#type().clone();
            }
        }

        if decl.r#type.is_none() {
            panic!("Type is not specified")
        }

        let value_id = self
            .symbol_table
            .push_value(&decl.name, decl.r#type.clone());
        decl.value_id = Some(value_id);
    }

    // ==================================================

    fn check_expr(&mut self, expr: &mut Expr, r#type: &Option<Type>) {
        match expr.kind() {
            ExprKind::Call => self.check_call_expr(expr.cast_mut::<CallExpr>()),
            ExprKind::Ident => self.check_ident_expr(expr.cast_mut::<IdentExpr>()),
            ExprKind::Member => (),
            ExprKind::Composite => self.check_composite_expr(expr.cast_mut::<CompositeExpr>(), r#type),
            _ => (),
        }
    }

    fn check_call_expr(&mut self, call_expr: &mut CallExpr) {
        self.check_expr(&mut call_expr.postfix_expr, &None);
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

        let entry = self
            .symbol_table
            .retrieve_value(&ident_expr.ident.name)
            .unwrap();
        if ident_expr.r#type.is_none() && entry.r#type.is_some() {
            ident_expr.r#type = entry.r#type.clone();
        }
    }

    fn check_composite_expr(&mut self, composite_expr: &mut CompositeExpr, r#type: &Option<Type>) {
        if r#type.is_some() {
            composite_expr.r#type = r#type.clone();
        }
        for (_, expr) in &mut composite_expr.fields {
            self.check_expr(expr, &None)
        }
    }

    // ==================================================

    fn check_type(&mut self, r#type: &mut Type) {
        match r#type.kind() {
            TypeKind::Ref => self.check_ref_type(r#type.cast_mut::<RefType>()),
            TypeKind::Array => self.check_array_type(r#type.cast_mut::<ArrayType>()),
            TypeKind::Func => self.check_func_type(r#type.cast_mut::<FuncType>()),
            TypeKind::Composite => self.check_composite_type(r#type.cast_mut::<CompositeType>()),
            _ => (),
        }
    }

    fn check_ref_type(&mut self, ref_type: &mut RefType) {
        if ref_type.type_id.is_none() {
            let entry = match ref_type.expr.kind() {
                ExprKind::Ident => {
                    let ident_expr = ref_type.expr.cast_mut::<IdentExpr>();
                    self.symbol_table
                        .retrieve_type(&ident_expr.ident.name)
                        .unwrap()
                }
                ExprKind::Member => todo!(),
                _ => panic!(),
            };
            ref_type.type_id = Some(entry.type_id);
        }
    }

    fn check_array_type(&mut self, array_type: &mut ArrayType) {
        self.check_type(&mut array_type.elem_type);
    }

    fn check_func_type(&mut self, func_type: &mut FuncType) {
        self.check_type(&mut func_type.return_type);
        for param in &mut func_type.params {
            self.check_param(param);
        }
    }

    fn check_composite_type(&mut self, composite_type: &mut CompositeType) {
        for field in &mut composite_type.fields {
            self.check_type(&mut field.r#type);
        }
    }

    // ==================================================

    fn check_ident(&mut self, ident: &mut Ident) {
        let decl_entry = self.symbol_table.retrieve_value(&ident.name).unwrap();
        ident.symbol_id = Some(decl_entry.value_id);
    }

    fn check_arg(&mut self, arg: &mut Arg) {
        self.check_expr(&mut arg.expr, &None);
    }

    fn check_param(&mut self, param: &mut Param) {
        self.check_type(&mut param.r#type);
    }
}
