#![feature(proc_macro_hygiene)]
mod lexer;



fn main() {
    let inp = "local slayy = 69.420";
    let lex = lexer::lexer::Lexer::new(inp);
    lex.for_each(|tok| println!("tok: {:?}", tok));
}
