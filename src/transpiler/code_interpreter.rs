use std::cell::RefCell;
use std::path::Path;
use color_eyre::eyre::eyre;
use crate::compile_rust_code;
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

    /// 向大模型发送 C/C++ 代码，获取转译的 Rust 代码，可以选择提供编译错误信息
    async fn get_suggestion<S, T>(&self, source_code: S, error_msg: Option<T>) -> color_eyre::Result<String>
    where
        S: Into<String>,
        T: Into<String>,
    {
        let prompt = if let Some(error_msg) = error_msg {
            format!("Here is Rust code: ```rust\n{}\n```. Here is compiler error from rustc: ```\n{}\n```. Try to fix the error", source_code.into(), error_msg.into())
        } else {
            format!("```cpp\n{}\n```", source_code.into())
        };
        let msg = Message::new(MessageRole::User, prompt);
        let response = self.conversation(msg).await?;
        Self::extract_rust_code_block(&response)
    }

    async fn conversation(&self, msg: Message) -> color_eyre::Result<String> {
        self.client.borrow_mut().conversation(msg).await
    }

    fn clear_conversation(&self) {
        self.client.borrow_mut().clear_history();
    }

    fn extract_rust_code_block(resp_text: &str) -> color_eyre::Result<String> {
        let re = regex::Regex::new(r"(?s)```rust\s+(.*?)\s+```")?;
        let captures = re.captures(resp_text).ok_or_else(|| eyre!("No Rust code block found in response: {resp_text}"))?;
        Ok(captures.get(1).unwrap().as_str().to_string())
    }
}

impl Transpiler for CodeInterpreter {
    async fn transpile<T: AsRef<str>>(&self, source: T) -> color_eyre::Result<String> {
        let source = source.as_ref();
        let mut resp = self.get_suggestion(source, Option::<&str>::None).await?;

        let mut max_tries = 3;
        let mut error_message = compile_rust_code(&resp);
        while !error_message.is_empty() && max_tries > 0 {
            max_tries -= 1;
            // 将上一步 LLM 返回的 Rust 代码作为输入，继续获取建议
            self.clear_conversation();
            resp = self.get_suggestion(&resp, Some(error_message)).await?;
            error_message = compile_rust_code(&resp);
        }

        Ok(resp)
    }

    async fn transpile_from_path<P: AsRef<Path>>(&self, _path: P) -> color_eyre::Result<Vec<TranspileInfo>> {
        todo!()
    }
}