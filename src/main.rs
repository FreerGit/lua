pub mod ast;
pub mod instruction;
pub mod lex;
pub mod parse;
// pub mod vm;

use std::{env, fmt, fs::File, io::Read};

use crate::{
    lex::{Lex, Token},
    parse::Parser,
};

// use crate::vm::VM;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} scripts", args[0]);
        return;
    }

    let mut file = File::open(&args[1]).unwrap();
    let mut source = String::new();
    file.read_to_string(&mut source).unwrap();

    // Create a lexer
    let lex = Lex::new(&source);
    let mut parser = Parser::new(lex);
    // Lex and print all tokens
    // loop {
    //     let token = lex.next();
    //     if token == Token::Eof {
    //         break;
    //     }
    // }

    dbg!(parser.parse().unwrap());
    // dbg!(parser.constants);
    // dbg!(parser.code);
}
