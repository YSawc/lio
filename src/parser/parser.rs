use super::super::location::location::*;
use super::super::node::node::*;
use super::super::token::token::*;
use super::error::*;

impl NodeSt {
    pub fn new_unary(c: Node, lhs: NodeSt) -> Self {
        Self {
            c,
            lhs: Some(Box::new(lhs)),
            ..Default::default()
        }
    }

    pub fn new_if(c: Node, cond: NodeSt) -> Self {
        Self {
            c,
            cond: Some(Box::new(cond)),
            ..Default::default()
        }
    }

    pub fn new_nds(c: Node, lhs: NodeSt, rhs: NodeSt) -> Self {
        Self {
            c,
            lhs: Some(Box::new(lhs)),
            rhs: Some(Box::new(rhs)),
            ..Default::default()
        }
    }

    pub fn new_num(mut t: Token) -> Result<Self, ParseError> {
        let n = Token::get_val(&mut t).or(Err(ParseError::NotNumber(t.to_owned())))?;
        Ok(Self {
            c: Annot::new(NodeKind::Num(n), t.loc),
            ..Default::default()
        })
    }

    pub fn new_ident(s: String, t: Token) -> Self {
        Self {
            c: Annot::new(NodeKind::Ident(s), t.loc),
            ..Default::default()
        }
    }
    pub fn new_node(c: Node) -> Self {
        Self {
            c,
            ..Default::default()
        }
    }
}

impl NodeSt {
    pub fn parser(it: &mut TokenIter) -> Result<Self, ParseError> {
        let lhs = Self::stmt(it)?;
        Ok(lhs)
    }

    pub fn stmt(it: &mut TokenIter) -> Result<NodeSt, ParseError> {
        match it.p.peek().unwrap().value {
            TokenKind::Return | TokenKind::Int | TokenKind::If | TokenKind::UnderScore => {
                match it.p.peek().unwrap() {
                    Token {
                        value: TokenKind::Return,
                        loc,
                    } => {
                        it.next_with_shadow();
                        let op = Node::ret(loc.to_owned());
                        let mut lhs = Self::cmp(it)?;
                        lhs = Self::new_unary(op, lhs);
                        if it.p.peek() == None {
                            return Err(ParseError::NotClosedStmt(
                                it.shadow_p.next().unwrap().to_owned(),
                            ));
                        }
                        it.expect_token(
                            TokenKind::SemiColon,
                            ParseError::NotClosedStmt(
                                it.p.to_owned().peek().unwrap().to_owned().to_owned(),
                            ),
                        )?;
                        return Ok(lhs);
                    }
                    Token {
                        value: TokenKind::Int,
                        loc,
                    } => {
                        it.p.next().unwrap();
                        let i = Node::int(loc.to_owned());
                        let (s, _) = it.expect_ident(ParseError::NotIdent(
                            it.p.to_owned().peek().unwrap().to_owned().to_owned(),
                        ))?;
                        let l = Self::new_ident(
                            s.to_owned(),
                            it.p.peek().unwrap().to_owned().to_owned(),
                        );
                        let l = Self::new_unary(i, l);
                        it.expect_token(
                            TokenKind::Assign,
                            ParseError::NotAssign(
                                it.p.to_owned().peek().unwrap().to_owned().to_owned(),
                            ),
                        )?;
                        let op = Node::new_assign(loc.to_owned());
                        let r = Self::cmp(it)?;
                        let lhs = Self::new_nds(op, l, r);
                        if it.p.peek() == None {
                            return Err(ParseError::NotClosedStmt(
                                it.p.peek().unwrap().to_owned().to_owned(),
                            ));
                        }
                        it.expect_token(
                            TokenKind::SemiColon,
                            ParseError::NotClosedStmt(
                                it.p.to_owned().peek().unwrap().to_owned().to_owned(),
                            ),
                        )?;
                        return Ok(lhs);
                    }
                    Token {
                        value: TokenKind::If,
                        loc,
                    } => {
                        it.p.next().unwrap();
                        let op = Node::mif(loc.to_owned());
                        it.expect_token(
                            TokenKind::LParen,
                            ParseError::NotOpenedParen(
                                it.p.to_owned().peek().unwrap().to_owned().to_owned(),
                            ),
                        )?;

                        let cond = Self::cmp(it)?;
                        it.expect_token(
                            TokenKind::RParen,
                            ParseError::NotClosedStmt(
                                it.p.to_owned().peek().unwrap().to_owned().to_owned(),
                            ),
                        )?;
                        let lhs = Self::new_if(op, cond.to_owned());
                        return Ok(lhs);
                    }
                    Token {
                        value: TokenKind::UnderScore,
                        loc,
                    } => {
                        it.next_with_shadow();
                        let u = Node::under_score(loc.to_owned());
                        let op = Self::new_node(u);
                        if it.p.peek() == None {
                            it.shadow_p.next();
                            return Err(ParseError::NotClosedStmt(
                                it.shadow_p.next().unwrap().to_owned(),
                            ));
                        }

                        it.unexpect_token(
                            TokenKind::SemiColon,
                            ParseError::UnexpectedUnderScoreOperator(
                                it.p.to_owned().peek().unwrap().to_owned().to_owned().loc,
                            ),
                        )?;

                        return Ok(op);
                    }
                    _ => unreachable!(),
                }
            }
            _ => return Self::cmp(it),
        }
    }

    pub fn cmp(it: &mut TokenIter) -> Result<NodeSt, ParseError> {
        let mut lhs = Self::expr(it)?;

        loop {
            match it.p.peek().map(|vt| vt.value.to_owned()) {
                Some(TokenKind::E) | Some(TokenKind::NE) | Some(TokenKind::L)
                | Some(TokenKind::LE) | Some(TokenKind::G) | Some(TokenKind::GE) => {
                    let op = match it.p.next().unwrap() {
                        Token {
                            value: TokenKind::E,
                            loc,
                        } => Node::eq(loc.to_owned()),
                        Token {
                            value: TokenKind::NE,
                            loc,
                        } => Node::neq(loc.to_owned()),
                        Token {
                            value: TokenKind::L,
                            loc,
                        } => Node::l(loc.to_owned()),
                        Token {
                            value: TokenKind::LE,
                            loc,
                        } => Node::le(loc.to_owned()),
                        Token {
                            value: TokenKind::G,
                            loc,
                        } => Node::g(loc.to_owned()),
                        Token {
                            value: TokenKind::GE,
                            loc,
                        } => Node::ge(loc.to_owned()),

                        _ => unreachable!(),
                    };
                    let rhs = Self::expr(it)?;

                    lhs = Self::new_nds(op, lhs, rhs);
                }
                _ => return Ok(lhs),
            }
        }
    }

    pub fn expr(it: &mut TokenIter) -> Result<NodeSt, ParseError> {
        let mut lhs = Self::mul(it)?;

        loop {
            match it.p.peek().map(|vt| vt.value.to_owned()) {
                Some(TokenKind::Plus) | Some(TokenKind::Minus) => {
                    let op = match it.p.next().unwrap() {
                        Token {
                            value: TokenKind::Plus,
                            loc,
                        } => Node::plus(loc.to_owned()),
                        Token {
                            value: TokenKind::Minus,
                            loc,
                        } => Node::minus(loc.to_owned()),
                        _ => unreachable!(),
                    };
                    let rhs = Self::mul(it)?;

                    lhs = Self::new_nds(op, lhs, rhs);
                }
                _ => return Ok(lhs),
            }
        }
    }

    pub fn mul(it: &mut TokenIter) -> Result<NodeSt, ParseError> {
        let mut lhs = Self::unary(it)?;

        loop {
            match it.p.peek().map(|vt| vt.value.to_owned()) {
                Some(TokenKind::Asterisk) | Some(TokenKind::Slash) | Some(TokenKind::Percent) => {
                    let op = match it.p.next().unwrap() {
                        Token {
                            value: TokenKind::Asterisk,
                            loc,
                        } => Node::mul(loc.to_owned()),
                        Token {
                            value: TokenKind::Slash,
                            loc,
                        } => Node::div(loc.to_owned()),
                        Token {
                            value: TokenKind::Percent,
                            loc,
                        } => Node::surplus(loc.to_owned()),

                        _ => unreachable!(),
                    };
                    let rhs = Self::unary(it)?;

                    lhs = Self::new_nds(op, lhs, rhs);
                }
                _ => return Ok(lhs),
            }
        }
    }

    pub fn unary(it: &mut TokenIter) -> Result<NodeSt, ParseError> {
        match it.p.peek().map(|vt| vt.value.to_owned()) {
            Some(TokenKind::LParen) => {
                let rhs = match it.p.next().unwrap() {
                    Token {
                        value: TokenKind::LParen,
                        ..
                    } => Self::cmp(it)?,
                    _ => unreachable!(),
                };
                if it.p.peek() == None {
                    return Err(ParseError::Eof);
                }
                it.expect_token(
                    TokenKind::RParen,
                    ParseError::NotClosedParen(
                        it.p.to_owned().peek().unwrap().to_owned().to_owned(),
                    ),
                )?;
                Ok(rhs)
            }
            _ => Self::primary(it),
        }
    }

    pub fn primary(it: &mut TokenIter) -> Result<NodeSt, ParseError> {
        if it.p.peek() == None {
            return Err(ParseError::Eof);
        }

        match it.p.peek().unwrap() {
            Token {
                value: TokenKind::Ident(s),
                ..
            } => {
                let t = it.p.to_owned();
                let lhs = Self::new_ident(s.to_owned(), it.p.next().unwrap().to_owned());
                if it.p.len() < 2 {
                    it.p = t.to_owned();
                }
                match it.p.peek().unwrap() {
                    Token {
                        value: TokenKind::Assign,
                        loc,
                    } => {
                        it.p.next();
                        let op = Node::assign(loc.to_owned());
                        let rhs = Self::expr(it)?;
                        let lhs = Self::new_nds(op, lhs, rhs);

                        it.expect_token(
                            TokenKind::SemiColon,
                            ParseError::NotClosedParen(
                                it.p.to_owned().peek().unwrap().to_owned().to_owned(),
                            ),
                        )?;
                        return Ok(lhs);
                    }
                    _ => {
                        it.p = t;
                        return Ok(Self::new_ident(
                            s.to_string(),
                            it.p.next().unwrap().to_owned(),
                        ));
                    }
                }
            }
            _ => return Ok(Self::new_num(it.p.next().unwrap().to_owned())?),
        }
    }
}

impl<'a> TokenIter<'a> {
    pub fn expect_ident(&mut self, err: ParseError) -> Result<(String, Loc), ParseError> {
        match self.p.peek().unwrap() {
            Token {
                value: TokenKind::Ident(s),
                ..
            } => Ok((s.to_string(), self.p.next().unwrap().loc.to_owned())),
            _ => Err(err),
        }
    }

    pub fn expect_token(&mut self, ty: TokenKind, err: ParseError) -> Result<Loc, ParseError> {
        if self.p.peek().unwrap().value == ty {
            Ok(self.p.next().unwrap().loc.to_owned())
        } else {
            Err(err)
        }
    }

    pub fn unexpect_token(&mut self, ty: TokenKind, err: ParseError) -> Result<(), ParseError> {
        if self.p.peek().unwrap().value == ty {
            Err(err)
        } else {
            Ok(())
        }
    }

    pub fn check_evaluate_type(&mut self) -> bool {
        match self.peek_value() {
            TokenKind::RBrace => true,
            _ => false,
        }
    }

    pub fn check_evaluate_void(&mut self) -> bool {
        match self.peek_value() {
            TokenKind::SemiColon => {
                self.next_with_shadow();
                if self.peek_value() == TokenKind::RBrace {
                    return true;
                }
                false
            }
            TokenKind::RBrace => true,
            _ => false,
        }
    }
}
