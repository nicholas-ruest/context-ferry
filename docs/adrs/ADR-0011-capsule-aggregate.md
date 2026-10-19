# ADR-0011: Capsule Aggregate

## Status
Accepted for the 2026-10-18 design baseline.

## Context
Make the capsule an invariant-holding aggregate. Context Ferry must preserve a typed, auditable boundary across heterogeneous inference runtimes.

## Viable alternatives
Raw vector; JSON map; typed capsule. Each was considered against safety, reproducibility, operational complexity and the real migration workload.

## Decision
Capsule binds selection, token usage, coverage and risk.

## Tradeoffs
Serialization becomes a compatibility surface.

## Consequences
The decision becomes part of the public compatibility and evidence contract. Downstream crates reference this ADR rather than duplicating policy.

## Validation
Empty and over-budget capsules fail. Raw evidence is bound to the exact candidate source state before promotion.
