use serde::{Deserialize, Serialize};
use petgraph::graph::{DiGraph, NodeIndex};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub id: Uuid,
    pub name: String,
    pub birth_date: Option<DateTime<Utc>>,
    pub death_date: Option<DateTime<Utc>>,
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
pub enum Relationship {
    Parent,
    Child,
    Spouse,
    Sibling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyTree {
    graph: DiGraph<Person, Relationship>,
    indices: HashMap<Uuid, NodeIndex>,
}

impl FamilyTree {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            indices: HashMap::new(),
        }
    }

    pub fn add_person(&mut self, person: Person) -> Uuid {
        let id = person.id;
        let index = self.graph.add_node(person);
        self.indices.insert(id, index);
        id
    }

    pub fn get_person(&self, id: Uuid) -> Option<&Person> {
        self.indices.get(&id).map(|&index| &self.graph[index])
    }

    pub fn update_person(&mut self, id: Uuid, person: Person) -> bool {
        if let Some(&index) = self.indices.get(&id) {
            self.graph[index] = person;
            true
        } else {
            false
        }
    }

    pub fn delete_person(&mut self, id: Uuid) -> bool {
        if let Some(&index) = self.indices.get(&id) {
            self.graph.remove_node(index);
            self.indices.remove(&id);
            true
        } else {
            false
        }
    }

    pub fn add_relationship(&mut self, from: Uuid, to: Uuid, relationship: Relationship) -> bool {
        if let (Some(&from_index), Some(&to_index)) = (self.indices.get(&from), self.indices.get(&to)) {
            self.graph.add_edge(from_index, to_index, relationship);
            true
        } else {
            false
        }
    }

    pub fn get_relationships(&self, id: Uuid) -> Vec<(Uuid, Relationship)> {
        if let Some(&index) = self.indices.get(&id) {
            self.graph
                .edges(index)
                .map(|edge| {
                    let target = self.graph[edge.target()].id;
                    (target, edge.weight().clone())
                })
                .collect()
        } else {
            Vec::new()
        }
    }
} 