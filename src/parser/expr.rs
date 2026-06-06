use crate::lexer::TokenKind;
use super::ast::*;
use super::Parser;
use super::ParseError;

impl Parser {
    pub(super) fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_and()?;

        while self.peek_is(TokenKind::Or) {
            self.advance();
            let right = self.parse_and()?;
            left = Expr::BinaryOp { op: BinOp::Or, left: Box::new(left), right: Box::new(right) };
        }

        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_equality()?;

        while self.peek_is(TokenKind::And) {
            self.advance();
            let right = self.parse_equality()?;
            left = Expr::BinaryOp { op: BinOp::And, left: Box::new(left), right: Box::new(right) };
        }

        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_comparison()?;
        loop {
            let op = match self.peek() {
                TokenKind::EqEq => BinOp::EqEq,
                TokenKind::BangEq => BinOp::NotEq,
                _ => break
            };
            self.advance();
            let right = self.parse_comparison()?;
            left = Expr::BinaryOp { op, left: Box::new(left), right: Box::new(right) };
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_additive()?;
        loop {
            let op = match self.peek() {
                TokenKind::Lt => BinOp::Lt,
                TokenKind::Gt => BinOp::Gt,
                TokenKind::LtEq => BinOp::LtEq,
                TokenKind::GtEq => BinOp::GtEq,
                _ => break
            };
            self.advance();
            let right = self.parse_additive()?;
            left = Expr::BinaryOp { op, left: Box::new(left), right: Box::new(right) };
        }
        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_multiplicative()?;

        loop {
            let op = match self.peek() {
                TokenKind::Plus => BinOp::Add,
                TokenKind::Minus => BinOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_multiplicative()?;
            left = Expr::BinaryOp { op, left: Box::new(left), right: Box::new(right) };
        }

        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_unary()?;

        loop {
            let op = match self.peek() {
                TokenKind::Asterisk => BinOp::Mul,
                TokenKind::Slash => BinOp::Div,
                _ => break,
            };
            self.advance();
            let right = self.parse_unary()?;
            left = Expr::BinaryOp { op, left: Box::new(left), right: Box::new(right) };
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        let op = match self.peek() {
            TokenKind::Minus => UnaryOp::Neg,
            TokenKind::Bang => UnaryOp::Not,
            _ => return self.parse_postfix(),
        };

        self.advance();
        let operand = self.parse_unary()?;
        Ok(Expr::UnaryOp { op, operand: Box::new(operand) })
    }

    fn parse_postfix(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_primary()?;

        loop {
            match self.peek() {
                TokenKind::Dot => {
                    self.advance();
                    let member = self.parse_identifier()?;
                    expr = Expr::MemberAccess { base: Box::new(expr), member };
                }
                TokenKind::LParen => {
                    let args = self.parse_call_args()?;
                    expr = Expr::Call { callee: Box::new(expr), args };
                }
                _ => break
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        match self.peek() {
            TokenKind::Int(value) => {
                self.advance();
                Ok(Expr::Int(IntLit { value }))
            }
            TokenKind::Float(value) => {
                self.advance();
                Ok(Expr::Float(FloatLit { value }))
            }
            TokenKind::True => {
                self.advance();
                Ok(Expr::Bool(BoolLit { value: true }))
            }
            TokenKind::False => {
                self.advance();
                Ok(Expr::Bool(BoolLit { value: false }))
            }
            TokenKind::LParen => {
                self.advance();
                self.skip_newlines();
                let expr = self.parse_expr()?;
                self.skip_newlines();
                self.consume(TokenKind::RParen)?;
                Ok(expr)
            }
            TokenKind::SelfTok => {
                self.advance();
                Ok(Expr::Identifier(Identifier { value: "self".into() }))
            }
            TokenKind::Identifier(_) => {
                Ok(Expr::Identifier(self.parse_identifier()?))
            }
            other => Err(self.err(format!("expected expression, got {:?}", other)))
        }
    }

    fn parse_call_args(&mut self) -> Result<Vec<Expr>, ParseError> {
        self.parse_comma_list(TokenKind::LParen, TokenKind::RParen, "argument", |p| p.parse_expr())
    }

    pub(super) fn parse_identifier(&mut self) -> Result<Identifier, ParseError> {
        match self.peek() {
            TokenKind::Identifier(value) => {
                self.advance();
                Ok(Identifier { value })
            }
            other => Err(self.err(format!("expected identifier, got {:?}", other)))
        }
    }
}
