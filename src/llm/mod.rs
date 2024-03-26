use serde::{Deserialize, Serialize};
use crate::llm::chatgpt::MessageRole;

pub mod chatgpt;
pub mod code_interpreter;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    role: MessageRole,
    content: String,
}

