
use crate::error::LabError;
use crate::config::Config;

use reqwest::header::{HeaderMap,HeaderValue,AUTHORIZATION,CONTENT_TYPE};
use serde_json::{json, Value};


pub struct Client {
    config: Config,
    http_client: reqwest::Client,
}

impl Client {
    pub fn new(config:Config) -> Result<Self, LabError> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", config.openai_api_key()))
                .map_err(|e| LabError::EnvVarNotFound(e.to_string()))?
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let http_client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .map_err(LabError::RequestError)?;

        Ok(Client {
            config,
            http_client,
        })
    }

    pub async fn chat_completion(&self,system_prompt:&str,user_message:&str) -> Result<Value,LabError> {
        let request_body = json!({
            "model": self.config.model(),
            "messages": [
                {
                    "role": "system",
                    "content": system_prompt
                },
                {
                    "role": "user",
                    "content": user_message
                }
            ]
        });

        let response: Value = self.http_client
            .post("https://api.openai.com/v1/chat/completions")
            .json(&request_body)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    pub async fn chat_completion_with_tools(
        &self,
        system_prompt: &str,
        user_message: &str,
        tools: Vec<Value>,
    ) -> Result<Value, LabError> {
        let request_body = json!({
            "model": self.config.model(),
            "messages": [
                {
                    "role": "system",
                    "content": system_prompt
                },
                {
                    "role": "user",
                    "content": user_message
                }
            ],
            "tools": tools,
            "tool_choice": "auto"
        });

        let response: Value = self.http_client
            .post("https://api.openai.com/v1/chat/completions")
            .json(&request_body)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    pub async fn handle_conversation(
        &self,
        messages: Vec<Value>,
        tools: Option<Vec<Value>>,
    ) -> Result<Value, LabError> {
        let mut request_body = json!({
            "model": self.config.model(),
            "messages": messages,
        });

        if let Some(tools) = tools {
            request_body["tools"] = json!(tools);
            request_body["tool_choice"] = json!("auto");
        }

        let response: Value = self.http_client
            .post("https://api.openai.com/v1/chat/completions")
            .json(&request_body)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    } 
}

