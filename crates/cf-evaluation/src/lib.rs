use cf_orchestrator::{Ferry, FerryOptions, frozen_request};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModeResult {
    pub mode: String,
    pub released: bool,
    pub selected: usize,
    pub tokens: u32,
    pub recalled: bool,
    pub evidence: bool,
    pub robust: bool,
    pub replayed_decisions: usize,
    pub objective: f64,
    pub failure: Option<String>,
}

pub fn evaluate_modes() -> Vec<ModeResult> {
    let modes = [
        ("combined", FerryOptions::default()),
        (
            "no-ruvector",
            FerryOptions {
                ruvector: false,
                ..FerryOptions::default()
            },
        ),
        (
            "no-rvf",
            FerryOptions {
                rvf: false,
                ..FerryOptions::default()
            },
        ),
        (
            "no-tiktoken",
            FerryOptions {
                tiktoken: false,
                ..FerryOptions::default()
            },
        ),
        (
            "no-conformal",
            FerryOptions {
                conformal: false,
                ..FerryOptions::default()
            },
        ),
        (
            "enterprise-only",
            FerryOptions {
                ruvector: false,
                rvf: false,
                tiktoken: true,
                conformal: false,
            },
        ),
        (
            "ruvnet-only",
            FerryOptions {
                ruvector: true,
                rvf: true,
                tiktoken: false,
                conformal: false,
            },
        ),
    ];
    modes
        .into_iter()
        .map(|(name, options)| {
            let d = tempfile::tempdir().expect("temporary evaluation directory");
            match Ferry::new(d.path().join("db").to_str().expect("utf8 path"), options)
                .execute(&frozen_request())
            {
                Ok(o) => {
                    let objective = o.capsule.events.iter().map(|e| e.importance).sum::<f64>()
                        * 10.0
                        - f64::from(o.capsule.used_tokens) / 20.0
                        + if o.telemetry.evidence_verified {
                            1.0
                        } else {
                            0.0
                        }
                        + if o.robust { 1.0 } else { 0.0 };
                    ModeResult {
                        mode: name.into(),
                        released: true,
                        selected: o.telemetry.selected_events,
                        tokens: o.telemetry.used_tokens,
                        recalled: o.telemetry.recall_count > 0,
                        evidence: o.telemetry.evidence_verified,
                        robust: o.robust,
                        replayed_decisions: o.replay.decisions.len(),
                        objective,
                        failure: None,
                    }
                }
                Err(error) => ModeResult {
                    mode: name.into(),
                    released: false,
                    selected: 0,
                    tokens: 0,
                    recalled: false,
                    evidence: false,
                    robust: options.conformal,
                    replayed_decisions: 0,
                    objective: 0.0,
                    failure: Some(error.to_string()),
                },
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ablations_remove_declared_capabilities() {
        let r = evaluate_modes();
        let combined = &r[0];
        assert!(combined.released && combined.recalled && combined.evidence && combined.robust);
        assert!(!r[1].recalled);
        assert!(!r[2].evidence);
        assert!(!r[4].robust);
        assert!(!r[3].released);
    }
}
