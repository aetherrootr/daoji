# Daoji document map

Document type: Repository map

## Purpose

This map is the primary documentation entry point for people and agents. Start
with the governing documents, then follow only the paths needed for the current
decision or task. English documents at applicable paths in the authoritative
branch are authoritative according to their type and scope. Candidate assets in
a change proposal are not authoritative.

## Governing documents

| Path | Role | Start here when |
| --- | --- | --- |
| [`docs/PROJECT_CHARTER.md`](docs/PROJECT_CHARTER.md) | Highest-priority project purpose, principles, and rules | Resolving authority, scope, or a project-level conflict |
| [`docs/standards/DOCUMENTATION_STANDARD.md`](docs/standards/DOCUMENTATION_STANDARD.md) | Required structure, length, linking, indexing, writing, and review rules for documentation | Creating, splitting, linking, reviewing, or maintaining a document |
| [`docs/standards/DEVELOPMENT_LIFECYCLE_STANDARD.md`](docs/standards/DEVELOPMENT_LIFECYCLE_STANDARD.md) | Required software-change stages, assets, verification, and exit conditions | Planning or executing a project change |

## Supporting paths

| Path | Role | Authority |
| --- | --- | --- |
| [`README.md`](README.md) | Project introduction and repository overview | Informative entry point |
| [`docs/references/`](docs/references/) | Incorporated profiles and informative external references | Each document states its type and role |
| [`docs/translation/zh-CN/`](docs/translation/zh-CN/) | Simplified Chinese views of project documents | Non-authoritative translations |
| [`docs/changes/`](docs/changes/) | Change entries, plans, records, and summaries | Workflow state belongs to each change entry |
| [`docs/design/`](docs/design/) | Architecture and important technical decisions | Authority derives from the authoritative branch |
| [`docs/policies/`](docs/policies/) | Reusable engineering rules and quality gates | Authority derives from the authoritative branch |

## Document graph

The project charter uses typed controlling relationships to require the
documentation and development lifecycle standards. The documentation standard
incorporates the repository Google style guide profile as lower-priority
editorial guidance. The development lifecycle
standard applies the charter and documentation rules to each change and links
change entries to their detailed lifecycle assets.

The initial scaffold change combines its intent, requirements, plans, records,
and summary in
[`docs/changes/0001-engineering-scaffold.md`](docs/changes/0001-engineering-scaffold.md).
Its separate architecture decision and Rust engineering policy are linked from
that entry. The
[`Daoji overall architecture`](docs/design/0002-overall-architecture.md) defines
the system's logical components, dependency rules, and generalization
boundaries without resolving deferred implementation decisions. A change that
moves, replaces, or removes a mapped path must update this map.


## Translations

The [Simplified Chinese document map](docs/translation/zh-CN/DOCUMENT_MAP.md)
provides a translated view of this index. Each translated document links to its
authoritative English source.
