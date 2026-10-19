# Implementation contract

## Stage order
This document, specification, architecture, integration matrix, ADR set, DDD set and guidance map are committed before Rust implementation. Code begins only after the design checkpoint exists.

## Required vertical slice
The `context-ferry example` command must process the frozen migration workload through policy, real tiktoken, RuVector persistence/recall, diverse selection, conformal gating, RVF sealing, destination verification and replay. It must emit machine-readable evidence.

## Engineering constraints
- Rust 1.85+ and a locked workspace.
- Typed public APIs and explicit errors at every boundary.
- No network calls or paid providers in the product path.
- Bounded subprocess execution for tiktoken; deterministic stdin and output parsing.
- Safe paths created by callers; no service-owned database beyond the supplied RuVector directory.
- No secrets or `.env` files committed.
- Every crate is a real workspace member with tests or exercised entrypoint.

## Validation contract
Run `cargo fmt --check`, strict workspace Clippy, locked tests, dependency/license/security checks, dependency-direction inspection, real integration/failure tests, the frozen evaluation and ingredient ablations. Bind raw outputs to an exact git state. The structural checker is necessary but not sufficient; a semantic caller-path and evidence review follows.

## Acceptance criteria
At least 20 meaningful implemented crates, 25 substantive ADRs, 12 substantive DDD documents, two animated SVGs, real executions of two Ruvnet components and tiktoken, the research mechanism in the real workflow, public repository and PR readback, exact-main CI, and two owner-read-back Gists.
