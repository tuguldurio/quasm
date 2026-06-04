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

        while self.peek_is(TokenKind::Or) {
            self.advance();
            let right = self.parse_and()?;
            left = Expression::BinaryOp { op: BinOp::Or, left: Box::new(left), right: Box::new(right) };
        }

        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_equality()?;

        while self.peek_is(TokenKind::And) {
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
                TokenKind::EqEq   => BinOp::EqEq,
                TokenKind::BangEq => BinOp::NotEq,
                _ => break
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
                TokenKind::Lt   => BinOp::Lt,
                TokenKind::Gt   => BinOp::Gt,
                TokenKind::LtEq => BinOp::LtEq,
                TokenKind::GtEq => BinOp::GtEq,
                _ => break
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
                TokenKind::Plus  => BinOp::Add,
                TokenKind::Minus => BinOp::Sub,
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
                TokenKind::Asterisk => BinOp::Mul,
                TokenKind::Slash    => BinOp::Div,
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
            TokenKind::Minus => UnaryOp::Neg,
            TokenKind::Bang  => UnaryOp::Not,
            _ => return self.parse_primary()
        };

        self.advance();
        let operand = self.parse_unary()?;
        Ok(Expression::UnaryOp { op, operand: Box::new(operand) })
    }

    fn parse_primary(&mut self) -> Result<Expression, ParseError> {
        match self.peek() {
            TokenKind::Int(value) => {
                self.advance();
                Ok(Expression::Int(IntLit { value }))
            }
            TokenKind::True => {
                self.advance();
                Ok(Expression::Bool(BoolLit { value: true }))
            }
            TokenKind::False => {
                self.advance();
                Ok(Expression::Bool(BoolLit { value: false }))
            }
            TokenKind::LParen => {
                self.advance();
                // newlines after '(' and before ')' are allowed
                self.skip_newlines();
                let expr = self.parse_expr()?;
                self.skip_newlines();
                self.consume(TokenKind::RParen)?;
                Ok(expr)
            }
            TokenKind::Identifier(_) => {
                let ident = self.parse_identifier()?;
                if self.peek() == TokenKind::LParen {
                    let args = self.parse_call_args()?;
                    Ok(Expression::Call { callee: ident, args })
                } else {
                    Ok(Expression::Identifier(ident))
                }
            }
            other => Err(self.err(format!("expected expression, got {:?}", other)))
        }
    }

    fn parse_call_args(&mut self) -> Result<Vec<Expression>, ParseError> {
        self.consume(TokenKind::LParen)?;
        let mut args = Vec::new();

        while !matches!(self.peek(), TokenKind::RParen | TokenKind::Eof) {
            args.push(self.parse_expr()?);
            if self.peek() == TokenKind::Comma {
                self.advance();
            } else {
                break;
            }
        }

        self.consume(TokenKind::RParen)?;
        Ok(args)
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
