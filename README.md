# Daoji

**Software development controlled by explicit engineering documents.**

This repository defines Daoji, a tool that coordinates AI agents through
explicit, versioned, and verifiable engineering documents. These documents make
project decisions persistent and reviewable. The project stores these decisions
outside temporary prompts.

The name *Daoji* comes from the Chinese term 道纪 (traditional: 道紀; pinyin:
*dào jì*) in [chapter 14 of the *Dao De Jing*](https://ctext.org/dao-de-jing/zh#n11605):
"执古之道，以御今之有。能知古始，是谓道纪。" In this project, *Daoji*
refers to the guiding order and traceable thread that connect the origin of a
decision to present work.

Daoji is designed not to replace the AI agents that produce code. It is designed
to provide the design, constraints, order of work, and acceptance criteria that
control their work.

## Development model

The development process has the following stages:

1. Intent
2. Requirements
3. Design
4. Engineering policies
5. Implementation plan
6. Verification plan
7. AI-generated code
8. Implementation record
9. Build and verification
10. Verification record
11. Change summary
12. Runnable software

An engineering asset is a document, software item, or record that the project
creates or maintains.

Each stage produces engineering assets that the project stores in version
control. Each later asset must link to the decisions that authorize it and the
constraints that apply to it. Verification evidence must show whether the
implementation satisfies those constraints.

A prompt or issue starts the process but does not become an authoritative
decision by itself. The project first converts the request into accepted intent
and requirements. A recorded design assessment determines whether the change
needs a separate design document or can keep a short design record in the
implementation plan. Before implementation, the project also checks which
engineering policies apply, identifies missing policies, and creates a
verification plan. Implementation tasks link to the specific references needed
for their scope. Static checks, tests, and reviews produce a verification record
with an explicit test conclusion. The
[Daoji development lifecycle standard](docs/standards/DEVELOPMENT_LIFECYCLE_STANDARD.md)
defines the complete process and the exit condition for each stage.

## Core thesis

AI agents can implement tasks with a limited scope. AI agents cannot reliably
retain project context for long periods. Large software systems need an
authoritative project record. This record controls decisions when information
sources disagree. People must be able to read this record. Agents must be able
to interpret it. Tools must be able to check it. The project must update the
record when the software changes.

Daoji uses the repository as its authoritative project record. Daoji is
designed to provide the following capabilities:

- Capture intent and convert it into reviewable requirements.
- Record links from requirements to design, plans, code, and tests.
- Apply project-specific engineering policies before and during code generation.
- Assign work with a defined scope to one or more AI agents.
- Define how the project tests and verifies each change before implementation.
- Record how the project implemented each change.
- Collect build and verification evidence.
- Create a change summary that is suitable for direct use as a pull request
  description.
- Detect inconsistencies between documented decisions and operating software.
- Require changes to update the relevant documents, not only the code.

## Current status

Daoji is beginning its Level 1 implementation. The repository contains a
minimal Rust command-line scaffold and the authoritative documents that permit
and constrain it. The first functional document validator remains future work.

The [Daoji project charter](docs/PROJECT_CHARTER.md) contains the initial
project requirements and highest-priority rules. The
[documentation standard](docs/standards/DOCUMENTATION_STANDARD.md) defines how project
documents are structured, limited, linked, indexed, and reviewed. The
[development lifecycle standard](docs/standards/DEVELOPMENT_LIFECYCLE_STANDARD.md) defines
how the project applies those rules to a software change.

Rust is the accepted language for the initial core model, validation,
coordination, and command-line interface. The project has not selected an
architecture or toolchain for unspecified future components. See the
[initial Rust architecture decision](docs/design/0001-initial-rust-architecture.md).

## Daoji self-development goal

Daoji should gradually use its own workflow to plan, implement, and verify
changes to Daoji. The initial documents define the first executable version.
That version validates and coordinates later changes. The project
achieves this goal only when the workflow requires review by a person and
records each decision so that a person can audit it later.

## Roadmap

This roadmap is informative and records the intended delivery order. It does
not replace accepted requirements, designs, policies, or plans. Each item must
enter the [Daoji development lifecycle](docs/standards/DEVELOPMENT_LIFECYCLE_STANDARD.md)
before implementation. The project can revise later items when evidence from an
earlier item changes their requirements or design.

- [ ] **Build the authoritative project-record core.** Define the engineering
  asset schema, identifiers, lifecycle states, typed links, precedence rules,
  and evidence model. Implement deterministic validation for document
  structure, metadata, links, state transitions, policy rules, and traceability.
  Completion means that later Daoji changes can require this validator as a
  quality gate.
- [ ] **Complete the document lifecycle in a local TUI.** Build a text user
  interface (TUI) for project-resident documents. Support focused editing,
  document-graph navigation, validation feedback, status views, human approval
  gates, and recovery from interrupted work. Guide a change from intent through
  accepted implementation and verification plans, then through records and the
  final change summary. This stage prepares and reviews work but does not yet
  invoke an agent automatically. Completion means that Daoji can use the TUI to
  define its own next executable change.
- [ ] **Connect external project documents to the TUI.** Add explicit adapters
  for external issues, specifications, decision records, and evidence. Preserve
  source identity and revisions, show synchronization and conflict state, and
  require a person to resolve authority conflicts. External sources remain
  inputs or navigation aids unless an accepted design and policy makes them
  part of the authoritative project record. Completion means that an external
  request can enter the local lifecycle without losing provenance or approval.
- [ ] **Coordinate agent work and achieve routine self-development.** Convert
  accepted plans into provider-neutral, scoped agent tasks with minimum
  sufficient context, permissions, constraints, outputs, and completion
  criteria. Define the common agent adapter contract for capability discovery,
  invocation, context delivery, permission mapping, events, cancellation,
  recovery, and normalized work results. Add tool and process isolation,
  single-agent and multi-agent coordination, verification execution, evidence
  retention, and recovery from incomplete work. Keep human approval at
  important decision points and require independent verification of Daoji.
  Completion means that routine Daoji changes use Daoji from intent through an
  auditable change summary.
- [ ] **Integrate the `pi` agent.** Map `pi` capabilities, sessions,
  permissions, tool events, cancellation, errors, and work results to the Daoji
  adapter contract. Add conformance and end-to-end tests with representative
  scoped tasks. Completion means that Daoji can execute and audit an accepted
  task through `pi`.
- [ ] **Integrate Claude Code.** Map Claude Code authentication, capabilities,
  sessions, permissions, tool events, cancellation, errors, and work results to
  the Daoji adapter contract. Add conformance and end-to-end tests with
  representative scoped tasks. Completion means that Daoji can execute and
  audit an accepted task through Claude Code.
- [ ] **Integrate Codex CLI.** Map Codex CLI authentication, capabilities,
  sessions, permissions, tool events, cancellation, errors, and work results to
  the Daoji adapter contract. Add conformance and end-to-end tests with
  representative scoped tasks. Completion means that Daoji can execute and
  audit an accepted task through Codex CLI.
- [ ] **Integrate Kimi Code.** Map Kimi Code authentication, capabilities,
  sessions, permissions, tool events, cancellation, errors, and work results to
  the Daoji adapter contract. Add conformance and end-to-end tests with
  representative scoped tasks. Completion means that Daoji can execute and
  audit an accepted task through Kimi Code.
- [ ] **Web development environment: support controlled remote development.**
  Expose the same document graph and lifecycle engine through a web interface.
  Add authenticated projects, isolated remote workspaces, resumable sessions,
  review and approval flows, streamed task and verification results, audit
  records, and controlled secret access. Define tenancy, privacy, retention,
  reliability, deployment, and operations policies before implementation.
- [ ] **Native macOS client: provide an integrated desktop workflow.** Reuse
  the accepted core formats and protocols. Add native project and document
  navigation, local workspace and keychain integration, notifications,
  accessibility, offline operation, signed distribution, updates, and recovery.
  Select the UI architecture only after macOS requirements and trade-offs are
  accepted.
- [ ] **Native Linux client: provide a distribution-portable desktop
  workflow.** Reuse the same accepted core formats and protocols while defining
  desktop integration, credential storage, sandboxing, packaging, updates,
  accessibility, and support boundaries across selected Linux environments.
  Select the toolkit and package formats only after the supported environments
  and release policy are accepted.

## Document map

The [Daoji document map](DOCUMENT_MAP.md) is the primary documentation entry
point for people and agents. It identifies governing documents, designs,
engineering policies, change records, references, and translations, and it
distinguishes their authority.

When starting work:

- Read the [project charter](docs/PROJECT_CHARTER.md) for the project purpose,
  scope, priorities, and highest-level rules.
- Read the [development lifecycle standard](docs/standards/DEVELOPMENT_LIFECYCLE_STANDARD.md)
  when you plan, implement, or verify a change.
- Read the [documentation standard](docs/standards/DOCUMENTATION_STANDARD.md)
  when you create, review, or maintain project documentation.
- Use the [complete document map](DOCUMENT_MAP.md) to locate designs,
  engineering policies, change records, and other project information.

## Development

Install the toolchain declared in `rust-toolchain.toml`. Run the local quality
checks from narrowest to broadest:

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-targets --all-features --locked
cargo doc --workspace --all-features --no-deps
cargo build --workspace --all-features --locked --release
```

CI also enforces line coverage, dependency advisories, licenses, sources, and
cross-platform tests. The commands and thresholds are controlled by the
[Rust engineering policy](docs/policies/RUST_ENGINEERING_POLICY.md).
