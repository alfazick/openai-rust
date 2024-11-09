use crate::error::LabError;
use dotenv::dotenv;
use std::env;

pub struct Config {
    openai_api_key: String,
    model:String,
}

impl Config {
    pub fn new() -> Result<Self,LabError> {
        dotenv().map_err(LabError::DotEnvError)?;

        let config = Config {
            openai_api_key:Self::get_env_var("OPENAI_API_KEY")?,
            model: Self::get_env_var("OPENAI_MODEL")?,
        };

        
        Ok(config)
    }


    fn get_env_var(key: &str) -> Result<String,LabError> {
        env::var(key).map_err(|_| LabError::EnvVarNotFound(key.to_string()))
    }

    pub fn openai_api_key(&self) -> &str {
        &self.openai_api_key
    }

    pub fn model(&self) -> &str {
        &self.model
    }


}