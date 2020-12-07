use crate::commands::validate::{self, Validator};
use crate::error::{error, warn, hint, success};
use crate::eval;
use crate::types::Value;
use std::collections::{HashMap, HashSet};

pub fn session_switch(
    c: &String,
    args: &Vec<Value>,
    flags: &HashSet<String>,
    vals: &HashMap<String, String>,
    ctx: &mut eval::Context,
) -> Option<(usize, Value)> {
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
Usage: session-switch [-help] <id>
Flags:
    -help
    Optional. Prints this help message.
Arguments:
    <id>
    The ID of the session to switch to.
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
        1 => { },
        _ => {
            error("`session-switch` must have exactly 1 argument".to_string());
            return Some((1, Value::Nil));
        }
    }
    let id = match &args[0] {
        Value::Int(x) => x,
        _ => {
            error("`session-switch` argument 0 must be an int".to_string());
            return Some((1, Value::Nil));
        }
    };

    if !ctx.all_sessions.contains_key(id) {
        error(format!("no such session {}", id));
        return Some((1, Value::Nil));
    }
    ctx.session = *id;
    success(format!("switched to session {}", id));

    Some((0, Value::Nil))
}
