# Volya — 项目设计文档

> **Volya** 是一个轻量级、现代化的 AI 驱动小说创作平台，基于 Tauri v2 + Vue 3 + TypeScript + Rust 构建。
> 以 AI Agent 为主导实现交互式创作，通过图结构（Graph）管理故事元素，支持权限控制的 Agent 协作工作流。

## 文档索引

| 文档 | 说明 |
|------|------|
| [01-architecture.md](./01-architecture.md) | 整体架构概览、技术选型、模块划分 |
| [02-data-model.md](./02-data-model.md) | 数据模型：Work/WorkData/Meta/Graph/Node/Edge |
| [03-agent-system.md](./03-agent-system.md) | Agent 系统：NovelAgent/Crew/Gate/Tool |
| [04-llm-integration.md](./04-llm-integration.md) | LLM 集成：Anthropic API/配置管理/流式通信 |
| [05-ipc-layer.md](./05-ipc-layer.md) | IPC 层：Tauri Commands/事件系统 |
| [06-frontend.md](./06-frontend.md) | 前端架构：组件树/状态管理/主题系统 |
| [07-development.md](./07-development.md) | 开发指南：环境搭建/构建命令/代码规范 |

## 项目概览

```
volya/
├── src/                          # Vue 3 前端（渲染层）
│   ├── components/               #  UI 组件
│   ├── composables/              #  组合式 API
│   └── styles/                   #  全局样式 / CSS 变量
├── src-tauri/
│   └── src/
│       ├── workspace/            #  作品数据管理
│       ├── graph/                #  故事图谱引擎
│       ├── agent/                #  Agent 系统
│       │   ├── llm/              #  LLM 集成
│       │   ├── gate.rs           #  权限控制
│       │   ├── tool.rs           #  工具系统
│       │   ├── novel_agent.rs    #  核心 Agent
│       │   └── session.rs        #  会话管理
│       └── ipc/                  #  前后端通信
├── docs/                         #  项目文档
├── package.json
└── Cargo.toml
```

## 核心概念

- **Work** — 作品，以文件夹形式存储在磁盘上
- **WorkData** — 作品运行时内存数据（Meta + Graph + 对话历史）
- **Graph** — 故事图谱（Node + Edge），核心创新点
- **NovelAgent** — 创作代理，用户与 AI 的交互入口
- **Gate** — 权限中间件（Lv0 仅建议 / Lv1 半自动 / Lv2 全自动）
- **Crew** — 子代理体系（规划中）
