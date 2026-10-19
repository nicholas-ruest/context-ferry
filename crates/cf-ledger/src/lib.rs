use cf_events::DomainEvent;use thiserror::Error;
#[derive(Debug,Error)]pub enum LedgerError{#[error("event sequence must increase")]Sequence}
#[derive(Default,Debug)]pub struct Ledger{events:Vec<DomainEvent>}
impl Ledger{pub fn append(&mut self,event:DomainEvent)->Result<(),LedgerError>{if self.events.last().is_some_and(|e|e.sequence>=event.sequence){return Err(LedgerError::Sequence)}self.events.push(event);Ok(())}#[must_use]pub fn events(&self)->&[DomainEvent]{&self.events}}
#[cfg(test)]mod tests{use super::*;use cf_events::EventKind;#[test]fn rejects_reorder(){let mut l=Ledger::default();l.append(DomainEvent::new(2,EventKind::RequestAccepted,"x")).unwrap();assert!(l.append(DomainEvent::new(1,EventKind::PolicyApplied,"x")).is_err());}}
