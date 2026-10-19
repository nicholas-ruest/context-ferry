use serde::{Deserialize,Serialize};
#[derive(Clone,Debug,Serialize,Deserialize)]pub struct EvaluationReceipt{pub suite_hash:String,pub replay_verified:bool,pub promotions:u32,pub authority:String}
impl EvaluationReceipt{#[must_use]pub fn advisory(suite_hash:&str,replay_verified:bool,promotions:u32)->Self{Self{suite_hash:suite_hash.into(),replay_verified,promotions,authority:"none".into()}}}
#[cfg(test)]mod tests{use super::*;#[test]fn authority_is_none(){assert_eq!(EvaluationReceipt::advisory("x",true,0).authority,"none");}}
