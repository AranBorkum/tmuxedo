use std::collections::HashMap;
use std::io::{self, Write};
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader},
};

use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use tokio::task;

use crate::plugins::{Plugin, check_for_update, remove_dir, run_plugins};
use crate::utils::format_plugin_dir_name;
use crate::{
    plugins::{git_clone, git_pull},
    register::TmuxPlugins,
    tmuxedo::Path,
    tui::WindowTab,
};

pub struct State {
    pub tab: WindowTab,
    pub selected_available_plugin_index: usize,
    pub selected_available_plugin_value: String,
    pub selected_installed_plugin_index: usize,
    pub selected_installed_plugin_value: String,
    pub toggle_available_list: bool,
    pub all_installed_plugins: HashMap<String, Plugin>,
    pub search_mode: bool,
    pub search_string: String,
    installed_themes: HashMap<String, Plugin>,
    installed_status_bars: HashMap<String, Plugin>,
    installed_plugins: HashMap<String, Plugin>,
    available_themes: HashMap<String, Plugin>,
    available_status_bars: HashMap<String, Plugin>,
    available_plugins: HashMap<String, Plugin>,
}

impl State {
    async fn get_all_installed_plugins() -> HashMap<String, Plugin> {
        let path = Path::PluginsConfig.get();
        let file = match File::open(path) {
            Ok(file) => file,
            _ => todo!(),
        };
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().map_while(Result::ok).collect();

        let mut plugins = HashMap::<String, Plugin>::new();
        for line in lines {
            let plugin = line;
            plugins.insert(
                plugin.clone(),
                Plugin {
                    path: plugin,
                    commit_hash: "".to_string(),
                    is_up_to_date: true,
                },
            );
        }

        plugins
    }

    pub async fn check_for_plugin_updated(&mut self) {
        let path = Path::PluginsConfig.get();
        let file = match File::open(path) {
            Ok(file) => file,
            _ => todo!(),
        };
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().map_while(Result::ok).collect();

        let mut handles = vec![];
        for line in lines {
            handles.push(task::spawn(async move { check_for_update(&line).await }));
        }
        for handle in handles {
            match handle.await {
                Ok(Ok(u)) => {
                    if let Some(val) = self.all_installed_plugins.get_mut(&u.0) {
                        val.set_commit_hash(u.1);
                    };
                }
                Ok(Err(err)) => {
                    eprintln!("Error checking update: {err}");
                }
                Err(join_err) => {
                    eprintln!("Task join error: {join_err}");
                }
            }
        }
    }

    fn get_installed_and_available(
        lines: &[String],
        tmux_plugin: TmuxPlugins,
    ) -> (HashMap<String, Plugin>, HashMap<String, Plugin>) {
        let mut installed = HashMap::<String, Plugin>::new();
        let mut available = HashMap::<String, Plugin>::new();

        for item in tmux_plugin.all() {
            let p = Plugin {
                path: item.clone(),
                commit_hash: "".to_string(),
                is_up_to_date: true,
            };
            if lines.contains(&item) {
                installed.insert(item.clone(), p);
            } else {
                available.insert(item.clone(), p);
            }
        }

        (installed, available)
    }

    pub async fn default() -> Self {
        let path = Path::PluginsConfig.get();
        let file = match File::open(path) {
            Ok(file) => file,
            _ => todo!(),
        };
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().map_while(Result::ok).collect();

        let all_installed_plugins = Self::get_all_installed_plugins().await;
        let installed_and_available_themes =
            Self::get_installed_and_available(&lines, TmuxPlugins::Themes);
        let installed_and_available_status_bars =
            Self::get_installed_and_available(&lines, TmuxPlugins::StatusBar);
        let installed_and_available_plugins =
            Self::get_installed_and_available(&lines, TmuxPlugins::Plugins);

        let selected_available_plugin_value = String::new();
        let mut selected_installed_plugin_value = String::new();

        let all_installed_plugins_as_vec: Vec<String> =
            all_installed_plugins.keys().cloned().collect();
        if !all_installed_plugins_as_vec.is_empty() {
            selected_installed_plugin_value = all_installed_plugins_as_vec[0].clone();
        }

        Self {
            tab: WindowTab::All,
            selected_available_plugin_index: 0,
            selected_available_plugin_value,
            selected_installed_plugin_index: 0,
            selected_installed_plugin_value,
            toggle_available_list: false,
            all_installed_plugins,
            search_mode: false,
            search_string: String::new(),
            installed_themes: installed_and_available_themes.0,
            installed_status_bars: installed_and_available_status_bars.0,
            installed_plugins: installed_and_available_plugins.0,
            available_themes: installed_and_available_themes.1,
            available_status_bars: installed_and_available_status_bars.1,
            available_plugins: installed_and_available_plugins.1,
        }
    }

    pub fn get_installed_plugins(&self) -> Vec<String> {
        let mut plugins: Vec<_> = match self.tab {
            WindowTab::All => self.all_installed_plugins.keys().cloned().collect(),
            WindowTab::Themes => self.installed_themes.keys().cloned().collect(),
            WindowTab::StatusBar => self.installed_status_bars.keys().cloned().collect(),
            WindowTab::Plugins => self.installed_plugins.keys().cloned().collect(),
        };

        match self.search_string.is_empty() {
            true => {
                plugins.sort();
                plugins
            }
            false => {
                let matcher = SkimMatcherV2::default();
                let mut results: Vec<_> = plugins
                    .iter()
                    .filter_map(|item| {
                        matcher
                            .fuzzy_match(item, &self.search_string)
                            .map(|score| (item, score))
                    })
                    .collect();

                results.sort_by(|a, b| b.1.cmp(&a.1));

                results
                    .into_iter()
                    .map(|(item, _)| item.to_string())
                    .collect()
            }
        }
    }

    pub fn get_available_plugins(&self) -> Vec<String> {
        let mut plugins = match self.tab {
            WindowTab::All => Vec::new(),
            WindowTab::Themes => self.available_themes.keys().cloned().collect(),
            WindowTab::StatusBar => self.available_status_bars.keys().cloned().collect(),
            WindowTab::Plugins => self.available_plugins.keys().cloned().collect(),
        };

        match self.search_string.is_empty() {
            true => {
                plugins.sort();
                plugins
            }
            false => {
                let matcher = SkimMatcherV2::default();
                let mut results: Vec<_> = plugins
                    .iter()
                    .filter_map(|item| {
                        matcher
                            .fuzzy_match(item, &self.search_string)
                            .map(|score| (item, score))
                    })
                    .collect();

                results.sort_by(|a, b| b.1.cmp(&a.1));

                results
                    .into_iter()
                    .map(|(item, _)| item.to_string())
                    .collect()
            }
        }
    }

    pub fn set_tab(&mut self, tab: WindowTab) {
        self.tab = tab;
        self.toggle_available_list = false;
    }

    pub fn toggle_available(&mut self) {
        self.toggle_available_list = !self.toggle_available_list
    }

    pub fn toggle_search_mode(&mut self) {
        self.search_mode = !self.search_mode;
    }

    pub fn push_letter_to_search_string(&mut self, ch: char) {
        self.search_string.push(ch);
    }

    pub fn pop_letter_from_search_string(&mut self) {
        self.search_string.pop();
    }

    pub fn clear_search_string(&mut self) {
        self.search_string = String::new();
    }

    pub fn next_available_plugin(&mut self) {
        let available_plugins = self.get_available_plugins();

        if !available_plugins.is_empty() {
            let n_plugins = available_plugins.len();
            if self.selected_available_plugin_index != n_plugins - 1 {
                self.selected_available_plugin_index += 1;
                self.selected_available_plugin_value =
                    available_plugins[self.selected_available_plugin_index].clone();
            }
        }
    }

    pub fn previous_available_plugin(&mut self) {
        if self.selected_available_plugin_index != 0 {
            self.selected_available_plugin_index -= 1;
            self.selected_available_plugin_value =
                self.get_available_plugins()[self.selected_available_plugin_index].clone();
        }
    }

    pub fn next_installed_plugin(&mut self) {
        let installed_plugins = self.get_installed_plugins();

        if !installed_plugins.is_empty() {
            let n_plugins = installed_plugins.len();
            if self.selected_installed_plugin_index != n_plugins - 1 {
                self.selected_installed_plugin_index += 1;
                self.selected_installed_plugin_value =
                    installed_plugins[self.selected_installed_plugin_index].clone();
            }
        }
    }

    pub fn previous_installed_plugin(&mut self) {
        if self.selected_installed_plugin_index != 0 {
            self.selected_installed_plugin_index -= 1;
            self.selected_installed_plugin_value =
                self.get_installed_plugins()[self.selected_installed_plugin_index].clone();
        }
    }

    pub fn reset_selected_available_plugin(&mut self) {
        let available_plugins = self.get_available_plugins();
        self.selected_available_plugin_index = 0;
        self.selected_available_plugin_value = match available_plugins.is_empty() {
            true => String::new(),
            false => available_plugins[0].clone(),
        };
    }

    pub fn reset_selected_installed_plugin(&mut self) {
        let installed_plugins = self.get_installed_plugins();
        self.selected_installed_plugin_index = 0;
        self.selected_installed_plugin_value = match installed_plugins.is_empty() {
            true => String::new(),
            false => installed_plugins[0].clone(),
        };
    }

    fn get_installed_plugin_dir_name(&self) -> Result<String, Box<dyn Error>> {
        let plugins = self.get_installed_plugins();
        let dir_name = format_plugin_dir_name(&plugins[self.selected_installed_plugin_index]);
        Ok(dir_name)
    }

    fn move_plugin_to_available_from_all_tab(&mut self, plugin: &str) {
        if let Some(p) = self.installed_themes.remove(plugin) {
            self.available_themes.insert(plugin.to_string(), p.clone());
            self.all_installed_plugins.remove(plugin);
        } else if let Some(p) = self.installed_status_bars.remove(plugin) {
            self.available_status_bars
                .insert(plugin.to_string(), p.clone());
            self.all_installed_plugins.remove(plugin);
        } else if let Some(p) = self.installed_plugins.remove(plugin) {
            self.available_plugins.insert(plugin.to_string(), p.clone());
            self.all_installed_plugins.remove(plugin);
        }
    }

    fn move_plugin_to_installed(&mut self, plugin: &str) {
        match self.tab {
            WindowTab::Themes => {
                if let Some(p) = self.available_themes.remove(plugin) {
                    self.installed_themes.insert(plugin.to_string(), p.clone());
                    self.all_installed_plugins.insert(plugin.to_string(), p);
                }
            }
            WindowTab::StatusBar => {
                if let Some(p) = self.available_status_bars.remove(plugin) {
                    self.installed_status_bars
                        .insert(plugin.to_string(), p.clone());
                    self.all_installed_plugins.insert(plugin.to_string(), p);
                }
            }
            WindowTab::Plugins => {
                if let Some(p) = self.available_plugins.remove(plugin) {
                    self.installed_plugins.insert(plugin.to_string(), p.clone());
                    self.all_installed_plugins.insert(plugin.to_string(), p);
                }
            }
            _ => todo!(),
        }
    }

    fn move_plugin_to_available(&mut self, plugin: &str) {
        match self.tab {
            WindowTab::Themes => {
                if let Some(p) = self.installed_themes.remove(plugin) {
                    self.available_themes.insert(plugin.to_string(), p.clone());
                    self.all_installed_plugins.remove(plugin);
                }
            }
            WindowTab::StatusBar => {
                if let Some(p) = self.installed_status_bars.remove(plugin) {
                    self.available_status_bars
                        .insert(plugin.to_string(), p.clone());
                    self.all_installed_plugins.remove(plugin);
                }
            }
            WindowTab::Plugins => {
                if let Some(p) = self.installed_plugins.remove(plugin) {
                    self.available_plugins.insert(plugin.to_string(), p.clone());
                    self.all_installed_plugins.remove(plugin);
                }
            }
            WindowTab::All => self.move_plugin_to_available_from_all_tab(plugin),
        }
    }

    fn write_installed_plugins(&self) -> io::Result<()> {
        let path = Path::PluginsConfig.get();
        let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;
        for plugin in &self.all_installed_plugins {
            writeln!(file, "{}", String::from(plugin.0))?;
        }

        Ok(())
    }

    pub async fn install_plugin(&mut self) {
        let plugins = self.get_available_plugins();
        let plugin = &plugins[self.selected_available_plugin_index];

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
            if let Some(val) = self.all_installed_plugins.get_mut(&plugin) {
                val.set_commit_hash(String::new());
            }
            run_plugins();
        }
    }

    pub fn remove_plugin(&mut self) {
        let plugins = self.get_installed_plugins();
        let plugin = &plugins[self.selected_installed_plugin_index];

        let _ = remove_dir(self.get_installed_plugin_dir_name().expect("REASON"));
        self.move_plugin_to_available(plugin);
        let _ = self.write_installed_plugins();
        run_plugins();
    }
}
