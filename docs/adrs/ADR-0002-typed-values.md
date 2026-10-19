# ADR-0002: Typed Values

## Status
Accepted for the 2026-10-18 design baseline.

## Context
Use validated value objects at all public boundaries. Context Ferry must preserve a typed, auditable boundary across heterogeneous inference runtimes.

## Viable alternatives
Strings everywhere; protobuf-first; Rust newtypes. Each was considered against safety, reproducibility, operational complexity and the real migration workload.

## Decision
Rust enums and structs carry sensitivity, destination and event identity.

## Tradeoffs
Compile-time clarity at the cost of conversion code.

## Consequences
The decision becomes part of the public compatibility and evidence contract. Downstream crates reference this ADR rather than duplicating policy.

## Validation
Invalid identifiers and sensitivity transitions are tested. Raw evidence is bound to the exact candidate source state before promotion.
