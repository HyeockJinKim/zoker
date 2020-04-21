use crate::error::{LexicalError, LexicalErrorType};
use crate::location::Location;
pub use crate::token::Tok;

pub type Spanned = (Location, Tok, Location);
pub type LexResult = Result<Spanned, LexicalError>;

use std::collections::HashMap;
use std::str::FromStr;
use unic_ucd_ident::{is_xid_continue, is_xid_start};

pub struct Lexer<T: Iterator<Item = char>> {
    chars: T,
    location: Location,
    chr: Option<char>,
    keywords: HashMap<String, Tok>,
}

pub fn make_tokenizer<'a>(source: &'a str) -> impl Iterator<Item = LexResult> + 'a {
    Lexer::new(source.chars())
}

fn get_keywords() -> HashMap<String, Tok> {
    let mut keywords = HashMap::new();

    keywords.insert(String::from("+"), Tok::Plus);
    keywords.insert(String::from("-"), Tok::Minus);
    keywords.insert(String::from("*"), Tok::Mul);
    keywords.insert(String::from("/"), Tok::Div);
    keywords.insert(String::from("%"), Tok::Mod);
    keywords.insert(String::from("**"), Tok::Pow);
    keywords.insert(String::from("!"), Tok::Not);
    keywords.insert(String::from("++"), Tok::PlusPlus);
    keywords.insert(String::from("--"), Tok::MinusMinus);
    keywords.insert(String::from("="), Tok::Assign);
    keywords.insert(String::from("&="), Tok::BitAndAssign);
    keywords.insert(String::from("^="), Tok::BitXorAssign);
    keywords.insert(String::from("|="), Tok::BitOrAssign);
    keywords.insert(String::from("<<="), Tok::LShiftAssign);
    keywords.insert(String::from(">>="), Tok::RShiftAssign);
    keywords.insert(String::from("+="), Tok::AddAssign);
    keywords.insert(String::from("-="), Tok::SubAssign);
    keywords.insert(String::from("*="), Tok::MulAssign);
    keywords.insert(String::from("/="), Tok::DivAssign);
    keywords.insert(String::from("%="), Tok::ModAssign);
    keywords.insert(String::from("<"), Tok::Lt);
    keywords.insert(String::from("<="), Tok::Le);
    keywords.insert(String::from(">"), Tok::Gt);
    keywords.insert(String::from(">="), Tok::Ge);
    keywords.insert(String::from("=="), Tok::Eq);
    keywords.insert(String::from("!="), Tok::NotEq);
    keywords.insert(String::from("&&"), Tok::And);
    keywords.insert(String::from("||"), Tok::Or);
    keywords.insert(String::from("&"), Tok::BitAnd);
    keywords.insert(String::from("|"), Tok::BitOr);
    keywords.insert(String::from("^"), Tok::BitXor);
    keywords.insert(String::from("uint256"), Tok::Uint256);
    keywords.insert(String::from("uint"), Tok::Uint256);
    keywords.insert(String::from("int256"), Tok::Int256);
    keywords.insert(String::from("int"), Tok::Int256);
    keywords.insert(String::from("bytes32"), Tok::Bytes32);
    keywords.insert(String::from("bool"), Tok::Bool);
    keywords.insert(String::from("bytes"), Tok::Bytes);
    keywords.insert(String::from("String"), Tok::String);
    keywords.insert(String::from("address"), Tok::Address);
    keywords.insert(String::from("function"), Tok::Function);
    keywords.insert(String::from("if"), Tok::If);
    keywords.insert(String::from("else"), Tok::Else);
    keywords.insert(String::from("for"), Tok::For);
    keywords.insert(String::from("in"), Tok::In);
    keywords.insert(String::from("("), Tok::LPar);
    keywords.insert(String::from(")"), Tok::RPar);
    keywords.insert(String::from("{"), Tok::LBrace);
    keywords.insert(String::from("}"), Tok::RBrace);
    keywords.insert(String::from(";"), Tok::Semi);

    keywords
}

impl<T> Lexer<T>
where
    T: Iterator<Item = char>,
{
    pub fn new(input: T) -> Self {
        Lexer {
            chars: input,
            location: Location::new(0, 0),
            chr: None,
            keywords: get_keywords(),
        }
    }

    fn next_token(&mut self) -> LexResult {
        let start = self.location;
        self.next_char();
        let token = if let Some(c) = self.chr {
            if self.is_identifier_start(c) {
                self.consume_identifier(c)?
            } else {
                self.consume_special_character(c)?
            }
        } else {
            // EOF
            Tok::EOF
        };
        let end = self.location;
        Ok((start, token, end))
    }

    fn next_char(&mut self) {
        let next = self.chars.next();
        self.chr = next;
        if let Some(c) = self.chr {
            if c == '\n' {
                self.location.new_line();
            } else {
                self.location.go_right();
            }
        }
    }

    fn is_identifier_start(&self, c: char) -> bool {
        c == '_' || is_xid_start(c)
    }

    fn is_identifier_continue(&self, c: char) -> bool {
        match c {
            '_' | '0'..='9' => true,
            c => is_xid_continue(c),
        }
    }

    fn consume_identifier(&mut self, c: char) -> Result<Tok, LexicalError> {
        let mut text = String::new();
        text.push(c);

        loop {
            self.next_char();
            if let Some(c) = self.chr {
                if self.is_identifier_continue(c) {
                    text.push(c);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        if self.keywords.contains_key(&text) {
            Ok(self.keywords[&text].clone())
        } else {
            Ok(Tok::Identifier { name: text })
        }
    }

    fn consume_special_character(&mut self, c: char) -> Result<Tok, LexicalError> {
        match c {
            '0'..='9' => self.lex_number(c),
            '"' | '\'' => self.lex_literal(c),
            _ => Err(LexicalError {
                error: LexicalErrorType::UnrecognizedToken,
            }),
        }
    }

    fn lex_number(&mut self, c: char) -> Result<Tok, LexicalError> {
        let mut text = String::new();
        text.push(c);
        loop {
            self.next_char();
            // TODO: Should be added _ decorator.
            if let Some(c) = self.chr {
                match c {
                    '0'..='9' => text.push(c),
                    _ => break,
                }
            } else {
                break;
            }
        }

        Ok(Tok::Number {
            number: u64::from_str(&text)?,
        })
    }

    fn lex_literal(&mut self, c: char) -> Result<Tok, LexicalError> {
        let mut text = String::new();
        let first = c;
        loop {
            self.next_char();
            // TODO: Should be added _ decorator.
            if let Some(c) = self.chr {
                if first == c {
                    break;
                }
                text.push(c);
            } else {
                break;
            }
        }
        Ok(Tok::Literal { literal: text })
    }
}

impl<T> Iterator for Lexer<T>
where
    T: Iterator<Item = char>,
{
    type Item = LexResult;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();

        match token {
            Ok((_, Tok::EOF, _)) => None,
            r => Some(r),
        }
    }
}
