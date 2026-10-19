use cf_types::EventId;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventKind {
    RequestAccepted,
    PolicyApplied,
    RecallCompleted,
    SelectionCompleted,
    RiskAccepted,
    EvidenceSealed,
    TransferVerified,
    ReplayCompleted,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct DomainEvent {
    pub sequence: u64,
    pub kind: EventKind,
    pub subject: Option<EventId>,
    pub detail: String,
}
impl DomainEvent {
    #[must_use]
    pub fn new(sequence: u64, kind: EventKind, detail: impl Into<String>) -> Self {
        Self {
            sequence,
            kind,
            subject: None,
            detail: detail.into(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn event_is_typed() {
        assert_eq!(
            DomainEvent::new(1, EventKind::RequestAccepted, "ok").sequence,
            1
        );
    }
}
