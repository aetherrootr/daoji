# Daoji documentation standard

Status: Effective draft 0

Document type: Documentation standard

Required use: The [Daoji project charter](PROJECT_CHARTER.md) requires all
project-maintained documentation to follow this standard. The charter has
priority if the documents conflict.

## Purpose

This standard defines how Daoji documentation is written, divided, linked,
indexed, reviewed, and maintained. It treats documentation as an engineered
system for people, agents, and validation tools.

This standard applies to authoritative documents, translations, repository
guides, operational instructions, and documentation embedded in other
engineering assets. Generated or imported documentation must comply, or its
controlling document must record the reason and provide a compliant project
entry point.

## Core requirements

Project documentation must have the following properties:

- **Authoritative:** A reader can identify which document controls a decision.
- **Purposeful:** Each document has a stated purpose, audience, scope, status,
  and document type.
- **Concise:** The document contains enough information for its purpose and no
  unrelated detail.
- **Linked:** The document links to controlling and related assets instead of
  duplicating their content.
- **Navigable:** Stable headings, descriptive links, and the repository map let
  readers locate details progressively.
- **Verifiable:** Requirements, procedures, and claims identify acceptance
  criteria or evidence when verification is applicable.
- **Maintainable:** Ownership, replacement, and update responsibilities are
  clear enough to prevent conflicting copies and stale guidance.

An authoritative document must provide the same controlling information to
people and agents. Metadata may help tools interpret a document, but it must not
conceal controlling information from people. The project must not maintain
independent human and agent versions that can become inconsistent. A generated
view is permitted when one authoritative source produces all views.

## Document structure

Each authoritative document must identify its status and document type near the
start. It must make its purpose, intended audience, scope, and applicable
software or decisions explicit. Until the project accepts a metadata schema,
prose and stable repository paths satisfy this requirement.

A document must place the information needed to decide whether to continue
reading before supporting detail. When applicable, use this order:

1. Purpose and scope.
2. Summary or required outcome.
3. Normative rules or task instructions.
4. Verification or exit criteria.
5. Links to detailed or related documents.
6. Informative rationale and sources.

Use headings that describe their content and preserve stable link targets.
Introduce a heading when more than three or four paragraphs address distinct
subtopics. A section should remain understandable when a reader follows a
direct link to it.

Keep requirements separate from examples and explanatory text. Examples may
clarify a rule but do not replace the rule. Mark informative material when a
reader could otherwise mistake it for a requirement.

## Length and attention budgets

Length limits are engineering defaults for focused use. They are not universal
limits of human cognition or AI context. Authors must optimize for successful
reading and task completion, not for a word count alone.

For English project documents, use the following default budgets:

| Content unit | Target | Required action above target |
| --- | --- | --- |
| Task-focused document or independently linked section | About 500-1,200 words | Add navigation and assess whether detail belongs in a separate document. |
| Quickstart | About 800-1,500 words | Split setup, first success, and next steps when they are independent tasks. |
| Concept or architecture overview | About 1,200-2,500 words | Keep an overview and move detailed decisions or component descriptions to linked documents. |
| Any prose document | At most 2,500 words by default; 4,000 words maximum | Record a split assessment above the default; split the document or approve an exception above the maximum. |

The budgets exclude machine-generated reference tables, schemas, and retained
evidence that readers locate selectively. These assets still require a concise
entry point and usable navigation. Dense security, legal, or unfamiliar
technical material should use shorter sections than the default.

A document that exceeds 1,200 words should include a summary and local
navigation when headings alone do not make its structure obvious. A document
that exceeds 2,500 words must record a split assessment near its metadata before
acceptance. The assessment must identify independently useful detail and either
split that detail or explain why readers benefit from one document. A document
that exceeds 4,000 words must do one of the following:

- Split detailed subjects into independently useful linked documents.
- Demonstrate that readers must consume the content as one unit, and record the
  reason, intended reading pattern, and review approval near the document
  metadata as a length exception.

Translations should provide information units comparable to the authoritative
source instead of enforcing an identical word or character count.

The project must not convert these human reading budgets into an AI token limit.
An agent context budget depends on the model, task, retrieval method, tool
history, and location of relevant information. A policy that sets a numeric
agent context limit must validate that limit on the applicable models and tasks.
The validation should test retrieval and task success with relevant information
at different positions and with irrelevant material present.

## Progressive disclosure and document graph

Documentation must form a directed graph of meaningful relationships. A
document must link to a controlling document and to the next level of detail
when those documents exist. Link text must identify the destination and, when
the relationship is not obvious, explain why the reader should follow it.

Authors must prefer a link to an authoritative section over copied text. A
short summary may repeat the minimum context needed to decide whether to follow
the link. The summary must not create an independent requirement or omit a
constraint that changes the meaning of the linked source.

Authors must separate detail into another document when one or more of the
following conditions apply:

- The detail has a different audience, purpose, owner, or update cycle.
- The detail is independently reusable or addressable.
- The detail obscures the current document's primary decision or task.
- The document exceeds the maximum length and has no approved exception.
- A reader or agent normally needs the detail only under a specific condition.

The original document must retain a necessary summary, state the condition for
following the link, and link directly to the relevant section when possible.
The linked document must link back to its controlling or parent context. The
project must detect and resolve authoritative documents that cannot be reached
from the repository map or another mapped authoritative document.

Links must use repository-relative paths for repository files. Authors must
update affected links in the same change that moves, renames, replaces, or
removes a document. A replacement document must identify the document that it
replaces when history alone does not make the relationship clear.

## Repository document map

The project must maintain [`DOCUMENT_MAP.md`](../DOCUMENT_MAP.md) at the
repository root as the primary documentation index for people and agents. The
map is an entry point, not a duplicate table of contents for every file.

The map must:

- Link to the highest-priority governing documents.
- Identify key paths for lifecycle work, engineering policies, designs,
  operations, references, translations, and retained records when they exist.
- State the purpose of each mapped path and which readers should start there.
- Distinguish authoritative documents from translations and informative
  references.
- Remain concise enough to scan in one focused reading unit.

A change that adds, moves, replaces, or removes a key path must update the map
in the same change. A reviewer must check map reachability and changed links
before accepting a documentation change.

## English writing rules

The project initially writes authoritative documents in English. When a
translation has a different meaning, the English document is authoritative.
Use a controlled technical style inspired by ASD Simplified Technical English
and structured technical publications. The project must not claim compliance
with an external standard unless an accepted policy defines and verifies that
compliance.

The following rules apply:

- Use US English and one term for one concept.
- Define project-specific terms and abbreviations on first use.
- Use short, direct sentences with one primary requirement or idea.
- Prefer subject-verb-object order, active voice, and present tense.
- Identify the actor responsible for an action.
- Put a condition before the instruction that depends on it.
- Make scope, conditions, exceptions, and priority explicit.
- Avoid ambiguous pronouns, unnecessary jargon, idioms, humor, and
  culture-specific references.
- Use sentence case for titles and headings.
- Use parallel structure, capitalization, and punctuation in lists.
- Use inclusive and accessible language.
- Use `must` for requirements, `should` for recommendations, `may` for
  permission, and `will` only for future events.
- Prefer wording that preserves meaning under machine translation.

Clarity takes priority over stylistic variety. The adopted
[Google developer documentation style guide profile](references/GOOGLE_DEVELOPER_DOCUMENTATION_STYLE_GUIDE.md)
provides additional editorial rules.

Authors and reviewers must apply documentation rules in this order:

1. The project charter.
2. This documentation standard.
3. Applicable accepted project policies and document-type standards.
4. The repository Google style guide profile.
5. Other external references.

The higher-priority rule controls a conflict. The project must record the reason
for a deliberate exception to an applicable rule.

## Review and maintenance

Before acceptance, a documentation change must be reviewed for:

- Technical accuracy and consistency with controlling documents.
- Clear audience, purpose, scope, status, and normative language.
- Compliance with the length budget or an approved exception.
- Appropriate summaries, splits, and progressively disclosed links.
- Valid links and reachability from the repository map.
- Accessibility, translatability, and consistent terminology.
- Updated related documents, translations, and replacement notices when
  applicable.

Review effort must increase with the harm that an error can cause and the number
of users, agents, or components that the document controls. Documentation
defects and outdated documents are engineering defects. The project should keep
one authoritative statement for each requirement and remove or mark obsolete
copies unless a tool generates them from that statement.

## Basis for the length defaults

The length budgets are adjustable project defaults. They use a planning rate
below the approximately 238 words per minute reported for average English
nonfiction silent reading because technical readers stop, inspect examples, and
make decisions. Research also associates task-unrelated thought with lower
reading comprehension, but does not establish one universal attention limit or
one optimal document length.

The project used the following informative sources:

- [How many words do we read per minute? A review and meta-analysis of reading rate](https://doi.org/10.1016/j.jml.2019.104047),
  for the English nonfiction reading-rate baseline.
- [Mind wandering and reading comprehension: A meta-analysis](https://doi.org/10.3758/s13423-022-02141-w),
  for the relationship between task-unrelated thought and comprehension.
- [Attention span during lectures: 8 seconds, 10 minutes, or more?](https://doi.org/10.1152/advan.00109.2016),
  for the warning against treating a short attention interval as a universal
  biological limit.

These sources inform the defaults but do not control the project. The project
must revise the budgets when user research, agent evaluation, or repository
metrics show that another structure produces better outcomes.
