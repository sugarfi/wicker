use crate::commands::validate::{self, Validator};
use crate::error::{error, warn, hint, success};
use crate::eval;
use crate::session::{local::LocalSession, web::WebSession};
use crate::types::Value;
use std::collections::{HashMap, HashSet};

pub fn session_create(
    c: &String,
    args: &Vec<Value>,
    flags: &HashSet<String>,
    vals: &HashMap<String, String>,
    ctx: &mut eval::Context,
) -> Option<(usize, Value)> {
    if !validate::validate_flags(
        &[
            Validator::CanHave("-overwrite".to_string()),
            Validator::CanHave("-help".to_string())
        ], 
        flags
    ) {
        return Some((1, Value::Nil));
    }

    if flags.contains(&"-help".to_string()) {
        println!(
            r#"
Usage: session-create [-overwrite] [-help] -type={{local|web}} [-url=<url>] [-id=<id>]
Flags:
    -help
    Optional. Print this help message.

    -overwrite
    Optional. Overwrite an existing session.

    -type={{local|web}}
    Required. Sets the type of the session to create.

    -url=<url>
    Required if -type=web. Sets the base URL to connect to.

    -id=<id>
    Optional. Sets the ID of the session to create; if unset, the next consecutive session ID will be used.
            "#);
        return Some((0, Value::Nil));
    }

    if !validate::validate_vals(
        &[
            Validator::MustHave("-type".to_string()),
            Validator::CanHave("-url".to_string()),
            Validator::CanHave("-id".to_string())
        ],
        vals,
    ) {
        return Some((1, Value::Nil));
    }

    let id = match vals.get(&"-id".to_string()) {
        Some(x) => match x.parse::<usize>() {
            Ok(y) => y,
            Err(_) => {
                error("`-id` argument of `session-switch` must be a valid int".to_string());
                return Some((1, Value::Nil));
            }
        },
        None => ctx.all_sessions.keys().fold(0, |a, &b| a.min(b)) + 1,
    };

    if ctx.all_sessions.contains_key(&id) && !flags.contains(&"-overwrite".to_string()) {
        warn("creating a session with this ID will overwrite an existing session. Run again with -overwrite to continue.".to_string());
        return Some((2, Value::Nil));
    }

    match &vals[&"-type".to_string()][..] {
        "local" => {
            ctx.all_sessions.insert(id, Box::new(LocalSession));
        }

        "web" => {
            let url = match vals.get(&"-url".to_string()) {
                Some(x) => x,
                None => {
                    error("When session type is `web`, a `-url` argument must be provided.".to_string());
                    return Some((1, Value::Nil));
                }
            };
            ctx.all_sessions.insert(
                id,
                Box::new(WebSession {
                    url: url.to_string(),
                }),
            );
        }

        x => {
            error(format!(
                "`-type` argument of `session-create` must be a valid session type, not {}",
                x
            ));
            hint("try using one of: `local`, `web`".to_string());
            return Some((1, Value::Nil));
        }
    }

    success(format!("session {} created", id));

    Some((0, Value::Nil))
}
