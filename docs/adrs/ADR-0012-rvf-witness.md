# ADR-0012: Rvf Witness

## Status
Accepted for the 2026-10-18 design baseline.

## Context
Seal capsule events with RVF witness chains. Context Ferry must preserve a typed, auditable boundary across heterogeneous inference runtimes.

## Viable alternatives
SHA file; HMAC service; RVF. Each was considered against safety, reproducibility, operational complexity and the real migration workload.

## Decision
Use rvf-crypto witness chains and verify before replay.

## Tradeoffs
Evidence proves integrity, not semantic truth.

## Consequences
The decision becomes part of the public compatibility and evidence contract. Downstream crates reference this ADR rather than duplicating policy.

## Validation
Byte tampering is rejected. Raw evidence is bound to the exact candidate source state before promotion.
