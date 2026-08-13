# Daoji project charter

Document type: Project charter

Length assessment: This charter keeps the initial project rules together so
that readers can evaluate their priority and consistency. Readers should use
the section headings and follow the linked standards for operational detail.

Required use: Project work must follow the charter at this path on the
authoritative branch. A replacement must use a typed supersession relationship
and an authorized merge.

## Purpose

This charter specifies Daoji, a tool that coordinates AI agents and applies
project rules to AI-assisted software engineering. Daoji must use
Document-Defined Software Development (DDSD) to create and change large software
systems. DDSD is a process in which authoritative documents specify and control
software work.

Daoji is not an AI agent. The specified engineering tool must invoke and
coordinate AI agents. It must give each agent context from the authoritative
project record and from the current change proposal. It must distinguish
candidate assets from authoritative assets. It must also give each agent a task with
a defined scope, constraints, and completion criteria. Daoji must record the
resulting changes and verification evidence in the change proposal.

In DDSD, each document has a type, scope, stable identity, and typed links to
related engineering assets. Git commits identify exact repository snapshots,
and Git blobs identify exact file revisions when needed. Only machine-readable
formats can have schema versions. When applicable, a lifecycle document must
include or link to acceptance criteria.
Documents describe work and control project decisions. Each software change
must link to the decisions that require it. Work on a development branch remains
a proposal. When the proposal has the required path-owner reviews and mandatory
checks, an authorized actor can merge it into the protected authoritative
branch. The merge atomically adopts its documents, software, and records into
the authoritative project record.

This charter is the highest-priority project document for Daoji. It defines
the project's purpose, values, and rules. All other project assets must be
consistent with this charter. Unless this charter or an authoritative project
policy incorporates them, external policies, standards, and engineering practices do not
directly control the project.

This charter uses the following terms:

- **Engineering asset:** A document, software item, or record that the project
  creates or maintains.
- **Authoritative document:** A document at an applicable path in the
  authoritative branch whose type and scope specify requirements, rules, or
  decisions for the project.
- **Authoritative project record:** The version-controlled collection of project
  information that the project uses as the official basis for decisions.
- **Authoritative branch:** The protected repository branch whose merged
  contents form the authoritative project record.
- **Authoritative snapshot:** The exact authoritative project record identified
  by a Git commit.
- **Change proposal:** The linked documents, software changes, records, and
  evidence on a development branch and its pull request. The change entry owns
  the proposal's workflow state.
- **Candidate asset:** An unmerged engineering asset in a change proposal. A
  candidate asset can control work within that proposal after applicable
  validation, but it is not authoritative.
- **Approved:** A review approval that the repository governance system reports
  as valid for the PR head Git commit of a change proposal.
- **Repository governance:** The path ownership, required review, required
  check, branch protection, merge authorization, and related repository rules
  that control whether the project can merge a change proposal.
- **AI agent provider:** A service or software system that provides access to an
  AI agent.

Document authority is derived, not stored as document metadata. A candidate
asset becomes authoritative when an authorized merge places its path in the
authoritative branch. Git history records revisions. Typed relationships record
semantic derivation, replacement, and historical retention.

## Problem statement

AI agents can produce useful code quickly. However, a workflow that gives work
to AI agents before the project creates authoritative documents has the
following problems in large projects:

- Temporary prompts contain the only record of project goals.
- Requirements, design decisions, and implementation details become mixed.
- Different agents make incompatible assumptions.
- Local changes violate constraints that apply to the entire system.
- Generated code appears complete without sufficient verification.
- The reason for a decision is lost after the interaction ends.
- Code and documentation become inconsistent because the project does not
  define which information controls decisions.

Daoji must retain project context. Daoji must assign work with a defined scope.
Daoji must record links between decisions. Daoji must not report lifecycle or
verification readiness without retained evidence. Only an authorized merge
adopts the work. Daoji must not assume that one model or agent can understand
the entire system at one time.

## Goals

1. Convert goals from people into explicit engineering assets that people can
   review.
2. Record links among requirements, design, policies, plans, code, tests, build
   results, and the behavior of released software.
3. Give each AI agent the minimum required context and a task with a clearly
   defined scope.
4. When automated checks are feasible, use them for project rules. When
   automated checks are not feasible, require an identified reviewer.
5. Detect contradictory information, missing decisions, outdated documents,
   and statements that lack evidence before these problems cause implementation
   defects.
6. Apply to both the development of new systems and the controlled modernization
   of large existing systems.
7. Ensure that the core concepts do not depend on a specific model, agent,
   programming language, build system, or source-code hosting provider.
8. Require retained verification evidence for every statement that work is
   complete.
9. Use Daoji's own rules and workflow to control its development.
10. Apply version control, review, validation, and repository-governed path
    ownership to design and documentation, as the project does for software
    assets.
11. Use the same authoritative document for people and software agents.
12. Use precise, simple, and translatable English so that readers from different
    linguistic and cultural backgrounds can understand the project.

## Non-goals

The following capabilities are outside the scope of Daoji:

- Daoji does not remove people from product or architectural decisions.
- Daoji does not guarantee correctness solely through document generation.
- Daoji does not act as an AI agent, general-purpose code editor, or chat
  interface.
- Daoji does not prescribe one software architecture or development
  methodology.
- Daoji does not hide uncertainty or automatically resolve important choices,
  including their benefits and disadvantages.
- Daoji does not accept generated code without project-defined verification.
- Daoji does not require all documentation to be natural-language prose.

Structured data, diagrams, schemas, decision records, executable checks, and
test specifications can control work when their type and scope at an applicable
path in the authoritative branch give them that semantic role.

## Foundational principles

The following principles control Daoji development.

### Document and agent responsibilities

Agents may analyze, propose, implement, and verify. They must not silently
create project-level requirements or policies. If neither the authoritative
record nor a documented decision in the current proposal provides a required
decision, the agent must request that decision. The agent must not make and
apply an undocumented project-level decision.

### Explicit document priority

Every engineering asset that is subject to project rules must have a defined
type and scope. If two assets conflict, the system must report the conflict
and use the documented priority rule. The system must not select text without
applying that rule.

### Asset traceability

The development model shows a sequence, but a project can return to an earlier
stage. A requirement can require a design change. Verification can show that a
requirement is incorrect. Implementation work can show that a plan is
incorrect. Daoji must record typed links for these changes. It must use Git
commit or blob identifiers when a link requires an exact repository revision.

### Agent task scope and completion criteria

An agent task must identify its inputs, permitted scope, constraints, expected
outputs, and completion criteria. An implementation plan is a set of these task
specifications. It is not an unstructured list of prompts.

### Verification evidence

"The tests should pass" is not evidence. The project must retain commands,
environment information, results, failures, exceptions, and relevant
engineering records. This information must show whether the work satisfies the
acceptance criteria.

### Authoritative documents for behavior changes

A behavior change begins by identifying the documents that define and permit
the current behavior. The change is incomplete if the code changes but the
applicable requirements, design, policies, plans, or verification criteria
remain outdated.

### Repository-governed adoption

Daoji does not maintain an independent system of project approval roles. The
repository governance system determines required reviewers from paths in the
actual repository diff, validates mandatory checks, restricts the authoritative
branch, and identifies who can merge. Paths declared in plans and records
support planning and traceability but do not control reviewer selection. A
review approval must apply to the PR head Git commit.
The repository rules determine when a later commit dismisses or invalidates
an approval.

All documents, software, and records on a development branch remain candidate assets
until merge. No change-entry workflow state, AI conclusion, comment, or local
transition can make a candidate asset authoritative. All required path-owner
reviews, successful mandatory checks, and an authorized merge together form the
adoption decision. Automation may review, check, or merge only when the
repository governance rules explicitly permit it.

Daoji must expose repository governance state to people and workflows. Daoji
must not duplicate or override the repository's final merge authorization. A
repository adapter must bind review and check state to the repository, target
branch, pull request, and PR head Git commit.

### Authoritative project record

The merged contents of the protected authoritative branch are initially the
authoritative project record. Development branches, pull requests, chat history,
and model memory can assist work but remain candidate or informative. They cannot
control authoritative project decisions before merge.

### Required documents for software assets

Before an agent creates a software asset on a development branch, an
authoritative document or a document in the same change proposal must require
or permit that asset. Software assets include source code, tests, schemas,
configuration, build logic, deployment definitions, generated outputs, and
automation. A candidate document can justify candidate implementation work but
cannot broaden permissions, grant secret access, or override an authoritative
policy before merge.

The project does not need one document for every file. Each software asset must
link to a document that states why the asset is needed, which constraints apply,
and how required checks determine merge readiness. A prototype plan in the same
change proposal may permit a prototype before final design. The plan must state
the question to investigate, scope, lifetime, and criteria for deleting the
prototype or incorporating it into the system.

### Required documentation and design

Documentation and design are required parts of the software project. The
project must apply version control, review, validation, and repository path
governance according to the affected paths, type, scope, and relationships of
each document or design. Implementation work must include the required
documents and designs. Before the project
completes implementation, the project must create these documents and designs.

If neither an authoritative document nor the current change proposal defines a
user-visible capability, the project must not merge that capability. If merged
implementation and authoritative documentation disagree, the project must
record a defect. The project must resolve the conflict through the documented
change process.

### Documentation governance

The [Daoji documentation standard](standards/DOCUMENTATION_STANDARD.md) defines the
required structure, length budgets, progressive disclosure, document graph,
repository map, language, review, and maintenance rules for all
project-maintained documentation. Project work must follow that standard.

The standard must preserve one controlling source for people and agents, use
precise and translatable English for authoritative documents, and require
documentation to remain reachable from the root document map. This charter has
priority if it conflicts with the documentation standard.

## Lifecycle stages and exit conditions

Daoji development has the following stages:

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

Every software change must trace its source request through candidate or
authoritative intent, requirements, design, applicable engineering policies,
implementation and verification plans, implementation and verification
records, and a change summary. When the project cannot retain the source request
safely, it must record the reason. The project may combine assets for a small
change, but it must not omit required decisions, evidence, review,
traceability, or repository governance.

Before implementation starts, the change proposal must contain the controlling
documents and pass the pre-implementation checks defined by the lifecycle
standard. These documents remain candidate assets. The project must complete the
implementation record, verification record, and change summary before merge.
The merge atomically updates the authoritative branch with the candidate
repository snapshot. Review, check, and merge metadata remain linked governance
evidence.

The [Daoji development lifecycle standard](standards/DEVELOPMENT_LIFECYCLE_STANDARD.md)
defines the workflow, lifecycle document-tailoring rules, engineering policy
assessment, stage requirements, verification layers, and exit conditions.
Project work must follow that standard and the
[documentation standard](standards/DOCUMENTATION_STANDARD.md).

## Initial engineering asset model

The Purpose section defines the term **engineering asset**. Engineering assets
define, implement, verify, provide information for the operation of, or record
the history of a software system.

Daoji must represent at least the following engineering asset classes:

| Engineering asset | Required information |
| --- | --- |
| Intent | The reason for the intended outcome and a description of that outcome. |
| Requirement | An observable condition that the system must satisfy. |
| Design | How the system satisfies the requirements. |
| Decision record | An important decision, the reason for that decision, and the alternatives that the project considered. |
| Engineering policy | A rule that applies to one or more agent tasks. |
| Implementation plan | Ordered agent tasks with defined scopes that specify how agents implement the design. |
| Verification plan | The methods, commands, environments, inputs, and expected results for testing and verification. |
| Work result | The changes made, outputs produced, and information discovered by an agent. |
| Implementation record | The implemented changes, affected components, and deviations from the controlling authoritative or candidate design and plan. |
| Verification record | Recorded results that show whether the work satisfied the applicable acceptance criteria. |
| Change summary | A concise list of changes, implementation notes, verification results, risks, and unresolved work that is suitable for a PR description. |
| Release record | The released software version and its supported operating environment. |

A later design document must define machine-readable formats and their metadata
schemas. A schema version applies only to a machine-readable format. Each
engineering asset must have a permanent identifier, document or asset type,
scope, and typed links to related assets. Maintenance and review responsibility
derive from repository path governance. Only the change
entry stores the change proposal's workflow state. Git history provides revision
history, and Git commit or blob identifiers identify exact revisions. The
documentation standard defines the interim document metadata and linking rules.

Before the project defines the identifier and metadata schema, each engineering
asset must have a repository path or other stable location. Each document must
state its document type and make its purpose and scope clear. Documents may use
repository paths and section names for temporary links.

For each requirement, the project should maintain one authoritative source. A
replacement must use typed `supersedes` and `superseded-by` relationships. The
project can remove the replaced document or retain it with a typed historical
relationship.

Design detail must increase with risk and unresolved questions. The development
lifecycle standard defines when a change requires a separate design document
and when an implementation plan can contain a short design record.

## Temporary document priority order

Until the authoritative branch contains complete rules for document priority, the system must
report semantic conflicts and resolve them by using the following priority
order. Operational permissions use only the authoritative snapshot identified
by the target-branch Git commit. Merge validation uses the candidate post-merge
snapshot, including candidate additions, replacements, and removals:

1. The project charter in the evaluated snapshot
2. Intent and requirements in the evaluated snapshot
3. Design decisions in the evaluated snapshot
4. Project standards and applicable engineering policies in the evaluated snapshot
5. Implementation plans in the evaluated snapshot
6. Agent instructions and work results
7. Generated implementation details

An unresolved conflict between assets at the same priority blocks lifecycle
readiness and merge. A candidate asset cannot relax an authoritative permission
or security constraint for work performed before merge.

Verification evidence remains important although the priority order does not
include evidence. Evidence shows whether work satisfies the applicable
requirements and acceptance criteria. Evidence alone does not change those
requirements.

## Daoji self-development

Daoji should gradually use its own workflow to develop itself. The following
levels define progress toward that capability.

### Level 0: Manual documentation

People create and review the charter and documents created from the charter's
requirements. General-purpose tools and agents may assist, but Daoji has no
executable implementation.

### Level 1: Document validation

A minimal tool validates engineering asset structure, types, identifiers, typed
links, change-entry workflow state, and basic policy rules for the Daoji
repository.

### Level 2: Plan and task creation

Daoji creates agent tasks with defined scopes from authoritative assets and the
current change proposal. Daoji produces task specifications that do not depend
on one AI agent provider. Repository governance controls final adoption of the
resulting change.

### Level 3: Coordinated execution

Daoji invokes one or more interfaces for AI agents. Daoji limits each
agent's context and task scope, collects the results, and runs verification
commands defined in the repository.

### Level 4: Development controlled by Daoji

Daoji uses its own workflow for routine development. Each Daoji change links
its intent, requirements, design, implementation, verification, and release
records. Required path-owner reviews and mandatory checks must be valid for the
PR head Git commit before an authorized merge updates the authoritative branch.

Daoji must not be the only system that verifies Daoji. Daoji's own check results
are not sufficient proof that Daoji is correct. A person or an
independent tool must review the initial implementation components, schemas,
policy changes, and verification mechanisms. The amount of review must increase
with the risk.

## Initial engineering policies

Before implementation-specific policies exist, the following project rules
apply:

1. Do not select a technology only to start coding. An authoritative requirement
   or a requirement and design decision in the current change proposal must
   justify the choice.
2. Keep requirements separate from examples and explanatory text.
3. After the project defines the identifier scheme, give each engineering asset
   a permanent identifier.
4. Record uncertainty, missing information, and unresolved decisions. Do not
   present a model-generated assumption as an authoritative project decision.
5. Prefer coordination and validation processes that produce the same result for
   the same input and that reviewers can inspect.
6. Until model output passes the relevant checks, treat the output as untrusted
   input.
7. Unless an authoritative design and policy require access to secrets and specify
   controls for that access, never expose secrets to an agent.
8. The core model must not depend on one AI agent provider. Put
   provider-specific behavior in separate provider interfaces.
9. Define each failure condition and its recovery procedure. Do not mark the
   change proposal workflow complete when agent work remains incomplete.
10. When implementation behavior changes, the same change proposal must include
    the required documentation and verification updates.

The [documentation standard](standards/DOCUMENTATION_STANDARD.md) controls documentation
creation, writing, linking, review, maintenance, and defect handling.

These initial policies do not form a complete implementation policy set. The
[development lifecycle standard](standards/DEVELOPMENT_LIFECYCLE_STANDARD.md#current-engineering-policy-gaps)
records the known gaps and defines how each change checks policy completeness.

## First documentation cycle

The first documentation cycle should produce the following engineering assets:

1. An intent document for the first milestone in which Daoji helps to develop
   Daoji.
2. Functional and quality requirements for Level 1 document validation.
3. A domain model for engineering assets, typed links, change-proposal workflow
   transitions, and evidence.
4. Architecture decisions for repository layout, document formats, validation,
   and the initial command-line interface.
5. Project engineering policies that close the known gaps applicable to the
   selected technology, architecture, implementation, and verification scope.
6. An implementation plan with agent tasks and completion criteria that link to
   each requirement's acceptance criteria.
7. A verification plan that defines how the project verifies each requirement
   and acceptance criterion.
8. Production code that the project creates only after it completes items 1
   through 7.
9. An implementation record that describes the implemented result.
10. A verification record that contains the actual commands, environment, and
    results.
11. A change summary that is suitable for direct use as a PR description.

## Open questions

The following are intentionally unresolved:

- Which parts of documents should be structured data versus prose?
- Which typed relationship vocabulary and validation rules should represent
  priority, supersession, derivation, and historical retention?
- What is the smallest useful Level 1 validator?
- How should Daoji represent repository review, check, and merge state across
  different source-code hosting providers?
- How should the project retain evidence without making repositories too large
  for normal use?
- How should Daoji begin to manage an existing repository when the code is the
  only record of the software design?
- Which agent interface lets Daoji use different AI agent providers without
  limiting all agents to the functions of the least capable provider?
- How should Daoji report and resolve conflicting requirements and policies?

The next requirements and design stages must address these questions. The
implementation must not answer them without an explicit, recorded decision.

## Informative references

The project used the following sources when writing this charter. These sources
do not control Daoji. Only requirements in authoritative Daoji engineering assets
apply to the project.

- [Claude's Constitution](https://www.anthropic.com/constitution), for the use
  of an explicit and transparent document that states values, priorities, and
  rules for resolving conflicts.
- [Documentation-Driven Development](https://gist.github.com/zsup/9434452), for
  defining capabilities in documentation before implementation and keeping
  documentation, tests, and software consistent and versioned.
- [Software Engineering at Google: Documentation](https://abseil.io/resources/swe-book/html/ch10.html),
  for applying version control and review processes to documentation, writing
  for a declared audience, and maintaining authoritative documents.
- [Design Docs at Google](https://www.industrialempathy.com/posts/design-docs-at-google/),
  for recording design context, goals, non-goals, decisions, alternatives,
  competing benefits and disadvantages, and concerns that apply to multiple
  system components before implementation.
- ATA iSpec 2200, for its structured approach to technical information.
- ASD Simplified Technical English, for controlled language that improves
  clarity, consistency, and accuracy under translation.
