# 开发指南

## 环境要求

| 工具 | 版本要求 | 说明 |
|------|---------|------|
| Node.js | >= 20 | 前端构建 |
| pnpm | >= 9 | 包管理器 |
| Rust | >= 1.75 | 后端编译 |
| Tauri CLI | 2.x | Tauri 工具链 |

## 快速开始

```bash
# 1. 安装依赖
pnpm install

# 2. 开发模式（含热重载）
pnpm tauri dev

# 3. 生产构建
pnpm tauri build
```

## 项目命令

| 命令 | 说明 |
|------|------|
| `pnpm dev` | 启动 Vite 开发服务器（仅前端） |
| `pnpm build` | 前端构建（TypeScript 检查 + Vite 打包） |
| `pnpm tauri dev` | 启动 Tauri 开发模式（前后端 + 桌面窗口） |
| `pnpm tauri build` | 生产构建（打包为可安装程序） |
| `cd src-tauri && cargo check` | 仅检查 Rust 编译 |
| `cd src-tauri && cargo build` | 仅编译 Rust 后端 |

## 项目结构规范

```
src/                    # Vue 前端
  components/           #   UI 组件（PascalCase.vue）
  composables/          #   组合式 API（useXxx.ts）
  styles/               #   样式文件

src-tauri/src/          # Rust 后端
  workspace/            #   作品数据
  graph/                #   图谱引擎
  agent/                #   Agent 系统
    llm/                #     LLM 集成
  ipc/                  #   通信层
```

## 开发规范

### Rust

- 使用 `thiserror` 定义错误类型
- 使用 `async-trait` 实现异步 trait
- Command 参数使用 snake_case（Tauri 自动转换 camelCase）
- 事件名使用 `agent:xxx` 命名空间

### TypeScript / Vue

- 使用 `<script setup lang="ts">` 语法
- Composition API / composable 模式
- 事件监听使用 `listen()` + 清理函数模式
- CSS 变量使用 `var(--xxx)` 引用主题变量

## Git 规范

```
<type>: <简短描述>

<详细说明（可选）>

<footer（可选）>
```

类型：`feat` / `fix` / `docs` / `refactor` / `chore`

示例：
```
feat: 添加 Gate 权限中间件 Lv0~Lv2

实现三级权限控制：
- Lv0: 仅建议，所有修改需审批
- Lv1: 半自动，危险操作需审批
- Lv2: 全自动
```
