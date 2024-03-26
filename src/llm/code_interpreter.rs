use color_eyre::eyre::eyre;
use reqwest::Client;
use serde::Deserialize;
use crate::llm::chatgpt::MessageRole;
use crate::llm::Message;

pub struct CodeInterpreterClient {
    client: Client,
    history: Vec<Message>,
}

const API_URL: &'static str = "http://127.0.0.1:8000/completions";

impl CodeInterpreterClient {
    pub fn init<S: Into<String>>(system_prompt: S) -> Self {
        Self {
            client: Client::new(),
            history: vec![Message::new(MessageRole::System, system_prompt)],
        }
    }

    pub async fn completion(&self, messages: &[Message]) -> color_eyre::Result<String> {
        let resp = self.client.post(API_URL)
            .json(messages)
            .send()
            .await?
            .json::<CompletionResponse>()
            .await?;

        resp.data().map(ToString::to_string)
    }

    /// 进行对话，大模型将知道上下文
    pub async fn conversation(&mut self, message: Message) -> color_eyre::Result<String> {
        self.history.push(message);

        let resp = self.completion(&self.history).await?;
        let assistant_message = Message::new(MessageRole::Assistant, resp.clone());

        self.history.push(assistant_message);

        Ok(resp)
    }

    /// 清除对话上下文
    pub fn clear_history(&mut self) {
        self.history.truncate(1);
    }
}

#[derive(Deserialize)]
pub struct CompletionResponse {
    code: i32,
    msg: String,
    data: Option<String>,
}

impl CompletionResponse {
    pub fn data(&self) -> color_eyre::Result<&str> {
        match &self.data {
            Some(data) => Ok(data),
            None => Err(eyre!("{}", self.msg)),
        }
    }
}