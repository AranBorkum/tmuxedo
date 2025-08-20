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
    Search,
    ExitSearch,
    FindSearch,
    ClearSearch,
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
            Self::Search => String::from("/"),
            Self::ExitSearch => String::from("esc"),
            Self::FindSearch => String::from("enter"),
            Self::ClearSearch => String::from("esc"),
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
            Self::Search => String::from("search"),
            Self::ExitSearch => String::from("exit search"),
            Self::FindSearch => String::from("confirm"),
            Self::ClearSearch => String::from("clear search"),
        }
    }
}

pub fn get(state: &State) -> Vec<Binding> {
    match state.search_mode {
        true => vec![Binding::ExitSearch, Binding::FindSearch],
        false => {
            if state.tab == WindowTab::All {
                let mut bindings = vec![
                    Binding::Quit,
                    Binding::Next,
                    Binding::Previous,
                    Binding::Search,
                    Binding::Update,
                    Binding::Delete,
                ];
                if !state.search_string.is_empty() {
                    bindings.push(Binding::ClearSearch);
                }
                bindings
            } else if state.toggle_available_list {
                let mut bindings = vec![
                    Binding::Quit,
                    Binding::Next,
                    Binding::Previous,
                    Binding::Search,
                    Binding::ToggleInstalled,
                    Binding::Install,
                ];
                if !state.search_string.is_empty() {
                    bindings.push(Binding::ClearSearch);
                }
                bindings
            } else {
                let mut bindings = vec![
                    Binding::Quit,
                    Binding::Next,
                    Binding::Previous,
                    Binding::Search,
                    Binding::ToggleAvailable,
                    Binding::Update,
                    Binding::Delete,
                ];
                if !state.search_string.is_empty() {
                    bindings.push(Binding::ClearSearch);
                }
                bindings
            }
        }
    }
}
