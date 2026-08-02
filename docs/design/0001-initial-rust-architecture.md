# Initial Rust architecture

Status: Accepted 0

Document type: Architecture decision

Decision owner: Project owner

Approval: The project owner approved Rust for the initial scaffold and deferred
all additional toolchain decisions in the source conversation on 2026-08-02.

## Context

Daoji begins as a document and metadata validator, but later self-development
levels coordinate AI agents, execute tools, and retain evidence. The engineering
scaffold therefore must support both a small first implementation and a future
agent harness. The source request asks for strict automated testing and linting
because AI agents will generate implementation changes.

Comparable specification tools use different stacks. GitHub Spec Kit uses
Python with Ruff and pytest. OpenSpec uses TypeScript, ESLint, and Vitest.
Comparable agent harnesses provide a more relevant signal for the initial core:
OpenAI Codex and Goose use Rust for process and protocol logic. These projects
also show that additional toolchains increase build, dependency, interface, and
test costs and should follow concrete product requirements.

## Decision

Daoji will use Rust for the initial implementation:

- Rust owns the domain model, document validation, policies, orchestration,
  provider-neutral protocols, process control, and command-line interface.
- The initial scaffold contains one Rust binary crate. A workspace split
  requires a demonstrated component boundary.
- This decision does not select a user-interface architecture or an additional
  language, runtime, package manager, or build system.
- A future change may add another toolchain only when accepted requirements and
  design justify its behavior, boundary, maintenance cost, and verification.

Rust 2024 with a minimum supported Rust version of 1.85.1 is the initial
toolchain. The scaffold has no runtime dependencies. Later dependencies require
the review defined in the Rust engineering policy.

## Benefits

- Rust provides a conventional, ready-made verification suite through `rustc`,
  `cargo`, `rustfmt`, Clippy, the built-in test harness, and documentation tests.
  Established Cargo tools also cover code coverage, security advisories,
  licenses, and dependency policy.
- The type system, compiler, `unsafe_code = "forbid"`, and strict Clippy rules
  turn defined classes of defects into mechanical failures before review.
- These existing checks let the repository keep its Rust engineering policy
  concise instead of restating a large set of rules that tools cannot enforce.
- Rust can produce an optimized, standalone native binary without requiring a
  language runtime on the user's machine. Performance claims for functional
  code still require benchmarks against accepted requirements.
- Deferring other toolchains avoids additional lockfiles, duplicated checks,
  and premature interfaces before corresponding requirements exist.

## Disadvantages

- Rust source and Cargo-based quality tools require compilation, which adds
  local and CI time and storage. The project has not yet measured that cost.
- Native binary distribution requires builds and tests for each supported
  operating-system and processor target.
- Compiler and lint success cannot establish requirement correctness. Tests,
  retained evidence, and human review remain necessary.
- The fixed minimum toolchain needs deliberate upgrades and compatibility tests.

## Alternatives not selected

**Python only:** This option can implement Level 1 and matches Spec Kit. Python
is a viable option and was not excluded for an inherent safety defect. Meeting
this change's selected gates would require the project to
choose and coordinate separate formatting, linting, type checking, testing,
coverage, audit, license, packaging, and runtime policies. The initial scaffold
instead selects Rust's conventional Cargo-centered checks and standalone binary.

**TypeScript only:** This option matches OpenSpec and has strong Markdown and UI
libraries. TypeScript is also viable. The initial scaffold did not select it
because the accepted decision favors a standardized Rust verification suite and
a native binary without a separately installed language runtime. This decision
does not evaluate TypeScript for an unspecified future component.

**Multiple toolchains immediately:** This option could prove an integration
boundary early. It was not selected because no accepted requirement defines a
capability that needs another toolchain. Adding one now would require additional
policies, lockfiles, verification commands, and integration tests without adding
accepted behavior.

## Consequences

The first functional change should keep parsing and validation in Rust. This
decision gives no preference to a language or framework for unspecified future
components. A proposal for an additional toolchain must compare using Rust,
adding that toolchain, and other applicable alternatives at that time.

## References

- [Engineering scaffold change](../changes/0001-engineering-scaffold.md)
- [Rust engineering policy](../policies/RUST_ENGINEERING_POLICY.md)
- [Daoji project charter](../PROJECT_CHARTER.md)
- [OpenAI Codex](https://github.com/openai/codex)
- [Goose](https://github.com/block/goose)
- [GitHub Spec Kit](https://github.com/github/spec-kit)
- [OpenSpec](https://github.com/Fission-AI/OpenSpec)
