# Engineering scaffold change

Status: Accepted 0

Document type: Combined change entry, intent, requirements, implementation plan,
verification plan, implementation record, verification record, and change
summary

Decision owner: Project owner

## Source request

On 2026-08-02, the project owner requested an engineering scaffold based on the
project charter, comparable specification and agent-harness projects, and the
strict automated test and lint needs of AI-driven development. During design
review, the owner approved Rust for the initial implementation and deferred all
additional toolchain and interface decisions to future accepted requirements.

## Intent

Create the smallest useful engineering foundation that makes future
AI-generated Rust changes mechanically reviewable. The scaffold affects project
maintainers and implementation agents. Success means that a clean checkout can
format, lint, test, document, audit, and build the placeholder command-line
process with deterministic commands and strict CI gates.

This change does not implement document validation, agent coordination, a user
interface, another toolchain, publishing, installation, telemetry, or a release.

## Requirements

### ES-001: Reproducible Rust scaffold

The repository must contain a minimal Rust 2024 command-line package that builds
with the declared minimum toolchain and committed dependency lockfile. Running
the placeholder binary must terminate successfully without output or side
effects.

Acceptance criteria: `cargo build --locked --release` succeeds, and an
integration test verifies successful process termination.

### ES-002: Strict static checks

The scaffold must reject formatting differences, compiler warnings, Clippy
warnings, unsafe code, undocumented public items, and production uses of
`unwrap`, `expect`, panic, `todo!`, or `unimplemented!`.

Acceptance criteria: the formatting and Clippy commands in VP-001 and VP-002
succeed with warnings treated as errors.

### ES-003: Automated tests and coverage

The scaffold must run deterministic tests on Linux, macOS, and Windows. The
initial executable lines must have 100 percent line coverage.

Acceptance criteria: VP-003 and VP-005 pass, and CI defines the three-platform
test matrix.

### ES-004: Supply-chain controls

The scaffold must lock dependencies, audit advisories, restrict licenses and
sources, reject duplicate and wildcard dependencies, pin CI actions, and
configure automated update proposals.

Acceptance criteria: VP-006 passes and repository review confirms full action
commit hashes and Dependabot configuration.

### ES-005: Deferred toolchain decisions

The accepted architecture must assign the initial core behavior to Rust without
selecting a language, framework, runtime, or build system for unspecified future
components.

Acceptance criteria: the architecture decision records the current scope,
trade-off, and rejected alternatives; this change creates no additional
toolchain assets.

## Design assessment

A separate design was required because the change selects foundational
technology, establishes patterns for later changes, and affects the future
runtime architecture. The accepted
[initial Rust architecture](../design/0001-initial-rust-architecture.md)
addresses ES-001 and ES-005. It uses one dependency-free binary crate and
defers all architecture decisions for unspecified future components.

## Policy applicability and completeness

The accepted [Rust engineering policy](../policies/RUST_ENGINEERING_POLICY.md)
controls coding, static analysis, dependencies, tests, interfaces, supply-chain
security, and generated assets. The documentation standard controls all changed
documents.

Security applies to dependency and CI integrity; the executable has no data,
network, file-system, secret, or subprocess access. Privacy, data handling,
reliability, performance, observability, accessibility, internationalization,
migration, deployment, operation, and release do not apply to this
non-functional placeholder. Those categories require new or revised policies
before dependent implementation. No exception is approved for this change.

## Implementation plan

### AT-001: Record controlling decisions

Scope: Create this change entry, the architecture decision, the Rust policy, and
map links. Expected outputs are accepted English documents. Completion requires
traceability from ES-001 through ES-005 to design, policy, tasks, and checks.

References: this document; the architecture decision; the Rust policy; the
[project charter](../PROJECT_CHARTER.md); the
[development lifecycle standard](../DEVELOPMENT_LIFECYCLE_STANDARD.md).

### AT-002: Create minimal Rust assets

Dependency: AT-001. Scope: package metadata, fixed toolchain, empty successful
CLI entry point, integration test, lockfile, and ignored generated directories.
The task must add no runtime dependency or functional validator behavior.
Completion requires local VP-001 through VP-004 and VP-007 to pass.

References: ES-001 through ES-003; the architecture decision; the Rust policy;
VP-001 through VP-004 and VP-007 below.

### AT-003: Add automated quality gates

Dependency: AT-002. Scope: CI, dependency policy, and automated update
configuration. Completion requires all required CI jobs to be represented and
the locally available checks to pass. Checks unavailable locally must remain
identified in the verification record until CI runs them.

References: ES-002 through ES-004; the Rust policy; VP-001 through VP-007 below.

The project owner approves the architecture and policy decisions. An agent may
implement all three tasks. A person must review the resulting change and hosted
CI evidence before merge.

## Verification plan

The checks run from fast and narrow to broad. Inputs are the clean repository
and locked dependencies. Expected result for every command is exit status zero.
No check needs secrets, test data, retries, cleanup, or an external service other
than dependency and advisory retrieval.

| ID | Requirement | Method and command | Environment |
| --- | --- | --- | --- |
| VP-001 | ES-002 | `cargo fmt --all --check` | Local and Linux CI |
| VP-002 | ES-002 | `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | Local and Linux CI |
| VP-003 | ES-001, ES-003 | `cargo test --workspace --all-targets --all-features --locked` | Local and Linux, macOS, Windows CI |
| VP-004 | ES-002 | `cargo doc --workspace --all-features --no-deps` | Local and Linux CI |
| VP-005 | ES-003 | `cargo llvm-cov --workspace --all-features --locked --fail-under-lines 100` | Linux CI |
| VP-006 | ES-004 | `cargo audit --deny warnings` and `cargo deny check` | Linux CI |
| VP-007 | ES-001 | `cargo build --workspace --all-features --locked --release` | Local and Linux CI |

Document and metadata validation is manual because Daoji has not implemented
Level 1 validation. Review checks metadata, links, requirement identifiers, and
consistency. Integration, system, migration, performance, and reliability tests
are omitted because the placeholder has no corresponding boundary or behavior.
Hosted CI logs are retained by the source-code hosting provider; this record
retains the reproducible commands and conclusion.

## Implementation record

Implementation status: Complete

AT-001 added this entry, the architecture decision, policy, and document-map
links. AT-002 added a dependency-free Rust binary whose only behavior is a
successful exit, plus one process-level integration test. AT-003 added strict
workspace lint settings, a fixed toolchain, Cargo Deny policy, hash-pinned GitHub
Actions, cross-platform CI, coverage and supply-chain jobs, and Dependabot.

Changed components: `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`,
`deny.toml`, `src/main.rs`, `tests/cli.rs`, `.github/workflows/ci.yml`,
`.github/dependabot.yml`, `.gitignore`, `README.md`, `DOCUMENT_MAP.md`, and the
three lifecycle document paths. The non-authoritative Simplified Chinese README
and document map were synchronized with their English sources. There are no
deviations from the accepted design or plan.

## Verification record

Verification status: Local pass; hosted verification pending

Environment: macOS Darwin on arm64, Rust 1.85.1 and Cargo 1.85.1. Commands use
the repository root as the working directory and the committed lockfile.

Local results on 2026-08-02:

| Plan item | Actual result | Status |
| --- | --- | --- |
| VP-001 | No formatting differences | Pass |
| VP-002 | Compilation and strict Clippy completed with no warnings | Pass |
| VP-003 | One integration test passed; no tests failed | Pass |
| VP-004 | Rust documentation built with warnings denied | Pass |
| VP-005 | 3 of 3 executable lines covered, 100 percent line coverage | Pass |
| VP-006 | 1 lockfile crate scanned; no advisory, license, source, or dependency-policy failure | Pass |
| VP-007 | Optimized release binary built successfully | Pass |

The first VP-006 attempt with `cargo-audit 0.21.2` and `cargo-deny 0.18.3`
could not parse a current RustSec CVSS 4.0 advisory. This was a verification-tool
compatibility failure, not a project advisory. The tools and CI pins were
updated to `cargo-audit 0.22.2` and `cargo-deny 0.20.2`; both repeated checks
passed. These tools require Rust 1.88 to build, but they inspect the project
without changing its Rust 1.85.1 minimum toolchain.

Overall conclusion: Inconclusive until hosted Linux, macOS, and Windows jobs run.
All repeatable local checks pass without exceptions. The only residual risk is
that the GitHub Actions environments and pinned actions have not yet executed.
Manual documentation review found the required metadata and valid local links;
the English and Simplified Chinese repository summaries are consistent.

## Change summary

### Purpose

Establish a strict, reproducible engineering scaffold for future AI-generated
Daoji implementation work.

### Related requirements

ES-001 through ES-005 in this change entry; the initial Rust architecture; the
Rust engineering policy; AT-001 through AT-003; VP-001 through VP-007.

### Changes

- Add a dependency-free Rust 2024 command-line scaffold and integration test.
- Add strict formatting, Clippy, documentation, test, coverage, build, advisory,
  license, source, and dependency gates.
- Add three-platform CI with immutable action references and automated update
  proposals.
- Record the accepted initial Rust architecture and deferred toolchain scope.

### Implementation notes

Additional toolchains and unspecified interface architectures are intentionally
undecided. There are no plan deviations.

### Verification

See the verification record in this document. Local and hosted results must be
complete before merge.

### Risks and compatibility

Rust increases clean build time and contributor learning cost. The placeholder
introduces no user-visible behavior, persistence, network access, deployment, or
compatibility promise.

### Documentation

This change updates the README and document map and adds architecture, policy,
and lifecycle records.

### Unresolved work

Implementing Level 1 validation or adding another toolchain requires a separate
accepted change. Hosted CI evidence is pending.
