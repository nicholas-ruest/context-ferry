use cf_capsule::ContextCapsule;use cf_rvf::Seal;use serde::{Deserialize,Serialize};
#[derive(Clone,Debug,Serialize,Deserialize)]pub struct TransferEnvelope{pub capsule:ContextCapsule,pub witness:Option<Seal>}
impl TransferEnvelope{#[must_use]pub const fn new(capsule:ContextCapsule,witness:Option<Seal>)->Self{Self{capsule,witness}}}
#[cfg(test)]mod tests{use super::*;#[test]fn witness_is_explicit(){let _:fn(ContextCapsule,Option<Seal>)->TransferEnvelope=TransferEnvelope::new;}}
