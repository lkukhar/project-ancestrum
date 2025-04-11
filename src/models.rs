use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDate;
use petgraph::graph::{DiGraph, NodeIndex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub id: Uuid,
    pub name: String,
    pub birth_date: Option<NaiveDate>,
    pub death_date: Option<NaiveDate>,
    pub gender: Gender,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyTree {
    graph: DiGraph<Person, Relationship>,
    root: Option<NodeIndex>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Relationship {
    Parent,
    Child,
    Sibling,
    Spouse,
}

impl FamilyTree {
    pub fn new() -> Self {
        FamilyTree {
            graph: DiGraph::new(),
            root: None,
        }
    }

    pub fn add_person(&mut self, person: Person) -> NodeIndex {
        let node_index = self.graph.add_node(person);
        if self.root.is_none() {
            self.root = Some(node_index);
        }
        node_index
    }

    pub fn add_relationship(&mut self, from: NodeIndex, to: NodeIndex, relationship: Relationship) {
        self.graph.add_edge(from, to, relationship);
    }

    pub fn get_person(&self, node_index: NodeIndex) -> Option<&Person> {
        self.graph.node_weight(node_index)
    }

    pub fn get_relationships(&self, node_index: NodeIndex) -> Vec<(NodeIndex, &Relationship)> {
        self.graph
            .edges(node_index)
            .map(|edge| (edge.target(), edge.weight()))
            .collect()
    }
} 