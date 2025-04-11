use iced::{
    widget::{button, column, container, row, text, text_input, Column},
    Alignment, Element, Length, Sandbox, Settings,
};
use crate::models::{FamilyTree, Person, Relationship};
use petgraph::graph::NodeIndex;

#[derive(Debug, Clone)]
pub enum Message {
    AddPerson,
    Save,
    NameChanged(String),
    SelectPerson(NodeIndex),
    // TODO: Add more messages for other interactions
}

pub struct AncestrumApp {
    tree: FamilyTree,
    selected_person: Option<NodeIndex>,
    is_adding_person: bool,
    new_person: Person,
}

impl Sandbox for AncestrumApp {
    type Message = Message;

    fn new() -> Self {
        AncestrumApp {
            tree: FamilyTree::new(),
            selected_person: None,
            is_adding_person: false,
            new_person: Person {
                id: uuid::Uuid::new_v4(),
                name: String::new(),
                birth_date: None,
                death_date: None,
                gender: crate::models::Gender::Other,
                notes: String::new(),
            },
        }
    }

    fn title(&self) -> String {
        String::from("Ancestrum - Family Tree Visualizer")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::AddPerson => {
                self.is_adding_person = true;
            }
            Message::Save => {
                // TODO: Implement save functionality
            }
            Message::NameChanged(name) => {
                self.new_person.name = name;
            }
            Message::SelectPerson(index) => {
                self.selected_person = Some(index);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = column![
            // Top bar
            row![
                button("Add Person").on_press(Message::AddPerson),
                button("Save").on_press(Message::Save),
            ]
            .spacing(10)
            .align_items(Alignment::Center),

            // Main content
            if self.is_adding_person {
                self.view_add_person()
            } else if let Some(person_index) = self.selected_person {
                self.view_person_card(person_index)
            } else {
                self.view_tree()
            }
        ]
        .spacing(20)
        .padding(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

impl AncestrumApp {
    fn view_add_person(&self) -> Element<Message> {
        column![
            text("Add New Person").size(20),
            text_input("Name", &self.new_person.name)
                .on_input(Message::NameChanged)
                .padding(10),
            // TODO: Add more input fields
            button("Add").on_press(Message::AddPerson),
        ]
        .spacing(10)
        .into()
    }

    fn view_person_card(&self, person_index: NodeIndex) -> Element<Message> {
        if let Some(person) = self.tree.get_person(person_index) {
            let mut content = column![
                text(format!("Name: {}", person.name)).size(20),
                text(format!("Gender: {:?}", person.gender)),
            ];

            if let Some(birth_date) = person.birth_date {
                content = content.push(text(format!("Birth Date: {}", birth_date)));
            }
            if let Some(death_date) = person.death_date {
                content = content.push(text(format!("Death Date: {}", death_date)));
            }

            content = content.push(text(format!("Notes: {}", person.notes)));

            // Add relationship buttons
            content = content.push(text("Relationships:").size(16));
            for (related_index, relationship) in self.tree.get_relationships(person_index) {
                if let Some(related_person) = self.tree.get_person(related_index) {
                    content = content.push(
                        button(format!("{} ({:?})", related_person.name, relationship))
                            .on_press(Message::SelectPerson(related_index)),
                    );
                }
            }

            content.spacing(10).into()
        } else {
            text("Person not found").into()
        }
    }

    fn view_tree(&self) -> Element<Message> {
        // TODO: Implement tree visualization using iced's canvas
        text("Family Tree Visualization (Coming Soon)").into()
    }
} 