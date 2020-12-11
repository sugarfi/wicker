use crate::commands::validate::{self, Validator};
use crate::error::{error, hint, success, warn};
use crate::eval;
use crate::session;
use crate::types::Value;
use dyn_clone;
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

pub fn delete(
    c: &String,
    args: &Vec<Value>,
    flags: &HashSet<String>,
    vals: &HashMap<String, String>,
    ctx: &mut eval::Context,
) -> Option<(usize, Value)> {
    let session = dyn_clone::clone_box(&*ctx.all_sessions[&ctx.session]);

    if !validate::validate_flags(&[Validator::CanHave("-help".to_string())], flags) {
        return Some((1, Value::Nil));
    }

    if flags.contains(&"-help".to_string()) {
        println!(
            r#"
Usage: delete [-help] <file>*
Flags:
    -help
    Optional. Prints this help message.
Argument:
    <file>*
    The file(s) to delete. Exact behavior depends on the session:
    - In a local session, if the path is relative, it will be joined
      with the current directory and deleted. If it is absolute, it will
      be deleted.
    - In a web session, the path will be joined with the current
      directory and a DELETE request will be sent to the server with
      that path.
            "#
        );
        return Some((0, Value::Nil));
    }

    if !validate::validate_vals(&[], vals) {
        return Some((1, Value::Nil));
    }

    match args.len() {
        0 => {
            error("`delete` requires at least 1 argument".to_string());
            return Some((1, Value::Nil));
        }

        _ => {
            for file in args.iter() {
                match file {
                    Value::Str(x) => match session.delete(x.to_string(), ctx) {
                        Ok(val) => {
                            success(format!("successfully deleted {}", x));
                        }
                        Err(e) => match e {
                            session::DeleteError::NoPermission => {
                                error(format!("cannot delete {}: permission denied", x));
                                return Some((1, Value::Nil));
                            }

                            session::DeleteError::DoesNotExist => {
                                error(format!("cannot delete {}: file does not exist", x));
                                return Some((1, Value::Nil));
                            }

                            session::DeleteError::IOError => {
                                error(format!("cannot delete {}: I/O error", x));
                                return Some((1, Value::Nil));
                            }
                        },
                    },
                    _ => {
                        error("argument to `delete` must be a string".to_string());
                        return Some((1, Value::Nil));
                    }
                }
            }
        }
    }

    return Some((0, Value::Nil));
}

