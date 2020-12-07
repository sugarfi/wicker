#![allow(unused_imports)]
use crate::error::error;
use crate::eval;
use crate::types::Value;
use std::collections::{HashMap, HashSet};
mod session_create;
mod session_switch;
mod read;
mod validate;
mod exec;

pub fn call(
    c: &String,
    args: &Vec<Value>,
    flags: &HashSet<String>,
    vals: &HashMap<String, String>,
    ctx: &mut eval::Context,
) -> Option<(usize, Value)> {
    match &c[..] {
        "session-create" => {
            return session_create::session_create(c, args, flags, vals, ctx);
        }
        "session-switch" => {
            return session_switch::session_switch(c, args, flags, vals, ctx);
        }
        "read" => {
            return read::read(c, args, flags, vals, ctx);
        }
        "exit" => {
            std::process::exit(0);
        }
        _ => {
            match exec::exec(c, args, flags, vals, ctx) {
                Ok(x) => x,
                Err(e) => {
                    // error(format!("Command `{}` does not exist", c));
                    error(e);
                    Some((1, Value::Nil))
                }
            }
        }
    }
}
