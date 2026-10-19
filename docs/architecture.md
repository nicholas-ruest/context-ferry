# Context Ferry architecture

## Architectural thesis
The system is a policy-enforced context exchange, not a summarizer. Selection, statistical risk, evidence integrity and replay are separate bounded contexts joined by typed ports. The source runtime cannot claim successful transfer until the destination verifier accepts the RVF-bound capsule and replay reconstructs required state.

## End-to-end flow
`request -> policy -> tiktoken measurement -> RuVector memory/recall -> diverse selection -> conformal gate -> capsule -> RVF seal -> transfer -> verification -> replay -> evidence`

## Workspace map
| Crate | Responsibility | Context | Typed API | Caller/entrypoint | References | Acceptance |
| --- | --- | --- | --- | --- | --- | --- |
| cf-types | Shared value objects and event schema | all domain contexts | ContextEvent, Sensitivity, Destination | all libraries | ADR-0002; DDD-01 | validation and serde round-trip |
| cf-request | Migration request aggregate | Intake | MigrationRequest::new | cf-policy, cf-orchestrator | ADR-0003; DDD-02 | invalid budget/clearance rejection |
| cf-events | Domain-event vocabulary | Evidence | DomainEvent, EventKind | memory, ledger, telemetry, rvf | ADR-0004; DDD-03 | monotone sequence |
| cf-policy | Residency and sensitivity policy | Policy | authorize | cf-orchestrator | ADR-0005; DDD-04 | secret filtering |
| cf-token-budget | Token budget accounting | Budget | BudgetLedger | tiktoken, orchestrator | ADR-0006; DDD-05 | overflow rejection |
| cf-tiktoken | OpenAI tiktoken adapter | Budget | TiktokenCounter::count | orchestrator | ADR-0007; DDD-05 | real cl100k_base subprocess |
| cf-memory | RuVector semantic memory | Recall | ContextMemory::remember/recall | orchestrator | ADR-0008; DDD-06 | persistent insert/search |
| cf-selection | Diverse knapsack selector | Selection | select | capsule, conformal, evaluation | ADR-0009; DDD-07 | budget and diversity |
| cf-conformal | Omission-risk upper bound | Risk | upper_bound, permits | orchestrator, evaluation | ADR-0010; DDD-08 | finite-sample quantile |
| cf-capsule | Capsule aggregate | Exchange | ContextCapsule::build | transfer, recovery, evaluation | ADR-0011; DDD-09 | aggregate invariants |
| cf-rvf | RVF witness adapter | Evidence | seal, verify | transfer, verifier | ADR-0012; DDD-10 | tamper rejection |
| cf-ledger | Append-only audit ledger | Evidence | Ledger::append | orchestrator | ADR-0013; DDD-10 | sequence enforcement |
| cf-transfer | Transfer envelope | Exchange | TransferEnvelope::new | verifier | ADR-0014; DDD-09 | payload/seal binding |
| cf-verifier | Destination verification | Exchange | verify_envelope | orchestrator | ADR-0015; DDD-09 | fail closed |
| cf-recovery | Deterministic replay | Recovery | replay | orchestrator | ADR-0016; DDD-11 | state reconstruction |
| cf-telemetry | Runtime metrics | Operations | Telemetry | orchestrator, evaluation | ADR-0017; DDD-12 | counter invariants |
| cf-config | Environment configuration | Operations | Config::from_env | cli, orchestrator | ADR-0018; DDD-12 | bounds/defaults |
| cf-evaluation | Baselines and ablations | Evaluation | evaluate_modes | api, metaharness | ADR-0019; DDD-12 | seven mode comparison |
| cf-orchestrator | Application workflow | Application | Ferry::execute | api, cli | ADR-0020; DDD-02 | full vertical slice |
| cf-api | JSON request/response facade | Interface | run_json | cli | ADR-0021; DDD-02 | boundary validation |
| cf-cli | Operator executable | Interface | context-ferry binary | operator | ADR-0022; DDD-02 | example workload |
| cf-ruflo | Coordination receipt model | Evidence | CoordinationReceipt | orchestrator | ADR-0023; DDD-10 | authority none |
| cf-metaharness | Advisory evaluation receipt | Evaluation | EvaluationReceipt | api | ADR-0024; DDD-12 | promotion authority none |

## Dependency rules
- Domain value crates cannot depend on adapters or interfaces.
- Application orchestration owns sequencing but not adapter internals.
- Upstream adapters expose typed errors and bounded calls.
- Evidence is append-only; evaluators and coordination systems have authority `none`.
- `cf-cli` and `cf-api` are the only interface roots. All other crates have reverse workspace callers.

## Failure model
Invalid requests, policy denial, tokenizer failure, memory failure, conformal rejection, witness failure, budget overflow and replay inconsistency are distinct typed failures. No fallback silently widens clearance, budget, authority, or release criteria.
