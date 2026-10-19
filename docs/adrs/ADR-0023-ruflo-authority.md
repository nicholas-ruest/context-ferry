# ADR-0023: Ruflo Authority

## Status
Accepted for the 2026-10-18 design baseline.

## Context
Use Ruflo for coordination provenance only. Context Ferry must preserve a typed, auditable boundary across heterogeneous inference runtimes.

## Viable alternatives
Ruflo decides release; no coordination; authority none. Each was considered against safety, reproducibility, operational complexity and the real migration workload.

## Decision
Ruflo receipts record topology and command with authority none.

## Tradeoffs
Cannot substitute for implementation or tests.

## Consequences
The decision becomes part of the public compatibility and evidence contract. Downstream crates reference this ADR rather than duplicating policy.

## Validation
Version/swarm execution receipt is stored. Raw evidence is bound to the exact candidate source state before promotion.
