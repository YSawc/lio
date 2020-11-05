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

    pub fn new_if(c: Node, cond: NodeSt, vstmt: Vec<NodeSt>) -> Self {
        Self {
            c,
            cond: Some(Box::new(cond)),
            if_stmts: Some(Box::new(vstmt)),
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
    pub fn parser(
        mut it: &mut std::iter::Peekable<std::slice::Iter<Annot<TokenKind>>>,
    ) -> Result<Self, ParseError> {
        let lhs = Self::stmt(&mut it)?;
        Ok(lhs)
    }

    pub fn stmt(
        it: &mut std::iter::Peekable<std::slice::Iter<Annot<TokenKind>>>,
    ) -> Result<NodeSt, ParseError> {
        match it.peek().unwrap().value {
            TokenKind::Return | TokenKind::Int | TokenKind::If | TokenKind::UnderScore => {
                match it.peek().unwrap() {
                    Token {
                        value: TokenKind::Return,
                        loc,
                    } => {
                        let mut et = it.clone();
                        it.next().unwrap();
                        let op = Node::ret(loc.to_owned());
                        let mut lhs = Self::cmp(it)?;
                        lhs = Self::new_unary(op, lhs);
                        if it.peek() == None {
                            et.next();
                            return Err(ParseError::NotClosedStmt(et.next().unwrap().to_owned()));
                        }
                        expect_token(
                            TokenKind::SemiColon,
                            ParseError::NotClosedStmt(it.peek().unwrap().to_owned().to_owned()),
                            it,
                        )?;
                        return Ok(lhs);
                    }
                    Token {
                        value: TokenKind::Int,
                        loc,
                    } => {
                        it.next().unwrap();
                        let i = Node::int(loc.to_owned());
                        let (s, _) = expect_ident(
                            ParseError::NotIdent(it.peek().unwrap().to_owned().to_owned()),
                            it,
                        )?;
                        let l =
                            Self::new_ident(s.to_owned(), it.peek().unwrap().to_owned().to_owned());
                        let l = Self::new_unary(i, l);
                        expect_token(
                            TokenKind::Assign,
                            ParseError::NotAssign(it.peek().unwrap().to_owned().to_owned()),
                            it,
                        )?;
                        let op = Node::new_assign(loc.to_owned());
                        let r = Self::cmp(it)?;
                        let lhs = Self::new_nds(op, l, r);
                        if it.peek() == None {
                            return Err(ParseError::NotClosedStmt(
                                it.peek().unwrap().to_owned().to_owned(),
                            ));
                        }
                        expect_token(
                            TokenKind::SemiColon,
                            ParseError::NotClosedStmt(it.peek().unwrap().to_owned().to_owned()),
                            it,
                        )?;
                        return Ok(lhs);
                    }
                    Token {
                        value: TokenKind::If,
                        loc,
                    } => {
                        it.next().unwrap();
                        let op = Node::mif(loc.to_owned());
                        expect_token(
                            TokenKind::LParen,
                            ParseError::NotOpenedParen(it.peek().unwrap().to_owned().to_owned()),
                            it,
                        )?;
                        let cond = Self::cmp(it)?;
                        expect_token(
                            TokenKind::RParen,
                            ParseError::NotClosedStmt(it.peek().unwrap().to_owned().to_owned()),
                            it,
                        )?;
                        expect_token(
                            TokenKind::LBrace,
                            ParseError::NotOpenedStmt(it.peek().unwrap().to_owned().to_owned()),
                            it,
                        )?;

                        let mut if_stmts: Vec<NodeSt> = vec![];
                        while it.peek().unwrap().value != TokenKind::RBrace {
                            if_stmts.push(Self::stmt(it)?);
                        }

                        let mut lhs = Self::new_if(op, cond.to_owned(), if_stmts);

                        expect_token(
                            TokenKind::RBrace,
                            ParseError::NotClosedStmt(it.peek().unwrap().to_owned().to_owned()),
                            it,
                        )?;
                        match it.peek().unwrap() {
                            Token {
                                value: TokenKind::Else,
                                ..
                            } => {
                                it.next().unwrap();
                                expect_token(
                                    TokenKind::LBrace,
                                    ParseError::NotOpenedStmt(
                                        it.peek().unwrap().to_owned().to_owned(),
                                    ),
                                    it,
                                )?;

                                let mut else_if_stmts: Vec<NodeSt> = vec![];
                                while it.peek().unwrap().value != TokenKind::RBrace {
                                    else_if_stmts.push(Self::stmt(it)?);
                                }

                                lhs.else_if_stmts = Some(Box::new(else_if_stmts));

                                expect_token(
                                    TokenKind::RBrace,
                                    ParseError::NotClosedStmt(
                                        it.peek().unwrap().to_owned().to_owned(),
                                    ),
                                    it,
                                )?;
                                return Ok(lhs);
                            }
                            _ => return Ok(lhs),
                        }
                    }
                    Token {
                        value: TokenKind::UnderScore,
                        loc,
                    } => {
                        let mut et = it.clone();
                        it.next().unwrap();
                        let u = Node::under_score(loc.to_owned());
                        let op = Self::new_node(u);
                        if it.peek() == None {
                            et.next();
                            return Err(ParseError::NotClosedStmt(et.next().unwrap().to_owned()));
                        }
                        match it.peek().unwrap() {
                            Token {
                                value: TokenKind::SemiColon,
                                ..
                            } => {
                                it.next();
                            }
                            _ => (),
                        }
                        return Ok(op);
                    }
                    _ => unreachable!(),
                }
            }
            _ => {
                let lhs = Self::cmp(it)?;
                match it.peek().unwrap().value {
                    TokenKind::SemiColon | TokenKind::RBrace => match it.peek().unwrap() {
                        Token {
                            value: TokenKind::SemiColon,
                            ..
                        } => {
                            it.next();
                            return Ok(lhs);
                        }
                        Token {
                            value: TokenKind::RBrace,
                            ..
                        } => {
                            return Ok(lhs);
                        }
                        _ => unreachable!(),
                    },
                    _ => return Err(ParseError::NotClosedStmt(it.next().unwrap().to_owned())),
                }
            }
        }
    }

    pub fn cmp(
        mut it: &mut std::iter::Peekable<std::slice::Iter<Annot<TokenKind>>>,
    ) -> Result<NodeSt, ParseError> {
        let mut lhs = Self::expr(it)?;

        loop {
            match it.peek().map(|vt| vt.value.to_owned()) {
                Some(TokenKind::E) | Some(TokenKind::NE) | Some(TokenKind::L)
                | Some(TokenKind::LE) | Some(TokenKind::G) | Some(TokenKind::GE) => {
                    let op = match it.next().unwrap() {
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
                    let rhs = Self::expr(&mut it)?;

                    lhs = Self::new_nds(op, lhs, rhs);
                }
                _ => return Ok(lhs),
            }
        }
    }

    pub fn expr(
        mut it: &mut std::iter::Peekable<std::slice::Iter<Annot<TokenKind>>>,
    ) -> Result<NodeSt, ParseError> {
        let mut lhs = Self::mul(it)?;

        loop {
            match it.peek().map(|vt| vt.value.to_owned()) {
                Some(TokenKind::Plus) | Some(TokenKind::Minus) => {
                    let op = match it.next().unwrap() {
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
                    let rhs = Self::mul(&mut it)?;

                    lhs = Self::new_nds(op, lhs, rhs);
                }
                _ => return Ok(lhs),
            }
        }
    }

    pub fn mul(
        mut it: &mut std::iter::Peekable<std::slice::Iter<Annot<TokenKind>>>,
    ) -> Result<NodeSt, ParseError> {
        let mut lhs = Self::unary(it)?;

        loop {
            match it.peek().map(|vt| vt.value.to_owned()) {
                Some(TokenKind::Asterisk) | Some(TokenKind::Slash) | Some(TokenKind::Percent) => {
                    let op = match it.next().unwrap() {
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
                    let rhs = Self::unary(&mut it)?;

                    lhs = Self::new_nds(op, lhs, rhs);
                }
                _ => return Ok(lhs),
            }
        }
    }

    pub fn unary(
        mut it: &mut std::iter::Peekable<std::slice::Iter<Annot<TokenKind>>>,
    ) -> Result<NodeSt, ParseError> {
        match it.peek().map(|vt| vt.value.to_owned()) {
            Some(TokenKind::LParen) => {
                let rhs = match it.next().unwrap() {
                    Token {
                        value: TokenKind::LParen,
                        ..
                    } => Self::cmp(&mut it)?,
                    _ => unreachable!(),
                };
                if it.peek() == None {
                    return Err(ParseError::Eof);
                }
                expect_token(
                    TokenKind::RParen,
                    ParseError::NotClosedParen(it.peek().unwrap().to_owned().to_owned()),
                    it,
                )?;
                Ok(rhs)
            }
            _ => Self::primary(it),
        }
    }

    pub fn primary(
        it: &mut std::iter::Peekable<std::slice::Iter<Annot<TokenKind>>>,
    ) -> Result<NodeSt, ParseError> {
        if it.peek() == None {
            return Err(ParseError::Eof);
        }

        match it.peek().unwrap() {
            Token {
                value: TokenKind::Ident(s),
                ..
            } => {
                let t = it.to_owned();
                let lhs = Self::new_ident(s.to_owned(), it.next().unwrap().to_owned());
                if it.len() < 2 {
                    *it = t.to_owned();
                }
                match it.peek().unwrap() {
                    Token {
                        value: TokenKind::Assign,
                        loc,
                    } => {
                        it.next().unwrap();
                        let op = Node::assign(loc.to_owned());
                        let rhs = Self::expr(it)?;
                        let lhs = Self::new_nds(op, lhs, rhs);
                        return Ok(lhs);
                    }
                    _ => {
                        *it = t;
                        return Ok(Self::new_ident(
                            s.to_string(),
                            it.next().unwrap().to_owned(),
                        ));
                    }
                }
            }
            _ => return Ok(Self::new_num(it.next().unwrap().to_owned())?),
        }
    }
}

fn expect_ident(
    err: ParseError,
    it: &mut std::iter::Peekable<std::slice::Iter<Annot<TokenKind>>>,
) -> Result<(String, Loc), ParseError> {
    match it.peek().unwrap() {
        Token {
            value: TokenKind::Ident(s),
            ..
        } => Ok((s.to_string(), it.next().unwrap().loc.to_owned())),
        _ => Err(err),
    }
}

fn expect_token(
    ty: TokenKind,
    err: ParseError,
    it: &mut std::iter::Peekable<std::slice::Iter<Annot<TokenKind>>>,
) -> Result<Loc, ParseError> {
    if it.peek().unwrap().value == ty {
        Ok(it.next().unwrap().loc.to_owned())
    } else {
        Err(err)
    }
}
