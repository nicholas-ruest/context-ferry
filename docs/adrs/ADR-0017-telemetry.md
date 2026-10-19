# ADR-0017: Telemetry

## Status
Accepted for the 2026-10-18 design baseline.

## Context
Expose domain metrics without sensitive payloads. Context Ferry must preserve a typed, auditable boundary across heterogeneous inference runtimes.

## Viable alternatives
Raw event logs; no metrics; counters. Each was considered against safety, reproducibility, operational complexity and the real migration workload.

## Decision
Record counts, tokens, recall and verification flags only.

## Tradeoffs
Lower diagnostic detail protects context.

## Consequences
The decision becomes part of the public compatibility and evidence contract. Downstream crates reference this ADR rather than duplicating policy.

## Validation
Metrics match workflow transitions. Raw evidence is bound to the exact candidate source state before promotion.
