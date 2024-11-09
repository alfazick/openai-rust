use openai_rust::{Config, Client, Tool, LabError};
use openai_rust::tools::WeatherTool;
use openai_rust::Agent;

// Usage example in main.rs
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new()?;
    let client = Client::new(config)?;
    let agent = Agent::new(client);

    // The agent will automatically use the weather tool if needed
    let response = agent.chat("What's the weather like in San Francisco?").await?;
    println!("Agent: {}", response);

    // This won't use any tools since none are needed
    let response = agent.chat("What is 2+2?").await?;
    println!("Agent: {}", response);

    Ok(())

}