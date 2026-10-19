# ADR-0010: Conformal Gate

## Status
Accepted for the 2026-10-18 design baseline.

## Context
Gate release with split-conformal omission risk. Context Ferry must preserve a typed, auditable boundary across heterogeneous inference runtimes.

## Viable alternatives
Mean loss; fixed threshold; conformal quantile. Each was considered against safety, reproducibility, operational complexity and the real migration workload.

## Decision
Use finite-sample upper quantile over calibration residuals.

## Tradeoffs
Exchangeability is a documented assumption.

## Consequences
The decision becomes part of the public compatibility and evidence contract. Downstream crates reference this ADR rather than duplicating policy.

## Validation
No-conformal ablation removes the gate. Raw evidence is bound to the exact candidate source state before promotion.
