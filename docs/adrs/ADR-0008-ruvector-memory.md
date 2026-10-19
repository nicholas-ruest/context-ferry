# ADR-0008: Ruvector Memory

## Status
Accepted for the 2026-10-18 design baseline.

## Context
Use RuVector for analogue memory. Context Ferry must preserve a typed, auditable boundary across heterogeneous inference runtimes.

## Viable alternatives
In-memory scan; SQLite shadow store; RuVector. Each was considered against safety, reproducibility, operational complexity and the real migration workload.

## Decision
Persist compact event vectors and query analogues with VectorDB.

## Tradeoffs
Embedding is deliberately transparent and small.

## Consequences
The decision becomes part of the public compatibility and evidence contract. Downstream crates reference this ADR rather than duplicating policy.

## Validation
Insert/search/readback is executed. Raw evidence is bound to the exact candidate source state before promotion.
