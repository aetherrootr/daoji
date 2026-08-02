# Daoji document map

Status: Effective draft 0

Document type: Repository map

## Purpose

This map is the primary documentation entry point for people and agents. Start
with the governing documents, then follow only the paths needed for the current
decision or task. English documents are authoritative unless a document states
otherwise.

## Governing documents

| Path | Role | Start here when |
| --- | --- | --- |
| [`docs/PROJECT_CHARTER.md`](docs/PROJECT_CHARTER.md) | Highest-priority project purpose, principles, and rules | Resolving authority, scope, or a project-level conflict |
| [`docs/DOCUMENTATION_STANDARD.md`](docs/DOCUMENTATION_STANDARD.md) | Required structure, length, linking, indexing, writing, and review rules for documentation | Creating, splitting, linking, reviewing, or maintaining a document |
| [`docs/DEVELOPMENT_LIFECYCLE_STANDARD.md`](docs/DEVELOPMENT_LIFECYCLE_STANDARD.md) | Required software-change stages, assets, verification, and exit conditions | Planning or executing a project change |

## Supporting paths

| Path | Role | Authority |
| --- | --- | --- |
| [`README.md`](README.md) | Project introduction and repository overview | Informative entry point |
| [`docs/references/`](docs/references/) | Adopted profiles and informative external references | Each document states its status |
| [`docs/translation/zh-CN/`](docs/translation/zh-CN/) | Simplified Chinese views of project documents | Non-authoritative translations |

## Current authoritative graph

The project charter requires the documentation and development lifecycle
standards. The documentation standard adopts the repository Google style guide
profile as lower-priority editorial guidance. The development lifecycle
standard applies the charter and documentation rules to each change and links
change entries to their detailed lifecycle assets.

The repository does not yet contain separate requirement, design, policy,
operation, or change-record directories. Add those key paths here when the
project creates them. A change that moves, replaces, or removes a mapped path
must update this map.

## Translations

The [Simplified Chinese document map](docs/translation/zh-CN/DOCUMENT_MAP.md)
provides a translated view of this index. Each translated document links to its
authoritative English source.
