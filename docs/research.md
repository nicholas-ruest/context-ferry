# Research and source dossier

## Candidate comparison
| Candidate | Distinct system value | 20-crate justification | Decision |
| --- | --- | --- | --- |
| Context Ferry | Verifiable, budgeted agent-state migration across inference boundaries | Intake, policy, tokenization, memory, selection, risk, exchange, replay and evidence are independently testable boundaries | Selected |
| Model cost dashboard | Observability only; utility-scale | Weak | Rejected |
| Prompt validator | Mostly checks input | Weak | Rejected |

## Pinned ingredients
- OpenAI tiktoken, `openai/tiktoken@4e71bbe0c078468e00fefbf94b39849389f346e5`, package 0.12.0, MIT. Executed through a bounded Python subprocess using `cl100k_base`.
- RuVector, `ruvnet/ruvector@46d816ddc4c2a8a06cd3613534a79b1d557d858b`; `ruvector-core` 2.4.0, Apache-2.0. Executed through `VectorDB` insert/search/readback.
- RVF Crypto 0.2.0 from the RuVector line, Apache-2.0. Executed through witness-chain creation and verification.
- Ruflo 3.56.10, upstream `ruvnet/ruflo@b93144803bb171b12993b715fe7b2dd936b6edd5`, MIT. Used for coordination/provenance receipts only, never as implementation evidence.
- MetaHarness source `ruvnet/metaharness@e0dfd44da72b7adfb7c57fdd42d124be58ed7086`; Darwin and Flywheel are advisory evaluation/replay tools with authority `none`.

## Frontier mechanism
The system implements distribution-free split-conformal upper bounds over calibrated omission loss. The operational gate uses the finite calibration set to choose an upper quantile and adds it to observed capsule omission loss. This converts compression uncertainty into an explicit release decision. It is paired with monotone submodular selection under a token knapsack constraint so diversity, not transcript recency alone, drives the capsule.

Primary basis: Angelopoulos and Bates, *A Gentle Introduction to Conformal Prediction and Distribution-Free Uncertainty Quantification* (2023), DOI 10.48550/arXiv.2107.07511. The implementation is deliberately small and auditable; the limitations section avoids claiming exchangeability in arbitrary production traffic.
