# ADR-0019: Evaluation Modes

## Status
Accepted for the 2026-10-18 design baseline.

## Context
Make ingredient ablations executable modes. Context Ferry must preserve a typed, auditable boundary across heterogeneous inference runtimes.

## Viable alternatives
Narrative claims; separate scripts; typed modes. Each was considered against safety, reproducibility, operational complexity and the real migration workload.

## Decision
One evaluator runs combined and six ablations over the same fixture.

## Tradeoffs
Some ablations are intentionally degraded implementations.

## Consequences
The decision becomes part of the public compatibility and evidence contract. Downstream crates reference this ADR rather than duplicating policy.

## Validation
Results serialize and differ on expected dimensions. Raw evidence is bound to the exact candidate source state before promotion.
