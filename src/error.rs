use std::env;

pub fn error(msg: String) {
    if env::var("NO_COLOR").unwrap_or(String::new()).len() > 0 {
        println!("✘ Error: {} ✘", msg);
    } else {
        println!("\x1b[0;31m✘ Error: {} ✘\x1b[0m", msg);
    }
}

pub fn hint(msg: String) {
    if env::var("NO_COLOR").unwrap_or(String::new()).len() > 0 {
        println!("Hint: {}", msg);
    } else {
        println!("\x1b[33mHint: {}\x1b[0m", msg);
    }
}

pub fn warn(msg: String) {
    if env::var("NO_COLOR").unwrap_or(String::new()).len() > 0 {
        println!("Warn: {}", msg);
    } else {
        println!("\x1b[33mWarn: {}\x1b[0m", msg);
    }
}

pub fn success(msg: String) {
    if env::var("NO_COLOR").unwrap_or(String::new()).len() > 0 {
        println!("✔️ {} ✔️", msg);
    } else {
        println!("\x1b[32m✔️ {} ✔️\x1b[0m", msg);
    }
}
