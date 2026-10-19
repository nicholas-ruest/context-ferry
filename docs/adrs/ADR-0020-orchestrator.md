# ADR-0020: Orchestrator

## Status
Accepted for the 2026-10-18 design baseline.

## Context
Centralize workflow order in application layer. Context Ferry must preserve a typed, auditable boundary across heterogeneous inference runtimes.

## Viable alternatives
Adapters call each other; event bus; orchestrator. Each was considered against safety, reproducibility, operational complexity and the real migration workload.

## Decision
Ferry orchestrator owns sequencing through typed ports.

## Tradeoffs
A central path is easier to audit but can grow.

## Consequences
The decision becomes part of the public compatibility and evidence contract. Downstream crates reference this ADR rather than duplicating policy.

## Validation
E2E test traverses real adapters. Raw evidence is bound to the exact candidate source state before promotion.
