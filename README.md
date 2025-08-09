# Tmuxedo: a cleaner, modular approach to configuring tmux

`tmuxedo` helps you break down your tmux configuration into modular files and manage plugins in a structured, organized manner. This makes your tmux config easier to maintain, extend, and understand.

---
### Features

- 🔧 **Modular config** — break up your `.tmux.conf` into logical, maintainable chunks
- 📦 **Plugin manager** — declarative plugin list, zero boilerplate
- 🔄 **Auto-updates** — clones and updates plugins automatically
- ⚡ **One command to rule them all** — apply config + plugins in one go (`tmuxedo`)

---
### Installation

Install from crates.io:

```bash
cargo install tmuxedo
```

Or build it manually:

```bash
git clone https://github.com/AranBorkum/tmuxedo
cd tmuxedo
cargo install --path .
```

---
### Getting started
When you first run `tmuxedo`, it will create the following directories and files:

- `~/.config/tmux/tmuxedo/`: where your modular configuration files live
- `~/.config/tmux/plugins/`: where plugins are cloned
- `~/.config/tmux/tmuxedo/plugins.conf`: defines your desired plugins

#### Adding Configurations

To define tmux settings, create `.conf` files in the `tmuxedo/` directory. You can name them however you like, as long as they end in `.conf`.

Example: `bindings.conf`

```bash
unbind C-b
set-option -g prefix C-a
bind-key C-a send-prefix

unbind r
bind r run-shell tmuxedo 
```

#### Adding Plugins

Add plugin repository names to `plugins.conf`. For example:

```bash
AranBorkum/tmux-cookie-cutter
tmux-plugins/tmux-yank
```

`tmuxedo` will clone these into your plugins directory and keep them up to date.

#### Applying Changes

To apply your configuration and plugins, simply run:

```bash
tmuxedo
```

You can also bind it to a key in tmux (e.g., `<prefix>r`) for convenience.

