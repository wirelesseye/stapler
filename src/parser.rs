use std::{fs::File, mem};

use crate::{
    ast::{
        decl::Decl,
        ident::Ident,
        module_ast::ModuleAST,
        stmt::{DeclStmt, ExprStmt, ExternStmt, Stmt, ReturnStmt},
        types::{FuncType, IntType, Type, PtrType, RefType, ArrayType}, param::Param, expr::{CallExpr, Expr, IdentExpr, IntLiteralExpr, StrLiteralExpr}, arg::Arg,
    },
    lexer::Lexer,
    token::{Token, TokenKind},
};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    curr_token: Token,
    module_name: String,
}

impl<'a> Parser<'a> {
    pub fn new(module_name: &str, input: &'a File) -> Self {
        let mut lexer = Lexer::new(input);
        let curr_token = lexer.next_token();

        Self {
            lexer,
            curr_token,
            module_name: module_name.to_string(),
        }
    }

    pub fn parse(&mut self) -> ModuleAST {
        self.parse_module()
    }

    // ==================================================

    fn accept_token(&mut self) -> Token {
        mem::replace(&mut self.curr_token, self.lexer.next_token())
    }

    fn expect_token(&mut self, expected_kind: TokenKind) -> Token {
        if self.curr_token.is_kind(expected_kind) {
            self.accept_token()
        } else {
            panic!(
                "{:?} expected here: {:?}",
                expected_kind,
                self.curr_token.begin()
            );
        }
    }

    // ==================================================

    fn parse_module(&mut self) -> ModuleAST {
        let mut stmts: Vec<Stmt> = Vec::new();

        while !self.curr_token.is_kind(TokenKind::EOF) {
            stmts.push(self.parse_stmt());
        }

        ModuleAST::new(self.module_name.clone(), stmts)
    }

    // ==================================================

    fn parse_stmt(&mut self) -> Stmt {
        match self.curr_token.kind() {
            TokenKind::Let | TokenKind::Export => self.parse_decl_stmt().into(),
            TokenKind::Extern => self.parse_extern_stmt().into(),
            TokenKind::Return => self.parse_return_stmt().into(),
            _ => ExprStmt::new(self.parse_expr()).into(),
        }
    }

    fn parse_extern_stmt(&mut self) -> ExternStmt {
        self.expect_token(TokenKind::Extern);
        self.expect_token(TokenKind::LeftBrace);

        let mut decl_list = Vec::new();
        while !self.curr_token.is_kind(TokenKind::RightBrace) {
            decl_list.push(self.parse_decl_stmt());
        }

        self.accept_token();
        ExternStmt::new(decl_list)
    }

    fn parse_decl_stmt(&mut self) -> DeclStmt {
        let mut is_export = false;
        if self.curr_token.is_kind(TokenKind::Export) {
            is_export = true;
            self.accept_token();
        }

        self.expect_token(TokenKind::Let);

        let mut decls: Vec<Decl> = Vec::new();
        decls.push(self.parse_decl());

        while self.curr_token.is_kind(TokenKind::Comma) {
            self.accept_token();
            decls.push(self.parse_decl());
        }

        DeclStmt::new(decls, is_export)
    }

    fn parse_return_stmt(&mut self) -> ReturnStmt {
        self.expect_token(TokenKind::Return);
        let expr = self.parse_expr();
        ReturnStmt::new(Some(expr))
    }

    // ==================================================

    fn parse_decl(&mut self) -> Decl {
        let ident = self.parse_ident();
        let mut r#type = None;

        if self.curr_token.is_kind(TokenKind::Colon) {
            self.accept_token();
            r#type = Some(self.parse_type());
        }

        if self.curr_token.is_kind(TokenKind::Assign) {
            self.accept_token();
            let value = self.parse_expr();
            Decl::new(ident, r#type, Some(value))
        } else {
            Decl::new(ident, r#type, None)
        }
    }

    fn parse_expr(&mut self) -> Expr {
        match self.curr_token.kind() {
            TokenKind::Identifier =>{
                let ident = self.parse_ident();
                if self.curr_token.kind() == TokenKind::LeftParen {
                    let arg_list = self.parse_arg_list();
                    CallExpr::new(ident, arg_list).into()
                } else {
                    IdentExpr::new(ident).into()
                }
            },
            TokenKind::IntLiteral => {
                let token = self.accept_token();
                IntLiteralExpr::new(token.spelling().to_string()).into()
            },
            TokenKind::StrLiteral => {
                let token = self.accept_token();
                let spelling = token.spelling();
                StrLiteralExpr::new(spelling[1..spelling.len()-1].to_string()).into()
            },
            _ => panic!("unexpected expression at {:?}", self.curr_token.begin()),
        }
    }

    // ==================================================

    fn parse_ident(&mut self) -> Ident {
        let token = self.expect_token(TokenKind::Identifier);
        Ident::new(token.spelling().to_string())
    }

    // ==================================================

    fn parse_type(&mut self) -> Type {
        let r#type = match self.curr_token.kind() {
            TokenKind::I8 => {
                self.accept_token();
                IntType::I8.into()
            }
            TokenKind::I32 => {
                self.accept_token();
                IntType::I32.into()
            }
            TokenKind::I64 => {
                self.accept_token();
                IntType::I64.into()
            },
            TokenKind::Multiply => self.parse_ptr_type().into(),
            TokenKind::LeftParen => self.parse_func_type().into(),
            TokenKind::Identifier => self.parse_ref_type().into(),
            _ => panic!(
                "Unexpected token when parsing type: {}",
                self.curr_token.spelling()
            ),
        };

        if self.curr_token.is_kind(TokenKind::LeftBracket) {
            self.accept_token();
            self.expect_token(TokenKind::RightBracket);
            ArrayType::new(r#type).into()
        } else {
            r#type
        }
    }

    fn parse_ptr_type(&mut self) -> PtrType {
        self.expect_token(TokenKind::Multiply);
        let pointee = self.parse_type();
        PtrType::new(pointee)
    }

    fn parse_func_type(&mut self) -> FuncType {
        let (param_list, is_var_args) = self.parse_param_list();
        self.expect_token(TokenKind::Arrow);
        let return_type = self.parse_type();
        FuncType::new(return_type, param_list, is_var_args)
    }

    fn parse_ref_type(&mut self) -> RefType {
        let ident = self.parse_ident();
        RefType::new(ident)
    }

    // ==================================================

    fn parse_param_list(&mut self) -> (Vec<Param>, bool) {
        self.expect_token(TokenKind::LeftParen);

        if self.curr_token.kind() == TokenKind::RightParen {
            return (Vec::new(), false);
        }

        let list = self.parse_proper_param_list();

        self.expect_token(TokenKind::RightParen);

        return list;
    }

    fn parse_proper_param_list(&mut self) -> (Vec<Param>, bool) {
        if self.curr_token.is_kind(TokenKind::Ellipsis) {
            self.accept_token();
            if self.curr_token.is_kind(TokenKind::RightParen) {
                return (Vec::new(), true);
            } else {
                panic!("Unexpected token after ellipsis")
            }
        }

        let mut list = Vec::new();
        list.push(self.parse_param());

        while self.curr_token.is_kind(TokenKind::Comma) {
            self.accept_token();
            if self.curr_token.is_kind(TokenKind::Ellipsis) {
                self.accept_token();
                list.push(self.parse_param());
                if self.curr_token.is_kind(TokenKind::RightParen) {
                    return (list, true);
                } else {
                    panic!("Unexpected token after ellipsis")
                }
            } else {
                list.push(self.parse_param());
            }
            
        }

        return (list, false);
    }

    fn parse_param(&mut self) -> Param {
        let ident = self.parse_ident();
        self.expect_token(TokenKind::Colon);
        let r#type = self.parse_type();
        Param::new(ident, r#type)
    }

    fn parse_arg_list(&mut self) -> Vec<Arg> {
        self.expect_token(TokenKind::LeftParen);

        if self.curr_token.kind() == TokenKind::RightParen {
            return Vec::new();
        }

        let list = self.parse_proper_arg_list();

        self.expect_token(TokenKind::RightParen);

        return list;
    }

    fn parse_proper_arg_list(&mut self) -> Vec<Arg> {
        let mut list = Vec::new();
        list.push(Arg::new(self.parse_expr()));

        while self.curr_token.kind() == TokenKind::Comma {
            self.accept_token();
            list.push(Arg::new(self.parse_expr()));
        }

        return list;
    }
}
