use std::{env, fs, path::Path};

use serde::Deserialize;

const MODEL: &str = "gpt-3.5-turbo";
const TOKEN_FILENAME: &str = ".howto";

pub fn make_request(query: &str) -> String {
    let token = load_token();

    let client = reqwest::blocking::Client::new();
    let rsp_text = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(get_request_body(query))
        .send()
        .unwrap()
        .text()
        .unwrap();
    let rsp: Response = serde_json::from_str(&rsp_text).unwrap();
    rsp.choices[0].message.content.clone()
}

#[derive(Deserialize)]
struct Response {
    choices: Vec<ResponseChoice>,
}

#[derive(Deserialize)]
struct ResponseChoice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

fn load_token() -> String {
    let home_path = env::var("HOME").unwrap();
    let token_path = Path::new(&home_path).join(TOKEN_FILENAME);
    fs::read_to_string(token_path).unwrap().trim().to_owned()
}

fn get_request_body(query: &str) -> String {
    let param_messages = format!("[{}, {}]", get_system_message(), get_user_message(query));
    format!(
        "{{\"model\": \"{}\", \"messages\": {}}}",
        MODEL, param_messages
    )
}

fn get_system_message() -> &'static str {
    "{\"role\": \"system\", \"content\": \"You are a programming assistant, skilled in terminal commands. Respond to a prompt with a really short explanation and example of bash command solving the prompted problem. Keep it as short as possible and with plain text without formatting.\"}"
}

fn get_user_message(query: &str) -> String {
    format!("{{\"role\": \"user\", \"content\": \"{}\"}}", query)
}
