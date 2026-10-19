# Context Ferry specification

## Original problem
Long-running AI agents accumulate decisions, commitments, tool observations, policy constraints, and sensitive material. Moving work to a different model, provider, edge node, or offline runtime commonly means copying an oversized transcript or using an unverifiable summary. The former violates token, cost, latency, or residency limits; the latter can omit decision-critical state without detection.

## Intended users
- Platform engineers operating multi-provider or sovereign inference fleets.
- Reliability and security teams responsible for agent handoffs.
- Regulated operators who need a verifiable record of what context crossed a boundary.

## System capability
Context Ferry constructs a token-bounded, policy-filtered, semantically recalled, conformal-risk-gated and cryptographically sealed context capsule, transfers it to a new inference boundary, verifies it, and deterministically replays its decision state.

No single ingredient provides this capability. OpenAI tiktoken measures the real target encoding but does not select, recall, govern, seal, transfer, or replay context. RuVector recalls analogous context but does not enforce budgets or prove integrity. RVF proves evidence integrity but does not optimize fidelity. The conformal mechanism bounds omission risk but needs the complete pipeline.

## Bounded contexts
1. Intake and policy: typed events, migration requests, sensitivity and clearance.
2. Budget and selection: real token measurement, semantic recall, submodular selection and conformal risk.
3. Capsule exchange: capsule construction, RVF sealing, transfer and verification.
4. Recovery and evidence: deterministic replay, append-only ledger, telemetry and evaluation.

## End-to-end workload
An agent session containing objectives, customer constraints, tool findings, a rejected unsafe action, and pending work must move from a cloud model to an edge runtime with a 140-token context budget and internal-data clearance. Context Ferry validates the request, rejects disallowed secrets, measures cl100k_base tokens, stores and recalls analogous context through RuVector, selects high-value diverse events, computes an upper omission-risk bound, seals the evidence with RVF, transfers and verifies the capsule, and replays the resulting state.

## Success criteria
- The combined path completes and produces a verified, replayable capsule within its token budget.
- Secret material never appears in a destination without secret clearance.
- Removing RuVector reduces analogue recall.
- Removing RVF removes cryptographic evidence or makes verification fail closed.
- Removing tiktoken changes exact token accounting and is recorded as an ablation.
- Removing conformal control removes the explicit omission-risk gate.
- Tampered capsules and over-budget requests fail closed.
- Every library crate has a real workspace caller; executable roots are useful entrypoints.

## Non-goals
Context Ferry does not call paid inference APIs, claim semantic equivalence of different models, or treat a statistical bound as a guarantee of correctness. Deterministic fixtures make the system reproducible, not universally representative.
