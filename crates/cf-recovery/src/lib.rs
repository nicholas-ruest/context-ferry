use cf_capsule::ContextCapsule;use serde::{Deserialize,Serialize};
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]pub struct ReplayState{pub objectives:Vec<String>,pub decisions:Vec<String>,pub pending:Vec<String>}
#[must_use]pub fn replay(c:&ContextCapsule)->ReplayState{let mut s=ReplayState{objectives:vec![],decisions:vec![],pending:vec![]};for e in &c.events{match e.kind.as_str(){"objective"=>s.objectives.push(e.text.clone()),"decision"|"rejected"=>s.decisions.push(e.text.clone()),"pending"=>s.pending.push(e.text.clone()),_=>{}}}s}
#[cfg(test)]mod tests{use super::*;#[test]fn replay_state_is_comparable(){assert_eq!(ReplayState{objectives:vec![],decisions:vec![],pending:vec![]},ReplayState{objectives:vec![],decisions:vec![],pending:vec![]});}}
