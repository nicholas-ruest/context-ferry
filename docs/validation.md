# Validation

## Candidate

Implementation source: `ecc5502d37c01c52954e8029c701b32dd0999b19`.

Environment:

- `rustc 1.99.0 (b940084d7 2026-09-28)`
- `cargo 1.99.0 (5f94df478 2026-08-27)`
- `cargo-deny 0.20.2`
- `cargo-audit 0.22.2`
- Node `v20.20.2`
- Python `3.12.3`
- OpenAI tiktoken `0.12.0`

## Commands and results

| Gate | Exact command | Result |
| --- | --- | --- |
| Format | `cargo fmt --all --check` | exit 0 |
| Strict lint | `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | exit 0 |
| Locked tests | `PYTHONPATH=.python cargo test --locked --workspace --all-features` | exit 0; 23 passed |
| Dependency direction | `python3 scripts/check_dependency_direction.py` | 23 packages, 39 internal edges, no orphans |
| License/advisory/source | `cargo deny --locked --config deny.toml check all` | exit 0 |
| Vulnerability audit | `cargo audit --file Cargo.lock` | exit 0; no vulnerability, one allowed unmaintained warning |
| End-to-end | `cargo run --locked -p cf-cli --bin context-ferry -- example` | exit 0 |
| Evaluation | `cargo run --locked -p cf-cli --bin context-ferry -- evaluate` | exit 0; seven modes |
| Ruflo | `npx --yes ruflo@3.56.10 swarm init --topology hierarchical --max-agents 4 --strategy specialized` | exit 0 |
| Darwin | `npx --yes @metaharness/darwin@0.12.0 bench verify evidence/raw/darwin-suite.json` | exit 0 |
| Flywheel | signed one-generation replay through `@metaharness/flywheel@0.1.12` | pass; all replay checks |
| Checker tests | `node --test scripts/check-project-contract.test.mjs` at the pinned checker checkout | 3/3 passed |
| Structural checker | `node check-project-contract.mjs /home/ruv/work/context-ferry` | structural pass |

## Structural gate

Checker source commit: `80f006f6960bc4e491e97f51e890ff708e81aa88`.

- Checker SHA-256: `cd72ca58dcc38255ea48acb467293886c52ab8fb677d26c4463fa839b8e31250`
- Regression-test SHA-256: `cd288bc30a76ffb67cc39efc7fb4c61bd439130d1e53a3c06e147cc72adc8afa`
- Result: `STRUCTURAL_PASS_REQUIRES_EVIDENCE_REVIEW`
- Counts: 25 ADRs, 12 DDD documents, 23 crates, two animated SVGs
- Failures: none

## Security waiver

`RUSTSEC-2025-0141` marks bincode 2.0.1 unmaintained, not vulnerable. It is transitively pinned by `ruvector-core 2.4.0`; the advisory reports no safe upgrade. The exact advisory is ignored in `deny.toml` with rationale. Cargo Audit independently reported no vulnerabilities and one allowed unmaintained warning.

## Raw evidence

Command output and machine-readable results are under `evidence/raw/`. The evaluation dataset is a deterministic project fixture; results are not claims about paid model quality or production latency.
