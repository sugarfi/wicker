#![allow(dead_code)]
#![allow(unused_variables)]
use rustyline::error::ReadlineError;
use rustyline::Editor;
mod commands;
mod error;
mod eval;
mod parser;
mod prompt;
mod session;
mod types;
use crate::error::error;
use crate::eval::Context;
use crate::types::Value;

/// Throw a parser error and print it
macro_rules! parser_error {
    ( $msg:expr, $line:expr, $pos:expr ) => {{
        error($msg.to_string());
        println!("  |\n1 |{}", $line);
        println!("  |\x1b[33m{}↑ here\x1b[0m", " ".repeat($pos - 1));
        continue;
    }};
}

fn main() {
    let mut rl = Editor::<()>::new();
    let mut ctx = Context::new();

    loop {
        match rl.readline(&prompt::prompt(&ctx)) {
            Ok(line) => {
                // the user inserted nothing
                if line.len() < 1 {
                    continue;
                }

                rl.add_history_entry(line.as_str());
                let ast = parser::parse(line.clone());
                match ast {
                    Err(parser::ParserError::BadToken(pos)) => {
                        parser_error!("bad token", line, pos)
                    }
                    Err(parser::ParserError::ExpectedWhitespace(pos)) => {
                        parser_error!("expected whitespace", line, pos)
                    }
                    Err(parser::ParserError::ExpectedName(pos)) => {
                        parser_error!("expected name", line, pos)
                    }
                    Err(parser::ParserError::ExpectedFlag(pos)) => {
                        parser_error!("expected flag", line, pos)
                    }
                    Err(parser::ParserError::ExpectedInt(pos)) => {
                        parser_error!("expected integer", line, pos)
                    }
                    Err(parser::ParserError::ExpectedStr(pos)) => {
                        parser_error!("expected string", line, pos)
                    }
                    Err(parser::ParserError::EOF(pos)) => {
                        parser_error!("unexpected EOF", line, pos)
                    }
                    _ => {}
                };
                let ast = ast.unwrap();
                let (_, res) = ctx.eval(&ast);
                match res {
                    Value::Str(x) => {
                        if x.ends_with("\n") {
                            print!("{}", x);
                        } else {
                            println!("{}", x);
                        }
                    }

                    Value::Int(x) => {
                        println!("{}", x);
                    }

                    Value::Nil => {}

                    x => {
                        println!("{:?}", x);
                    }
                }
            }

            Err(ReadlineError::Eof) => {
                println!("\x1b[0;33mRecieved Ctrl-D, exiting...\x1b[0m");
                break;
            }

            Err(ReadlineError::Interrupted) => {
                println!("\x1b[0;33mRecieved Ctrl-C, continuing...\x1b[0m");
                continue;
            }

            Err(err) => {
                error(err.to_string());
            }
        }
    }
}
