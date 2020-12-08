use crate::commands::validate::{self, Validator};
use crate::error::{error, hint, success, warn};
use crate::eval;
use crate::session;
use crate::session::SessionType;
use crate::types::Value;
use dyn_clone;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

pub fn cd(
    c: &String,
    args: &Vec<Value>,
    flags: &HashSet<String>,
    vals: &HashMap<String, String>,
    ctx: &mut eval::Context,
) -> Option<(usize, Value)> {
    let mut session = ctx.all_sessions.get_mut(&ctx.session).unwrap();

    if !validate::validate_flags(&[Validator::CanHave("-help".to_string())], flags) {
        return Some((1, Value::Nil));
    }

    if flags.contains(&"-help".to_string()) {
        println!(
            r#"
Usage: cd [-help] <path>
Flags:
    -help
    Optional. Prints this help message.
Argument:
    <path>
    The path to change directory to. Must be a directory.
            "#
        );
        return Some((0, Value::Nil));
    }

    if !validate::validate_vals(&[], vals) {
        return Some((1, Value::Nil));
    }

    match args.len() {
        1 => {}
        _ => {
            error("`cd` requires at least 1 argument".to_string());
            return Some((1, Value::Nil));
        }
    }

    let path = PathBuf::from(match &args[0] {
        Value::Str(x) => x,
        _ => {
            error("argument 0 of `cd` must be a string".to_string());
            return Some((1, Value::Nil));
        }
    });

    match session.get_type() {
        SessionType::Local => {
            let res = match fs::canonicalize(if path.is_relative() {
                Path::new(&session.get_cwd()).join(path.clone())
            } else {
                path
            }) {
                Ok(x) => x,
                Err(_) => {
                    error("argument 0 of `cd` must be an existing directory".to_string());
                    return Some((1, Value::Nil));
                }
            };

            if !res.exists() || !res.is_dir() {
                error("argument 0 of `cd` must be an existing directory".to_string());
                return Some((1, Value::Nil));
            }

            session.set_cwd(res.to_str().unwrap().to_string());
        }

        SessionType::Web => {
            let res = if path.is_relative() {
                Path::new(&session.get_cwd()).join(path.clone())
            } else {
                path
            };

            session.set_cwd(res.to_str().unwrap().to_string());
        }
    };

    success(format!("changed directory to {}", session.get_cwd()));

    Some((0, Value::Nil))
}
