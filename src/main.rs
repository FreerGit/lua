// pub mod instruction;
pub mod lex;
// pub mod parse;
// pub mod vm;

use std::{env, fmt, fs::File, io::Read};

use crate::lex::{Lex, Token};

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
    let mut lex = Lex::new(&source);

    // Lex and print all tokens
    loop {
        let token = lex.next();
        println!("{:?}", &token);
        if token == Token::Eof {
            break;
        }
        // match &token {
        //     Token::Eof => break,
        //     Token::Name(s) => println!("Name: {}", s),
        //     Token::String(s) => println!("String: {}", s),
        //     Token::Number(n) => println!("Number: {}", n),
        // }
    }
}
