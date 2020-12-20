use crate::commands::validate::{self, Validator};
use crate::error::{error, hint, success, warn};
use crate::eval;
use crate::types::{Table, Value};
use crate::session::SessionType;
use std::collections::{HashMap, HashSet};

pub fn session_list(
    c: &String,
    args: &Vec<Value>,
    flags: &HashSet<String>,
    vals: &HashMap<String, String>,
    ctx: &mut eval::Context,
) -> Option<(usize, Value)> {
    if !validate::validate_flags(&[Validator::CanHave("-help".to_string())], flags) {
        return Some((1, Value::Nil));
    }

    if flags.contains(&"-help".to_string()) {
        println!(
            r#"
Usage: session-list [-help]
Flags:
    -help
    Optional. Prints this help message.
            "#
        );
        return Some((0, Value::Nil));
    }

    if !validate::validate_vals(&[], vals) {
        return Some((1, Value::Nil));
    }

    match args.len() {
        0 => {
            let mut table = Table {
                headers: vec![Value::Str("session".to_string()), Value::Str("type".to_string())],
                rows: vec![]
            };
            for (id, session) in &ctx.all_sessions {
                table.rows.push(vec![
                                    Value::Int(*id), 
                                    Value::Str(match session.get_type() {
                                        SessionType::Local => "local",
                                        SessionType::Web => "web"
                                    }.to_string())
                ]);
            }
            return Some((0, Value::Table(table)));
        }
        _ => {
            error("`session-list` must have exactly 0 arguments".to_string());
            return Some((1, Value::Nil));
        }
    }

    Some((0, Value::Nil))
}

