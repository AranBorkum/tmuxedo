use crate::{state::State, tui::WindowTab};

pub enum Binding {
    Quit,
    ToggleInstalled,
    ToggleAvailable,
    Next,
    Previous,
    Install,
    Update,
    Delete,
}

impl Binding {
    pub fn key(&self) -> String {
        match self {
            Self::Quit => String::from("q"),
            Self::ToggleInstalled => String::from("C-o"),
            Self::ToggleAvailable => String::from("C-o"),
            Self::Next => String::from("j"),
            Self::Previous => String::from("k"),
            Self::Install => String::from("I"),
            Self::Update => String::from("U"),
            Self::Delete => String::from("X"),
        }
    }

    pub fn repr(&self) -> String {
        match self {
            Self::Quit => String::from("quit"),
            Self::ToggleInstalled => String::from("toggle installed"),
            Self::ToggleAvailable => String::from("toggle available"),
            Self::Next => String::from("next"),
            Self::Previous => String::from("previous"),
            Self::Install => String::from("install"),
            Self::Update => String::from("update"),
            Self::Delete => String::from("delete"),
        }
    }
}

fn default() -> Vec<Binding> {
    vec![Binding::Quit, Binding::Next, Binding::Previous]
}

pub fn get(state: &State) -> Vec<Binding> {
    if state.tab == WindowTab::All {
        vec![Binding::Quit]
    } else {
        let mut default = default();

        if state.toggle_available_list {
            default.push(Binding::ToggleInstalled);
            default.push(Binding::Install);
        } else {
            default.push(Binding::ToggleAvailable);
            default.push(Binding::Update);
            default.push(Binding::Delete);
        }

        default
    }
}
