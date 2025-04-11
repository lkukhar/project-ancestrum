use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDate;

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
    #[serde(skip)]
    graph: DiGraph<Person, Relationship>,
    #[serde(skip)]
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

    pub fn get_relationships(&self, person_id: Uuid) -> Vec<(&Person, &Relationship)> {
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