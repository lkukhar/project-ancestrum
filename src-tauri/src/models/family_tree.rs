use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;
use super::{Person, Relationship};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct FamilyTree {
    #[serde(skip)]
    graph: DiGraph<Person, Relationship>,
    #[serde(skip)]
    id_to_index: HashMap<String, NodeIndex>,
}

impl FamilyTree {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            id_to_index: HashMap::new(),
        }
    }

    pub fn add_person(&mut self, person: Person) -> Result<Person, String> {
        let id = person.id.clone();
        let node_index = self.graph.add_node(person);
        self.id_to_index.insert(id.clone(), node_index);
        Ok(self.graph[node_index].clone())
    }

    pub fn get_person(&self, id: &str) -> Option<&Person> {
        self.id_to_index.get(id).map(|&index| &self.graph[index])
    }

    pub fn update_person(&mut self, id: &str, person: Person) -> Result<Person, String> {
        let index = self.id_to_index.get(id)
            .ok_or_else(|| format!("Person with id {} not found", id))?;
        self.graph[*index] = person.clone();
        Ok(person)
    }

    pub fn delete_person(&mut self, id: &str) -> Result<(), String> {
        let index = self.id_to_index.remove(id)
            .ok_or_else(|| format!("Person with id {} not found", id))?;
        self.graph.remove_node(index);
        Ok(())
    }

    pub fn add_relationship(
        &mut self,
        from_id: &str,
        to_id: &str,
        relationship: Relationship,
    ) -> Result<(), String> {
        let from_index = self.id_to_index.get(from_id)
            .ok_or_else(|| format!("Person with id {} not found", from_id))?;
        let to_index = self.id_to_index.get(to_id)
            .ok_or_else(|| format!("Person with id {} not found", to_id))?;
        
        self.graph.add_edge(*from_index, *to_index, relationship);
        Ok(())
    }
} 