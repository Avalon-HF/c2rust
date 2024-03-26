use std::cell::RefCell;
use std::path::Path;
use color_eyre::eyre::eyre;
use crate::llm::chatgpt::MessageRole;
use crate::llm::code_interpreter::CodeInterpreterClient;
use crate::llm::Message;
use crate::transpiler::{TranspileInfo, Transpiler};

const SYSTEM_PROMPT: &'static str = r#"Transpile C/C++ source code to Rust code. You must follow the rules below:
1. Your response must contain ONLY Rust code with plain text format, not markdown format. Do not give me any other information.
2. Do not try to fix any errors in C/C++ code while transpiling. Just transpile it to Rust code as-is, but you can add comments to explain the code.
3. The transpiled code must be as close as possible to the original code. Do not try to optimize the code.
4. The transpiled code must be idiomatic Rust code. Do not use unsafe code unless it is necessary.
5. Since C++ could overload functions with same name, you should add number of parameters as suffix to the function name to make it unique in Rust code.
6. Try to use smart pointers in Rust code as much as possible. For example, use Box, Rc, Arc, etc. to manage memory"#;

pub struct CodeInterpreter {
    client: RefCell<CodeInterpreterClient>,
}

impl CodeInterpreter {
    pub fn init() -> Self {
        let client = CodeInterpreterClient::init(SYSTEM_PROMPT);
        Self {
            client: RefCell::new(client),
        }
    }

    async fn send_message<S: Into<String>>(&self, source_code: S) -> color_eyre::Result<String> {
        let msg = Message::new(MessageRole::User, source_code.into());
        todo!()
    }

    async fn conversation(&self, msg: Message) -> color_eyre::Result<String> {
        self.client.borrow_mut().conversation(msg).await
    }

    fn extract_rust_code_block(resp_text: &str) -> color_eyre::Result<String> {
        let re = regex::Regex::new(r"(?s)```rust\s+(.*?)\s+```")?;
        let captures = re.captures(resp_text).ok_or_else(|| eyre!("No Rust code block found in response: {resp_text}"))?;
        Ok(captures.get(1).unwrap().as_str().to_string())
    }
}

impl Transpiler for CodeInterpreter {
    async fn transpile<T: AsRef<str>>(&self, source: T) -> color_eyre::Result<String> {
        todo!("transpile many times with compiler error handling")
    }

    async fn transpile_from_path<P: AsRef<Path>>(&self, path: P) -> color_eyre::Result<Vec<TranspileInfo>> {
        todo!()
    }
}