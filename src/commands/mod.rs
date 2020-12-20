#![allow(unused_imports)]
use crate::error::error;
use crate::eval;
use crate::types::Value;
use std::collections::{HashMap, HashSet};
mod cd;
mod exec;
mod read;
mod session_create;
mod session_switch;
mod session_list;
mod validate;
mod write;
mod delete;

pub fn call(
    c: &String,
    all: &Vec<Value>,
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
        "session-list" => {
            return session_list::session_list(c, args, flags, vals, ctx);
        }
        "write" => {
            return write::write(c, args, flags, vals, ctx);
        }
        "read" => {
            return read::read(c, args, flags, vals, ctx);
        }
        "cd" => {
            return cd::cd(c, args, flags, vals, ctx);
        }
        "delete" => {
            return delete::delete(c, args, flags, vals, ctx);
        }
        "exit" => {
            std::process::exit(0);
        }
        _ => exec::exec(c, all, ctx),
    }
}
