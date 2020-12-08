use crate::commands::validate::{self, Validator};
use crate::error::{error, hint, success, warn};
use crate::eval;
use crate::session::{self, WriteError};
use crate::types::Value;
use dyn_clone;
use std::collections::{HashMap, HashSet};

pub fn write(
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
Usage: write [-help] [-to=<file>] <data>
Flags:
    -help
    Optional. Prints this help message.
    
    -to=<id>
    Optional. If set, data will be written to this file, instead of
    stdout.
Argument:
    <data>
    The data to write. By default it is written to stdout - if -to
    is set, it will be written to that file instead. Exact behavior
    of writing to a file depends on the session:
    - In a local session, if the path is relative, it will be joined
      with the current directory and written. If it is absolute, it
      will be written without joining.
    - In a web session, the path will be joined with the current
      directory. Then a GET request is made to check if the file
      exists. If it does, a PATCH request is sent to update it -
      otherwise, a PUT request is sent to create it. If either
      the PUT or PATCH request comes back with a 405 error, the write
      fails with an I/O error.
            "#
        );
        return Some((0, Value::Nil));
    }

    if !validate::validate_vals(&[Validator::CanHave("-to".to_string())], vals) {
        return Some((1, Value::Nil));
    }

    let out = String::new();
    match args.len() {
        1 => {
            let data = match &args[0] {
                Value::Str(x) => x,
                _ => {
                    error("argument 0 of `write` must be a string.".to_string());
                    return Some((1, Value::Nil));
                }
            };

            if vals.contains_key("-to") {
                match session.write(vals["-to"].clone(), data.clone(), ctx) {
                    Ok(_) => {}
                    Err(WriteError::NoPermission) => {
                        error(format!("cannot write {}: permission denied", vals["-to"]));
                        return Some((1, Value::Nil));
                    }
                    Err(WriteError::IOError) => {
                        error(format!("cannot write {}: I/O error", vals["-to"]));
                        return Some((1, Value::Nil));
                    }
                };
            } else {
                if data.ends_with("\n") {
                    print!("{}", data);
                } else {
                    println!("{}", data);
                }
            }
            success(format!(
                "successfully wrote to {}",
                if vals.contains_key(&"-to".to_string()) {
                    vals["-to"].clone()
                } else {
                    "stdout".to_string()
                }
            ));
        }

        _ => {
            error("`write` requires exactly 1 argument.".to_string());
            return Some((1, Value::Nil));
        }
    }

    return Some((0, Value::Nil));
}
