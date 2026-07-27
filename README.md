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

Daoji is in its initial documentation phase. The repository intentionally
contains only documentation. The first implementation must use the
document-based process that Daoji is designed to provide.

The [Daoji project charter](docs/PROJECT_CHARTER.md) contains the initial
project requirements and rules.

## Daoji self-development goal

Daoji should gradually use its own workflow to plan, implement, and verify
changes to Daoji. The initial documents define the first executable version.
That version validates and coordinates later changes. The project
achieves this goal only when the workflow requires review by a person and
records each decision so that a person can audit it later.

## Repository layout

The repository contains the following files:

- `README.md`: introduces Daoji and the repository.
- `docs/PROJECT_CHARTER.md`: defines the initial project requirements and rules.
- `docs/references/GOOGLE_DEVELOPER_DOCUMENTATION_STYLE_GUIDE.md`: contains the
  version of the Google writing guidance that Daoji uses.

The project has not selected an implementation language, AI agent provider, or
runtime architecture. The project must document its requirements and design
before it selects these technologies.
