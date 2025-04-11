use petgraph::graph::NodeIndex;
use crate::models::family_tree::FamilyTree;
use crate::storage::{load_tree, save_tree};
use crate::ui::message::Message;
use iced::{Application, executor, theme, widget::{button, column, text}};

pub struct AncestrumApp {
    tree: FamilyTree,
    selected_person: Option<NodeIndex>,
}


impl Application for AncestrumApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = theme::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self {
            tree: load_tree().unwrap_or_else(|_| FamilyTree::new()),
            selected_person: None,
        }, iced::Command::none())
    }

    fn title(&self) -> String {
        "Ancestrum".into()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Save => {
                let _ = save_tree(&self.tree);
                iced::Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<Self::Message> {
        column![
            text("Hello, world!"),
            button("Save").on_press(Message::Save)
        ].into()
    }

    fn theme(&self) -> Self::Theme {
        theme::Theme::default()
    }
}

