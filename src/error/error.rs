use super::super::location::location::*;
use super::super::map::error::*;
use super::super::parser::error::*;
use super::super::token::error::*;
use std::error::Error as StdError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Token(TokenError),
    Parse(ParseError),
    Map(MapError),
}

impl From<TokenError> for Error {
    fn from(e: TokenError) -> Self {
        Error::Token(e)
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Error::Parse(e)
    }
}

impl StdError for TokenError {}

impl StdError for ParseError {}

impl StdError for MapError {}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Token(tok) => Some(tok),
            Self::Parse(parse) => Some(parse),
            Self::Map(map) => Some(map),
        }
    }
}

pub fn print_annot(input: &str, loc: Loc) {
    eprintln!("{}", input);
    eprintln!(
        "{}{}",
        " ".repeat(loc.f as usize),
        "^".repeat((loc.e - loc.e) as usize)
    );
}

impl Error {
    pub fn show_diagnostic(&self, input: &str) {
        use self::Error::*;

        let (e, loc): (&dyn StdError, Loc) = match self {
            Token(e) => (e, e.loc.clone()),
            Parse(e) => {
                let loc = match e {
                    ParseError::NotNumber(loc, ..)
                    | ParseError::NotOperator(loc, ..)
                    | ParseError::NotImplementedOperator(loc, ..)
                    | ParseError::NotOpenedParen(loc, ..)
                    | ParseError::NotClosedParen(loc, ..)
                    | ParseError::NotOpenedStmt(loc, ..)
                    | ParseError::NotClosedStmt(loc, ..)
                    | ParseError::OperatorAfterRetrun(loc, ..)
                    | ParseError::NotIdent(loc, ..)
                    | ParseError::NotAssign(loc, ..)
                    | ParseError::NotDefinitionVar(loc, ..)
                    | ParseError::NotLBrace(loc, ..)
                    | ParseError::NotRBrace(loc, ..)
                    | ParseError::OperatorOutOfFnction(loc, ..) => loc.loc.clone(),
                    ParseError::UndefinedVariable(loc)
                    | ParseError::UnusedVariable(loc)
                    | ParseError::NotMatchReturnType(loc)
                    | ParseError::UnexpectedUnderScoreOperator(loc) => loc.clone(),
                    ParseError::Eof => Loc::new(input.len() as u8, input.len() as u8 + 1),
                };
                (e, loc)
            }
            Map(e) => (e, e.loc.clone()),
        };

        eprintln!("{}", e);
        print_annot(input, loc)
    }
}

pub fn show_trace<E: StdError>(e: E) {
    eprintln!("{}", e);
    let mut source = e.source();

    while let Some(e) = source {
        eprintln!("caused by {}", e);
        source = e.source()
    }
}
