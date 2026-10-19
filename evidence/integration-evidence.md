# Executed integration evidence

| Boundary | Execution | Observable result | Failure control |
| --- | --- | --- | --- |
| OpenAI tiktoken 0.12.0 | `python3` subprocess using `cl100k_base` | exact 36-token selected capsule under a 38-token budget | timeout and non-zero exit fail closed |
| RuVector Core 2.4.0 | `VectorDB` insert/search/readback | recall count 3 in end-to-end workload | adapter returns typed errors |
| RVF Crypto 0.2.0 | witness create/verify | destination accepted untampered capsule; tamper test rejected mutation | verification precedes replay |
| Ruflo 3.56.10 | CLI topology initialization | source-bound swarm receipt | authority none |
| Darwin 0.12.0 | repository-native benchmark suite creation/verification | frozen hash verified | advisory only |
| Flywheel 0.1.12 | signed one-generation policy replay | all replay checks passed | advisory only |

The combined workload filtered one secret before optimization, selected four authorized events, passed conformal release at upper risk 0.5103802281 below the 0.55 ceiling, verified evidence, and replayed one objective, one decision and one pending item.
