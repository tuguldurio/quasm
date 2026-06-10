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
                    let field = self.parse_identifier()?;
                    expr = Expr::FieldAccess { base: Box::new(expr), field };
                }
                TokenKind::LParen => {
                    let args = self.parse_call_args()?;
                    expr = Expr::Call { callee: Box::new(expr), args };
                }
                TokenKind::LBracket => {
                    self.advance();
                    let index = self.parse_expr()?;
                    self.consume(TokenKind::RBracket)?;
                    expr = Expr::Index { base: Box::new(expr), index: Box::new(index) };
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
                Ok(Expr::Literal(Literal::Int(value)))
            }
            TokenKind::Float(value) => {
                self.advance();
                Ok(Expr::Literal(Literal::Float(value)))
            }
            TokenKind::True => {
                self.advance();
                Ok(Expr::Literal(Literal::Bool(true)))
            }
            TokenKind::False => {
                self.advance();
                Ok(Expr::Literal(Literal::Bool(false)))
            }
            TokenKind::LParen => {
                self.advance();
                self.skip_newlines();
                let expr = self.parse_expr()?;
                self.skip_newlines();
                self.consume(TokenKind::RParen)?;
                Ok(expr)
            }
            TokenKind::LBracket => {
                let elems = self.parse_comma_list(TokenKind::LBracket, TokenKind::RBracket, "element", |p| p.parse_expr())?;
                Ok(Expr::Array(elems))
            }
            TokenKind::LBrace => {
                Ok(Expr::Block(self.parse_block()?))
            }
            TokenKind::If => {
                self.advance();
                self.parse_if_expr()
            }
            TokenKind::Match => {
                self.parse_match_expr()
            }
            TokenKind::VerBar | TokenKind::Or => {
                self.parse_closure()
            }
            TokenKind::Identifier(_) => {
                Ok(Expr::Identifier(self.parse_identifier()?))
            }
            other => Err(self.err(format!("expected expression, got {:?}", other)))
        }
    }

    fn parse_if_expr(&mut self) -> Result<Expr, ParseError> {
        let condition = self.parse_expr()?;
        let then_block = self.parse_block()?;

        let else_branch = match self.peek() {
            TokenKind::Elif => {
                self.advance();
                Some(Box::new(self.parse_if_expr()?))
            }
            TokenKind::Else => {
                self.advance();
                Some(Box::new(Expr::Block(self.parse_block()?)))
            }
            _ => None
        };

        Ok(Expr::If { condition: Box::new(condition), then_block, else_branch })
    }

    fn parse_call_args(&mut self) -> Result<Vec<Expr>, ParseError> {
        self.parse_comma_list(TokenKind::LParen, TokenKind::RParen, "argument", |p| p.parse_expr())
    }

    fn parse_closure(&mut self) -> Result<Expr, ParseError> {
        // || lexes as the Or token, so an empty parameter list arrives as a single token
        let params = if self.peek_is(TokenKind::Or) {
            self.advance();
            Vec::new()
        } else {
            self.parse_comma_list(TokenKind::VerBar, TokenKind::VerBar, "parameter", |p| p.parse_param())?
        };

        let ret = if self.peek_is(TokenKind::Arrow) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        self.consume(TokenKind::FatArrow)?;
        let body = self.parse_expr()?;

        Ok(Expr::Closure { params, ret, body: Box::new(body) })
    }

    fn parse_match_expr(&mut self) -> Result<Expr, ParseError> {
        self.consume(TokenKind::Match)?;
        let subject = self.parse_expr()?;
        let arms = self.parse_braced_list("match arm", |p| p.parse_match_arm())?;
        Ok(Expr::Match { subject: Box::new(subject), arms })
    }

    fn parse_match_arm(&mut self) -> Result<MatchArm, ParseError> {
        let pattern = self.parse_pattern()?;

        let guard = if self.peek_is(TokenKind::If) {
            self.advance();
            Some(self.parse_expr()?)
        } else {
            None
        };

        self.consume(TokenKind::FatArrow)?;
        let body = self.parse_expr()?;
        Ok(MatchArm { pattern, guard, body })
    }

    fn parse_pattern(&mut self) -> Result<Pattern, ParseError> {
        let first = self.parse_single_pattern()?;
        if !self.peek_is(TokenKind::VerBar) {
            return Ok(first);
        }

        let mut alternatives = vec![first];
        while self.peek_is(TokenKind::VerBar) {
            self.advance();
            alternatives.push(self.parse_single_pattern()?);
        }
        Ok(Pattern::Or(alternatives))
    }

    fn parse_single_pattern(&mut self) -> Result<Pattern, ParseError> {
        match self.peek() {
            TokenKind::Int(value) => {
                self.advance();
                Ok(Pattern::Literal(Literal::Int(value)))
            }
            TokenKind::Float(value) => {
                self.advance();
                Ok(Pattern::Literal(Literal::Float(value)))
            }
            TokenKind::True => {
                self.advance();
                Ok(Pattern::Literal(Literal::Bool(true)))
            }
            TokenKind::False => {
                self.advance();
                Ok(Pattern::Literal(Literal::Bool(false)))
            }
            TokenKind::Minus => {
                self.advance();
                match self.peek() {
                    TokenKind::Int(value) => {
                        self.advance();
                        Ok(Pattern::Literal(Literal::Int(-value)))
                    }
                    TokenKind::Float(value) => {
                        self.advance();
                        Ok(Pattern::Literal(Literal::Float(-value)))
                    }
                    other => Err(self.err(format!("expected numeric literal after '-' in pattern, got {:?}", other)))
                }
            }
            TokenKind::Identifier(_) => {
                let name = self.parse_identifier()?;
                if name.value == "_" {
                    return Ok(Pattern::Wildcard);
                }
                if self.peek_is(TokenKind::LParen) {
                    let args = self.parse_comma_list(TokenKind::LParen, TokenKind::RParen, "pattern", |p| p.parse_pattern())?;
                    return Ok(Pattern::Constructor { name, args });
                }
                Ok(Pattern::Identifier(name))
            }
            TokenKind::LBracket => self.parse_array_pattern(),
            other => Err(self.err(format!("expected pattern, got {:?}", other)))
        }
    }

    fn parse_array_pattern(&mut self) -> Result<Pattern, ParseError> {
        let elements = self.parse_comma_list(TokenKind::LBracket, TokenKind::RBracket, "pattern", |p| {
            if !p.peek_is(TokenKind::DotDot) {
                return p.parse_pattern();
            }
            p.advance();
            let name = if matches!(p.peek(), TokenKind::Identifier(_)) {
                Some(p.parse_identifier()?)
            } else {
                None
            };
            Ok(Pattern::Rest(name))
        })?;

        let rest_count = elements.iter().filter(|e| matches!(e, Pattern::Rest(_))).count();
        if rest_count > 1 {
            return Err(self.err("array pattern can have at most one rest pattern '..'"));
        }
        if rest_count == 1 && !matches!(elements.last(), Some(Pattern::Rest(_))) {
            return Err(self.err("rest pattern '..' must be the last element of an array pattern"));
        }

        Ok(Pattern::Array(elements))
    }
}
