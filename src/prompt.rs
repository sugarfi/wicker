use crate::eval;
use crate::session::SessionType;
use std::env;

pub fn prompt(ctx: &eval::Context) -> String {
    return
        if env::var("NO_COLOR").unwrap_or(String::new()).len() > 0 {
            format!(
                "{} {} [{}]{} > ",
                ctx.all_sessions[&ctx.session].get_cwd(),
                ctx.status,
                match ctx.all_sessions[&ctx.session].get_type() {
                    SessionType::Local => "local",
                    SessionType::Web => "web",
                },
                ctx.session
            )
        } else {
            format!(
                "\x1b[35m{} \x1b[34m{}{} \x1b[36msession:[{}]{} \x1b[0m> ",
                ctx.all_sessions[&ctx.session].get_cwd(),
                match ctx.status {
                    0 => "\x1b[32m",
                    2 => "\x1b[33m",
                    _ => "\x1b[31m",
                },
                ctx.status,
                ctx.session,
                match ctx.all_sessions[&ctx.session].get_type() {
                    SessionType::Local => "local",
                    SessionType::Web => "web",
                }
            )
        };
}
