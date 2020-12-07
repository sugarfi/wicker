use crate::eval;
use crate::session::{local::LocalSession, web::WebSession, SessionType};

pub fn prompt(ctx: &eval::Context) -> String {
    return format!(
        "\x1b[35m{} \x1b[34m{}{} \x1b[36msession:[{}]{} \x1b[0m> ",
        ctx.cwd,
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
    );
}
