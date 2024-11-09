use serde_json::{json, Value};

pub struct ChatMessage {
    role: String,
    content:String,
    tool_calls:Option<Vec<Value>>,
    tool_call_id:Option<String>,
    name:Option<String>,
}

impl ChatMessage{
    pub fn new_user(content: &str) -> Self {
        Self {
            role: "user".to_string(),
            content: content.to_string(),
            tool_calls: None,
            tool_call_id:None,
            name: None,
        }
    }

    pub fn to_json(&self) -> Value {
        let mut message = json!({
            "role": self.role,
            "content": self.content
        });

        if let Some(tool_call_id) = &self.tool_call_id {
            message["tool_call_id"] = json!(tool_call_id);
        }
        
        message

    }


}