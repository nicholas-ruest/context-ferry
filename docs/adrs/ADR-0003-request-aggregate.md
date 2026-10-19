# ADR-0003: Request Aggregate

## Status
Accepted for the 2026-10-18 design baseline.

## Context
Make migration request the intake aggregate. Context Ferry must preserve a typed, auditable boundary across heterogeneous inference runtimes.

## Viable alternatives
Function arguments; mutable session object; immutable aggregate. Each was considered against safety, reproducibility, operational complexity and the real migration workload.

## Decision
An immutable request owns destination clearance and budget.

## Tradeoffs
Changes require a new request, improving auditability.

## Consequences
The decision becomes part of the public compatibility and evidence contract. Downstream crates reference this ADR rather than duplicating policy.

## Validation
Zero budget and empty destination fail. Raw evidence is bound to the exact candidate source state before promotion.
