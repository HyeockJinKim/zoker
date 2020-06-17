use crate::error::{LexicalError, LexicalErrorType};
use crate::location::Location;
pub use crate::token::Tok;

pub type Spanned = (Location, Tok, Location);
pub type LexResult = Result<Spanned, LexicalError>;

use num_bigint::BigUint;
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
    keywords.insert(String::from("contract"), Tok::Contract);
    keywords.insert(String::from("memory"), Tok::Contract);
    keywords.insert(String::from("storage"), Tok::Contract);
    keywords.insert(String::from("if"), Tok::If);
    keywords.insert(String::from("else"), Tok::Else);
    keywords.insert(String::from("for"), Tok::For);
    keywords.insert(String::from("in"), Tok::In);
    keywords.insert(String::from("returns"), Tok::Returns);
    keywords.insert(String::from("return"), Tok::Return);

    keywords
}

impl<T> Lexer<T>
where
    T: Iterator<Item = char>,
{
    fn new(input: T) -> Self {
        Lexer {
            chars: input,
            location: Location::new(0, 0),
            chr: None,
            keywords: get_keywords(),
        }
    }

    fn next_token(&mut self) -> LexResult {
        if self.chr.is_none() {
            self.next_char();
        }
        if let Some(c) = self.chr {
            let start = self.location;
            let token = if self.is_identifier_start(c) {
                self.consume_identifier(c)?
            } else {
                self.consume_special_character(c)?
            };
            let end = self.location;
            self.skip_blank();
            Ok((start, token, end))
        } else {
            // End Of File
            Ok((self.location, Tok::EOF, self.location))
        }
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

    fn is_blank(&self, c: char) -> bool {
        c == ' ' || c == '\n' || c == '\t'
    }

    fn is_identifier_continue(&self, c: char) -> bool {
        match c {
            '_' | '0'..='9' => true,
            c => is_xid_continue(c),
        }
    }

    fn skip_blank(&mut self) {
        loop {
            if let Some(c) = self.chr {
                if self.is_blank(c) {
                    self.next_char();
                    continue;
                }
            }
            break;
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
            _ => self.consume_multiple_special_character(),
        }
    }

    fn consume_multiple_special_character(&mut self) -> Result<Tok, LexicalError> {
        let mut text = String::new();
        let mut token = None;
        while let Some(c) = self.chr {
            text.push(c);
            match text.as_str() {
                "<" => token = Some(Tok::Lt),
                ">" => token = Some(Tok::Gt),
                "=" => token = Some(Tok::Assign),
                "+" => token = Some(Tok::Plus),
                "-" => token = Some(Tok::Minus),
                "*" => token = Some(Tok::Mul),
                "/" => token = Some(Tok::Div),
                "%" => token = Some(Tok::Mod),
                "!" => token = Some(Tok::Not),
                "&" => token = Some(Tok::BitAnd),
                "|" => token = Some(Tok::BitOr),
                "^" => token = Some(Tok::BitXor),
                "?" => token = Some(Tok::Question),
                ":" => token = Some(Tok::Colon),
                "<<" => token = Some(Tok::LShift),
                ">>" => token = Some(Tok::RShift),
                "," => {
                    token = Some(Tok::Comma);
                    break;
                }
                "{" => {
                    token = Some(Tok::LBrace);
                    break;
                }
                "}" => {
                    token = Some(Tok::RBrace);
                    break;
                }
                "(" => {
                    token = Some(Tok::LPar);
                    break;
                }
                ")" => {
                    token = Some(Tok::RPar);
                    break;
                }
                ";" => {
                    token = Some(Tok::Semi);
                    break;
                }
                "**" => {
                    token = Some(Tok::Pow);
                    break;
                }
                "++" => {
                    token = Some(Tok::PlusPlus);
                    break;
                }
                "--" => {
                    token = Some(Tok::MinusMinus);
                    break;
                }
                "&=" => {
                    token = Some(Tok::BitAndAssign);
                    break;
                }
                "|=" => {
                    token = Some(Tok::BitOrAssign);
                    break;
                }
                "^=" => {
                    token = Some(Tok::BitXorAssign);
                    break;
                }
                "+=" => {
                    token = Some(Tok::AddAssign);
                    break;
                }
                "-=" => {
                    token = Some(Tok::SubAssign);
                    break;
                }
                "*=" => {
                    token = Some(Tok::MulAssign);
                    break;
                }
                "/=" => {
                    token = Some(Tok::DivAssign);
                    break;
                }
                "%=" => {
                    token = Some(Tok::ModAssign);
                    break;
                }
                "==" => {
                    token = Some(Tok::Eq);
                    break;
                }
                "!=" => {
                    token = Some(Tok::NotEq);
                    break;
                }
                "<=" => {
                    token = Some(Tok::Le);
                    break;
                }
                ">=" => {
                    token = Some(Tok::Ge);
                    break;
                }
                "&&" => {
                    token = Some(Tok::And);
                    break;
                }
                "||" => {
                    token = Some(Tok::Or);
                    break;
                }
                "<<=" => {
                    token = Some(Tok::LShiftAssign);
                    break;
                }
                ">>=" => {
                    token = Some(Tok::RShiftAssign);
                    break;
                }
                _ => return self.check_token(token),
            }
            self.next_char();
        }
        self.next_char();
        self.check_token(token)
    }

    fn check_token(&self, token: Option<Tok>) -> Result<Tok, LexicalError> {
        if let Some(t) = token {
            Ok(t)
        } else {
            Err(LexicalError {
                error: LexicalErrorType::UnrecognizedToken {
                    tok: self.chr.unwrap(),
                },
                location: self.location,
            })
        }
    }

    fn lex_number(&mut self, c: char) -> Result<Tok, LexicalError> {
        let mut text = String::new();
        text.push(c);
        loop {
            self.next_char();
            if let Some(c) = self.chr {
                match c {
                    '0'..='9' => text.push(c),
                    '_' => {
                        self.next_char();
                        if let Some(c) = self.chr {
                            if let '0'..='9' = c {
                                text.push(c);
                            } else {
                                return Err(LexicalError {
                                    error: LexicalErrorType::UnrecognizedToken { tok: c },
                                    location: self.location,
                                });
                            }
                        } else {
                            return Err(LexicalError {
                                error: LexicalErrorType::UnrecognizedToken { tok: c },
                                location: self.location,
                            });
                        }
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }
        Ok(Tok::Num {
            number: BigUint::from_str(&text)?,
        })
    }

    fn lex_literal(&mut self, c: char) -> Result<Tok, LexicalError> {
        let mut text = String::new();
        let first = c;
        loop {
            self.next_char();
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
