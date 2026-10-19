# ADR-0005: Policy First

## Status
Accepted for the 2026-10-18 design baseline.

## Context
Apply sensitivity policy before optimization. Context Ferry must preserve a typed, auditable boundary across heterogeneous inference runtimes.

## Viable alternatives
Select then redact; destination trust; policy-first. Each was considered against safety, reproducibility, operational complexity and the real migration workload.

## Decision
Policy filters before tokenization, memory and selection.

## Tradeoffs
May exclude useful context but prevents later leakage.

## Consequences
The decision becomes part of the public compatibility and evidence contract. Downstream crates reference this ADR rather than duplicating policy.

## Validation
Secret events are absent under internal clearance. Raw evidence is bound to the exact candidate source state before promotion.
