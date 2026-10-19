# ADR-0001: System Boundary

## Status
Accepted for the 2026-10-18 design baseline.

## Context
Treat the product as a context exchange system, not a summarizer. Context Ferry must preserve a typed, auditable boundary across heterogeneous inference runtimes.

## Viable alternatives
Transcript copy; free-form summary; typed exchange pipeline. Each was considered against safety, reproducibility, operational complexity and the real migration workload.

## Decision
The typed exchange pipeline separates policy, optimization, evidence and replay.

## Tradeoffs
More components and explicit handoffs; faults become attributable.

## Consequences
The decision becomes part of the public compatibility and evidence contract. Downstream crates reference this ADR rather than duplicating policy.

## Validation
E2E workload crosses every boundary. Raw evidence is bound to the exact candidate source state before promotion.
