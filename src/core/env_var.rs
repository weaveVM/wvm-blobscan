//! Module to handle env variables access
use dotenvy::dotenv;
use std::env;

pub fn load_env_vars() {
    dotenv().ok();
}

pub fn get_env_var(key: &str) -> Result<String, env::VarError> {
    match env::var(key) {
        Ok(val) => Ok(val),
        Err(e) => Err(e),
    }
}
