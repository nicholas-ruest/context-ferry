# Executor ledger

| Stage | Executor | Working directory | Outcome |
| --- | --- | --- | --- |
| Guidance, research, design | Codex via governed RuOS shell | `/home/ruv/work/context-ferry` | Design-first commit `3ce1ead3db198768aff3d3b68fea1ffe4ba8d15d` |
| Implementation | Codex via governed RuOS shell | same isolated checkout | 23 linked Rust crates and vertical slice |
| Validation | Rust 1.99 toolchain on RuOS | same checkout | fmt, strict Clippy, locked tests, security, integration and ablations executed |
| Coordination receipt | Ruflo 3.56.10 | same checkout | hierarchical four-agent topology; authority none |
| Evaluation receipt | Darwin 0.12.0 and Flywheel 0.1.12 | same checkout | frozen suite verified; signed replay verified |

No permission bypass, `dangerously-skip-permissions`, force push, secret extraction or protection change was used. Codex was available, so no executor failover was needed.
