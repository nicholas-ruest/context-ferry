use cf_request::MigrationRequest; use cf_types::ContextEvent;
#[must_use] pub fn authorize(request:&MigrationRequest)->Vec<ContextEvent>{request.events.iter().filter(|e|e.sensitivity.level()<=request.destination.clearance.level()).cloned().collect()}
#[cfg(test)] mod tests {use super::*;use cf_request::MigrationRequest;use cf_types::*;#[test]fn filters_secret(){let d=Destination::new("edge",Sensitivity::Internal).unwrap();let e=ContextEvent::new("s","secret","key",Sensitivity::Secret,1.0).unwrap();assert!(authorize(&MigrationRequest::new(d,20,0.5,vec![e]).unwrap()).is_empty());}}
