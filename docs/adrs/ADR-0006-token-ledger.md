# ADR-0006: Token Ledger

## Status
Accepted for the 2026-10-18 design baseline.

## Context
Use a hard token budget ledger. Context Ferry must preserve a typed, auditable boundary across heterogeneous inference runtimes.

## Viable alternatives
Character count; soft warning; hard ledger. Each was considered against safety, reproducibility, operational complexity and the real migration workload.

## Decision
Selection cannot commit an item beyond the remaining budget.

## Tradeoffs
Conservative failures are possible.

## Consequences
The decision becomes part of the public compatibility and evidence contract. Downstream crates reference this ADR rather than duplicating policy.

## Validation
Overflow test fails closed. Raw evidence is bound to the exact candidate source state before promotion.
