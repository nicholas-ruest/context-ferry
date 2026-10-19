# Dream Machine build - Context Ferry - 2026-10-18

Repository: https://github.com/nicholas-ruest/context-ferry
Implementation PR: https://github.com/nicholas-ruest/context-ferry/pull/1
Merged main: d4e578050d51a614d18d37c91e71282d88954ae4
Research Gist: https://gist.github.com/nicholas-ruest/6ced226e3516f9e9e834f058a039c7ed

## Capability

Context Ferry transports decision-relevant agent state across model, provider and edge-runtime boundaries without copying the full transcript or trusting an unverifiable summary.

The end-to-end path is: request -> policy -> exact token measurement -> RuVector recall -> diverse selection -> split-conformal release gate -> capsule -> RVF seal -> transfer -> destination verification -> deterministic replay -> evidence.

## Implemented architecture

- 23 linked Rust workspace crates
- 39 internal dependency edges; no orphan libraries
- 25 substantive ADRs and 12 DDD documents
- 23 locked tests
- two project-specific animated SVGs
- typed errors, fail-closed policy, bounded subprocess tokenization, append-only ledger and source-bound raw evidence

## Executed ingredients

- RuVector Core 2.4.0 for persistent analogue recall
- RVF Crypto 0.2.0 for witness sealing and tamper rejection
- Ruflo 3.56.10 for coordination provenance with authority none
- OpenAI tiktoken 0.12.0 for exact cl100k_base token measurement
- MetaHarness Darwin 0.12.0 and Flywheel 0.1.12 for advisory suite and signed replay evidence
- split-conformal omission-risk bounds based on Angelopoulos and Bates 2023

## Reproducible validation

```text
cargo fmt --all --check
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
PYTHONPATH=.python cargo test --locked --workspace --all-features
cargo deny --locked --config deny.toml check all
cargo audit --file Cargo.lock
python3 scripts/check_dependency_direction.py
cargo run --locked -p cf-cli --bin context-ferry -- example
cargo run --locked -p cf-cli --bin context-ferry -- evaluate
```

All passed. Cargo Audit found no vulnerability and one allowed unmaintained warning: bincode 2.0.1 is transitively pinned by RuVector; RUSTSEC-2025-0141 reports no safe upgrade and is documented in deny.toml.

The pinned factory checker at commit 80f006f6960bc4e491e97f51e890ff708e81aa88 passed its 3 tests and returned STRUCTURAL_PASS_REQUIRES_EVIDENCE_REVIEW with 25 ADRs, 12 DDDs, 23 crates and two animated SVGs. Independent semantic review passed.

## Frozen evaluation

| Mode | Released | Recall | Evidence | Robust | Objective |
| --- | --- | --- | --- | --- | ---: |
| combined | yes | yes | yes | yes | 37.0 |
| no RuVector | yes | no | yes | yes | 37.0 |
| no RVF | yes | yes | no | yes | 36.0 |
| no tiktoken | no | no | no | no release | 0.0 |
| no conformal | yes | yes | yes | no | 36.0 |
| enterprise only | yes | no | no | no | 35.0 |
| Ruvnet only | yes | yes | yes | no | 18.3 |

Removing tiktoken caused the conservative count to breach the risk ceiling and fail closed. Removing RuVector removed recall, RVF removed verifiable evidence, and conformal removal removed robust release assurance.

## Limitations

The embedding is transparent and reproducible, not production-grade. Conformal validity depends on exchangeability. RVF establishes integrity, not truth. The objective is a deterministic fixture score, not a production model-quality or latency claim. Ruflo and MetaHarness have advisory authority only.
