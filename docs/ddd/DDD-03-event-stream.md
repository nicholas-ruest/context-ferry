# DDD-03: Context event stream

## Bounded context
This document defines the context event stream boundary in Context Ferry and its relationship to the end-to-end migration workload.

## Ubiquitous language
ContextEvent is an entity; EventId and Sensitivity are value objects; ContextObserved and ContextFiltered are events.

## Aggregates, entities, and value objects
ContextEvent is an entity; EventId and Sensitivity are value objects; ContextObserved and ContextFiltered are events. Aggregate roots own state transitions; entities retain identity; value objects are immutable and validated.

## Invariants
Event identity and order are stable within a migration.

## Domain events
The boundary emits typed events after successful state transitions and never uses log text as the domain contract.

## Ports and adapters
Ports are Rust types owned by the consuming bounded context. Upstream adapters translate tiktoken, RuVector, RVF, Ruflo or MetaHarness output without leaking vendor types into domain crates.

## Context-map relationship
The boundary is connected by a published-language relationship and explicit anti-corruption layer. It cannot expand clearance, budget or promotion authority.

## Acceptance criteria
Focused tests enforce the invariant, and the frozen end-to-end workload exercises the boundary through a real caller.
