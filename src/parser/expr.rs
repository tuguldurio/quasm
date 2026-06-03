use crate::lexer::TokenKind;
use super::ast::*;
use super::Parser;
use super::ParseError;

impl Parser {
    pub(super) fn parse_expr(&mut self) -> Result<Expression, ParseError> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_and()?;

        while matches!(self.peek(), Some(TokenKind::Or)) {
            self.advance();
            let right = self.parse_and()?;
            left = Expression::BinaryOp { op: BinOp::Or, left: Box::new(left), right: Box::new(right) };
        }

        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_equality()?;

        while matches!(self.peek(), Some(TokenKind::And)) {
            self.advance();
            let right = self.parse_equality()?;
            left = Expression::BinaryOp { op: BinOp::And, left: Box::new(left), right: Box::new(right) };
        }
        
        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_comparison()?;
        loop {
            let op = match self.peek() {
                Some(TokenKind::EqEq)   => BinOp::EqEq,
                Some(TokenKind::BangEq) => BinOp::NotEq,
                _ => break,
            };
            self.advance();
            let right = self.parse_comparison()?;
            left = Expression::BinaryOp { op, left: Box::new(left), right: Box::new(right) };
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_additive()?;
        loop {
            let op = match self.peek() {
                Some(TokenKind::Lt)   => BinOp::Lt,
                Some(TokenKind::Gt)   => BinOp::Gt,
                Some(TokenKind::LtEq) => BinOp::LtEq,
                Some(TokenKind::GtEq) => BinOp::GtEq,
                _ => break,
            };
            self.advance();
            let right = self.parse_additive()?;
            left = Expression::BinaryOp { op, left: Box::new(left), right: Box::new(right) };
        }
        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_multiplicative()?;

        loop {
            let op = match self.peek() {
                Some(TokenKind::Plus)  => BinOp::Add,
                Some(TokenKind::Minus) => BinOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_multiplicative()?;
            left = Expression::BinaryOp { op, left: Box::new(left), right: Box::new(right) };
        }
        
        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_unary()?;
        
        loop {
            let op = match self.peek() {
                Some(TokenKind::Asterisk) => BinOp::Mul,
                Some(TokenKind::Slash)    => BinOp::Div,
                _ => break,
            };
            self.advance();
            let right = self.parse_unary()?;
            left = Expression::BinaryOp { op, left: Box::new(left), right: Box::new(right) };
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expression, ParseError> {
        let op = match self.peek() {
            Some(TokenKind::Minus) => UnaryOp::Neg,
            Some(TokenKind::Bang)  => UnaryOp::Not,
            _ => return self.parse_primary(),
        };

        self.advance();
        let operand = self.parse_unary()?;
        Ok(Expression::UnaryOp { op, operand: Box::new(operand) })
    }

    fn parse_primary(&mut self) -> Result<Expression, ParseError> {
        match self.peek() {
            Some(TokenKind::Int(n)) => {
                self.advance();
                Ok(Expression::Int(n))
            }
            Some(TokenKind::True) => {
                self.advance();
                Ok(Expression::Bool(true))
            }
            Some(TokenKind::False) => {
                self.advance();
                Ok(Expression::Bool(false))
            }
            Some(TokenKind::LParen) => {
                self.advance();
                // newlines after '(' and before ')' are allowed
                self.skip_newlines();
                let expr = self.parse_expr()?;
                self.skip_newlines();
                self.consume(TokenKind::RParen)?;
                Ok(expr)
            }
            other => Err(self.err(format!("expected expression, got {:?}", other))),
        }
    }
}
