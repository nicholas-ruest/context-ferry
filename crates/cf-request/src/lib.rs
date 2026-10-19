use cf_types::{ContextEvent, Destination};
use serde::{Deserialize, Serialize};
use thiserror::Error;
#[derive(Debug, Error)]
pub enum RequestError {
    #[error("budget must be between 1 and 100000")]
    Budget,
    #[error("events required")]
    Empty,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MigrationRequest {
    pub destination: Destination,
    pub budget: u32,
    pub max_risk: f64,
    pub events: Vec<ContextEvent>,
}
impl MigrationRequest {
    pub fn new(
        destination: Destination,
        budget: u32,
        max_risk: f64,
        events: Vec<ContextEvent>,
    ) -> Result<Self, RequestError> {
        if !(1..=100_000).contains(&budget) {
            return Err(RequestError::Budget);
        }
        if events.is_empty() {
            return Err(RequestError::Empty);
        }
        Ok(Self {
            destination,
            budget,
            max_risk: max_risk.clamp(0.0, 1.0),
            events,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use cf_types::Sensitivity;
    #[test]
    fn rejects_zero_budget() {
        let d = Destination::new("x", Sensitivity::Public).unwrap();
        assert!(MigrationRequest::new(d, 0, 0.2, vec![]).is_err());
    }
}
