use crate::graph::edge::{Edge, EdgeId, EdgeType};
use crate::graph::node::{Node, NodeCategory, NodeId};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

/// ============================================================
/// Graph — 作品的图结构容器
/// 管理 Node 与 Edge 的 CRUD，并提供 DAG 校验
/// ============================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graph {
    nodes: HashMap<NodeId, Node>,
    edges: HashMap<EdgeId, Edge>,
    /// 邻接表: 从 source 出发的边的目标
    adjacency: HashMap<NodeId, Vec<(EdgeId, NodeId)>>,
    /// 入度表: 指向某个节点的边
    in_degree: HashMap<NodeId, Vec<(EdgeId, NodeId)>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            adjacency: HashMap::new(),
            in_degree: HashMap::new(),
        }
    }

    // ─── Node CRUD ───

    pub fn add_node(&mut self, node: Node) {
        let id = node.id;
        self.nodes.insert(id, node);
        self.adjacency.entry(id).or_default();
        self.in_degree.entry(id).or_default();
    }

    pub fn remove_node(&mut self, node_id: &NodeId) -> Option<Node> {
        // 移除所有关联的边
        let related_edges: Vec<EdgeId> = self
            .edges
            .iter()
            .filter(|(_, e)| e.source_id == *node_id || e.target_id == *node_id)
            .map(|(id, _)| *id)
            .collect();
        for eid in related_edges {
            self.remove_edge(&eid);
        }

        let node = self.nodes.remove(node_id);
        self.adjacency.remove(node_id);
        self.in_degree.remove(node_id);
        node
    }

    pub fn get_node(&self, node_id: &NodeId) -> Option<&Node> {
        self.nodes.get(node_id)
    }

    pub fn get_node_mut(&mut self, node_id: &NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(node_id)
    }

    pub fn all_nodes(&self) -> Vec<&Node> {
        self.nodes.values().collect()
    }

    pub fn nodes_by_category(&self, category: &NodeCategory) -> Vec<&Node> {
        self.nodes
            .values()
            .filter(|n| std::mem::discriminant(&n.category) == std::mem::discriminant(category))
            .collect()
    }

    // ─── Edge CRUD ───

    /// 添加边，如果是 Event→Event 则校验 DAG
    pub fn add_edge(&mut self, edge: Edge) -> Result<(), GraphError> {
        // 校验 source 和 target 节点存在
        if !self.nodes.contains_key(&edge.source_id) {
            return Err(GraphError::NodeNotFound(edge.source_id));
        }
        if !self.nodes.contains_key(&edge.target_id) {
            return Err(GraphError::NodeNotFound(edge.target_id));
        }

        // 如果是 Event→Event 的 Sequence 边，校验 DAG
        if edge.edge_type == EdgeType::Sequence {
            let source_is_event = matches!(
                self.nodes.get(&edge.source_id).map(|n| &n.category),
                Some(NodeCategory::Event)
            );
            let target_is_event = matches!(
                self.nodes.get(&edge.target_id).map(|n| &n.category),
                Some(NodeCategory::Event)
            );
            if source_is_event && target_is_event {
                // 先"假装"添加，再校验环
                let source = edge.source_id;
                let target = edge.target_id;
                if would_create_cycle(&self.adjacency, source, target) {
                    return Err(GraphError::CycleDetected);
                }
            }
        }

        let id = edge.id;
        self.edges.insert(id, edge.clone());
        self.adjacency
            .entry(edge.source_id)
            .or_default()
            .push((id, edge.target_id));
        self.in_degree
            .entry(edge.target_id)
            .or_default()
            .push((id, edge.source_id));

        Ok(())
    }

    pub fn remove_edge(&mut self, edge_id: &EdgeId) -> Option<Edge> {
        let edge = self.edges.remove(edge_id)?;

        if let Some(adj) = self.adjacency.get_mut(&edge.source_id) {
            adj.retain(|(id, _)| id != edge_id);
        }
        if let Some(ind) = self.in_degree.get_mut(&edge.target_id) {
            ind.retain(|(id, _)| id != edge_id);
        }

        Some(edge)
    }

    pub fn get_edge(&self, edge_id: &EdgeId) -> Option<&Edge> {
        self.edges.get(edge_id)
    }

    pub fn all_edges(&self) -> Vec<&Edge> {
        self.edges.values().collect()
    }

    /// 获取某个节点的所有出边
    pub fn outgoing_edges(&self, node_id: &NodeId) -> Vec<&Edge> {
        self.adjacency
            .get(node_id)
            .map(|adj| {
                adj.iter()
                    .filter_map(|(eid, _)| self.edges.get(eid))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// 获取某个节点的所有入边
    pub fn incoming_edges(&self, node_id: &NodeId) -> Vec<&Edge> {
        self.in_degree
            .get(node_id)
            .map(|ind| {
                ind.iter()
                    .filter_map(|(eid, _)| self.edges.get(eid))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// 获取 Event 节点的拓扑排序（用于确定写作顺序）
    pub fn topological_sort_events(&self) -> Result<Vec<NodeId>, GraphError> {
        let event_ids: HashSet<NodeId> = self
            .nodes
            .iter()
            .filter(|(_, n)| matches!(n.category, NodeCategory::Event))
            .map(|(id, _)| *id)
            .collect();

        if event_ids.is_empty() {
            return Ok(vec![]);
        }

        // Kahn 算法
        let mut in_degree_count: HashMap<NodeId, usize> = HashMap::new();
        for &eid in &event_ids {
            in_degree_count.entry(eid).or_insert(0);
        }
        for (&target, sources) in &self.in_degree {
            if event_ids.contains(&target) {
                for (_, src) in sources {
                    if event_ids.contains(src) {
                        *in_degree_count.entry(target).or_insert(0) += 1;
                    }
                }
            }
        }

        let mut queue: VecDeque<NodeId> = in_degree_count
            .iter()
            .filter(|(_, &count)| count == 0)
            .map(|(id, _)| *id)
            .collect();

        let mut result = Vec::new();
        while let Some(node) = queue.pop_front() {
            result.push(node);
            if let Some(adj) = self.adjacency.get(&node) {
                for &(_, target) in adj {
                    if event_ids.contains(&target) {
                        if let Some(count) = in_degree_count.get_mut(&target) {
                            *count -= 1;
                            if *count == 0 {
                                queue.push_back(target);
                            }
                        }
                    }
                }
            }
        }

        if result.len() != event_ids.len() {
            return Err(GraphError::CycleDetected);
        }

        Ok(result)
    }
}

/// 检测添加边 source → target 是否会形成环
fn would_create_cycle(
    adjacency: &HashMap<NodeId, Vec<(EdgeId, NodeId)>>,
    source: NodeId,
    target: NodeId,
) -> bool {
    // 从 target 出发 BFS，看是否能回到 source
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(target);
    visited.insert(target);

    while let Some(current) = queue.pop_front() {
        if current == source {
            return true;
        }
        if let Some(neighbors) = adjacency.get(&current) {
            for &(_, neighbor) in neighbors {
                if visited.insert(neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }
    }
    false
}

#[derive(Debug, thiserror::Error)]
pub enum GraphError {
    #[error("Node {0} not found")]
    NodeNotFound(NodeId),
    #[error("Cannot add edge: would create a cycle")]
    CycleDetected,
}
