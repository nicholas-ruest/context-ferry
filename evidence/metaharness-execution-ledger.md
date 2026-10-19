# MetaHarness execution ledger

## Darwin

- Package: `@metaharness/darwin@0.12.0`
- Command: `bench create /home/ruv/work/context-ferry --out evidence/raw/darwin-suite.json`
- Observed suite: one repo-native task
- Frozen suite hash: `f25405ccc249...`
- Verification: `bench verify evidence/raw/darwin-suite.json`, exit 0
- Authority: advisory only

## Flywheel

- Package: `@metaharness/flywheel@0.1.12`
- Workload: one frozen generation comparing the enterprise-only policy with the combined Context Ferry policy
- Promotion: one signed promotion to `combined`
- Replay: passed all eight checks, including receipt verification, immutable root reachability, parent continuity, frozen gate re-execution and sealed-field authenticity
- Chain: `gen1(mode) -> gen0(root)`
- Authority: advisory only; product claims remain bound to Rust evaluation evidence
- Raw receipt: `evidence/raw/flywheel-replay.json`
