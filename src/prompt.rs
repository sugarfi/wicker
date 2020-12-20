use crate::eval;
use crate::session::SessionType;
use std::env;
use crate::config;
use crate::error::error;

pub fn prompt(ctx: &eval::Context, config: &config::Config) -> String {
    let mut out = String::new();

    let mut chars = config.prompt.format.chars().peekable();
    while chars.peek().is_some() {
        let c = chars.next().unwrap();
        match c {
            '%' => {
                let spec = match chars.next() {
                    None => {
                        error("% in prompt format must be followed by a specifier".to_string());
                        return "> ".to_string();
                    }
                    Some(x) => x
                };
                match spec {
                    'c' => {
                        out.push_str(&ctx.all_sessions[&ctx.session].get_cwd());
                    }
                    's' => {
                        out.push_str(&ctx.status.to_string());
                    }
                    't' => {
                        out.push_str(match ctx.all_sessions[&ctx.session].get_type() {
                            SessionType::Local => "local",
                            SessionType::Web => "web"
                        });
                    }
                    'S' => {
                        out.push_str(&ctx.session.to_string());
                    }
                    '%' => {
                        out += "%";
                    }
                    'e' => {
                        out += "\x1b";
                    }
                    _ => {
                        error(format!("invalid format specifier {}", c));
                        return "> ".to_string();
                    }
                }
            }
            _ => {
                out.push(c);
            }
        }
    };

    out
}
