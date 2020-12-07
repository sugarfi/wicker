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

fn main() {
    let mut rl = Editor::<()>::new();
    let mut ctx = Context::new();

    loop {
        match rl.readline(&prompt::prompt(&ctx)) {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let ast = parser::parse(line.clone());
                match ast {
                    Err(parser::ParserError::BadToken(pos)) => {
                        error("bad token".to_string());
                        println!("  |\n1 |{}", line);
                        println!("  |\x1b[33m{}↑ here\x1b[0m", " ".repeat(pos - 1));
                        continue;
                    }

                    Err(parser::ParserError::ExpectedWhitespace(pos)) => {
                        error("expected whitespace".to_string());
                        println!("  |\n1 |{}", line);
                        println!("  |\x1b[33m{}↑ here\x1b[0m", " ".repeat(pos - 1));
                        continue;
                    }

                    Err(parser::ParserError::ExpectedName(pos)) => {
                        error("expected name".to_string());
                        println!("  |\n1 |{}", line);
                        println!("  |\x1b[33m{}↑ here\x1b[0m", " ".repeat(pos - 1));
                        continue;
                    }

                    Err(parser::ParserError::ExpectedFlag(pos)) => {
                        error("expected flag".to_string());
                        println!("  |\n1 |{}", line);
                        println!("  |\x1b[33m{}↑ here\x1b[0m", " ".repeat(pos - 1));
                        continue;
                    }

                    Err(parser::ParserError::ExpectedInt(pos)) => {
                        error("expected integer".to_string());
                        println!("  |\n1 |{}", line);
                        println!("  |\x1b[33m{}↑ here\x1b[0m", " ".repeat(pos - 1));
                        continue;
                    }

                    Err(parser::ParserError::ExpectedStr(pos)) => {
                        error("expected string".to_string());
                        println!("  |\n1 |{}", line);
                        println!("  |\x1b[33m{}↑ here\x1b[0m", " ".repeat(pos - 1));
                        continue;
                    }

                    Err(parser::ParserError::EOF(pos)) => {
                        error("unexpected EOF".to_string());
                        println!("  |\n1 |{}", line);
                        println!("  |\x1b[33m{}↑ here\x1b[0m", " ".repeat(pos - 1));
                        continue;
                    }

                    _ => {}
                }
                let ast = ast.unwrap();
                let (_, res) = ctx.eval(&ast);
                match res {
                    Value::Str(x) => {
                        println!("{}", x);
                    }

                    Value::Int(x) => {
                        println!("{}", x);
                    }

                    Value::Nil => { }

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
                println!("\x1b[0;31m❌ Error processing command: ❌\x1b[0m\n{}", err);
            }
        }
    }
}
