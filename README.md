# openai-rust
openai+rust+tool+agent

openai-lab/
├── Cargo.toml
├── .env
├── README.md
├── src/
│   ├── main.rs              # Example usage and experiments
│   ├── lib.rs               # Library exports and public API
│   ├── error.rs             # Error handling
│   ├── config.rs            # Configuration management
│   ├── client.rs            # OpenAI API client
│   ├── tools/               # Function calling tools
│   │   ├── mod.rs          # Tool module exports
│   │   ├── weather.rs      # Weather tool implementation
│   │   ├── calculator.rs    # Calculator tool implementation
│   │   └── definitions.rs   # Tool traits and types
│   ├── models/              # Data structures
│   │   ├── mod.rs          # Model exports
│   │   └── chat.rs         # Chat-related types
│   └── experiments/         # Example experiments
        ├── mod.rs           # Experiment exports
        └── weather_chat.rs  # Weather chat example