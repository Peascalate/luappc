#[derive(Debug)]
pub enum Token {
    Ident(Vec<char>),
    Directive(Vec<char>),
    Number(f64),
    Assign,
    Local,
    Plus,
    Minus,
    Whitespace,
    Comment,
}

