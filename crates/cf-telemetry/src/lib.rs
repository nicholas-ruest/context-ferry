use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Telemetry {
    pub input_events: usize,
    pub authorized_events: usize,
    pub selected_events: usize,
    pub used_tokens: u32,
    pub recall_count: usize,
    pub evidence_verified: bool,
}
impl Telemetry {
    #[must_use]
    pub fn policy_drop_count(&self) -> usize {
        self.input_events.saturating_sub(self.authorized_events)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn drop_count_saturates() {
        assert_eq!(
            Telemetry {
                input_events: 1,
                authorized_events: 2,
                ..Telemetry::default()
            }
            .policy_drop_count(),
            0
        );
    }
}
