# Daoji project charter

Status: Effective draft 0

Document type: Project charter

Required use: Until an accepted revision replaces this charter, project work
must follow this charter.

## Purpose

This charter specifies Daoji, a tool that coordinates AI agents and applies
project rules to AI-assisted software engineering. Daoji must use
Document-Defined Software Development (DDSD) to create and change large software
systems. DDSD is a process in which authoritative documents specify and control
software work.

Daoji is not an AI agent. The specified engineering tool must invoke and
coordinate AI agents. It must give each agent context from accepted documents.
It must also give each agent a task with a defined scope, constraints, and
completion criteria. Daoji must record the resulting changes and verification
evidence in the authoritative project record.

In DDSD, each authoritative document has a version and a defined status. Each
lifecycle document for a software change must link to related engineering
assets. When applicable, it must include or link to acceptance criteria.
Documents describe work and control project decisions. Each software change
must link to the decisions that require it. When verification evidence shows
that software satisfies its requirements and accepted design decisions, the
project accepts the software.

This charter is the highest-priority project document for Daoji. It defines
the project's purpose, values, and rules. All other project assets must be
consistent with this charter. Unless this charter or an accepted project policy
adopts them, external policies, standards, and engineering practices do not
directly control the project.

This charter uses the following terms:

- **Engineering asset:** A document, software item, or record that the project
  creates or maintains.
- **Effective draft:** A draft document that the project explicitly uses to
  control work until an accepted revision replaces the draft.
- **Authoritative document:** An accepted document or effective project charter
  that specifies requirements, rules, or decisions that the project must follow.
- **Authoritative project record:** The version-controlled collection of project
  information that the project uses as the official basis for decisions.
- **Accepted asset:** An asset that has completed its required review. The
  project may use an accepted asset to define or perform work.
- **Approved:** Explicitly permitted by the person who is responsible for the
  decision.
- **AI agent provider:** A service or software system that provides access to an
  AI agent.

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
Daoji must record links between decisions. Daoji must require evidence before
Daoji accepts work as complete. Daoji must not assume that
one model or agent can understand the entire system at one time.

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
10. Apply version control, review, validation, ownership, and maintenance to
    design and documentation, as the project does for software assets.
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

After the project accepts them, structured data, diagrams, schemas, decision
records, executable checks, and test specifications may become authoritative
documents.

## Foundational principles

The following principles control Daoji development.

### Document and agent responsibilities

Agents may analyze, propose, implement, and verify. They must not silently
create project-level requirements or policies. If no authoritative document
provides a required decision, the agent must request that decision. The agent
must not make and apply an undocumented project-level decision.

### Explicit document priority

Every engineering asset that is subject to project rules must have a defined
status and scope. If two assets conflict, the system must report the conflict
and use the documented priority rule. The system must not select text without
applying that rule.

### Asset traceability

The development model shows a sequence, but a project can return to an earlier
stage. A requirement can require a design change. Verification can show that a
requirement is incorrect. Implementation work can show that a plan is
incorrect. Daoji must record links and revisions for these changes.

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

### Approval of important decisions

When an explicit policy permits routine automated acceptance, automation may
accept routine changes. A person must explicitly approve decisions that are
ambiguous, irreversible, high-risk, security-sensitive, or significantly
expensive.

### Authoritative project record

The version-controlled project repository is initially the authoritative
project record. Chat history and model memory can assist during a session. Chat
history and model memory cannot control project decisions.

### Required documents for software assets

Before the project creates a software asset, an accepted document must require
or permit that asset. Software assets include source code, tests, schemas,
configuration, build logic, deployment definitions, generated outputs, and
automation.

The project does not need one document for every file. Each software asset must
link to a document that states why the asset is needed, which constraints apply,
and how the project determines acceptance. If an accepted prototype plan permits
a prototype, the prototype may precede a final design. The plan must state the
question to investigate, scope, lifetime, and criteria for deleting the
prototype or incorporating it into the system.

### Required documentation and design

Documentation and design are required parts of the software project. The
project must apply version control, review, ownership, validation, and
maintenance according to the status of each document or design. Implementation
work must include the required documents and designs. Before the project
completes implementation, the project must create these documents and designs.

If no authoritative document defines a user-visible capability, the project
must not accept that capability. If the implementation and authoritative
documentation disagree, the project must record a defect. The project must
resolve the conflict through the documented change process.

### Clarity for people and agents

An authoritative document must provide the same information to people and
software agents. Metadata can help automated interpretation, but metadata must
not conceal information from people. Prose must not require implied
information, private context, or unstated conventions.

If requirements for readability by people and agents conflict, the document
must state the conflict. It must use explicit structure, definitions, examples,
or generated views from one authoritative document. The project must not
maintain independent documents for people and agents that can become
inconsistent.

### English writing rules

The project initially writes its documents in English. When a translation has a
different meaning, the English document is authoritative.
Daoji uses a controlled technical style inspired by ASD Simplified Technical
English and practices for structured technical publications. Unless a later
policy defines and verifies compliance, the project must not state that Daoji
complies with ASD-STE100, ATA iSpec 2200, or another external standard.

The following requirements apply to authoritative documents:

- Use US English.
- Use one term for one concept, and define project-specific terms.
- Use short, direct sentences that express one primary requirement or idea.
- Use standard subject-verb-object word order when possible.
- Use present tense for general behavior.
- Prefer active voice, and identify the actor responsible for an action.
- Put a condition before the instruction that depends on the condition.
- Make conditions, scope, exceptions, and priority explicit.
- Define abbreviations on first use, and avoid unnecessary abbreviations.
- Avoid phrasal verbs, jargon, idioms, rhetorical language, culture-specific
  references, and humor when a simpler expression is available.
- Avoid ambiguous pronouns and references such as "this" when the referent is
  not explicit.
- Repeat a noun or helper word when the repetition prevents ambiguity.
- Use sentence case for titles and headings.
- Use parallel structure, capitalization, and punctuation in lists.
- Use inclusive and accessible language.
- Use examples to clarify rules without allowing examples to become rules.
- Use `must` for requirements, `should` for recommendations, `may` for
  permission, and `will` only for an event that occurs in the future.
- Prefer wording that preserves meaning under machine translation.

Clarity takes priority over stylistic variety. Use precise language, but do not
add unnecessary complexity.

The repository copy of the
[Google developer documentation style guide](references/GOOGLE_DEVELOPER_DOCUMENTATION_STYLE_GUIDE.md)
is the default editorial standard for Daoji documentation. The repository
copy records its source review date and upstream sources. A change to an
upstream source does not become a project requirement until the project updates
and accepts the repository copy.

Authors and reviewers must apply writing rules in the following priority order:

1. This charter
2. Accepted project-specific policies
3. The repository copy of the Google developer documentation style guide
4. Other external references

The controlled-English and machine-translation requirements in this charter
have priority over conflicting guidance in the Google guide. The project must
record the reason for each deliberate exception to an applicable rule.

## Lifecycle stages and exit conditions

The following sections define each lifecycle stage and its exit condition.

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

An existing authoritative document can satisfy an item when the proposed change
links to the applicable content. A small change may combine items in one short
change document, but each required section must remain explicit.

Before implementation starts, the project must accept the intent, requirements,
design, implementation plan, verification plan, and any new or changed
engineering policies that apply to the work. Before the project accepts or
merges the implemented change, it must complete the implementation record,
verification record, and change summary.

For an exploratory prototype, an accepted prototype plan may satisfy the design,
implementation plan, and verification plan requirements. The prototype plan
must state provisional design constraints, the tasks, and the verification
criteria. The prototype remains subject to the lifetime and adoption rules in
the Required documents for software assets section.

### Intent

An intent document states the intended outcome, reason, affected people and
groups, constraints, and measures of success. The document may contain
incomplete information, but it must identify each known uncertainty.

Exit condition: the document identifies the intended outcome and the person who
has the required decision role.

### Requirements

Requirements define observable capabilities, qualities, constraints, and
acceptance criteria. Before the design stage, requirements must not select an
implementation.

Exit condition: each requirement has a unique identifier and links to an intent
document. Each requirement is testable or has defined criteria for review by a
person.

### Design

Design defines system boundaries, responsibilities, interfaces, data models,
failure behavior, security assumptions, and important choices. The design must
state the benefits and disadvantages of each important choice. The project
should keep records of important decisions and rejected alternatives.

Exit condition: the design addresses accepted requirements and states known
risks and unresolved decisions.

### Engineering policies

Policies define rules that apply to multiple agent tasks. Policies can
define rules for coding, dependencies, compatibility, testing, security, system
monitoring, documentation, and release practices.

Exit condition: the project identifies the applicable policies. Each policy
states whether a tool or a person checks compliance. The project records the
reason for each exception and the required approval.

### Implementation plan

The plan divides the design into ordered agent tasks with defined scopes. Each
agent task states its dependencies, affected components, required context,
expected engineering assets, and verification steps. Each task must also state
completion criteria that link to the applicable requirement acceptance criteria.

Exit condition: the plan is feasible and links to the authoritative documents
that require the work. The plan identifies relevant risks and gives an agent
enough information to perform the work without making undocumented architecture
decisions.

### Verification plan

The verification plan defines how the project tests and verifies the proposed
change. The plan must link each test or review method to the applicable
requirement and acceptance criteria. It must identify the test type, commands,
environment, required inputs, expected results, and the person or tool that
performs the verification.

Exit condition: the verification plan covers every applicable acceptance
criterion and contains enough information for a person or tool to perform the
verification.

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

Exit condition: all mandatory checks pass, or a person with the required
decision role explicitly approves each exception.

### Verification record

The verification record contains the actual evidence from testing and
verification. It must contain the commands, environment, inputs, expected
results, actual results, pass or fail status, failures, and approved exceptions.
It must link each result to the applicable requirement, acceptance criterion,
verification plan item, and agent task.

Exit condition: the verification record contains enough information for a
person or tool to evaluate each acceptance criterion and reproduce each
repeatable check.

### Change summary

The change summary provides a concise description of a proposed software change.
The change summary must be suitable for direct use as a pull request (PR)
description without rewriting the content.

The change summary must contain the following sections:

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
| Implementation record | The implemented changes, affected components, and deviations from the accepted design and plan. |
| Verification record | Recorded results that show whether the work satisfied the applicable acceptance criteria. |
| Change summary | A concise list of changes, implementation notes, verification results, risks, and unresolved work that is suitable for a PR description. |
| Release record | The released software version and its supported operating environment. |

A later design document must define the file formats and metadata schema. After
the project accepts that design, each engineering asset must have a permanent
identifier, a defined lifecycle status, a named owner, a revision history, and
links to related assets. Each link must state the type of relationship. Each
authoritative document must also identify its purpose, intended audience, scope,
and the software or decisions to which it applies.

Before the project accepts the metadata schema, each engineering asset must have
a repository path or other stable location. Each authoritative document must
state its status and document type and must make its purpose and scope clear.
The version-control history provides the temporary revision history. Documents
may use repository paths and section names for temporary links.

For each requirement, the project should maintain one authoritative version.
The project should mark an obsolete document as replaced or remove the obsolete
document.

When work has greater risk or more unresolved questions, the required detail in
a design document must increase. The document must state the context, goals,
non-goals, proposed design, important benefits and disadvantages, considered
alternatives, and unresolved questions. When a listed subject applies to the
design, the document must address that subject. These subjects include
security, privacy, reliability, system monitoring and diagnosis, accessibility,
use in different languages and regions, migration, and system operation. A
small or simple change may use a short design record. The change must still have
an authoritative document that requires or permits the change and must have
acceptance criteria.

## Temporary document priority order

Until the project accepts complete rules for decision roles and document
priority, the system must report conflicts and resolve them by using this
priority order:

1. The current Daoji project charter
2. Accepted intent and requirements
3. Accepted design decisions
4. Applicable engineering policies
5. Accepted implementation plans
6. Agent instructions and work results
7. Generated implementation details

Verification evidence remains important although the priority order does not
include evidence. Evidence shows whether work satisfies the applicable
requirements and acceptance criteria. Evidence alone does not change those
requirements.

## Planned change process

For each proposed software change, the project must perform the following
process:

1. Identify the affected authoritative documents.
2. Create or update the intent, requirements, design, applicable engineering
   policies, implementation plan, and verification plan. The implementation
   plan must contain the agent task specifications.
3. Determine whether the proposal contains all required information and does
   not contradict itself.
4. When a required decision or required information is missing, request a
   decision from a person.
5. Accept the intent, requirements, design, implementation plan, verification
   plan, and any new or changed engineering policies before implementation
   starts.
6. Assign tasks to agents and collect structured results.
7. Create or update the implementation record.
8. Run policy checks, builds, tests, and other verification procedures.
9. Create the verification record and link its evidence to the applicable
    acceptance criteria and agent tasks.
10. Create or update the change summary.
11. Reject, revise, or accept the change.
12. Store the updated authoritative documents and evidence in version control.

Daoji must support or automate this process at the applicable self-development
level. If the current Daoji level does not support a step, people and
general-purpose tools must perform that step.

When new information shows that an assumption is incorrect, the process must
permit a return to an earlier stage.

## Daoji self-development

Daoji should gradually use its own workflow to develop itself. The following
levels define progress toward that capability.

### Level 0: Manual documentation

People create and review the charter and documents created from the charter's
requirements. General-purpose tools and agents may assist, but Daoji has no
executable implementation.

### Level 1: Document validation

A minimal tool validates engineering asset structure, identifiers, links,
statuses, and basic policy rules for the Daoji repository.

### Level 2: Plan and task creation

Daoji creates agent tasks with defined scopes from accepted engineering assets.
Daoji produces task specifications that do not depend on
one AI agent provider. People approve plans and assign tasks to agents.

### Level 3: Coordinated execution

Daoji invokes one or more interfaces for AI agents. Daoji limits each
agent's context and task scope, collects the results, and runs verification
commands defined in the repository.

### Level 4: Development controlled by Daoji

Daoji uses its own workflow for routine development. Each Daoji change links
its intent, requirements, design, implementation, verification, and release
records. People continue to approve important decisions at defined approval
checkpoints.

Daoji must not be the only system that verifies Daoji. Daoji's own check results
are not sufficient proof that Daoji is correct. A person or an
independent tool must review the initial implementation components, schemas,
policy changes, and verification mechanisms. The amount of review must increase
with the risk.

## Initial engineering policies

Before implementation-specific policies exist, the following project rules
apply:

1. Do not select a technology only to start coding. An accepted requirement or
   design decision must justify the choice.
2. Keep requirements separate from examples and explanatory text.
3. After the project defines the identifier scheme, give each engineering asset
   a permanent identifier.
4. Record uncertainty, missing information, and unresolved decisions. Do not add
   assumptions that a person has not approved.
5. Prefer coordination and validation processes that produce the same result for
   the same input and that reviewers can inspect.
6. Until model output passes the relevant checks, treat the output as untrusted
   input.
7. Unless an accepted design and policy require access to secrets and specify
   controls for that access, never expose secrets to an agent.
8. The core model must not depend on one AI agent provider. Put
   provider-specific behavior in separate provider interfaces.
9. Define each failure condition and its recovery procedure. Do not mark
   incomplete agent work as accepted.
10. When implementation behavior changes, the same proposed change must include
    the required documentation and verification updates.
11. Before writing a document that is intended to become authoritative, identify
    its audience and purpose.
12. Review documents for technical accuracy, audience clarity, and consistent
    language. Increase the review effort when errors in a document can cause
    greater harm or when the document affects more users or components.
13. Documents must contain all information required for their stated purpose,
    but no unnecessary information.
14. Track documentation defects and outdated documents as engineering defects.
15. Maintain one authoritative document for each requirement. Unless a tool
    generates copies from the authoritative document, do not create duplicate
    requirements.

## First documentation cycle

The first documentation cycle should produce the following engineering assets:

1. An intent document for the first milestone in which Daoji helps to develop
   Daoji.
2. Functional and quality requirements for Level 1 document validation.
3. A domain model for engineering assets, links, status transitions, and
   evidence.
4. Architecture decisions for repository layout, document formats, validation,
   and the initial command-line interface.
5. Project engineering policies.
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
- How should documents record their priority, approval status, and replacement
  by newer documents?
- How should Daoji distinguish outdated assets from intentionally historical
  records?
- What is the smallest useful Level 1 validator?
- For which operations must a person give approval by default?
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
do not control Daoji. Only requirements in accepted Daoji engineering assets
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
