# Design-to-implementation traceability

| Requirement | Design | Planned source | Planned evidence |
| --- | --- | --- | --- |
| Policy-filtered migration | ADR-0005, DDD-04 | `cf-policy`, `cf-orchestrator` | secret-denial test |
| Exact enterprise token count | ADR-0007 | `cf-tiktoken` | subprocess receipt/test |
| Semantic analogue recall | ADR-0008 | `cf-memory` | RuVector round-trip |
| Budgeted diverse selection | ADR-0009 | `cf-selection` | budget/diversity tests |
| Statistical omission gate | ADR-0010, DDD-08 | `cf-conformal` | ablation comparison |
| Tamper-evident transfer | ADR-0012/15 | `cf-rvf`, `cf-verifier` | tamper rejection |
| Replay | ADR-0016 | `cf-recovery` | deterministic reconstruction |
| Auditable evaluation | ADR-0019/23/24 | evaluation/Ruflo/MetaHarness crates | raw ledgers and receipts |

Statuses are updated only after source-bound evidence exists.
