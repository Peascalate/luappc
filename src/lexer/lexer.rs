
extern crate plex;
use plex::lexer;
use crate::lexer::token::Token;

lexer! {
    fn next_token(text: 'a) -> Token;
    r#"[ \t\r\n]+"# => Token::Whitespace,
    r#"--[^\n]*"# => Token::Comment,

    r#"[0-9\.]+"# => {
        if let Ok(i) = text.parse() {
            Token::Number(i)
        } else {
            panic!("Number {} is out of range [IEEE754 64-bit]", text)
        }
    },
    r#"\#dir\[[a-zA-Z_][a-zA-Z0-9_]*\]"# => Token::Directive(text.chars().collect()),
    r#"="# => Token::Assign,
    r#"local"# => Token::Local,
    r#"[a-zA-Z_][a-zA-Z0-9_]*"# => Token::Ident(text.chars().collect()),
    r#"."# => panic!("unexpected character: {}", text),
}

pub struct Lexer<'a> {
    original: &'a str,
    remaining: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Lexer<'a> {
        Lexer {
            original: s,
            remaining: s,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (Token, Span);
    fn next(&mut self) -> Option<(Token, Span)> {
        loop {
            let (tok, span) = if let Some((tok, new_remaining)) = next_token(self.remaining) {
                let lo = self.original.len() - self.remaining.len();
                let hi = self.original.len() - new_remaining.len();
                self.remaining = new_remaining;
                (tok, Span { lo, hi })
            } else {
                return None;
            };
            match tok {
                Token::Whitespace | Token::Comment => {
                    continue;
                }
                tok => {
                    return Some((tok, span));
                }
            }
        }
    }
}
