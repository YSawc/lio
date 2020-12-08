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

    pub fn new_c(c: Node, cond: NodeSt) -> Self {
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

    pub fn new_ret_set(r: ReturnSet) -> Self {
        Self {
            ret_set: Some(Box::new(r)),
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

    pub fn stmt(it: &mut TokenIter) -> Result<Self, ParseError> {
        match it.p.peek().unwrap().value {
            TokenKind::Return
            | TokenKind::Int
            | TokenKind::If
            | TokenKind::While
            | TokenKind::LBrace
            | TokenKind::Pipe
            | TokenKind::UnderScore => match it.p.peek().unwrap() {
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
                    it.next();
                    let i = Node::int(loc.to_owned());
                    let r = it.consume_ret_set()?;
                    let l = Self::new_ret_set(r);
                    let l = Self::new_unary(i, l);
                    it.expect_token(
                        TokenKind::Assign,
                        ParseError::NotAssign(
                            it.p.to_owned().peek().unwrap().to_owned().to_owned(),
                        ),
                    )?;
                    let op = Node::new_assign(loc.to_owned());
                    let lhs = Self::new_unary(op, l);
                    return Ok(lhs);
                }
                Token {
                    value: TokenKind::If,
                    loc,
                } => {
                    it.next();
                    let op = Node::mif(loc.to_owned());
                    let cond = it.consume_cond()?;
                    let lhs = Self::new_c(op, cond);
                    return Ok(lhs);
                }
                Token {
                    value: TokenKind::While,
                    loc,
                } => {
                    it.next();
                    let op = Node::mwhile(loc.to_owned());
                    let cond = it.consume_cond()?;
                    let lhs = Self::new_c(op, cond);
                    return Ok(lhs);
                }
                Token {
                    value: TokenKind::LBrace,
                    loc,
                } => {
                    let st = Node::st(loc.to_owned());
                    let lhs = Self::new_node(st);
                    return Ok(lhs);
                }
                Token {
                    value: TokenKind::Pipe,
                    loc,
                } => {
                    let op = Node::pipe(loc.to_owned());
                    let lhs = Self::new_node(op);
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
            },
            _ => return Self::cmp(it),
        }
    }

    pub fn cmp(it: &mut TokenIter) -> Result<Self, ParseError> {
        let mut lhs = Self::expr(it)?;

        loop {
            match it.p.peek().map(|vt| vt.value.to_owned()).unwrap() {
                TokenKind::E
                | TokenKind::NE
                | TokenKind::L
                | TokenKind::LE
                | TokenKind::G
                | TokenKind::GE => {
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

    pub fn expr(it: &mut TokenIter) -> Result<Self, ParseError> {
        let mut lhs = Self::mul(it)?;

        loop {
            match it.p.peek().map(|vt| vt.value.to_owned()).unwrap() {
                TokenKind::Plus | TokenKind::Minus => {
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

    pub fn mul(it: &mut TokenIter) -> Result<Self, ParseError> {
        let mut lhs = Self::unary(it)?;

        loop {
            match it.p.peek().map(|vt| vt.value.to_owned()).unwrap() {
                TokenKind::Asterisk | TokenKind::Slash | TokenKind::Percent => {
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

    pub fn unary(it: &mut TokenIter) -> Result<Self, ParseError> {
        match it.p.peek().map(|vt| vt.value.to_owned()) {
            Some(TokenKind::LParen) => {
                let rhs = match it.peek_token() {
                    Token {
                        value: TokenKind::LParen,
                        loc,
                    } => match it.look_ahead_of_rparen()? {
                        true => match it.sort_parse_type() {
                            ParseKind::Expression => Self::cmp(it)?,
                            ParseKind::Type => unimplemented!(),
                        },
                        false => {
                            let nd = Node::l_touple(loc.to_owned());
                            let op = Self::new_node(nd);
                            return Ok(op);
                        }
                    },
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

    pub fn primary(it: &mut TokenIter) -> Result<Self, ParseError> {
        if it.p.peek() == None {
            return Err(ParseError::Eof);
        }

        match it.p.peek().unwrap() {
            Token {
                value: TokenKind::Ident(s),
                ..
            } => {
                let t = it.p.to_owned();
                let l = Self::new_ident(s.to_owned(), it.p.next().unwrap().to_owned());
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
                        let lhs = Self::new_unary(op, l);
                        return Ok(lhs);
                    }
                    _ => {
                        it.p = t;
                        return Ok(Self::new_ident(s.to_string(), it.next()));
                    }
                }
            }
            _ => return Ok(Self::new_num(it.next())?),
        }
    }
}

impl<'a> TokenIter<'a> {
    pub fn expect_ident(&mut self, err: ParseError) -> Result<String, ParseError> {
        match self.p.peek().unwrap() {
            Token {
                value: TokenKind::Ident(s),
                ..
            } => {
                self.next();
                Ok(s.to_string())
            }
            _ => Err(err),
        }
    }

    pub fn expect_token(&mut self, ty: TokenKind, err: ParseError) -> Result<Loc, ParseError> {
        if self.p.peek().unwrap().value == ty {
            Ok(self.next().loc.to_owned())
        } else {
            Err(err)
        }
    }

    pub fn expect_peek_token(&mut self, ty: TokenKind, err: ParseError) -> Result<Loc, ParseError> {
        if self.p.peek().unwrap().value == ty {
            Ok(self.peek_shadow().loc.to_owned())
        } else {
            Err(err)
        }
    }

    pub fn consume_cond(&mut self) -> Result<NodeSt, ParseError> {
        self.expect_token(
            TokenKind::LParen,
            ParseError::NotOpenedParen(self.p.to_owned().peek().unwrap().to_owned().to_owned()),
        )?;

        let cond = NodeSt::cmp(self)?;

        self.expect_token(
            TokenKind::RParen,
            ParseError::NotClosedStmt(self.p.to_owned().peek().unwrap().to_owned().to_owned()),
        )?;

        Ok(cond)
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

    pub fn consume_ret_set(&mut self) -> Result<ReturnSet, ParseError> {
        match self.p.peek().unwrap() {
            Token {
                value: TokenKind::Ident(s),
                ..
            } => {
                self.next();
                let r = ReturnSet::new_single(s.to_owned());
                Ok(r)
            }
            Token {
                value: TokenKind::LParen,
                ..
            } => Ok(ReturnSet::new_touple(self.consume_touple()?)),
            _ => Err(ParseError::NotIdent(
                self.p.to_owned().peek().unwrap().to_owned().to_owned(),
            )),
        }
    }

    pub fn consume_touple(&mut self) -> Result<Vec<String>, ParseError> {
        self.expect_token(
            TokenKind::LParen,
            ParseError::NotOpenedParen(self.p.to_owned().peek().unwrap().to_owned().to_owned()),
        )?;

        self.copy_iter();
        let p = self.peek_shadow();
        let mut sv: Vec<String> = vec![];

        sv.push(self.expect_ident(ParseError::NotIdent(p.to_owned()))?);

        if self.peek_value() != TokenKind::RParen {
            while self.peek_value() == TokenKind::Comma {
                self.next();
                sv.push(self.expect_ident(ParseError::NotIdent(p.to_owned()))?);
                if self.peek_value() == TokenKind::RParen {
                    break;
                }
            }
        }

        self.expect_token(
            TokenKind::RParen,
            ParseError::NotClosedParen(self.p.to_owned().peek().unwrap().to_owned().to_owned()),
        )?;

        Ok(sv)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParseKind {
    Expression,
    Type,
}

impl<'a> TokenIter<'a> {
    pub fn sort_parse_type(&mut self) -> ParseKind {
        if self.peek_expression_or() {
            return ParseKind::Expression;
        }

        if self.peek_type_or() {
            return ParseKind::Type;
        }

        unimplemented!()
    }

    pub fn expect_type(&mut self) -> Result<NodeSt, ParseError> {
        match self.peek_value() {
            TokenKind::Int | TokenKind::Nill => {
                let tok = self.peek_shadow();
                match self.next().value {
                    TokenKind::Int => {
                        let op = Node::int(tok.loc);
                        let lhs = NodeSt::new_node(op);
                        return Ok(lhs);
                    }
                    TokenKind::Nill => {
                        let op = Node::nill(tok.loc);
                        let lhs = NodeSt::new_node(op);
                        return Ok(lhs);
                    }
                    _ => unreachable!(),
                }
            }
            _ => Err(ParseError::NotType(self.peek_shadow())),
        }
    }

    pub fn consume_type(&mut self) -> Result<NodeSt, ParseError> {
        let need_close_paren = match self.peek_value() {
            TokenKind::LParen => {
                self.next_with_shadow();
                true
            }
            _ => false,
        };

        let nd = self.expect_type()?;

        if need_close_paren {
            self.expect_token(
                TokenKind::RParen,
                ParseError::NotClosedStmt(self.p.to_owned().peek().unwrap().to_owned().to_owned()),
            )?;
        }

        return Ok(nd);
    }

    pub fn consume_token(&mut self, tk: TokenKind) -> bool {
        if self.peek_value() == tk {
            self.next();
            return true;
        }
        false
    }

    pub fn look_ahead_of_rparen(&mut self) -> Result<bool, ParseError> {
        self.copy_iter();
        let mut pi = 0;
        let mut bi = 0;

        while !(self.peek_value() == TokenKind::RParen && pi == 1 && bi == 0) {
            if self.peek_value() == TokenKind::LParen {
                pi += 1;
            } else if self.peek_value() == TokenKind::RParen {
                pi -= 1;
            } else if self.peek_value() == TokenKind::LBrace {
                bi += 1;
            } else if self.peek_value() == TokenKind::RBrace {
                if pi != 0 {
                    return Err(ParseError::NotClosedParen(self.peek_token()));
                }
                bi -= 1;
            }
            self.next();
        }

        self.back_to_shadow();
        self.next();

        Ok(self.peek_expression_or())
    }
}
