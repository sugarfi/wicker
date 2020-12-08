use crate::commands::validate::{self, Validator};
use crate::error::{error, warn, hint, success};
use crate::eval;
use crate::types::Value;
use crate::session;
use std::collections::{HashMap, HashSet};
use dyn_clone;

pub fn read(
    c: &String,
    args: &Vec<Value>,
    flags: &HashSet<String>,
    vals: &HashMap<String, String>,
    ctx: &mut eval::Context,
) -> Option<(usize, Value)> {
    let session = dyn_clone::clone_box(&*ctx.all_sessions[&ctx.session]);

    if !validate::validate_flags(
        &[
            Validator::CanHave("-help".to_string())
        ], 
        flags
    ) {
        return Some((1, Value::Nil));
    }

    if flags.contains(&"-help".to_string()) {
        println!(
            r#"
Usage: read [-help] <file>+
Flags:
    -help
    Optional. Prints this help message.
Argument:
    <file>+
    The file(s) to read from. Exact behavior depends on the session:
    - In a local session, if the path is relative, it will be joined
      with the current directory and read. If it is absolute, it will
      be read.
    - In a web session, the path will be joined with the current
      directory and read.
            "#
        );
        return Some((0, Value::Nil));
    }

    if !validate::validate_vals(
        &[
        ], 
        vals
    ) {
        return Some((1, Value::Nil));
    }

    match args.len() {
        0 => {
            error("`read` must have at least 1 argument".to_string());
            return Some((1, Value::Nil));
        }

        _ => {
            let mut out = String::new();
            for file in args.iter() {
                match file {
                    Value::Str(x) => {
                        match session.read(x.to_string(), ctx) {
                            Ok(val) => {
                                out += &val;
                            }
                            Err(e) => {
                                match e {
                                    session::ReadError::NoPermission => {
                                        error(format!("cannot read {}: permission denied", x));
                                        return Some((1, Value::Nil));
                                    }

                                    session::ReadError::DoesNotExist => {
                                        error(format!("cannot read {}: file does not exist", x));
                                        return Some((1, Value::Nil));
                                    }

                                    session::ReadError::IOError => {
                                        error(format!("cannot read {}: I/O error", x));
                                        return Some((1, Value::Nil));
                                    }

                                    session::ReadError::URLError => {
                                        error(format!("cannot read {}: URL error", x));
                                        return Some((1, Value::Nil));
                                    }
                                } 
                            }
                        }
                    }
                    _ => {
                        error("argument to `read` must be a string".to_string());
                        return Some((1, Value::Nil));
                    }
                }
            }

            return Some((0, Value::Str(out)));
        }
    }
}
