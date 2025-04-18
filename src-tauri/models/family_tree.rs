use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use std::collections::HashMap;
use uuid::Uuid;

use super::{Person, Gender, Relationship};

#[derive(Debug)]
pub struct FamilyTree {
    graph: DiGraph<Person, Relationship>,
    id_to_index: HashMap<String, NodeIndex>,
}

impl FamilyTree {
    pub fn new() -> Self {
        FamilyTree {
            graph: DiGraph::new(),
            id_to_index: HashMap::new(),
        }
    }

    pub fn add_person(&mut self, name: String) -> Result<String, String> {
        let id = Uuid::new_v4().to_string();
        let person = Person {
            id: id.clone(),
            name,
            birth_date: None,
            death_date: None,
            gender: Gender::Other,
            notes: String::new(),
        };
        let index = self.graph.add_node(person);
        self.id_to_index.insert(id.clone(), index);
        Ok(id)
    }

    pub fn get_person(&self, id: &str) -> Option<&Person> {
        self.id_to_index
            .get(id)
            .and_then(|&index| self.graph.node_weight(index))
    }

    pub fn update_person(
        &mut self,
        id: &str,
        name: String,
        birth_date: Option<String>,
        death_date: Option<String>,
        gender: Gender,
        notes: String,
    ) -> Result<(), String> {
        if let Some(&index) = self.id_to_index.get(id) {
            if let Some(person) = self.graph.node_weight_mut(index) {
                person.name = name;
                person.birth_date = birth_date;
                person.death_date = death_date;
                person.gender = gender;
                person.notes = notes;
                Ok(())
            } else {
                Err("Person not found in graph".to_string())
            }
        } else {
            Err("Person ID not found".to_string())
        }
    }

    pub fn delete_person(&mut self, id: &str) -> Result<(), String> {
        if let Some(&index) = self.id_to_index.get(id) {
            self.graph.remove_node(index);
            self.id_to_index.remove(id);
            Ok(())
        } else {
            Err("Person not found".to_string())
        }
    }

    pub fn add_relationship(
        &mut self,
        from_id: &str,
        to_id: &str,
        relationship: Relationship,
    ) -> Result<(), String> {
        let from_index = self.id_to_index
            .get(from_id)
            .ok_or_else(|| "From person not found".to_string())?;
        let to_index = self.id_to_index
            .get(to_id)
            .ok_or_else(|| "To person not found".to_string())?;
        
        self.graph.add_edge(*from_index, *to_index, relationship);
        Ok(())
    }

    pub fn get_relationships(&self, id: &str) -> Result<Vec<(String, String)>, String> {
        let index = self.id_to_index
            .get(id)
            .ok_or_else(|| "Person not found".to_string())?;
        
        let mut relationships = Vec::new();
        
        // Get outgoing edges (relationships where this person is the source)
        for edge in self.graph.edges(*index) {
            if let Some(person) = self.graph.node_weight(edge.target()) {
                relationships.push((
                    format!("{:?}", edge.weight()),
                    person.id.clone(),
                ));
            }
        }
        
        // Get incoming edges (relationships where this person is the target)
        for edge in self.graph.edges_directed(*index, petgraph::Direction::Incoming) {
            if let Some(person) = self.graph.node_weight(edge.source()) {
                relationships.push((
                    format!("{:?}", edge.weight()),
                    person.id.clone(),
                ));
            }
        }
        
        Ok(relationships)
    }
} 