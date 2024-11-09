use super::definitions::{Tool, ToolDefinition};
use async_trait::async_trait;
use serde_json::{json, Value};
use crate::error::LabError;

pub struct WeatherTool;

#[async_trait]
impl Tool for WeatherTool {
    fn name(&self) -> &str {
        "get_weather"
    }


    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "get_weather".to_string(),
            description:"Get the current weather in a location".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string",
                        "description": "The city and state, e.g. San Francisco, CA"
                    }
                },
                "required": ["location"]
            }),
        }
    }


    async fn execute(&self, args: &Value) -> Result<Value,LabError> {
        let location = args["location"]
            .as_str()
            .ok_or_else(|| LabError::ToolError("Location not provided".to_string()))?;

        // In a real implementation, you would call a weather API here
        Ok(json!({
            "temperature": 72,
            "condition": "sunny",
            "location": location
        }))
    }
}