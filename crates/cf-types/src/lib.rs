use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EventId(String);
impl EventId {
    pub fn parse(value: impl Into<String>) -> Result<Self, &'static str> {
        let value=value.into();
        if value.trim().is_empty() || value.len()>64 { return Err("invalid event id"); }
        Ok(Self(value))
    }
}
impl fmt::Display for EventId { fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{write!(f,"{}",self.0)} }

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Sensitivity { Public, Internal, Secret }
impl Sensitivity { #[must_use] pub const fn level(self)->u8 { match self { Self::Public=>0,Self::Internal=>1,Self::Secret=>2 } } }

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ContextEvent {
    pub id: EventId,
    pub kind: String,
    pub text: String,
    pub sensitivity: Sensitivity,
    pub importance: f64,
    pub tokens: u32,
}
impl ContextEvent {
    pub fn new(id:&str,kind:&str,text:&str,sensitivity:Sensitivity,importance:f64)->Result<Self,&'static str>{
        if kind.trim().is_empty() || text.trim().is_empty() || !(0.0..=1.0).contains(&importance){return Err("invalid event");}
        Ok(Self{id:EventId::parse(id)?,kind:kind.into(),text:text.into(),sensitivity,importance,tokens:0})
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Destination { pub name:String, pub clearance:Sensitivity }
impl Destination { pub fn new(name:&str,clearance:Sensitivity)->Result<Self,&'static str>{if name.trim().is_empty(){Err("empty destination")}else{Ok(Self{name:name.into(),clearance})}} }

#[cfg(test)] mod tests { use super::*; #[test] fn validates_values(){assert!(EventId::parse("").is_err()); assert!(Destination::new("edge",Sensitivity::Internal).is_ok());} }
