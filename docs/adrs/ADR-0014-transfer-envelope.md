# ADR-0014: Transfer Envelope

## Status
Accepted for the 2026-10-18 design baseline.

## Context
Separate capsule from transport envelope. Context Ferry must preserve a typed, auditable boundary across heterogeneous inference runtimes.

## Viable alternatives
Serialize capsule alone; shared DB; envelope. Each was considered against safety, reproducibility, operational complexity and the real migration workload.

## Decision
Envelope binds destination, capsule and witness material.

## Tradeoffs
Adds versioning responsibility.

## Consequences
The decision becomes part of the public compatibility and evidence contract. Downstream crates reference this ADR rather than duplicating policy.

## Validation
Destination mismatch fails. Raw evidence is bound to the exact candidate source state before promotion.
