use std::{fs::File, mem};
use crate::asts::*;
use crate::lexer::Lexer;
use crate::{expr_stmt, stmt_ast, type_ast};
use crate::token::{Token, TokenKind};

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
            panic!("{:?} expected here", expected_kind);
        }
    }

    pub fn parse(&mut self) -> ProgramAST {
        self.parse_program()
    }

    // ================== PROGRAM ==================

    fn parse_program(&mut self) -> ProgramAST {
        let mut stmts: Vec<StmtAST> = Vec::new();

        while self.curr_token.kind() != TokenKind::EOF {
            stmts.append(&mut self.parse_stmt());
        }

        ProgramAST::new(BlockAST::new(stmts))
    }

    // ================== STATEMENTS ==================

    fn parse_stmt(&mut self) -> Vec<StmtAST> {
        match self.curr_token.kind() {
            TokenKind::Let => self.parse_decl(),
            TokenKind::Extern => vec![stmt_ast!(self.parse_extern())],
            _ => vec![stmt_ast!(self.parse_expr())],
        }
    }

    fn parse_extern(&mut self) -> ExternStmt {
        self.expect_token(TokenKind::Extern);
        let block = self.parse_block();
        ExternStmt::new(block)
    }

    fn parse_block(&mut self) -> BlockAST {
        self.expect_token(TokenKind::LeftBrace);

        let mut stmts: Vec<StmtAST> = Vec::new();

        while self.curr_token.kind() != TokenKind::RightBrace {
            if self.curr_token.kind() == TokenKind::EOF {
                panic!("unterminated block");
            }

            stmts.append(&mut self.parse_stmt());
        }

        self.accept_token();
        BlockAST::new(stmts)
    }

    // ================== DECLARATIONS ==================

    fn parse_decl(&mut self) -> Vec<StmtAST> {
        self.expect_token(TokenKind::Let);

        let mut stmt_list: Vec<StmtAST> = Vec::new();
        self.parse_decl_body(&mut stmt_list);

        while self.curr_token.kind() == TokenKind::Comma {
            self.accept_token();
            self.parse_decl_body(&mut stmt_list);
        }

        return stmt_list;
    }

    fn parse_decl_body(&mut self, stmt_list: &mut Vec<StmtAST>) {
        let ident = self.parse_ident();
        let mut r#type = None;

        if self.curr_token.kind() == TokenKind::Colon {
            self.accept_token();
            r#type = Some(self.parse_type());
        }

        let decl = DeclStmt::new(ident.clone(), r#type);
        stmt_list.push(stmt_ast!(decl));

        if self.curr_token.kind() == TokenKind::Assign {
            self.accept_token();
            let rhs = self.parse_expr();
            let assign_expr = AssignExpr::new(ident, rhs);
            stmt_list.push(stmt_ast!(expr_stmt!(assign_expr)));
        }
    }

    // ================== EXPRESSIONS ==================

    fn parse_expr(&mut self) -> ExprStmt {
        match self.curr_token.kind() {
            TokenKind::Identifier =>{
                let ident = self.parse_ident();
                if self.curr_token.kind() == TokenKind::LeftParen {
                    let arg_list = self.parse_arg_list();
                    let call = CallExpr::new(ident, arg_list);
                    expr_stmt!(call)
                } else {
                    expr_stmt!(ident)
                }
            },
            TokenKind::IntLiteral => {
                let token = self.accept_token();
                let int_literal = IntLiteralExpr::new(token.spelling());
                expr_stmt!(int_literal)
            },
            TokenKind::StrLiteral => {
                let token = self.accept_token();
                let spelling = token.spelling();
                let str_literal = StrLiteralExpr::new(&spelling[1..spelling.len()-1]);
                expr_stmt!(str_literal)
            },
            _ => panic!("unexpected expression at {:?}", self.curr_token.begin()),
        }
    }

    fn parse_ident(&mut self) -> IdentExpr {
        let token = self.expect_token(TokenKind::Identifier);
        IdentExpr::new(token.spelling())
    }

    // ================== TYPES ==================

    fn parse_type(&mut self) -> TypeAST {
        match self.curr_token.kind() {
            TokenKind::I8 => {
                self.accept_token();
                type_ast!(PrimitiveType::I8)
            },
            TokenKind::I32 => {
                self.accept_token();
                type_ast!(PrimitiveType::I32)
            },
            TokenKind::I64 => {
                self.accept_token();
                type_ast!(PrimitiveType::I64)
            },
            TokenKind::Multiply => type_ast!(self.parse_pointer()),
            TokenKind::LeftParen => type_ast!(self.parse_func_type()),
            _ => panic!("unknown type: {}", self.curr_token.spelling())
        }
    }

    fn parse_pointer(&mut self) -> PointerType {
        self.expect_token(TokenKind::Multiply);
        let pointee = self.parse_type();
        PointerType::new(pointee)
    }

    fn parse_func_type(&mut self) -> FuncType {
        let param_list = self.parse_param_list();
        self.expect_token(TokenKind::Arrow);
        let return_type = self.parse_type();
        FuncType::new(param_list, return_type)
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

