# Daoji 文档地图

> 本文档是非权威的简体中文翻译。请参阅[英文原文](../../../DOCUMENT_MAP.md)；如有语义冲突，以英文原文为准。

状态：生效草案 0

文档类型：仓库地图

## 目的

本地图是面向人和智能体的主要文档入口。先从治理文档开始，再只访问当前决策或任务所需的路径。除非文档另有说明，否则以英文文档为权威版本。

## 治理文档

| 路径 | 作用 | 从此处开始的场景 |
| --- | --- | --- |
| [`docs/PROJECT_CHARTER.md`](PROJECT_CHARTER.md) | 最高优先级的项目目的、原则和规则 | 处理权威性、范围或项目级冲突 |
| [`docs/DOCUMENTATION_STANDARD.md`](DOCUMENTATION_STANDARD.md) | 文档结构、长度、链接、索引、写作和评审的强制规则 | 创建、拆分、链接、评审或维护文档 |
| [`docs/DEVELOPMENT_LIFECYCLE_STANDARD.md`](DEVELOPMENT_LIFECYCLE_STANDARD.md) | 软件变更所需的阶段、资产、验证和退出条件 | 规划或执行项目变更 |

## 支持路径

| 路径 | 作用 | 权威性 |
| --- | --- | --- |
| [`README.md`](README.md) | 项目介绍和仓库概览 | 资料性入口 |
| [`docs/references/`](references/) | 已采用配置和资料性外部参考 | 每份文档分别声明其状态 |
| [`docs/translation/zh-CN/`](.) | 项目文档的简体中文视图 | 非权威译文 |
| [`docs/changes/`](../../changes/) | 变更条目、计划、记录和摘要 | 每份文档分别声明其状态 |
| [`docs/design/`](../../design/) | 架构和重要技术决策 | 每份文档分别声明其状态 |
| [`docs/policies/`](../../policies/) | 可复用工程规则和质量门禁 | 每份文档分别声明其状态 |

## 当前权威文档图

项目章程要求项目遵循文档标准和开发生命周期标准。文档标准采用仓库中的 Google 风格指南配置作为较低优先级的编辑指导。开发生命周期标准将章程和文档规则应用于每项变更，并将变更条目链接到其详细生命周期资产。

初始脚手架变更将其意图、需求、计划、记录和摘要合并在
[`docs/changes/0001-engineering-scaffold.md`](../../changes/0001-engineering-scaffold.md)
中。该条目链接到单独的架构决策和 Rust 工程政策。移动、替换或删除已映射路径的
变更必须更新本地图。

相关简体中文译文：

- [工程脚手架变更](changes/0001-engineering-scaffold.md)
- [初始 Rust 架构](design/0001-initial-rust-architecture.md)
- [Rust 工程政策](policies/RUST_ENGINEERING_POLICY.md)
