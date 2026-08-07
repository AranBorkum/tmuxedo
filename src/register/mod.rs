mod plugins;
mod status_bar;
mod themes;

pub enum TmuxPlugins {
    Themes,
    StatusBar,
    Plugins,
}

impl TmuxPlugins {
    fn to_vec_of_string(&self, collection: &[&str]) -> Vec<String> {
        collection.iter().map(|e| e.to_string()).collect()
    }

    pub fn all(&self) -> Vec<String> {
        match self {
            Self::Themes => self.to_vec_of_string(themes::THEMES),
            Self::StatusBar => self.to_vec_of_string(status_bar::STATUS_BAR),
            Self::Plugins => self.to_vec_of_string(plugins::PLUGINS),
        }
    }
}
