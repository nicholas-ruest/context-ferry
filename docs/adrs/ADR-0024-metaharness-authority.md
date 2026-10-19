# ADR-0024: Metaharness Authority

## Status
Accepted for the 2026-10-18 design baseline.

## Context
Keep Darwin/Flywheel advisory. Context Ferry must preserve a typed, auditable boundary across heterogeneous inference runtimes.

## Viable alternatives
Automatic promotion; ignore tools; advisory receipts. Each was considered against safety, reproducibility, operational complexity and the real migration workload.

## Decision
Evaluation receipts explicitly encode authority none.

## Tradeoffs
Human/governed release remains required.

## Consequences
The decision becomes part of the public compatibility and evidence contract. Downstream crates reference this ADR rather than duplicating policy.

## Validation
Replay and suite hashes are recorded. Raw evidence is bound to the exact candidate source state before promotion.
