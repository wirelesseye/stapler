use std::{cell::RefCell, collections::HashMap, process::Command};

use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    types::{AnyTypeEnum, BasicMetadataTypeEnum, BasicTypeEnum},
    values::{AnyValue, AnyValueEnum, BasicValueEnum},
    AddressSpace,
};

use crate::ast::{
    decl::Decl,
    expr::{
        CallExpr, CompositeExpr, Expr, ExprKind, IdentExpr, IntLiteralExpr, MemberExpr,
        StrLiteralExpr,
    },
    module_ast::ModuleAST,
    stmt::{DeclStmt, ExprStmt, ExternStmt, ReturnStmt, Stmt, StmtKind, TypeStmt},
    types::{CompositeType, FuncType, IntType, PtrType, RefType, Type, TypeKind},
};

pub struct Codegen<'ctx> {
    context: Context,
    decl_map: RefCell<HashMap<u64, (AnyTypeEnum<'ctx>, AnyValueEnum<'ctx>)>>,
    type_map: RefCell<HashMap<u64, AnyTypeEnum<'ctx>>>,
}

impl<'ctx> Codegen<'ctx> {
    pub fn new() -> Self {
        Self {
            context: Context::create(),
            decl_map: RefCell::new(HashMap::new()),
            type_map: RefCell::new(HashMap::new()),
        }
    }

    pub fn build_module(&'ctx self, ast: &ModuleAST, output: Option<&str>) {
        let module = self.context.create_module(&ast.name);

        let i32_type = self.context.i32_type();
        let main_fn_type = i32_type.fn_type(&[], false);
        let function = module.add_function("main", main_fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        let builder = self.context.create_builder();
        builder.position_at_end(basic_block);
        for stmt in &ast.stmts {
            self.build_stmt(&module, &builder, stmt);
        }
        builder.build_return(Some(&i32_type.const_int(0, false)));

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

    // ==================================================

    fn set_value(
        &self,
        value_id: u64,
        llvm_type: AnyTypeEnum<'ctx>,
        llvm_value: AnyValueEnum<'ctx>,
    ) {
        self.decl_map
            .borrow_mut()
            .insert(value_id, (llvm_type, llvm_value));
    }

    fn get_value(&self, value_id: u64) -> (AnyTypeEnum, AnyValueEnum) {
        let decl_map = self.decl_map.borrow();
        decl_map.get(&value_id).unwrap().clone()
    }

    fn set_type(&self, type_id: u64, llvm_type: AnyTypeEnum<'ctx>) {
        self.type_map.borrow_mut().insert(type_id, llvm_type);
    }

    fn get_type(&self, type_id: u64) -> AnyTypeEnum {
        let type_map = self.type_map.borrow();
        type_map.get(&type_id).unwrap().clone()
    }

    // ==================================================

    fn build_stmt(&'ctx self, module: &Module<'ctx>, builder: &Builder<'ctx>, stmt: &Stmt) {
        match stmt.kind() {
            StmtKind::Extern => self.build_extern_stmt(module, builder, stmt.cast::<ExternStmt>()),
            StmtKind::Decl => self.build_decl_stmt(module, builder, stmt.cast::<DeclStmt>()),
            StmtKind::Expr => self.build_expr_stmt(module, builder, stmt.cast::<ExprStmt>()),
            StmtKind::Return => self.build_return_stmt(module, builder, stmt.cast::<ReturnStmt>()),
            StmtKind::Type => self.build_type_stmt(module, builder, stmt.cast::<TypeStmt>()),
        }
    }

    fn build_extern_stmt(
        &'ctx self,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        extern_stmt: &ExternStmt,
    ) {
        for decl_stmt in &extern_stmt.decl_stmts {
            self.build_decl_stmt(module, builder, decl_stmt)
        }
    }

    fn build_decl_stmt(
        &'ctx self,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        decl_stmt: &DeclStmt,
    ) {
        for decl in &decl_stmt.decls {
            self.build_decl(module, builder, decl);
        }
    }

    fn build_expr_stmt(
        &'ctx self,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        expr_stmt: &ExprStmt,
    ) {
        self.build_expr(module, builder, &expr_stmt.expr);
    }

    fn build_return_stmt(
        &'ctx self,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        return_stmt: &ReturnStmt,
    ) {
        if let Some(expr) = &return_stmt.expr {
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

    fn build_type_stmt(
        &'ctx self,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        type_stmt: &TypeStmt,
    ) {
        let llvm_type = self.compile_type(&type_stmt.r#type);
        self.set_type(type_stmt.ident.symbol_id.unwrap(), llvm_type);
    }

    // ==================================================

    fn build_decl(&'ctx self, module: &Module<'ctx>, builder: &Builder<'ctx>, decl: &Decl) {
        if decl.r#type.as_ref().unwrap().kind() == TypeKind::Func {
            self.build_func_decl(module, builder, decl);
            return;
        }

        if let Some(expr) = &decl.value {
            let llvm_type = self.compile_type(decl.r#type.as_ref().unwrap());
            let llvm_value = self.build_expr(module, builder, expr);
            self.set_value(decl.value_id.unwrap(), llvm_type, llvm_value);
        }
    }

    fn build_func_decl(
        &'ctx self,
        module: &Module<'ctx>,
        builder: &Builder,
        decl: &Decl,
    ) -> inkwell::values::FunctionValue {
        let name = &decl.name;
        let llvm_func_type =
            self.compile_func_type(decl.r#type.as_ref().unwrap().cast::<FuncType>());
        let llvm_func_value = module.add_function(name, llvm_func_type, None);

        self.set_value(
            decl.value_id.unwrap(),
            llvm_func_type.into(),
            llvm_func_value.into(),
        );
        return llvm_func_value;
    }

    // ==================================================

    fn build_expr(
        &'ctx self,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        expr: &Expr,
    ) -> AnyValueEnum {
        match expr.kind() {
            ExprKind::Call => self
                .build_call_expr(module, builder, expr.cast::<CallExpr>())
                .as_any_value_enum(),
            ExprKind::IntLiteral => self
                .build_int_literial_expr(module, builder, expr.cast::<IntLiteralExpr>())
                .as_any_value_enum(),
            ExprKind::StrLiteral => self
                .build_str_literial_expr(module, builder, expr.cast::<StrLiteralExpr>())
                .as_any_value_enum(),
            ExprKind::Ident => self.build_ident_expr(module, builder, expr.cast::<IdentExpr>()),
            ExprKind::Member => todo!(),
            ExprKind::Composite => self
                .build_composite_expr(module, builder, expr.cast::<CompositeExpr>())
                .as_any_value_enum(),
        }
    }

    fn build_call_expr(
        &'ctx self,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        call_expr: &CallExpr,
    ) -> inkwell::values::CallSiteValue {
        let value = self.build_expr(module, builder, &call_expr.postfix_expr);
        let function = value.into_function_value();
        let args: Vec<inkwell::values::BasicMetadataValueEnum> = call_expr
            .args
            .iter()
            .map(|arg| {
                let llvm_value = self.build_expr(module, builder, &arg.expr);
                match llvm_value {
                    AnyValueEnum::ArrayValue(_) => todo!(),
                    AnyValueEnum::IntValue(_) => llvm_value.into_int_value().into(),
                    AnyValueEnum::FloatValue(_) => todo!(),
                    AnyValueEnum::PhiValue(_) => todo!(),
                    AnyValueEnum::FunctionValue(_) => todo!(),
                    AnyValueEnum::PointerValue(_) => llvm_value.into_pointer_value().into(),
                    AnyValueEnum::StructValue(_) => todo!(),
                    AnyValueEnum::VectorValue(_) => todo!(),
                    AnyValueEnum::InstructionValue(_) => todo!(),
                    AnyValueEnum::MetadataValue(_) => todo!(),
                }
            })
            .collect();
        builder.build_call(function, &args, "")
    }

    fn build_str_literial_expr(
        &'ctx self,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        str_literial: &StrLiteralExpr,
    ) -> inkwell::values::GlobalValue {
        builder.build_global_string_ptr(&str_literial.value, "")
    }

    fn build_int_literial_expr(
        &'ctx self,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        int_literial: &IntLiteralExpr,
    ) -> inkwell::values::IntValue {
        self.context
            .i32_type()
            .const_int(str::parse::<u64>(&int_literial.value).unwrap(), false)
    }

    fn build_ident_expr(
        &'ctx self,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        ident_expr: &IdentExpr,
    ) -> AnyValueEnum {
        let (_t, v) = self.get_value(ident_expr.ident.symbol_id.unwrap());
        v
    }

    // fn build_member_expr(
    //     &'ctx self,
    //     module: &Module<'ctx>,
    //     builder: &Builder<'ctx>,
    //     member_expr: &MemberExpr,
    // ) -> AnyTypeEnum {
    //     let composite_value = self
    //         .build_expr(module, builder, &member_expr.postfix_expr)
    //         .into_struct_value();
    // }

    fn build_composite_expr(
        &'ctx self,
        module: &Module<'ctx>,
        builder: &Builder<'ctx>,
        composite_expr: &CompositeExpr,
    ) -> inkwell::values::PointerValue {
        let r#type = composite_expr.r#type.as_ref().unwrap();
        let llvm_type = match r#type.kind() {
            TypeKind::Ref => {
                let ref_type = r#type.cast::<RefType>();
                self.get_type(ref_type.type_id.unwrap()).into_struct_type()
            }
            _ => panic!("Invalid CompositeType"),
        };
        let instance = builder.build_alloca(llvm_type, "");
        for (i, (name, expr)) in composite_expr.fields.iter().enumerate() {
            let ptr = builder
                .build_struct_gep(llvm_type, instance, i.try_into().unwrap(), name)
                .unwrap();
            let llvm_value = self.build_expr(module, builder, expr);
            let value: BasicValueEnum = match llvm_value {
                AnyValueEnum::ArrayValue(_) => todo!(),
                AnyValueEnum::IntValue(_) => llvm_value.into_int_value().into(),
                AnyValueEnum::FloatValue(_) => todo!(),
                AnyValueEnum::PhiValue(_) => todo!(),
                AnyValueEnum::FunctionValue(_) => todo!(),
                AnyValueEnum::PointerValue(_) => llvm_value.into_pointer_value().into(),
                AnyValueEnum::StructValue(_) => todo!(),
                AnyValueEnum::VectorValue(_) => todo!(),
                AnyValueEnum::InstructionValue(_) => todo!(),
                AnyValueEnum::MetadataValue(_) => todo!(),
            };
            builder.build_store(ptr, value);
        }

        instance
    }

    // ==================================================

    fn compile_type(&self, r#type: &Type) -> AnyTypeEnum {
        match r#type.kind() {
            TypeKind::Int => self.compile_int_type(r#type.cast::<IntType>()).into(),
            TypeKind::Func => self.compile_func_type(r#type.cast::<FuncType>()).into(),
            TypeKind::Ptr => self.compile_ptr_type(r#type.cast::<PtrType>()).into(),
            TypeKind::Ref => self.compile_ref_type(r#type.cast::<RefType>()).into(),
            TypeKind::Array => todo!(),
            TypeKind::Composite => self
                .compile_composite_type(r#type.cast::<CompositeType>())
                .into(),
        }
    }

    fn compile_func_type(&self, func_type: &FuncType) -> inkwell::types::FunctionType {
        let return_type = self.compile_type(&func_type.return_type);
        let params = if func_type.is_var_args {
            &func_type.params[..func_type.params.len() - 1]
        } else {
            &func_type.params
        };
        let param_types: Vec<BasicMetadataTypeEnum> = params
            .iter()
            .map(|param| {
                let llvm_type = self.compile_type(&param.r#type);
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
            AnyTypeEnum::ArrayType(_) => return_type
                .into_array_type()
                .fn_type(&param_types, func_type.is_var_args),
            AnyTypeEnum::FloatType(_) => return_type
                .into_float_type()
                .fn_type(&param_types, func_type.is_var_args),
            AnyTypeEnum::FunctionType(_) => return_type
                .into_function_type()
                .ptr_type(AddressSpace::default())
                .fn_type(&param_types, func_type.is_var_args),
            AnyTypeEnum::IntType(_) => return_type
                .into_int_type()
                .fn_type(&param_types, func_type.is_var_args),
            AnyTypeEnum::PointerType(_) => return_type
                .into_pointer_type()
                .fn_type(&param_types, func_type.is_var_args),
            AnyTypeEnum::StructType(_) => return_type
                .into_struct_type()
                .fn_type(&param_types, func_type.is_var_args),
            AnyTypeEnum::VectorType(_) => return_type
                .into_vector_type()
                .fn_type(&param_types, func_type.is_var_args),
            AnyTypeEnum::VoidType(_) => return_type
                .into_void_type()
                .fn_type(&param_types, func_type.is_var_args),
        }
    }

    fn compile_int_type(&self, int_type: &IntType) -> inkwell::types::IntType {
        match int_type {
            IntType::I8 => self.context.i8_type(),
            IntType::I32 => self.context.i32_type(),
            IntType::I64 => self.context.i64_type(),
        }
    }

    fn compile_ptr_type(&self, ptr_type: &PtrType) -> inkwell::types::PointerType {
        let pointee_type = self.compile_type(&ptr_type.pointee);
        match pointee_type {
            AnyTypeEnum::ArrayType(_) => pointee_type
                .into_array_type()
                .ptr_type(AddressSpace::default()),
            AnyTypeEnum::FloatType(_) => pointee_type
                .into_float_type()
                .ptr_type(AddressSpace::default()),
            AnyTypeEnum::FunctionType(_) => pointee_type
                .into_function_type()
                .ptr_type(AddressSpace::default()),
            AnyTypeEnum::IntType(_) => pointee_type
                .into_int_type()
                .ptr_type(AddressSpace::default()),
            AnyTypeEnum::PointerType(_) => pointee_type
                .into_pointer_type()
                .ptr_type(AddressSpace::default()),
            AnyTypeEnum::StructType(_) => pointee_type
                .into_struct_type()
                .ptr_type(AddressSpace::default()),
            AnyTypeEnum::VectorType(_) => pointee_type
                .into_vector_type()
                .ptr_type(AddressSpace::default()),
            AnyTypeEnum::VoidType(_) => panic!("Cannot point to void"),
        }
    }

    fn compile_ref_type(&self, ref_type: &RefType) -> AnyTypeEnum {
        self.get_type(
            ref_type
                .type_id
                .expect(&format!("Failed to get ref type: {:?}", ref_type)),
        )
        // self.context
        //     .opaque_struct_type(&ref_type.name)
        //     .ptr_type(AddressSpace::default())
    }

    fn compile_composite_type(&self, composite_type: &CompositeType) -> inkwell::types::StructType {
        let field_types: Vec<BasicTypeEnum> = composite_type
            .fields
            .iter()
            .map(|field| {
                let llvm_type = self.compile_type(&field.r#type);
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
                    AnyTypeEnum::VoidType(_) => panic!("Cannot have void type in fields"),
                }
            })
            .collect();
        self.context.struct_type(&field_types, false)
    }
}
