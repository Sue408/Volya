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

// ─── 单元测试 ───
#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::node::{EntityKind, NodeData};

    fn make_node(name: &str, category: NodeCategory) -> Node {
        let data = match &category {
            NodeCategory::Entity(EntityKind::Character) => NodeData::Character {
                name: name.to_string(), age: None, gender: None,
                appearance: None, personality: None, background: None, motivation: None,
            },
            NodeCategory::Event => NodeData::Event {
                title: name.to_string(), summary: None, chapter_id: None,
            },
            NodeCategory::Arc => NodeData::Arc {
                title: name.to_string(), summary: None, status: Default::default(),
            },
            NodeCategory::Chapter => NodeData::Chapter {
                title: name.to_string(), content: String::new(), word_count: 0, status: Default::default(),
            },
            _ => NodeData::Concept {
                name: name.to_string(), description: None, category: None, rules: None,
            },
        };
        Node::new(name, category, data)
    }

    #[test]
    fn test_graph_new_is_empty() {
        let g = Graph::new();
        assert!(g.all_nodes().is_empty());
        assert!(g.all_edges().is_empty());
    }

    #[test]
    fn test_add_and_get_node() {
        let mut g = Graph::new();
        let node = make_node("测试角色", NodeCategory::Entity(EntityKind::Character));
        let id = node.id;
        g.add_node(node);
        assert_eq!(g.all_nodes().len(), 1);
        assert!(g.get_node(&id).is_some());
        assert_eq!(g.get_node(&id).unwrap().name, "测试角色");
    }

    #[test]
    fn test_remove_node_also_removes_edges() {
        let mut g = Graph::new();
        let a = make_node("A", NodeCategory::Event);
        let b = make_node("B", NodeCategory::Event);
        let a_id = a.id;
        let b_id = b.id;
        g.add_node(a);
        g.add_node(b);
        let edge = Edge::new(a_id, b_id, EdgeType::Sequence, "A → B");
        g.add_edge(edge).unwrap();
        assert_eq!(g.all_edges().len(), 1);

        g.remove_node(&a_id);
        assert!(g.get_node(&a_id).is_none());
        assert_eq!(g.all_edges().len(), 0); // 关联边被清除
    }

    #[test]
    fn test_add_edge_missing_source() {
        let mut g = Graph::new();
        let b = make_node("B", NodeCategory::Event);
        let b_id = b.id;
        g.add_node(b);
        let edge = Edge::new(uuid::Uuid::new_v4(), b_id, EdgeType::RelatesTo, "");
        match g.add_edge(edge) {
            Err(GraphError::NodeNotFound(_)) => {},
            other => panic!("期望 NodeNotFound，得到: {:?}", other),
        }
    }

    #[test]
    fn test_event_sequence_cycle_detection() {
        let mut g = Graph::new();
        let a = make_node("A", NodeCategory::Event);
        let b = make_node("B", NodeCategory::Event);
        let c = make_node("C", NodeCategory::Event);
        let a_id = a.id;
        let b_id = b.id;
        let c_id = c.id;
        g.add_node(a);
        g.add_node(b);
        g.add_node(c);

        // A → B → C
        g.add_edge(Edge::new(a_id, b_id, EdgeType::Sequence, "")).unwrap();
        g.add_edge(Edge::new(b_id, c_id, EdgeType::Sequence, "")).unwrap();

        // C → A 应该形成环
        match g.add_edge(Edge::new(c_id, a_id, EdgeType::Sequence, "")) {
            Err(GraphError::CycleDetected) => {},
            other => panic!("期望 CycleDetected，得到: {:?}", other),
        }
    }

    #[test]
    fn test_topological_sort() {
        let mut g = Graph::new();
        let a = make_node("A", NodeCategory::Event);
        let b = make_node("B", NodeCategory::Event);
        let c = make_node("C", NodeCategory::Event);
        let a_id = a.id;
        let b_id = b.id;
        let c_id = c.id;
        g.add_node(a);
        g.add_node(b);
        g.add_node(c);

        g.add_edge(Edge::new(a_id, b_id, EdgeType::Sequence, "")).unwrap();
        g.add_edge(Edge::new(b_id, c_id, EdgeType::Sequence, "")).unwrap();

        let sorted = g.topological_sort_events().unwrap();
        assert_eq!(sorted.len(), 3);
        // A 必须在 B 之前，B 必须在 C 之前
        let pos = |id: &NodeId| sorted.iter().position(|x| x == id).unwrap();
        assert!(pos(&a_id) < pos(&b_id));
        assert!(pos(&b_id) < pos(&c_id));
    }

    #[test]
    fn test_outgoing_and_incoming_edges() {
        let mut g = Graph::new();
        let a = make_node("A", NodeCategory::Event);
        let b = make_node("B", NodeCategory::Event);
        let a_id = a.id;
        let b_id = b.id;
        g.add_node(a);
        g.add_node(b);
        g.add_edge(Edge::new(a_id, b_id, EdgeType::Influences, "A influences B")).unwrap();

        let out = g.outgoing_edges(&a_id);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].edge_type, EdgeType::Influences);

        let inc = g.incoming_edges(&b_id);
        assert_eq!(inc.len(), 1);
        assert_eq!(inc[0].description, "A influences B");
    }
}
