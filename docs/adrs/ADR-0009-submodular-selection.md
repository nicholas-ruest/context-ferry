# ADR-0009: Submodular Selection

## Status
Accepted for the 2026-10-18 design baseline.

## Context
Select diverse utility under a knapsack budget. Context Ferry must preserve a typed, auditable boundary across heterogeneous inference runtimes.

## Viable alternatives
Newest-first; importance sort; diverse greedy. Each was considered against safety, reproducibility, operational complexity and the real migration workload.

## Decision
Greedy marginal utility covers event kinds while respecting exact tokens.

## Tradeoffs
Approximate optimization favors auditability.

## Consequences
The decision becomes part of the public compatibility and evidence contract. Downstream crates reference this ADR rather than duplicating policy.

## Validation
Diversity and budget invariants are tested. Raw evidence is bound to the exact candidate source state before promotion.
