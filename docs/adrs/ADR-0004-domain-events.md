# ADR-0004: Domain Events

## Status
Accepted for the 2026-10-18 design baseline.

## Context
Record monotone domain events. Context Ferry must preserve a typed, auditable boundary across heterogeneous inference runtimes.

## Viable alternatives
Log strings; tracing only; typed events. Each was considered against safety, reproducibility, operational complexity and the real migration workload.

## Decision
Typed events have sequence, kind and detail.

## Tradeoffs
Schema evolution must be managed.

## Consequences
The decision becomes part of the public compatibility and evidence contract. Downstream crates reference this ADR rather than duplicating policy.

## Validation
Ledger rejects non-monotone sequences. Raw evidence is bound to the exact candidate source state before promotion.
