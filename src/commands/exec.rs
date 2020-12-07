use std::collections::{HashMap, HashSet};
use std::process::Command;

use crate::eval;
use crate::types::Value;

/// Execute a binary command with the Command module
pub fn exec(
    c: &String,
    args: &Vec<Value>,
    flags: &HashSet<String>,
    vals: &HashMap<String, String>,
    ctx: &mut eval::Context,
) -> Result<Option<(usize, Value)>, String> {
    let cmd = Command::new(c)
        .args(args.iter().map(|v| match v {
            // hahahaha
            Value::Str(s) => (*s).clone().to_string(),
            Value::Int(i) => format!("{}", i),
            // TODO: actually do this
            Value::Table(_) => format!("!!Table!!"),
            Value::Nil => "Nil".to_string(),
        }))
        .status();

    match cmd {
        Ok(x) => Ok(Some((match x.code() {
            Some(i) => i as usize,
            None => 0usize,
        }, Value::Nil))),
        Err(e) => Err(format!("Failed to execute \"{}\"", c)),
    }
}
