use std::io::{self, Write};
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader},
};

use crate::plugins::{remove_dir, run_plugins};
use crate::{
    plugins::{git_clone, git_pull},
    register::TmuxPlugins,
    tmuxedo::Path,
    tui::WindowTab,
};

pub struct State {
    pub tab: WindowTab,
    pub selected_available_plugin: usize,
    pub selected_installed_plugin: usize,
    pub toggle_available_list: bool,
    all_installed_plugins: Vec<String>,
    installed_themes: Vec<String>,
    installed_status_bars: Vec<String>,
    installed_plugins: Vec<String>,
    available_themes: Vec<String>,
    available_status_bars: Vec<String>,
    available_plugins: Vec<String>,
}

impl State {
    pub fn default() -> Self {
        let mut all_installed_plugins: Vec<String> = Vec::new();
        let mut installed_themes: Vec<String> = Vec::new();
        let mut installed_status_bars: Vec<String> = Vec::new();
        let mut installed_plugins: Vec<String> = Vec::new();
        let mut available_themes: Vec<String> = Vec::new();
        let mut available_status_bars: Vec<String> = Vec::new();
        let mut available_plugins: Vec<String> = Vec::new();

        let path = Path::PluginsConfig.get();
        let file = match File::open(path) {
            Ok(file) => file,
            _ => todo!(),
        };
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().map_while(Result::ok).collect();

        for plugin in &lines {
            all_installed_plugins.push(plugin.to_string());
        }

        for theme in TmuxPlugins::Themes.all() {
            if lines.contains(&theme) {
                installed_themes.push(theme);
            } else {
                available_themes.push(theme);
            }
        }

        for status_bar in TmuxPlugins::StatusBar.all() {
            if lines.contains(&status_bar) {
                installed_status_bars.push(status_bar);
            } else {
                available_status_bars.push(status_bar);
            }
        }

        for plugin in TmuxPlugins::Plugins.all() {
            if lines.contains(&plugin) {
                installed_plugins.push(plugin);
            } else {
                available_plugins.push(plugin);
            }
        }

        all_installed_plugins.sort();
        installed_themes.sort();
        installed_status_bars.sort();
        installed_plugins.sort();
        available_themes.sort();
        available_status_bars.sort();
        available_plugins.sort();

        Self {
            tab: WindowTab::All,
            selected_available_plugin: 0,
            selected_installed_plugin: 0,
            toggle_available_list: false,
            all_installed_plugins,
            installed_themes,
            installed_status_bars,
            installed_plugins,
            available_themes,
            available_status_bars,
            available_plugins,
        }
    }

    pub fn get_installed_plugins(&self) -> Vec<String> {
        match self.tab {
            WindowTab::All => self.all_installed_plugins.clone(),
            WindowTab::Themes => self.installed_themes.clone(),
            WindowTab::StatusBar => self.installed_status_bars.clone(),
            WindowTab::Plugins => self.installed_plugins.clone(),
        }
    }

    pub fn get_available_plugins(&self) -> Vec<String> {
        match self.tab {
            WindowTab::Themes => self.available_themes.clone(),
            WindowTab::StatusBar => self.available_status_bars.clone(),
            WindowTab::Plugins => self.available_plugins.clone(),
            _ => Vec::new(),
        }
    }

    pub fn set_tab(&mut self, tab: WindowTab) {
        self.tab = tab;
    }

    pub fn toggle_available(&mut self) {
        self.toggle_available_list = !self.toggle_available_list
    }

    pub fn next_available_plugin(&mut self) {
        if !self.get_available_plugins().is_empty() {
            let n_plugins = self.get_available_plugins().len();
            if self.selected_available_plugin != n_plugins - 1 {
                self.selected_available_plugin += 1
            }
        }
    }

    pub fn previous_available_plugin(&mut self) {
        if self.selected_available_plugin != 0 {
            self.selected_available_plugin -= 1
        }
    }

    pub fn next_installed_plugin(&mut self) {
        if !self.get_installed_plugins().is_empty() {
            let n_plugins = self.get_installed_plugins().len();
            if self.selected_installed_plugin != n_plugins - 1 && n_plugins > 0 {
                self.selected_installed_plugin += 1
            }
        }
    }

    pub fn previous_installed_plugin(&mut self) {
        if self.selected_installed_plugin != 0 {
            self.selected_installed_plugin -= 1
        }
    }

    pub fn reset_selected_available_plugin(&mut self) {
        self.selected_available_plugin = 0;
    }

    pub fn reset_selected_installed_plugin(&mut self) {
        self.selected_installed_plugin = 0;
    }

    fn get_installed_plugin_dir_name(&self) -> Result<String, Box<dyn Error>> {
        let plugins = self.get_installed_plugins();
        Ok(plugins[self.selected_installed_plugin]
            .split("/")
            .nth(1)
            .expect("REASON")
            .to_string())
    }

    fn move_plugin_to_installed(&mut self, plugin: &str) {
        match self.tab {
            WindowTab::Themes => {
                self.installed_themes.push(plugin.to_owned());
                self.all_installed_plugins.push(plugin.to_owned());
                self.installed_themes.sort();
                self.all_installed_plugins.sort();
                self.available_themes.remove(self.selected_available_plugin);
            }
            WindowTab::StatusBar => {
                self.installed_status_bars.push(plugin.to_owned());
                self.all_installed_plugins.push(plugin.to_owned());
                self.installed_status_bars.sort();
                self.all_installed_plugins.sort();
                self.available_status_bars
                    .remove(self.selected_available_plugin);
            }
            WindowTab::Plugins => {
                self.installed_plugins.push(plugin.to_owned());
                self.all_installed_plugins.push(plugin.to_owned());
                self.installed_plugins.sort();
                self.all_installed_plugins.sort();
                self.available_plugins
                    .remove(self.selected_available_plugin);
            }
            _ => todo!(),
        }
    }

    fn move_plugin_to_available(&mut self, plugin: &str) {
        match self.tab {
            WindowTab::Themes => {
                self.available_themes.push(plugin.to_owned());
                self.all_installed_plugins
                    .retain(|s| s != &plugin.to_owned());
                self.available_themes.sort();
                self.installed_themes.remove(self.selected_installed_plugin);
            }
            WindowTab::StatusBar => {
                self.available_status_bars.push(plugin.to_owned());
                self.all_installed_plugins
                    .retain(|s| s != &plugin.to_owned());
                self.available_status_bars.sort();
                self.installed_status_bars
                    .remove(self.selected_installed_plugin);
            }
            WindowTab::Plugins => {
                self.available_plugins.push(plugin.to_owned());
                self.all_installed_plugins
                    .retain(|s| s != &plugin.to_owned());
                self.available_plugins.sort();
                self.installed_plugins
                    .remove(self.selected_installed_plugin);
            }
            _ => todo!(),
        }
    }

    fn write_installed_plugins(&self) -> io::Result<()> {
        let path = Path::PluginsConfig.get();
        let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;
        for plugin in &self.all_installed_plugins {
            writeln!(file, "{}", String::from(plugin))?;
        }

        Ok(())
    }

    pub async fn install_plugin(&mut self) {
        let plugins = self.get_available_plugins();
        let plugin = &plugins[self.selected_available_plugin];

        let status = git_clone(plugin, None).await.expect("REASON");
        if status.success() {
            self.move_plugin_to_installed(plugin);
            let _ = self.write_installed_plugins();
            run_plugins();
        }
    }

    pub async fn update_plugin(&mut self) {
        let plugin: String = self.get_installed_plugin_dir_name().expect("REASON");
        let status = git_pull(&plugin).await.expect("REASON");
        if status.success() {
            run_plugins();
        }
    }

    pub fn remove_plugin(&mut self) {
        let plugins = self.get_installed_plugins();
        let plugin = &plugins[self.selected_installed_plugin];
        let _ = remove_dir(self.get_installed_plugin_dir_name().expect("REASON"));
        self.move_plugin_to_available(plugin);
        let _ = self.write_installed_plugins();
        run_plugins();
    }
}
