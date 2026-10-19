# ADR-0013: Append Only Ledger

## Status
Accepted for the 2026-10-18 design baseline.

## Context
Keep an append-only transfer ledger. Context Ferry must preserve a typed, auditable boundary across heterogeneous inference runtimes.

## Viable alternatives
Mutable status row; logs; append-only events. Each was considered against safety, reproducibility, operational complexity and the real migration workload.

## Decision
Ledger only accepts increasing sequence values.

## Tradeoffs
Corrections require compensating events.

## Consequences
The decision becomes part of the public compatibility and evidence contract. Downstream crates reference this ADR rather than duplicating policy.

## Validation
Out-of-order append fails. Raw evidence is bound to the exact candidate source state before promotion.
