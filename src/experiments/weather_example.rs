use crate::{Client, Config, Tool, WeatherTool, LabError};
use serde_json::json;

pub async fn run_weather_example() -> Result<(), LabError> {
    let config = Config::new()?;
    let client = Client::new(config)?;
    let weather_tool = WeatherTool;

    // Setup tool
    let tools = vec![weather_tool.definition().to_json()];

    // First message
    let response = client
        .chat_completion_with_tools(
            "You are a helpful weather assistant. Always check the weather when asked about it.",
            "What's the weather like in San Francisco?",
            tools,
        )
        .await?;

    // Handle tool calls if present
    if let Some(tool_calls) = response["choices"][0]["message"]["tool_calls"].as_array() {
        for tool_call in tool_calls {
            let function_name = tool_call["function"]["name"].as_str().unwrap();
            if function_name == "get_weather" {
                let args = &tool_call["function"]["arguments"];
                let weather_data = weather_tool.execute(
                    &serde_json::from_str(args.as_str().unwrap()).unwrap()
                ).await?;
                println!("Weather data: {:?}", weather_data);
            }
        }
    }

    Ok(())
}