# Independent semantic and evidence review

Reviewed implementation source: `ecc5502d37c01c52954e8029c701b32dd0999b19`.

## Meaningful boundaries

- Cargo metadata reports 23 workspace packages and 39 internal dependency edges.
- No package is an orphan. `cf-cli` is the intended executable root; every library has a reverse workspace caller.
- All 23 crate directories contain typed implementation over 100 bytes.
- Twenty-two crates have unit tests; the CLI is exercised through its API dependency and end-to-end command.
- The workspace contains 1,309 Rust lines across 24 source/integration files.
- Public APIs join policy, tokenization, memory, selection, risk, evidence, transfer, verification and replay in one executable call path.

## Source-bound upstream evidence

- OpenAI tiktoken 0.12.0 executed `cl100k_base` and measured exact capsule tokens.
- RuVector Core 2.4.0 executed persistent insert, search and recall.
- RVF Crypto 0.2.0 executed witness creation, verification and tamper rejection.
- Ruflo 3.56.10 initialized a hierarchical topology as provenance with authority none.
- Darwin 0.12.0 created and verified suite hash `f25405ccc249b53f3c52fb6893f611ca2b03d09ece53ade1b0324951b8e956f7`.
- Flywheel 0.1.12 produced a signed promotion and verified replay chain; it remains advisory.

## Failure paths

Tests reject secrets before optimization, zero budgets, token overflow, event reordering, RVF tampering, unknown commands and risk-ceiling violations. Destination replay occurs only after evidence and budget verification.

## Ablations

The same frozen fixture executed combined, no-RuVector, no-RVF, no-tiktoken, no-conformal, enterprise-only and Ruvnet-only modes. Each major ingredient removal has an observable capability loss. The no-tiktoken mode fails closed because its conservative approximate count breaches the risk ceiling.

## Decision

The crates are linked and behavior-bearing rather than decorative. Documentation maps each crate to bounded context, typed API, caller, ADR/DDD and acceptance criteria. Structural pass plus this source/evidence review supports publication, subject to CI, PR, rendered README and Gist readback.
