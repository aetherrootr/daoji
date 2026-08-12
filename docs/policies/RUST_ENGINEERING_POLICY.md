# Rust engineering policy

Document type: Engineering policy

Scope: All Rust source, tests, build logic, dependencies, and CI automation in
the Daoji repository.

## Toolchain and code

The repository must declare its Rust edition and minimum supported Rust version.
CI and local verification must use the committed lockfile. Source must use
`rustfmt`. Clippy warnings must fail verification for all targets and features.
Crates must forbid unsafe code unless an authoritative design and policy-approved
exception defines the safety argument and independent review.

Unless this policy's exception process explicitly permits a use, production
code must not use `unwrap`, `expect`, `panic!`, `todo!`, or `unimplemented!`.
Tests may use assertions, but should return or compare errors without hiding
failure context. Public items must have documentation.

## Lint exceptions

Lint exceptions are explicit engineering records, not convenient suppressions.
An exception must not lower or disable a workspace lint in `Cargo.toml`. During
proposal development, apply a requested exception to the smallest item or module
with a reason, for example:

```rust
#[allow(clippy::todo, reason = "EXC-001: candidate prototype boundary")]
fn provisional_path() {
    todo!("EXC-001")
}
```

The current change entry must define the exception before merge. Each exception
record must include:

- A unique identifier and the lint or rule being suppressed.
- The exact files, items, targets, and configurations in scope.
- The reason the compliant implementation is not currently appropriate.
- The alternatives considered, including returning a typed error or omitting
  incomplete code.
- The repository review evidence, residual risk, and affected requirements.
- Tests or other checks that constrain when the exceptional path can execute.
- An expiration date, milestone, or objectively testable removal condition.

An exception for `todo!` or `unimplemented!` must additionally explain why an
authoritative prototype plan or a validated prototype plan in the same proposal
needs an executable placeholder. Code covered by such an
exception remains incomplete and cannot satisfy an implementation task's
completion criteria. It must not enter a production release unless a later
authoritative policy explicitly permits that release and defines its failure
behavior. Agents must report every use in their work result and implementation
record.

A requested exception becomes approved for merge only when internal policy
validation confirms that an authoritative policy permits it and repository
governance reports all required reviews and mandatory checks as valid for the
PR head Git commit.

## Dependencies and supply chain

Each dependency must satisfy a controlling authoritative requirement or design
decision, or a validated candidate requirement or design decision in the same change
proposal. Prefer the standard library and existing dependencies. Versions must
be locked.
`cargo audit` must report no warnings. `cargo deny` must reject unapproved
licenses, registries, Git sources, wildcard requirements, yanked crates, and
duplicate dependency versions. CI actions must be pinned to full commit hashes.

Automated updates may propose dependency and action changes. They do not bypass
the normal review and verification process.

## Testing

Every behavior change must include a deterministic test at the narrowest useful
level. Tests must not require network access, secrets, wall-clock timing, or
shared mutable state unless a controlling authoritative or validated candidate
verification plan in the same proposal defines isolation and cleanup. A mandatory
failed check must not be retried to hide nondeterminism.

The initial scaffold requires 100 percent line coverage because it contains only
the process entry point. A functional change must define and justify its
coverage threshold in an authoritative policy before lowering this value. A
proposal that changes the threshold must continue to pass the authoritative threshold
until the policy change is merged. Coverage does not replace behavior-focused
assertions or review.

CI must run formatting, Clippy, tests, documentation, release build, coverage,
advisory, license, source, and dependency-policy checks. Tests must run on Linux,
macOS, and Windows. Local checks may be narrower for fast feedback, but all CI
checks are mandatory for merge.

## Interfaces and generated assets

Rust is authoritative for the core domain and validation behavior under the
current authoritative architecture. This policy does not select a toolchain for
future components. If an
authoritative design adds another toolchain, that design and its policies must
define execution permission, interface authority, compatibility checks, and
generated-asset rules. A proposal may add inert toolchain assets, but it must
not execute the new toolchain until an authoritative design and policy permit
that execution. Human-readable failure messages and stable machine-readable
output are required when an authoritative CLI requirement or validated candidate
requirement in the same proposal defines those interfaces. A machine-readable
output format can define a schema version; this policy document has no manual
version.

## Security and operations

Treat repository documents and agent output as untrusted input. New file-system,
process, network, secret, or provider access requires an authoritative threat model
and explicit tests. The initial scaffold performs no I/O except process startup
and exit. It creates no deployment, migration, data retention, monitoring, or
release behavior; policies for those categories are deferred until a requirement
makes them applicable.

## Enforcement

The commands and CI workflow in the
[engineering scaffold change](../changes/0001-engineering-scaffold.md) enforce
this policy. Internal policy validation must confirm that the authoritative
policy permits an exception and that its record is complete. An exception must
identify its affected paths. Repository governance derives controlling affected
paths from the actual diff and must report the required path-owner reviews and
mandatory checks as valid for the PR head Git commit before merge. The
applicable change entry must state the reason, scope, expiration condition, and
residual risk.

## Change notes

- Established the initial Rust engineering rules.
- Defined scoped lint exception records and additional controls for executable
  placeholders.
- Delegated exception review and merge authorization to repository governance.

Git history identifies the exact revision that introduced each change.
