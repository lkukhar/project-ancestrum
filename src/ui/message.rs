use petgraph::graph::NodeIndex;

#[derive(Debug, Clone)]
pub enum Message {
    AddPerson,
    Save,
    NameChanged(String),
    SelectPerson(NodeIndex),
    // TODO: Add more messages for other interactions
}