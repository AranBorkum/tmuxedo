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
            Self::Quit => "q",
            Self::ToggleInstalled => "C-o",
            Self::ToggleAvailable => "C-o",
            Self::Next => "j",
            Self::Previous => "k",
            Self::Install => "I",
            Self::Update => "U",
            Self::Delete => "X",
            Self::Search => "/",
            Self::ExitSearch => "esc",
            Self::FindSearch => "enter",
            Self::ClearSearch => "esc",
        }
        .to_string()
    }

    pub fn repr(&self) -> String {
        match self {
            Self::Quit => "quit",
            Self::ToggleInstalled => "toggle installed",
            Self::ToggleAvailable => "toggle available",
            Self::Next => "next",
            Self::Previous => "previous",
            Self::Install => "install",
            Self::Update => "update",
            Self::Delete => "delete",
            Self::Search => "search",
            Self::ExitSearch => "exit search",
            Self::FindSearch => "confirm",
            Self::ClearSearch => "clear search",
        }
        .to_string()
    }
}

fn search_mode_bindings() -> Vec<Binding> {
    vec![Binding::ExitSearch, Binding::FindSearch]
}

fn normal_mode_bindings(state: &State) -> Vec<Binding> {
    let mut defaults = vec![
        Binding::Quit,
        Binding::Next,
        Binding::Previous,
        Binding::Search,
    ];

    let mut extra;
    if state.tab == WindowTab::All {
        extra = vec![Binding::Update, Binding::Delete];
    } else if state.toggle_available_list {
        extra = vec![Binding::ToggleInstalled, Binding::Install];
    } else {
        extra = vec![Binding::ToggleAvailable, Binding::Update, Binding::Delete];
    }

    if !state.search_string.is_empty() {
        extra.push(Binding::ClearSearch);
    }

    defaults.extend(extra);
    defaults
}

pub fn get(state: &State) -> Vec<Binding> {
    match state.search_mode {
        true => search_mode_bindings(),
        false => normal_mode_bindings(state),
    }
}
