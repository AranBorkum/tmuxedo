# Tmuxedo 🕴️ — A Cleaner, Modular Approach to Tmux Configuration

**Tmuxedo** helps you break down your tmux configuration into modular files and manage plugins in a clean, structured way.  
Maintainable, extensible, and fast - keep your tmux setup under control.

---

### ✨ Features

- 🔧 **Modular config** – split your `.tmux.conf` into logical, reusable files
- 📦 **Built-in plugin manager** – declarative config, zero boilerplate
- 🔄 **Automatic plugin updates** – clones and keeps plugins up to date
- ⚡ **One command to rule them all** – apply config + plugins in one shot

---

### 📦 Installation

Install from [crates.io](https://crates.io/crates/tmuxedo):

```bash
cargo install tmuxedo
```

Or build from source:

```bash
git clone https://github.com/AranBorkum/tmuxedo
cd tmuxedo
cargo install --path .
```

---

### 🚀 Getting Started

Running `tmuxedo` for the first time sets up:

- `~/.config/tmux/tmuxedo/` – your modular config directory
- `~/.config/tmux/plugins/` – plugin installation directory
- `~/.config/tmux/tmuxedo/plugins.conf` – your plugin manifest

Add this line to the end of your `.tmux.conf` to hook it all up:

```tmux
run-shell 'tmuxedo'
```

---

### 🛠 Adding Config Files

Drop `.conf` files into `~/.config/tmux/tmuxedo/`. You can name them however you like.

**Example: `bindings.conf`**

```tmux
unbind C-b
set-option -g prefix C-a
bind-key C-a send-prefix

unbind r
bind r run-shell tmuxedo
```

---

### 🔌 Managing Plugins

#### Via TUI

Run the built-in terminal UI:

```bash
tmuxedo --tui
```

Or use the key binding: `<prefix> + C-t` (defined by default).

The TUI lets you:

- Install plugins from the known list
- Update or remove existing plugins
- Add new ones manually

If a plugin isn't listed, manually add it to `plugins.conf`, and consider submitting a PR to include it for others!

---

### 🔄 Applying Changes

To apply your full configuration (including plugins), just run:

```bash
tmuxedo
```

For convenience, bind it to a key in tmux (e.g. `<prefix> + r`):

```tmux
bind r run-shell tmuxedo
```

---

### ⚠️ Important Update (v0.1.12+)

**Breaking Change for Existing Users:** As of version **0.1.12**, installed plugins now include the GitHub username in their directory name (e.g., `username_repo_name`) to prevent namespace conflicts.

**How to Update:** If you are upgrading from an older version, you must clear your old plugins directory to allow `tmuxedo` to re-install them with the correct structure.

1. Delete your existing plugins directory:
   ```bash
   rm -rf ~/.config/tmux/plugins
   ```
2. Re-run `tmuxedo` to re-install plugins with updated naming convention.

---

### 🙌 Contributions Welcome

Found a bug? Want to suggest a plugin or feature?  
Open an issue or PR on [GitHub](https://github.com/AranBorkum/tmuxedo)!
