use cf_types::ContextEvent;use rvf_crypto::{WitnessEntry,create_witness_chain,shake256_256,verify_witness_chain};use serde::{Deserialize,Serialize};use thiserror::Error;
#[derive(Clone,Debug,Serialize,Deserialize)]pub struct Seal{pub chain:Vec<u8>}
#[derive(Debug,Error)]pub enum SealError{#[error("empty payload or invalid RVF witness")]Invalid}
pub fn seal(events:&[ContextEvent])->Result<Seal,SealError>{if events.is_empty(){return Err(SealError::Invalid)}let entries=events.iter().enumerate().map(|(i,e)|WitnessEntry{prev_hash:[0;32],action_hash:shake256_256(format!("{}:{}:{}",e.id,e.kind,e.text).as_bytes()),timestamp_ns:i as u64+1,witness_type:18}).collect::<Vec<_>>();Ok(Seal{chain:create_witness_chain(&entries)})}
pub fn verify(seal:&Seal)->Result<(),SealError>{verify_witness_chain(&seal.chain).map(|_|()).map_err(|_|SealError::Invalid)}
#[cfg(test)]mod tests{use super::*;use cf_types::Sensitivity;#[test]fn tamper_rejected(){let e=ContextEvent::new("a","x","y",Sensitivity::Public,1.0).unwrap();let mut s=seal(&[e]).unwrap();verify(&s).unwrap();s.chain[0]^=1;assert!(verify(&s).is_err());}}
