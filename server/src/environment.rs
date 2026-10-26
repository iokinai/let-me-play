use std::env;

use thiserror::Error;
use trl::Trl;

#[derive(Debug, Error)]
pub enum EnvError {
    #[error("Environment variable {0} is missing")]
    MissingVariable(String),
}

#[derive(Trl)]
#[getters]
pub struct Environment {
    backend_addr: String,
    frontend_url: String,
}

impl Environment {
    pub fn parse() -> Result<Environment, EnvError> {
        dotenv::dotenv().ok();

        let backend_addr = env::var("BACKEND_ADDR")
            .map_err(|_| EnvError::MissingVariable(String::from("BACKEND_ADDR")))?[..]
            .trim()
            .to_string();

        let frontend_url = env::var("FRONTEND_URL")
            .map_err(|_| EnvError::MissingVariable(String::from("FRONTEND_URL")))?[..]
            .trim()
            .to_string();

        Ok(Environment {
            backend_addr,
            frontend_url,
        })
    }
}
