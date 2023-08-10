use std::process::Command;

use inkwell::{builder::Builder, context::Context, module::Module, values::AnyValue, AddressSpace};

use crate::ast::{
    decl::Decl,
    expr::{CallExpr, Expr, ExprKind, IntLiteralExpr, StrLiteralExpr},
    module::ModuleAST,
    stmt::{DeclStmt, ExprStmt, ExternStmt, ReturnStmt, Stmt, StmtKind},
    types::{FuncType, IntType, Type, TypeKind},
};

pub struct Codegen {
    context: Context,
}

impl Codegen {
    pub fn new() -> Self {
        Self {
            context: Context::create(),
        }
    }

    pub fn compile_module(&self, ast: &ModuleAST, output: Option<&str>) {
        let module = self.context.create_module(ast.name());

        let i32_type = self.context.i32_type();
        let main_fn_type = i32_type.fn_type(&[], false);
        let function = module.add_function("main", main_fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        let builder = self.context.create_builder();
        builder.position_at_end(basic_block);
        for stmt in ast.stmts() {
            self.build_stmt(&module, &builder, stmt);
        }

        if let Some(output) = output {
            module.print_to_file(format!("{}.ll", output)).unwrap();
            Command::new("llc")
                .args([
                    "-filetype=obj",
                    &format!("{}.ll", output),
                    "-o",
                    &format!("{}.o", output),
                ])
                .output()
                .expect("Unable to run llc");
            Command::new("clang")
                .args([&format!("{}.o", output), "-o", output])
                .output()
                .expect("Unable to run clang");
        } else {
            module.print_to_stderr();
        }
    }

    fn build_stmt<'ctx>(&'ctx self, module: &Module<'ctx>, builder: &Builder<'ctx>, stmt: &Stmt) {
        match stmt.kind() {
            StmtKind::Extern => self.build_extern_stmt(module, builder, stmt.cast::<ExternStmt>()),
            StmtKind::Decl => self.build_decl_stmt(module, builder, stmt.cast::<DeclStmt>()),
            StmtKind::Expr => self.build_expr_stmt(module, builder, stmt.cast::<ExprStmt>()),
            StmtKind::Return => self.build_return_stmt(module, builder, stmt.cast::<ReturnStmt>()),
        }
    }

    fn build_extern_stmt<'ctx>(
        &'ctx self,
        module: &Module<'ctx>,
        builder: &Builder,
        extern_stmt: &ExternStmt,
    ) {
        for decl_stmt in extern_stmt.decl_stmts() {
            self.build_decl_stmt(module, builder, decl_stmt)
        }
    }

    fn build_decl_stmt<'ctx>(
        &'ctx self,
        module: &Module<'ctx>,
        builder: &Builder,
        decl_stmt: &DeclStmt,
    ) {
        for decl in decl_stmt.decls() {
            self.build_decl(module, builder, decl);
        }
    }

    fn build_expr_stmt<'ctx>(
        &'ctx self,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        expr_stmt: &ExprStmt,
    ) {
        self.build_expr(module, builder, expr_stmt.expr());
    }

    fn build_return_stmt<'ctx>(
        &'ctx self,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        return_stmt: &ReturnStmt,
    ) {
        use inkwell::values::AnyValueEnum;
        if let Some(expr) = return_stmt.expr() {
            let llvm_value = self.build_expr(module, builder, expr);
            match llvm_value {
                AnyValueEnum::ArrayValue(_) => {
                    builder.build_return(Some(&llvm_value.into_array_value()));
                }
                AnyValueEnum::IntValue(_) => {
                    builder.build_return(Some(&llvm_value.into_int_value()));
                }
                AnyValueEnum::FloatValue(_) => {
                    builder.build_return(Some(&llvm_value.into_float_value()));
                }
                AnyValueEnum::PhiValue(_) => todo!(),
                AnyValueEnum::FunctionValue(_) => unreachable!(),
                AnyValueEnum::PointerValue(_) => {
                    builder.build_return(Some(&llvm_value.into_pointer_value()));
                }
                AnyValueEnum::StructValue(_) => {
                    builder.build_return(Some(&llvm_value.into_struct_value()));
                }
                AnyValueEnum::VectorValue(_) => {
                    builder.build_return(Some(&llvm_value.into_vector_value()));
                }
                AnyValueEnum::InstructionValue(_) => unreachable!(),
                AnyValueEnum::MetadataValue(_) => unreachable!(),
            }
        } else {
            builder.build_return(None);
        }
    }

    // ==================================================

    fn build_decl<'ctx>(&'ctx self, module: &Module<'ctx>, builder: &Builder, decl: &Decl) {
        let r#type = decl.r#type().unwrap();
        match r#type.kind() {
            TypeKind::Func => self.build_func_decl(module, builder, decl),
            _ => todo!(),
        }
    }

    fn build_func_decl<'ctx>(&'ctx self, module: &Module<'ctx>, builder: &Builder, decl: &Decl) {
        let name = decl.ident().value();
        let func_type = decl.r#type().unwrap().cast::<FuncType>();
        let llvm_func_type = self.compile_func_type(func_type);

        module.add_function(name, llvm_func_type, None);
    }

    // ==================================================

    fn build_expr<'ctx>(
        &'ctx self,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        expr: &Expr,
    ) -> inkwell::values::AnyValueEnum {
        match expr.kind() {
            ExprKind::Call => self
                .build_call_expr(module, builder, expr.cast::<CallExpr>())
                .as_any_value_enum(),
            ExprKind::IntLiteral => self
                .build_int_literial_expr(module, builder, expr.cast::<IntLiteralExpr>())
                .into(),
            ExprKind::StrLiteral => self
                .build_str_literial_expr(module, builder, expr.cast::<StrLiteralExpr>())
                .as_any_value_enum(),
            ExprKind::Ident => todo!(),
        }
    }

    fn build_call_expr<'ctx>(
        &'ctx self,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        call_expr: &CallExpr,
    ) -> inkwell::values::CallSiteValue {
        let func_name = call_expr.ident().value();
        let function = module.get_function(func_name).unwrap();
        let args: Vec<inkwell::values::BasicMetadataValueEnum> = call_expr
            .args()
            .iter()
            .map(|arg| {
                let llvm_value = self.build_expr(module, builder, arg.expr());
                match llvm_value {
                    inkwell::values::AnyValueEnum::ArrayValue(_) => todo!(),
                    inkwell::values::AnyValueEnum::IntValue(_) => {
                        llvm_value.into_int_value().into()
                    }
                    inkwell::values::AnyValueEnum::FloatValue(_) => todo!(),
                    inkwell::values::AnyValueEnum::PhiValue(_) => todo!(),
                    inkwell::values::AnyValueEnum::FunctionValue(_) => todo!(),
                    inkwell::values::AnyValueEnum::PointerValue(_) => {
                        llvm_value.into_pointer_value().into()
                    }
                    inkwell::values::AnyValueEnum::StructValue(_) => todo!(),
                    inkwell::values::AnyValueEnum::VectorValue(_) => todo!(),
                    inkwell::values::AnyValueEnum::InstructionValue(_) => todo!(),
                    inkwell::values::AnyValueEnum::MetadataValue(_) => todo!(),
                }
            })
            .collect();
        builder.build_call(function, &args, "")
    }

    fn build_str_literial_expr<'ctx>(
        &'ctx self,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        str_literial: &StrLiteralExpr,
    ) -> inkwell::values::GlobalValue {
        builder.build_global_string_ptr(str_literial.value(), "")
    }

    fn build_int_literial_expr<'ctx>(
        &'ctx self,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        int_literial: &IntLiteralExpr,
    ) -> inkwell::values::IntValue {
        self.context
            .i32_type()
            .const_int(str::parse::<u64>(int_literial.value()).unwrap(), false)
    }

    // ==================================================

    fn compile_type(&self, r#type: &Type) -> inkwell::types::AnyTypeEnum {
        match r#type.kind() {
            TypeKind::Int => self.compile_int_type(r#type.cast::<IntType>()).into(),
            TypeKind::Func => self.compile_func_type(r#type.cast::<FuncType>()).into(),
            TypeKind::Ptr => todo!(),
        }
    }

    fn compile_func_type(&self, func_type: &FuncType) -> inkwell::types::FunctionType {
        use inkwell::types::AnyTypeEnum;
        use inkwell::types::BasicMetadataTypeEnum;

        let return_type = self.compile_type(func_type.return_type());
        let param_types: Vec<BasicMetadataTypeEnum> = func_type
            .params()
            .iter()
            .map(|param| {
                let llvm_type = self.compile_type(param.r#type());
                match llvm_type {
                    AnyTypeEnum::ArrayType(_) => llvm_type.into_array_type().into(),
                    AnyTypeEnum::FloatType(_) => llvm_type.into_float_type().into(),
                    AnyTypeEnum::FunctionType(_) => llvm_type
                        .into_function_type()
                        .ptr_type(AddressSpace::default())
                        .into(),
                    AnyTypeEnum::IntType(_) => llvm_type.into_int_type().into(),
                    AnyTypeEnum::PointerType(_) => llvm_type.into_pointer_type().into(),
                    AnyTypeEnum::StructType(_) => llvm_type.into_struct_type().into(),
                    AnyTypeEnum::VectorType(_) => llvm_type.into_vector_type().into(),
                    AnyTypeEnum::VoidType(_) => panic!("Cannot have void type in params"),
                }
            })
            .collect();

        match return_type {
            AnyTypeEnum::ArrayType(_) => return_type.into_array_type().fn_type(&param_types, false),
            AnyTypeEnum::FloatType(_) => return_type.into_float_type().fn_type(&param_types, false),
            AnyTypeEnum::FunctionType(_) => return_type
                .into_function_type()
                .ptr_type(AddressSpace::default())
                .fn_type(&param_types, false),
            AnyTypeEnum::IntType(_) => return_type.into_int_type().fn_type(&param_types, false),
            AnyTypeEnum::PointerType(_) => {
                return_type.into_pointer_type().fn_type(&param_types, false)
            }
            AnyTypeEnum::StructType(_) => {
                return_type.into_struct_type().fn_type(&param_types, false)
            }
            AnyTypeEnum::VectorType(_) => {
                return_type.into_vector_type().fn_type(&param_types, false)
            }
            AnyTypeEnum::VoidType(_) => return_type.into_void_type().fn_type(&param_types, false),
        }
    }

    fn compile_int_type(&self, int_type: &IntType) -> inkwell::types::IntType {
        match int_type {
            IntType::I8 => self.context.i8_type(),
            IntType::I32 => self.context.i32_type(),
            IntType::I64 => self.context.i64_type(),
        }
    }
}
