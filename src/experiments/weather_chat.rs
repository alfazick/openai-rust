
use crate::{Client, Config, Tool, LabError};
use crate::tools::WeatherTool;

pub async fn run_weather_chat() -> Result<(), LabError> {
    let config = Config::new()?;
    let client = Client::new(config)?;
    let weather_tool = WeatherTool;

    let tools = vec![weather_tool.definition().to_json()];

    let response = client
        .chat_completion_with_tools(
            "You are a helpful weather assistant.",
            "What's the weather in San Francisco?",
            tools,
        )
        .await?;

    println!("Response: {:?}", response);
    Ok(())
}