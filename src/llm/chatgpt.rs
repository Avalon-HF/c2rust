use color_eyre::eyre::eyre;
use reqwest::Client;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

const API_URL: &'static str = "https://api.openai.com/v1/chat/completions";
// const MODEL_NAME: &'static str = "gpt-4";
const MODEL_NAME: &'static str = "gpt-4-turbo-preview";

pub struct ChatGPTClient {
    client: Client,
}

impl ChatGPTClient {
    pub fn init() -> color_eyre::Result<Self> {
        let api_key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| eyre!("OPENAI_API_KEY env must be set") )?;
        Ok(Self::new(api_key)?)
    }

    fn new<S: AsRef<str>>(api_key: S) -> color_eyre::Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", api_key.as_ref()))?);
        let client = Client::builder()
            .default_headers(headers)
            .build()?;
        Ok(Self {
            client,
        })
    }

    pub async fn completion(&self, messages: &[Message]) -> color_eyre::Result<CompletionResponse> {
        let req = CompletionRequest {
            model: MODEL_NAME,
            messages,
        };
        let resp = self.client.post(API_URL)
            .json(&req)
            .send()
            .await?
            .text()
            .await?;
        println!("completion resp: {}", resp);

        Ok(serde_json::from_str(&resp)?)
    }
}

#[derive(Debug, Serialize)]
pub struct CompletionRequest<'a> {
    model: &'a str,
    messages: &'a [Message],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    role: MessageRole,
    content: String,
}

impl Message {
    pub fn new<S: Into<String>>(role: MessageRole, content: S) -> Self {
        Self {
            role,
            content: content.into(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "assistant")]
    Assistant,
}

#[derive(Debug, Deserialize)]
pub struct CompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
    pub system_fingerprint: String,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub index: i32,
    pub message: Message,
    pub finish_reason: String,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}