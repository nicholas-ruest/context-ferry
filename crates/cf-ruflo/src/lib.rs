use serde::{Deserialize,Serialize};
#[derive(Clone,Debug,Serialize,Deserialize)]pub struct CoordinationReceipt{pub topology:String,pub authority:String,pub mission:String}
impl CoordinationReceipt{#[must_use]pub fn advisory(mission:&str)->Self{Self{topology:"hierarchical".into(),authority:"none".into(),mission:mission.into()}}}
#[cfg(test)]mod tests{use super::*;#[test]fn cannot_claim_authority(){assert_eq!(CoordinationReceipt::advisory("build").authority,"none");}}
