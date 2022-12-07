use crate::frontend::ast::Ast;
use crate::frontend::token::TokenKind::{CloseBracket, CloseParen, OpenParen};
use crate::frontend::token::{Token, TokenKind};

#[derive(Debug)]
struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens }
    }
    // Orders Of Precedence
    // Assignment
    // Object
    // AdditiveExpr
    // MultiplicativeExpr
    // Call
    // Member
    // PrimaryExpr
    fn parse(&mut self) -> Ast {
        let mut statements = vec![];
        while self.not_eof() {
            statements.push(self.parse_statement())
        }
        // let program = Program { statements };
        Ast::Program { statements }
    }

    fn not_eof(&self) -> bool {
        !matches!(self.tokens.first(), Some(Token::Eof))
    }

    fn parse_statement(&mut self) -> Ast {
        match self.at() {
            Some(Token::Let | Token::Const) => self.parse_variable_declaration(),
            _ => self.parse_expression(),
        }
    }

    fn parse_expression(&mut self) -> Ast {
        self.parse_assignment_expression()
    }

    fn parse_assignment_expression(&mut self) -> Ast {
        let left = self.parse_object_expression();
        match self.at() {
            Some(Token::Equals) => {
                self.eat();
                let value = self.parse_assignment_expression();
                self.expect(TokenKind::Semicolon, "expected semicolon");
                Ast::AssignmentExpr {
                    assignee: Box::new(left),
                    value: Box::new(value),
                }
            }
            _ => left,
        }
    }

    fn parse_additive_expression(&mut self) -> Ast {
        let mut left = self.parse_multiplicative_expression();
        while let Some(Token::BinaryOperator(op @ ('+' | '-'))) = self.at() {
            let operator = *op;
            self.eat();
            let right = self.parse_multiplicative_expression();
            left = Ast::BinaryExpr {
                left: Box::new(left),
                right: Box::new(right),
                operator,
            }
        }

        left
    }

    fn parse_multiplicative_expression(&mut self) -> Ast {
        let mut left = self.parse_member_call_expression();
        while let Some(Token::BinaryOperator(op @ ('*' | '%' | '/'))) = self.at() {
            let operator = *op;
            self.eat();
            let right = self.parse_member_call_expression();
            left = Ast::BinaryExpr {
                left: Box::new(left),
                right: Box::new(right),
                operator,
            }
        }

        left
    }

    fn at(&self) -> Option<&Token> {
        self.tokens.first()
    }

    fn eat(&mut self) -> Token {
        self.tokens.remove(0)
    }

    fn parse_primary_expression(&mut self) -> Ast {
        println!("{:#?}", self.at());
        match self.at() {
            Some(Token::OpenParen) => {
                self.eat();
                let expression = self.parse_expression();
                self.expect(CloseParen, "unexpected token found inside parentheses expression. expected close parentheses");
                expression
            }
            Some(Token::Number(num)) => {
                let num = *num;
                self.eat();
                Ast::NumericLiteral(num)
            }
            Some(Token::Identifier(id)) => {
                let variable_name = id.to_string();
                self.eat();
                Ast::Identifier(variable_name)
            }
            _ => {
                panic!("Unexpected token {:?}", self.at())
            }
        }
    }

    fn expect(&mut self, expected_token_kind: TokenKind, message: &str) -> Token {
        let token = self.eat();
        if token.kind() != expected_token_kind {
            panic!("Unexpected token found '{:?}'. {}", token, message)
        }
        token
    }

    fn parse_variable_declaration(&mut self) -> Ast {
        let is_const = matches!(self.eat(), Token::Const);
        let identifier = self.expect(TokenKind::Identifier, "expected identifier");
        match (identifier, self.at()) {
            (Token::Identifier(variable), Some(Token::Semicolon)) => {
                if is_const {
                    panic!("must assign value to constant expression. No value provided");
                }
                self.eat();
                Ast::VariableDeclaration {
                    constant: is_const,
                    value: Box::new(Ast::Identifier("null".to_string())),
                    identifier: variable,
                }
            }
            (Token::Identifier(variable), Some(_)) => {
                self.expect(TokenKind::Equals, "expected equals");
                let statement = Ast::VariableDeclaration {
                    constant: is_const,
                    value: Box::new(self.parse_expression()),
                    identifier: variable,
                };
                self.expect(TokenKind::Semicolon, "expected semicolon");
                statement
            }
            _ => {
                panic!("unexpected error");
            }
        }
    }

    fn parse_object_expression(&mut self) -> Ast {
        if !matches!(self.at(), Some(Token::OpenBrace)) {
            return self.parse_additive_expression();
        }
        self.eat();
        let mut properties: Vec<(String, Option<Box<Ast>>)> = Vec::new();
        while self.not_eof() && !matches!(self.at(), Some(Token::CloseBrace)) {
            let key = self.expect(TokenKind::Identifier, "object literal key expected");
            match (key, self.at()) {
                (Token::Identifier(var), Some(Token::Comma)) => {
                    // Allows short hand key pair : {key,}
                    self.eat();
                    properties.push((var, None));
                    continue;
                }
                (Token::Identifier(var), Some(Token::CloseBrace)) => {
                    // Allows short hand key pair : {key}
                    properties.push((var, None));
                    continue;
                }
                (Token::Identifier(var), Some(_)) => {
                    // Allows short hand key pair : {key: value} {key: value,}
                    self.expect(TokenKind::Colon, "colon expected after object literal key");
                    let expression = self.parse_expression();
                    properties.push((var, Some(Box::new(expression))));
                    if !matches!(self.at(), Some(Token::CloseBrace)) {
                        self.expect(
                            TokenKind::Comma,
                            "Expected comma or closing bracket following a property",
                        );
                    }
                }
                _ => {
                    panic!("never executed branch. object literal key expected");
                }
            }
        }
        self.expect(
            TokenKind::CloseBrace,
            "expected closing brace for object literal",
        );
        Ast::ObjectLiteral { properties }
    }

    fn parse_member_call_expression(&mut self) -> Ast {
        let member = self.parse_member_expression();
        if let Some(Token::OpenParen) = self.at() {
            return self.parse_call_expression(member);
        }
        member
    }

    fn parse_member_expression(&mut self) -> Ast {
        let mut object = self.parse_primary_expression();
        while let Some(Token::Dot | Token::OpenBracket) = self.at() {
            let operator = self.eat();

            let property: Ast;
            let computed: bool;

            if operator == Token::Dot {
                computed = false;
                property = self.parse_primary_expression();
                if !matches!(property, Ast::Identifier(_)) {
                    panic!("Cannot use dot operator without right hand side being a identifier");
                }
            } else {
                computed = true;
                property = self.parse_expression();
                self.expect(CloseBracket, "Missing closing bracket in computed value.");
            }

            object = Ast::MemberExpr {
                object: Box::new(object),
                property: Box::new(property),
                computed,
            }
        }
        object
    }

    fn parse_call_expression(&mut self, caller: Ast) -> Ast {
        let mut call_expr = Ast::CallExpr {
            caller: Box::new(caller),
            args: self.parse_args(),
        };
        println!("{:#?}", call_expr);
        if let Some(Token::OpenParen) = self.at() {
            call_expr = self.parse_call_expression(call_expr);
        }
        call_expr
    }

    fn parse_args(&mut self) -> Vec<Ast> {
        self.expect(OpenParen, "expect open params before params");
        let mut args: Vec<Ast> = vec![];
        if !matches!(self.at(), Some(Token::CloseParen)) {
            args = self.parse_args_list();
        }
        self.expect(CloseParen, "expect close parentheses following arguments");
        args
    }

    fn parse_args_list(&mut self) -> Vec<Ast> {
        let mut args: Vec<Ast> = vec![];
        args.push(self.parse_assignment_expression());
        while let Some(Token::Comma) = self.at() {
            self.eat();
            args.push(self.parse_assignment_expression());
        }
        args
    }
}

pub fn parse(tokens: Vec<Token>) -> Ast {
    let mut parser = Parser::new(tokens);
    parser.parse()
}
