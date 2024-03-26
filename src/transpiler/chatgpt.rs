use std::path::Path;
use color_eyre::eyre::eyre;
use crate::llm::chatgpt::{ChatGPTClient, MessageRole};
use crate::llm::Message;
use crate::transpiler::{TranspileInfo, Transpiler};

const SYSTEM_PROMPT: &'static str = r#"Transpile C/C++ source code to Rust code. You must follow the rules below:
1. Your response must contain ONLY Rust code with plain text format, not markdown format. Do not give me any other information.
2. Do not try to fix any errors in C/C++ code while transpiling. Just transpile it to Rust code as-is, but you can add comments to explain the code.
3. The transpiled code must be as close as possible to the original code. Do not try to optimize the code.
4. The transpiled code must be idiomatic Rust code. Do not use unsafe code unless it is necessary.
5. Since C++ could overload functions with same name, you should add number of parameters as suffix to the function name to make it unique in Rust code.
6. Try to use smart pointers in Rust code as much as possible. For example, use Box, Rc, Arc, etc. to manage memory.
"#;

pub struct ChatGPT {
    client: ChatGPTClient,
}

impl ChatGPT {
    pub fn init() -> color_eyre::Result<Self> {
        let client = ChatGPTClient::init()?;
        Ok(Self {
            client,
        })
    }

    pub async fn send_message<S: Into<String>>(&self, message: S) -> color_eyre::Result<String> {
        let init_message = Message::new(MessageRole::System, SYSTEM_PROMPT);
        let user_message = Message::new(MessageRole::User, message);
        let resp = self.client.completion(&[init_message, user_message]).await?;
        if resp.choices.is_empty() {
            return Err(eyre!("no response from chatgpt"));
        }
        let message = resp.choices[0].message.content();
        Ok(message.to_string())
    }
}

impl Transpiler for ChatGPT {
    async fn transpile<T: AsRef<str>>(&self, source: T) -> color_eyre::Result<String> {
        self.send_message(source.as_ref()).await
    }

    async fn transpile_from_path<P: AsRef<Path>>(&self, path: P) -> color_eyre::Result<Vec<TranspileInfo>> {
        todo!()
    }
}