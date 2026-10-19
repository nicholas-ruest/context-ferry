# Integration matrix

| Ingredient | Revision/version/license | Reused capability | Executed boundary | Crate | Test | Ablation |
| --- | --- | --- | --- | --- | --- | --- |
| RuVector | `46d816d…`, core 2.4.0, Apache-2.0 | Persistent vector recall | Rust dependency; VectorDB insert/search | `cf-memory` | persistent recall round-trip | `no-ruvector` |
| RVF Crypto | 0.2.0, Apache-2.0 | Witness-chain evidence | Rust dependency; create/verify/tamper reject | `cf-rvf`, `cf-verifier` | tamper rejection | `no-rvf` |
| OpenAI tiktoken | `4e71bbe…`, 0.12.0, MIT | `cl100k_base` token count | bounded Python subprocess | `cf-tiktoken` | known text count and E2E | `no-tiktoken` |
| Ruflo | `b931448…`, 3.56.10, MIT | coordination/provenance | CLI swarm receipt | `cf-ruflo` + evidence | version/swarm receipt | not a product ablation |
| MetaHarness | `e0dfd44…`, Darwin/Flywheel | candidate suite and signed replay | CLI/package execution | `cf-metaharness` + evidence | suite/replay receipt | advisory only |
| Conformal risk control | Angelopoulos/Bates 2023 | distribution-free omission-risk upper bound | native Rust algorithm | `cf-conformal` | quantile/gate tests | `no-conformal` |

Every mandatory ingredient is exercised in the end-to-end workflow or its source-bound evaluation. Logos and citations are not counted as integration.
