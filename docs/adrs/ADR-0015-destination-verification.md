# ADR-0015: Destination Verification

## Status
Accepted for the 2026-10-18 design baseline.

## Context
Verify before replay. Context Ferry must preserve a typed, auditable boundary across heterogeneous inference runtimes.

## Viable alternatives
Trust source; verify after use; verify before use. Each was considered against safety, reproducibility, operational complexity and the real migration workload.

## Decision
Destination checks clearance, budget and RVF evidence before replay.

## Tradeoffs
Availability yields to safety.

## Consequences
The decision becomes part of the public compatibility and evidence contract. Downstream crates reference this ADR rather than duplicating policy.

## Validation
Tamper and clearance tests fail closed. Raw evidence is bound to the exact candidate source state before promotion.
