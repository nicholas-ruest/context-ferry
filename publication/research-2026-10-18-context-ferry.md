# Dream Machine research - Context Ferry - 2026-10-18

## Selected hypothesis

A policy-filtered context exchange can preserve decision continuity across model, provider and edge-runtime boundaries with less data than transcript transfer, while making token cost, omission risk and evidence integrity independently testable.

The system authorizes events before optimization, uses official target-token measurement, recalls related context through RuVector, selects a diverse capsule under budget, blocks statistically unsafe omission, seals the transfer with RVF and verifies before deterministic replay.

## Material distinction

- Provenance Firebreak contains compromised software artifacts.
- Continuity Atlas plans regional service failover.
- ToolRecall evaluates tool memory.
- JitterMap controls retry amplification.
- Context Ferry transports agent decision state under privacy, fidelity and token constraints.

## Candidate comparison

| Candidate | System-scale value | Decision |
| --- | --- | --- |
| Context Ferry | Cross-runtime decision continuity with privacy, fidelity and evidence | selected |
| Transcript compressor | Mostly converts text; weak orchestration need | rejected as utility-scale |
| Provider prompt router | Primarily wraps APIs | rejected as adapter-scale |
| Rollback planner | Strong but overlaps recent continuity/provenance work | rejected as near-duplicate |

## Executed composition

| Ingredient | Pin | Reused capability | Boundary | Ablation |
| --- | --- | --- | --- | --- |
| RuVector | source `46d816ddc4c2a8a06cd3613534a79b1d557d858b`; crate 2.4.0 | persistent recall | `VectorDB` insert/search | no-RuVector |
| RVF Crypto | same source; crate 0.2.0 | witness sealing | Rust dependency | no-RVF |
| Ruflo | `b93144803bb171b12993b715fe7b2dd936b6edd5`; CLI 3.56.10 | provenance | CLI receipt | authority none |
| OpenAI tiktoken | `4e71bbe0c078468e00fefbf94b39849389f346e5`; 0.12.0 | exact target tokens | Python subprocess | no-tiktoken |
| MetaHarness | `e0dfd44da72b7adfb7c57fdd42d124be58ed7086`; Darwin 0.12.0, Flywheel 0.1.12 | advisory evaluation | CLI/library | authority none |
| Split conformal risk | Angelopoulos and Bates 2023 | omission bound | native Rust | no-conformal |

All cited software is MIT or Apache-2.0.

## Frontier mechanism

The release gate calibrates omission losses and computes a finite-sample split-conformal upper quantile. A capsule is released only when that bound is below the ceiling. Selection therefore does not imply release, and conservative token estimation can fail closed.

Citation: Angelopoulos and Bates, "A Gentle Introduction to Conformal Prediction and Distribution-Free Uncertainty Quantification," 2023, DOI 10.48550/arXiv.2107.07511.

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

The objective is fixture-specific, not a production model-quality claim.

## Primary sources

- https://github.com/ruvnet/ruvector/tree/46d816ddc4c2a8a06cd3613534a79b1d557d858b
- https://github.com/ruvnet/ruflo/tree/b93144803bb171b12993b715fe7b2dd936b6edd5
- https://github.com/ruvnet/agent-harness-generator/tree/e0dfd44da72b7adfb7c57fdd42d124be58ed7086
- https://github.com/openai/tiktoken/tree/4e71bbe0c078468e00fefbf94b39849389f346e5
- https://doi.org/10.48550/arXiv.2107.07511

## Limitations

The transparent embedding is reproducible but not a production model. Conformal validity depends on exchangeability. RVF proves integrity and order, not truth. The benchmark is a deterministic fixture. MetaHarness and Ruflo have advisory authority only.
