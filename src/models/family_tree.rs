use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use serde::{Serialize, Deserialize};
use crate::models::{person::Person, relationship::Relationship};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyTree {
    #[serde(skip)]
    graph: DiGraph<Person, Relationship>,
    #[serde(skip)]
    root: Option<NodeIndex>,
}

impl FamilyTree {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            root: None,
        }
    }

    pub fn add_person(&mut self, person: Person) -> NodeIndex {
        let index = self.graph.add_node(person);
        if self.root.is_none() {
            self.root = Some(index);
        }
        index
    }

    pub fn add_relationship(&mut self, from: NodeIndex, to: NodeIndex, relationship: Relationship) {
        self.graph.add_edge(from, to, relationship);
    }

    pub fn get_person(&self, index: NodeIndex) -> Option<&Person> {
        self.graph.node_weight(index)
    }

    pub fn get_relationships(&self, person_id: uuid::Uuid) -> Vec<(&Person, &Relationship)> {
        let node_index = match self.graph.node_indices()
            .find(|&i| self.graph[i].id == person_id) {
            Some(index) => index,
            None => return Vec::new(),
        };
        
        self.graph.edges(node_index)
            .map(|edge| (self.graph.node_weight(edge.target()).unwrap(), edge.weight()))
            .collect()
    }
} 