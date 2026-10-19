# DDD-11: Recovery bounded context

## Bounded context
This document defines the recovery bounded context boundary in Context Ferry and its relationship to the end-to-end migration workload.

## Ubiquitous language
ReplayState is the aggregate; Decision and PendingWork are entities; ReplayCompleted is an event.

## Aggregates, entities, and value objects
ReplayState is the aggregate; Decision and PendingWork are entities; ReplayCompleted is an event. Aggregate roots own state transitions; entities retain identity; value objects are immutable and validated.

## Invariants
Replay uses only verified selected context and is deterministic.

## Domain events
The boundary emits typed events after successful state transitions and never uses log text as the domain contract.

## Ports and adapters
Ports are Rust types owned by the consuming bounded context. Upstream adapters translate tiktoken, RuVector, RVF, Ruflo or MetaHarness output without leaking vendor types into domain crates.

## Context-map relationship
The boundary is connected by a published-language relationship and explicit anti-corruption layer. It cannot expand clearance, budget or promotion authority.

## Acceptance criteria
Focused tests enforce the invariant, and the frozen end-to-end workload exercises the boundary through a real caller.
