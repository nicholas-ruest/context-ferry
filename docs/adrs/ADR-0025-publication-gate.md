# ADR-0025: Publication Gate

## Status
Accepted for the 2026-10-18 design baseline.

## Context
Require exact-main readback before BUILT. Context Ferry must preserve a typed, auditable boundary across heterogeneous inference runtimes.

## Viable alternatives
Publish after local tests; checker only; full gate. Each was considered against safety, reproducibility, operational complexity and the real migration workload.

## Decision
Release requires structural plus semantic review, exact-main CI, repo and Gist readback.

## Tradeoffs
Completion takes longer and is evidence-heavy.

## Consequences
The decision becomes part of the public compatibility and evidence contract. Downstream crates reference this ADR rather than duplicating policy.

## Validation
Final report names exact SHA and receipts. Raw evidence is bound to the exact candidate source state before promotion.
