use std::env;
use thiserror::Error;
#[derive(Clone, Debug)]
pub struct Config {
    pub memory_path: String,
    pub max_budget: u32,
}
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("invalid CONTEXT_FERRY_MAX_BUDGET")]
    Budget,
}
impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let max_budget = env::var("CONTEXT_FERRY_MAX_BUDGET")
            .unwrap_or_else(|_| "10000".into())
            .parse::<u32>()
            .map_err(|_| ConfigError::Budget)?;
        if !(1..=100_000).contains(&max_budget) {
            return Err(ConfigError::Budget);
        }
        Ok(Self {
            memory_path: env::var("CONTEXT_FERRY_MEMORY")
                .unwrap_or_else(|_| "/tmp/context-ferry-memory".into()),
            max_budget,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn defaults_are_bounded() {
        assert!(Config::from_env().unwrap().max_budget <= 100_000);
    }
}
