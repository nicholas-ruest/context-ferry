# ADR-0007: Tiktoken Boundary

## Status
Accepted for the 2026-10-18 design baseline.

## Context
Execute official OpenAI tiktoken in a bounded sidecar. Context Ferry must preserve a typed, auditable boundary across heterogeneous inference runtimes.

## Viable alternatives
Approximation; third-party Rust port; official Python package. Each was considered against safety, reproducibility, operational complexity and the real migration workload.

## Decision
Pin tiktoken and invoke cl100k_base through a bounded subprocess.

## Tradeoffs
Python is an operational dependency, isolated behind a port.

## Consequences
The decision becomes part of the public compatibility and evidence contract. Downstream crates reference this ADR rather than duplicating policy.

## Validation
Known strings and full workload execute. Raw evidence is bound to the exact candidate source state before promotion.
