pub fn error(msg: String) {
    println!("\x1b[0;31m✘ Error: {} ✘\x1b[0m", msg);
}

pub fn hint(msg: String) {
    println!("\x1b[33mHint: {}\x1b[0m", msg);
}

pub fn warn(msg: String) {
    println!("\x1b[33mWarn: {}\x1b[0m", msg);
}

pub fn success(msg: String) {
    println!("\x1b[32m✔️ {} ✔️\x1b[0m", msg);
}
