use serde::Deserialize;

#[derive(Deserialize)]
pub struct Prompt {
    pub format: String
}

#[derive(Deserialize)]
pub struct Config {
    pub prompt: Prompt
}
