# ADR-0021: Json Api

## Status
Accepted for the 2026-10-18 design baseline.

## Context
Offer a narrow JSON facade. Context Ferry must preserve a typed, auditable boundary across heterogeneous inference runtimes.

## Viable alternatives
HTTP server now; library only; JSON facade. Each was considered against safety, reproducibility, operational complexity and the real migration workload.

## Decision
A typed JSON facade supports integration without a network daemon.

## Tradeoffs
No remote transport is supplied.

## Consequences
The decision becomes part of the public compatibility and evidence contract. Downstream crates reference this ADR rather than duplicating policy.

## Validation
Malformed JSON produces explicit error. Raw evidence is bound to the exact candidate source state before promotion.
