# ADR-0018: Runtime Config

## Status
Accepted for the 2026-10-18 design baseline.

## Context
Use bounded environment configuration. Context Ferry must preserve a typed, auditable boundary across heterogeneous inference runtimes.

## Viable alternatives
Config file secrets; constants; validated env. Each was considered against safety, reproducibility, operational complexity and the real migration workload.

## Decision
Environment supplies paths and limits with safe defaults and bounds.

## Tradeoffs
Startup can fail on invalid values.

## Consequences
The decision becomes part of the public compatibility and evidence contract. Downstream crates reference this ADR rather than duplicating policy.

## Validation
Invalid bounds tested. Raw evidence is bound to the exact candidate source state before promotion.
