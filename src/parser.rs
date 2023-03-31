use std::{fs::File, mem};
use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};
use crate::ast::*;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    curr_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a File) -> Self {
        let mut lexer = Lexer::new(input);
        let curr_token = lexer.next_token();

        Self {
            lexer,
            curr_token
        }
    }

    fn accept_token(&mut self) -> Token {
        mem::replace(&mut self.curr_token, self.lexer.next_token())
    }

    fn expect_token(&mut self, expected_kind: TokenKind) -> Token {
        if self.curr_token.kind() == expected_kind {
            self.accept_token()
        } else {
            panic!("{:?} expected here: {:?}", expected_kind, self.curr_token.begin());
        }
    }

    pub fn parse(&mut self, module_name: String) -> ModuleAST {
        self.parse_module(module_name)
    }

    // ================== MODULE ==================

    fn parse_module(&mut self, name: String) -> ModuleAST {
        let mut stmts: Vec<StmtAST> = Vec::new();

        while self.curr_token.kind() != TokenKind::EOF {
            stmts.push(self.parse_stmt());
        }

        ModuleAST::new(name, BlockAST::new(stmts))
    }

    // ================== STATEMENTS ==================

    fn parse_stmt(&mut self) -> StmtAST {
        match self.curr_token.kind() {
            TokenKind::Let => self.parse_decl().into(),
            TokenKind::Extern => self.parse_extern().into(),
            _ => self.parse_expr().into(),
        }
    }

    fn parse_extern(&mut self) -> ExternStmt {
        self.expect_token(TokenKind::Extern);
        self.expect_token(TokenKind::LeftBrace);

        let mut decl_list = Vec::new();
        while self.curr_token.kind() != TokenKind::RightBrace {
            decl_list.push(self.parse_decl());
        }

        self.accept_token();
        ExternStmt::new(decl_list)
    }

    fn parse_block(&mut self) -> BlockAST {
        self.expect_token(TokenKind::LeftBrace);

        let mut stmts: Vec<StmtAST> = Vec::new();

        while self.curr_token.kind() != TokenKind::RightBrace {
            if self.curr_token.kind() == TokenKind::EOF {
                panic!("unterminated block");
            }

            stmts.push(self.parse_stmt());
        }

        self.accept_token();
        BlockAST::new(stmts)
    }

    // ================== DECLARATIONS ==================

    fn parse_decl(&mut self) -> DeclStmt {
        self.expect_token(TokenKind::Let);

        let mut decls: Vec<Decl> = Vec::new();
        decls.push(self.parse_decl_pred());

        while self.curr_token.kind() == TokenKind::Comma {
            self.accept_token();
            decls.push(self.parse_decl_pred());
        }

        DeclStmt::new(decls)
    }

    fn parse_decl_pred(&mut self) -> Decl {
        let ident = self.parse_ident();
        let mut r#type = None;

        if self.curr_token.kind() == TokenKind::Colon {
            self.accept_token();
            r#type = Some(self.parse_type());
        }

        if self.curr_token.kind() == TokenKind::Assign {
            self.accept_token();
            let value = self.parse_expr();
            Decl::new(ident, r#type, Some(value))
        } else {
            Decl::new(ident, r#type, None)
        }
    }

    // ================== EXPRESSIONS ==================

    fn parse_expr(&mut self) -> ExprStmt {
        match self.curr_token.kind() {
            TokenKind::Identifier =>{
                let object = self.parse_object();
                if self.curr_token.kind() == TokenKind::LeftParen {
                    let arg_list = self.parse_arg_list();
                    CallExpr::new(object, arg_list).into()
                } else {
                    object.into()
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

    fn parse_ident(&mut self) -> IdentExpr {
        let token = self.expect_token(TokenKind::Identifier);
        IdentExpr::new(token.spelling().to_string())
    }

    fn parse_object(&mut self) -> ObjectExpr {
        let ident = self.parse_ident();
        ObjectExpr::new(ident, None)
    }

    // ================== TYPES ==================

    fn parse_type(&mut self) -> TypeAST {
        match self.curr_token.kind() {
            TokenKind::I8 => {
                self.accept_token();
                PrimitiveType::I8.into()
            },
            TokenKind::I32 => {
                self.accept_token();
                PrimitiveType::I32.into()
            },
            TokenKind::I64 => {
                self.accept_token();
                PrimitiveType::I64.into()
            },
            TokenKind::Multiply => self.parse_pointer_type().into(),
            TokenKind::LeftParen => self.parse_func_type().into(),
            _ => panic!("unknown type: {}", self.curr_token.spelling())
        }
    }

    fn parse_pointer_type(&mut self) -> PointerType {
        self.expect_token(TokenKind::Multiply);
        let pointee = self.parse_type();
        PointerType::new(pointee)
    }

    fn parse_func_type(&mut self) -> FuncType {
        let param_list = self.parse_param_list();
        self.expect_token(TokenKind::Arrow);
        let return_type = self.parse_type();
        FuncType::new(return_type, param_list)
    }

    // ================== PARAMETERS ==================

    fn parse_param_list(&mut self) -> Vec<ParamAST> {
        self.expect_token(TokenKind::LeftParen);

        if self.curr_token.kind() == TokenKind::RightParen {
            return Vec::new();
        }

        let list = self.parse_proper_param_list();

        self.expect_token(TokenKind::RightParen);

        return list;
    }

    fn parse_proper_param_list(&mut self) -> Vec<ParamAST> {
        let mut list = Vec::new();
        list.push(self.parse_param());

        while self.curr_token.kind() == TokenKind::Comma {
            self.accept_token();
            list.push(self.parse_param());
        }

        return list;
    }

    fn parse_param(&mut self) -> ParamAST {
        let ident = self.parse_ident();
        self.expect_token(TokenKind::Colon);
        let r#type = self.parse_type();
        ParamAST::new(ident, r#type)
    }

    fn parse_arg_list(&mut self) -> Vec<ExprStmt> {
        self.expect_token(TokenKind::LeftParen);

        if self.curr_token.kind() == TokenKind::RightParen {
            return Vec::new();
        }

        let list = self.parse_proper_arg_list();

        self.expect_token(TokenKind::RightParen);

        return list;
    }

    fn parse_proper_arg_list(&mut self) -> Vec<ExprStmt> {
        let mut list = Vec::new();
        list.push(self.parse_expr());

        while self.curr_token.kind() == TokenKind::Comma {
            self.accept_token();
            list.push(self.parse_expr());
        }

        return list;
    }
}

