use async_trait::async_trait;
use serde_json::Value;
use crate::error::LabError;
use serde_json::json;


#[async_trait]
pub trait Tool {
    fn name(&self) -> &str;
    fn definition(&self) -> ToolDefinition;
    async fn execute(&self, args: &Value) -> Result<Value, LabError>;
}

pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: Value,
}

impl ToolDefinition {
    pub fn to_json(&self) -> Value {
        json!({
            "type": "function",
            "function": {
                "name": self.name,
                "description": self.description,
                "parameters": self.parameters
            }
        })
    }
}

