// src/experiments/agent_example.rs
use crate::{Client, Config, Tool, WeatherTool, LabError};
use serde_json::{json, Value};

pub struct Agent {
    client: Client,
    tools: Vec<Box<dyn Tool>>,
}

impl Agent {
    pub fn new(client: Client) -> Self {
        let tools: Vec<Box<dyn Tool>> = vec![
            Box::new(WeatherTool)
        ];
        
        Self { client, tools }
    }

    pub async fn chat(&self, message: &str) -> Result<String, LabError> {
        // Collect tool definitions
        let tool_definitions: Vec<Value> = self.tools
            .iter()
            .map(|tool| tool.definition().to_json())
            .collect();

        // Make initial request
        let response = self.client
            .chat_completion_with_tools(
                "You are a helpful assistant. Use tools when needed to provide accurate information.",
                message,
                tool_definitions,
            )
            .await?;

        // Check if the model wants to use a tool
        if let Some(tool_calls) = response["choices"][0]["message"]["tool_calls"].as_array() {
            // Handle tool calls
            let mut messages = vec![
                json!({
                    "role": "user",
                    "content": message
                }),
                response["choices"][0]["message"].clone()
            ];

            // Execute each tool call
            for tool_call in tool_calls {
                let function_name = tool_call["function"]["name"].as_str().unwrap();
                let args: Value = serde_json::from_str(
                    tool_call["function"]["arguments"].as_str().unwrap()
                )?;

                // Find and execute the appropriate tool
                for tool in &self.tools {
                    if tool.name() == function_name {
                        let result = tool.execute(&args).await?;
                        
                        // Add tool result to messages
                        messages.push(json!({
                            "role": "tool",
                            "tool_call_id": tool_call["id"],
                            "name": function_name,
                            "content": result.to_string()
                        }));
                    }
                }
            }

            // Make final request with tool results
            let final_response = self.client.handle_conversation(messages, Some(tool_definitions)).await?;
            
            Ok(final_response["choices"][0]["message"]["content"]
                .as_str()
                .unwrap_or("No response")
                .to_string())
        } else {
            // No tools needed, return direct response
            Ok(response["choices"][0]["message"]["content"]
                .as_str()
                .unwrap_or("No response")
                .to_string())
        }
    }
}

