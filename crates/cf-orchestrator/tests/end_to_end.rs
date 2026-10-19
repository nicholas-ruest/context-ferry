use cf_orchestrator::{Ferry,FerryOptions,frozen_request};
#[test]fn migrates_and_rejects_tamper_by_contract(){let d=tempfile::tempdir().unwrap();let result=Ferry::new(d.path().join("memory").to_str().unwrap(),FerryOptions::default()).execute(&frozen_request()).unwrap();assert!(result.telemetry.evidence_verified);assert!(result.ledger_events>=8);assert_eq!(result.coordination.authority,"none");}
