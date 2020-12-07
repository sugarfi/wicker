use std::collections::{HashMap, HashSet};
use std::process::Command;

use crate::eval;
use crate::types::Value;
use crate::session::ExecError;
use crate::error::error;

use dyn_clone;

/// Execute a binary command with the Command module
pub fn exec(
    c: &String,
    args: &Vec<Value>,
    ctx: &mut eval::Context,
) -> Option<(usize, Value)> {
    let args: Vec<String> = args.iter().map(|v| match v {
            // hahahaha
            Value::Str(s) => (*s).clone().to_string(),
            Value::Int(i) => format!("{}", i),
            // TODO: actually do this
            Value::Table(_) => format!("!!Table!!"),
            Value::Nil => "Nil".to_string()}).collect();

    let session = dyn_clone::clone_box(&*ctx.all_sessions[&ctx.session]);

    match session.exec(c.to_string(), args, ctx) {
        Ok(o) => Some((0, Value::Nil)),
        Err(e) => {
            match e {
                ExecError::ExecFailure => {
                    error(format!("error executing {}: execution failed", c));
                    Some((1, Value::Nil))
                }
            }
        }
    }
}
