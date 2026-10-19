# ADR-0016: Deterministic Replay

## Status
Accepted for the 2026-10-18 design baseline.

## Context
Replay decision state deterministically. Context Ferry must preserve a typed, auditable boundary across heterogeneous inference runtimes.

## Viable alternatives
LLM resummary; transcript re-execution; typed replay. Each was considered against safety, reproducibility, operational complexity and the real migration workload.

## Decision
Replay derives objectives, decisions and pending work from selected events.

## Tradeoffs
Non-selected state cannot be recreated.

## Consequences
The decision becomes part of the public compatibility and evidence contract. Downstream crates reference this ADR rather than duplicating policy.

## Validation
Repeated replay is byte-equivalent. Raw evidence is bound to the exact candidate source state before promotion.
