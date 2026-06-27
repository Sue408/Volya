# 数据模型

## 作品存储结构

每个作品是一个独立的文件夹，存储在磁盘上：

```
works/
├── 我的小说/
│   ├── settings.json        # 作品配置 / Meta
│   ├── graph.json           # 图数据 (Node + Edge)
│   └── interactions.jsonl   # AI 对话历史
├── 另一部作品/
│   └── ...
```

## WorkData (运行时)

```rust
pub struct WorkData {
    pub meta: WorkMeta,              // 作品元数据
    pub graph: Graph,                // 故事图谱
    pub interactions: Vec<Interaction>, // 对话历史
}
```

## WorkMeta

作品的字段型数据：

| 字段 | 类型 | 说明 |
|------|------|------|
| `title` | `String` | 作品名称 |
| `completed_words` | `u64` | 已完成字数 |
| `target_words` | `Option<u64>` | 目标字数 |
| `permission_level` | `u8` | 权限等级 (0/1/2) |
| `total_tokens` | `u64` | 累计 token 消耗 |
| `style_guide` | `Option<String>` | 文风要求 |
| `work_type` | `Option<String>` | 作品类型 |
| `audience` | `Option<String>` | 目标受众 |
| `status` | `WorkStatus` | Draft / InProgress / Completed |

## Graph (故事图谱)

核心创新——以图结构管理故事元素。

### Node (节点)

四大类，Entity 下含五子类：

```
Node
├── Entity
│   ├── Character  — 角色 (名称/年龄/性别/外貌/性格/背景/动机)
│   ├── Location   — 地点 (名称/描述/气候/意义/历史)
│   ├── Faction    — 势力 (名称/描述/意识形态/目标/成员)
│   ├── Item       — 物品 (名称/描述/类型/属性/持有者)
│   └── Concept    — 核心设定 (名称/描述/类别/规则)
├── Arc            — 故事弧 (标题/摘要/状态)
├── Event          — 事件 (标题/摘要/所属章节)
└── Chapter        — 章节 (标题/正文/字数/状态)
```

### Edge (关系边)

采用 **基础类型 + 自然语言描述** 的结合方式：

| 基础类型 | 说明 |
|---------|------|
| `Contains` | 包含关系 (arc → event, arc → chapter) |
| `Sequence` | 顺序关系 (event → event)，**必须构成 DAG** |
| `Creates` | 创建/产生关系 |
| `Influences` | 影响/作用关系 |
| `References` | 引用关系 |
| `RelatesTo` | 通用关联 (由 description 定义具体语义) |

```rust
pub struct Edge {
    pub id: EdgeId,
    pub source_id: NodeId,
    pub target_id: NodeId,
    pub edge_type: EdgeType,     // 基础类型
    pub description: String,     // 自然语言描述（如 "Alice 深爱着 Bob"）
}
```

### DAG 约束

Event 节点之间的 Sequence 边必须构成**有向无环图**（DAG）。在添加边时通过 BFS 检测环：

```rust
// 从 target 出发 BFS，如果能回到 source → 拒绝添加
fn would_create_cycle(adjacency, source, target) -> bool;
```

同时提供**拓扑排序**功能，用于确定事件的写作顺序：

```rust
pub fn topological_sort_events(&self) -> Result<Vec<NodeId>, GraphError>;
```

## Interaction (对话记录)

```rust
pub struct Interaction {
    pub role: String,     // "user" | "assistant"
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub tool_calls: Option<Vec<ToolCallRecord>>,
}
```

以 JSONL 格式存储在磁盘，每行一条 JSON。
