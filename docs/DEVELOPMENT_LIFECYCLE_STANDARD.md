# Daoji development lifecycle standard

Status: Effective draft 0

Document type: Development standard

Required use: The [Daoji project charter](PROJECT_CHARTER.md) requires project
work to follow this standard. The charter has priority if the documents
conflict.

## Purpose

This standard defines how a software change moves from a source request to
accepted software and retained verification evidence. It defines lifecycle
stages, document-tailoring decisions, required references, verification work,
and exit conditions.

This standard applies to people, agents, and tools that plan, implement, review,
or verify a Daoji change. The charter defines project principles and the
engineering asset model. This standard defines the operational workflow.

## Workflow entry

A user prompt, issue, conversation, or other request can start the lifecycle.
The original request is input evidence. It is not an authoritative project
document. Before the project plans implementation, the project must convert the
request into reviewable intent and requirements. The project must preserve a
link or quotation that makes the source request traceable when the source can be
stored safely. The project must not treat inferred details as user decisions.

At the start of a change, the project must create or identify a change entry
that acts as the navigation point for the work. The entry can be a small change
document or another authoritative engineering asset in the repository. It must
link to the applicable lifecycle assets, record their status, and record each
decision to combine or omit a separate document. An external issue can be an
input source or navigation aid, but it cannot be the only record of a controlling
decision.

The change entry supports progressive disclosure. A person or agent starts with
the change entry and follows only the links needed for the current stage or
task.

## Required lifecycle assets

Every software change must produce or link to the following minimum document
set:

1. Intent and requirements that state the goal and required behavior.
2. A design and implementation plan that state how the project performs the
   change.
3. Applicable engineering policies that control implementation work.
4. A verification plan that states how the project tests and verifies the
   change.
5. An implementation record that states how the project implemented the change.
6. A verification record that contains the test and verification results.
7. A change summary that lists the changes and is suitable for use as a pull
   request description.

An existing authoritative document can satisfy an item when the change links to
the applicable content. A small change may combine items in one short change
document, but each required section must remain explicit. Combining assets does
not remove required information, review, acceptance, traceability, or exit
conditions. The change entry must identify where each required section exists.

Before implementation starts, the project must accept the intent, requirements,
design, implementation plan, verification plan, and any new or changed
engineering policies that apply to the work. Before the project accepts or
merges the implemented change, it must complete the implementation record,
verification record, and change summary.

For an exploratory prototype, an accepted prototype plan may satisfy the design,
implementation plan, and verification plan requirements. The prototype plan
must state provisional design constraints, tasks, and verification criteria. It
must also satisfy the prototype lifetime and adoption rules in the charter.

## Lifecycle stages and exit conditions

### Intent

An intent document states the intended outcome, reason, affected people and
groups, constraints, and measures of success. The document may contain
incomplete information, but it must identify each known uncertainty.

The project must derive the intent from the source request without adding
unstated scope. The intent must distinguish requested outcomes from proposed
solutions. When the request is ambiguous, the project must record questions and
obtain the required decisions before it accepts dependent requirements. Intent
and requirements may be sections of one document when they remain distinct.

Exit condition: the document identifies the intended outcome and the person who
has the required decision role. The document links to the source request or
records why the source cannot be retained.

### Requirements

Requirements define observable capabilities, qualities, constraints, and
acceptance criteria. Before the design stage, requirements must not select an
implementation.

After accepting the intent, the project must analyze it to identify functional
behavior, quality attributes, constraints, affected users and systems,
out-of-scope behavior, assumptions, and acceptance criteria. The project must
record missing information as an unresolved question instead of selecting a
convenient interpretation. Examples can clarify a requirement but cannot
replace the normative requirement or its acceptance criteria.

Exit condition: each requirement has a unique identifier and links to an intent
document. Each requirement is testable or has defined criteria for review by a
person.

### Design

Design defines system boundaries, responsibilities, interfaces, data models,
failure behavior, security assumptions, and important choices. The design must
state the benefits and disadvantages of each important choice. The project
should keep records of important decisions and rejected alternatives.

Design detail must increase with risk and unresolved questions. A separate
design document must state the context, goals, non-goals, proposed design,
important benefits and disadvantages, considered alternatives, and unresolved
questions. When a subject applies, the design must address security, privacy,
reliability, monitoring and diagnosis, accessibility, internationalization and
localization, migration, deployment, and operation.

Before accepting the implementation plan, the project must perform and record a
design assessment. The change entry records the initial assessment outcome. The
assessment must determine whether the change requires a separate design
document. A separate design document is required when the change does one or
more of the following:

- Introduces or changes architecture, component responsibilities, public
  interfaces, persistent data, schemas, or cross-component behavior.
- Makes an important technology or dependency choice.
- Changes a security or privacy boundary, trust model, reliability objective,
  compatibility promise, migration procedure, deployment model, or operating
  procedure.
- Has significant cost, risk, irreversibility, uncertainty, or alternatives
  that reviewers must evaluate.
- Establishes a pattern that later changes are expected to follow.

A local, low-risk change may proceed without a separate design document when it
does not meet these conditions. In that case, the implementation plan must
contain a **Design assessment** section. That section must explain why a
separate document is unnecessary, describe the implementation approach, and
link to the existing design that the change preserves. The section is the
design record for that change. A missing separate design document never permits
an agent to make an undocumented design decision during implementation.

Exit condition: the design addresses accepted requirements and states known
risks and unresolved decisions. The design assessment records and justifies
whether the design is a separate document or a section of the implementation
plan.

### Engineering policies

Policies define rules that apply to multiple agent tasks. Policies can define
rules for coding, dependencies, compatibility, testing, security, system
monitoring, documentation, and release practices.

For each change, the project must create a policy applicability and completeness
assessment. The assessment must examine at least coding, static analysis,
dependencies and software supply chain, interfaces and compatibility, testing,
security, privacy, data handling, reliability and performance, observability,
documentation, accessibility, internationalization, migration, deployment,
operation, and release. It must identify:

- The accepted policies that apply to the change.
- The person or tool that checks each applicable rule.
- Categories that do not apply, with the reason.
- Categories that apply but have no sufficient accepted policy.
- Required policy changes, exceptions, owners, and approvals.

When an applicable policy is missing or insufficient, the project must create
or update and accept the policy before dependent implementation starts. The
project must not hide a reusable engineering rule in a prompt or one
implementation task. If the project cannot define the rule yet, dependent
implementation must not start. An exception can modify an accepted policy for a
specific change, but an exception cannot replace a missing applicable policy.

Exit condition: the project identifies the applicable policies. Each policy
states whether a tool or a person checks compliance. The project records the
reason for each exception and the required approval. The completeness
assessment shows that every applicable category has a sufficient accepted
policy and that every exception to that policy has the required approval.

### Implementation plan

The implementation plan divides the design into ordered agent tasks with
defined scopes. Each task states its dependencies, affected components, required
context, expected engineering assets, and verification steps. Each task must
also state completion criteria that link to the applicable requirement
acceptance criteria.

The implementation plan must link to the design assessment and the policy
applicability and completeness assessment. It must order prerequisite document,
policy, schema, migration, code, test, and operational work. It must identify
which tasks a person must perform or approve and which tasks an agent or tool
may perform.

Each agent task must place a **References** section next to its implementation
instructions. The section must provide the minimum sufficient context and use
specific document sections or asset identifiers when possible. It must include,
when applicable:

- The intent, requirements, acceptance criteria, and design decisions that
  authorize the task.
- The engineering policies and approved exceptions that constrain the task.
- The relevant interfaces, schemas, component documents, source locations, and
  dependency records.
- The verification plan items, existing tests, test data rules, tool
  configuration, and commands that determine completion.
- The migration, deployment, operation, monitoring, security, and rollback
  documents affected by the task.

References must be ordered from task-specific material to broader project
material. A task should link to a relevant section instead of copying a complete
document. The task must state when a discovered condition requires the agent to
follow an additional reference or stop and request a decision. This structure
lets an agent disclose context progressively without losing authoritative
constraints.

Exit condition: the plan is feasible and links to the authoritative documents
that require the work. The plan identifies relevant risks and gives an agent
enough information to perform the work without making undocumented architecture
or policy decisions. Every task has completion criteria and colocated references
that are sufficient for its scope.

### Verification plan

The verification plan defines how the project tests and verifies the proposed
change. The plan must link each test or review method to the applicable
requirement and acceptance criteria. It must identify the test type, commands,
environment, required inputs, expected results, and the person or tool that
performs the verification.

The plan must define the required verification layers and their execution
order. The plan must consider document and metadata validation, formatting,
linting, type checking, other static analysis, dependency and license checks,
security scanning, unit tests, integration tests, system or end-to-end tests,
build and packaging checks, migration tests, performance or reliability tests,
and review by a person. Applicable engineering policies determine the exact
checks and pass thresholds. The plan must justify each omitted layer that would
normally apply to the affected software.

The plan must separate checks that an implementation task runs for fast local
feedback from checks that the project runs on the combined change. It must
define how the project controls test data, secrets, external services,
non-determinism, retries, and cleanup when these subjects apply. It must also
state what evidence the project retains and what result requires a return to an
earlier lifecycle stage.

Exit condition: the verification plan covers every applicable acceptance
criterion and contains enough information for a person or tool to perform the
verification. The plan covers applicable policy checks and justifies omitted
verification layers.

### AI-generated code

Agents perform agent tasks from an accepted implementation plan. The environment
for each agent has defined permissions and limits. Changes must remain within
scope and follow applicable policies. Agents must report information that shows
that assumptions from earlier stages are incorrect.

Exit condition: the agent has produced the specified changes, local checks pass,
and the work result records all deviations and unresolved issues.

### Implementation record

The implementation record describes the implemented result. It must identify
the changed components and behavior. When applicable, it must also identify
changes to interfaces, data, schemas, configuration, dependencies, build logic,
deployment, migration, and operation. It must record deviations from the
accepted design or implementation plan and link to the affected source files.

Exit condition: the implementation record describes the integrated changes and
records every known deviation and unresolved implementation issue.

### Build and verification

The project runs its defined build, static analysis, tests, security checks, and
acceptance procedures on the combined software changes. Each evidence record
must link to the requirements and agent tasks that it verifies.

Unless an applicable policy or verification plan requires a different order,
the project performs verification from faster and narrower checks to broader
checks:

1. Validate changed documents, metadata, identifiers, links, and generated-file
   consistency.
2. Run formatting, linting, type checking, and other required static analysis.
3. Run dependency, license, secret, and security checks.
4. Run affected unit tests.
5. Run affected integration tests.
6. Run required system, end-to-end, migration, performance, reliability, build,
   packaging, and acceptance checks.
7. Perform required review by a person or an independent tool.

A failure must remain visible. The project must not replace a failed mandatory
check with a narrower successful check. The project must return to requirements,
design, policy, planning, or implementation when evidence invalidates an
earlier assumption.

Exit condition: the project completes each planned check that can run, preserves
the results and failures, and identifies every check that could not run. A
failed, incomplete, or omitted mandatory check must remain unresolved unless a
person with the required decision role explicitly approves an exception under
an accepted policy.

### Verification record

The verification record contains the actual evidence from testing and
verification. It must contain the commands, environment, inputs, expected
results, actual results, pass or fail status, failures, and approved exceptions.
It must link each result to the applicable requirement, acceptance criterion,
verification plan item, and agent task.

The verification record is the test conclusion document for the change. It must
contain an overall conclusion with one of the following outcomes: pass, fail,
pass with approved exceptions, or inconclusive. The conclusion must summarize
coverage of requirements and policies, failed or omitted checks, approved
exceptions, residual risks, non-repeatable evidence, and required follow-up
work. Raw logs may be separate evidence, but the conclusion must link to them
and remain understandable without reading every log.

The project must not accept or merge a change with a fail or inconclusive
conclusion. A pass with approved exceptions is eligible for acceptance only
when each exception satisfies the applicable policy and approval rule.

Exit condition: the verification record contains enough information for a
person or tool to evaluate each acceptance criterion and reproduce each
repeatable check. Its overall conclusion is supported by the recorded results,
and no failure or omission is hidden.

### Change summary

The change summary provides a concise description of a proposed software change.
It must be suitable for direct use as a pull request (PR) description without
rewriting the content. It must contain the following sections:

- **Purpose:** The reason for the change and the intended outcome.
- **Related requirements:** Links to the intent, requirements, design decisions,
  implementation plan, and verification plan that require or permit the change.
- **Changes:** A list of affected software assets or components and added,
  changed, or removed behavior.
- **Implementation notes:** Important implementation details and deviations from
  the accepted design or implementation plan.
- **Verification:** The verification methods, commands, and results, with links
  to the verification record.
- **Risks and compatibility:** Known risks and effects on compatibility,
  security, privacy, migration, deployment, and operation.
- **Documentation:** Documentation that the change adds or updates.
- **Unresolved work:** Known limitations, unresolved issues, and later work.

The project must generate or update the change summary from the authoritative
engineering assets for the change. A copy in a PR description is not an
authoritative project document. If the change or its verification results
change, the project must update the authoritative change summary and the PR
description.

Exit condition: the change summary is consistent with the implementation and
verification records and is ready for use in a PR description.

### Runnable software

The resulting software can run in its specified environment. Records must link
the software to its source, documents, build inputs, and verification evidence.

Exit condition: the software satisfies the release or deployment criteria, and
the delivered behavior links to an accepted intent document.

## Change process

For each proposed software change, the project must perform the following
process:

1. Capture the source request and create or identify the change entry.
2. Identify the affected authoritative documents and current software behavior.
3. Derive or update the intent. Record uncertainties and ask a person for each
   required intent decision that is missing.
4. Accept the intent.
5. Derive or update the requirements and acceptance criteria. Resolve or record
   missing requirement decisions, and then accept the requirements.
6. Record the initial design assessment outcome in the change entry. Create a
   separate design document when the assessment requires one. Otherwise,
   reserve the design assessment and implementation approach sections in the
   implementation plan.
7. Assess the applicability and completeness of engineering policies. Create or
   update applicable policies and record approved exceptions before dependent
   work starts.
8. Create the implementation plan. Define ordered agent tasks, completion
   criteria, and progressively disclosed references near each task.
9. Create the verification plan. Map acceptance criteria and applicable policy
   rules to static checks, tests, reviews, evidence, and pass thresholds.
10. Determine whether the pre-implementation assets contain all required
    information, are mutually consistent, and satisfy their exit conditions.
11. Accept the design, implementation plan, verification plan, and new or
    changed engineering policies before implementation starts.
12. Assign implementation and test tasks to people, agents, or tools. Collect
    structured work results and stop when a task requires an undocumented
    design or policy decision.
13. Create or update the implementation record.
14. Run the planned policy checks, static analysis, unit tests, integration
    tests, broader tests, builds, and reviews on the combined change.
15. Create the verification record and test conclusion. Link evidence to the
    applicable acceptance criteria, policies, verification plan items, and
    agent tasks.
16. Create or update the change summary.
17. Reject, revise, or accept the change according to the recorded conclusion
    and approval rules.
18. Store the updated authoritative documents and retained evidence in version
    control.

Daoji must support or automate this process at the applicable self-development
level. If the current Daoji level does not support a step, people and
general-purpose tools must perform that step.

When new information shows that an assumption is incorrect, the process must
permit a return to an earlier stage.

## Current engineering policy gaps

The initial policies in the charter do not form a complete implementation policy
set. The project has not yet accepted specific policies for the following
subjects:

- Source code style, formatting, linting, type checking, static analysis, code
  review, and generated code.
- Dependency selection, version locking, updates, vulnerability response,
  licenses, provenance, and software supply chain controls.
- Interface, data format, and schema compatibility, versioning, and deprecation.
- Test levels, coverage expectations, isolation, test data, non-determinism, and
  pass thresholds.
- Authentication, authorization, threat modeling, sensitive logging, and
  security incident response beyond the initial secret-access rule.
- Privacy, data classification, retention, deletion, and regional constraints.
- Reliability objectives, performance, capacity, monitoring, logging, tracing,
  alerting, diagnosis, backup, and recovery.
- Accessibility, internationalization, localization, migration, deployment,
  operation, release, rollback, and artifact integrity.

This list is a known policy-gap record, not a set of implied policies. Before
the first implementation selects a technology or affects one of these subjects,
the project must define and accept the applicable policy.
