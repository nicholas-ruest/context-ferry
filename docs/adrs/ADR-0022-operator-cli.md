# ADR-0022: Operator Cli

## Status
Accepted for the 2026-10-18 design baseline.

## Context
Provide an executable example and evaluation CLI. Context Ferry must preserve a typed, auditable boundary across heterogeneous inference runtimes.

## Viable alternatives
Library only; interactive TUI; deterministic CLI. Each was considered against safety, reproducibility, operational complexity and the real migration workload.

## Decision
CLI offers example and evaluate commands with JSON output.

## Tradeoffs
Surface is intentionally narrow.

## Consequences
The decision becomes part of the public compatibility and evidence contract. Downstream crates reference this ADR rather than duplicating policy.

## Validation
Both commands run in CI. Raw evidence is bound to the exact candidate source state before promotion.
