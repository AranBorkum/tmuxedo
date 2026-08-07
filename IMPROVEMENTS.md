# Tmuxedo Project Improvements

Generated: 2026-05-20

## **Critical Issues**

### 1. **Replace `todo!()` macros** (state.rs:43, 70, 123)
Three `todo!()` calls will panic if file operations fail. Replace with proper error handling:
```rust
let file = File::open(path)?;  // Return error instead of todo!()
```

**Files affected:**
- `src/state.rs:43`
- `src/state.rs:70`
- `src/state.rs:123`

### 2. **Remove invalid `clippy` dev-dependency** (Cargo.toml:22)
Clippy is built into Cargo - this dependency causes warnings:
```toml
[dev-dependencies]
# Remove: clippy = "0.0.302"
```

**Files affected:**
- `Cargo.toml:22`

### 3. **Reduce `.expect("REASON")` usage**
16 instances of `expect`/`unwrap` throughout codebase. These should propagate errors properly, especially in:
- state.rs:405, 406, 414, 424, 434, 444
- plugins.rs:133

**Files affected:**
- `src/state.rs` (multiple locations)
- `src/plugins.rs:133`

## **Code Quality Improvements**

### 4. **Improve error messages**
Generic placeholders like `expect("REASON")` don't help debugging. Use descriptive messages:
```rust
// Instead of:
git_clone(plugin, None).await.expect("REASON")

// Use:
git_clone(plugin, None).await.expect("Failed to clone plugin")
```

### 5. **Add logging/tracing**
For a CLI tool, consider adding `tracing` or `env_logger` to help users debug issues instead of silent failures or eprintln.

**Suggested dependencies:**
```toml
tracing = "0.1"
tracing-subscriber = "0.3"
```

### 6. **Consolidate duplicate fuzzy search logic**
`get_installed_plugins()` and `get_available_plugins()` in state.rs have identical fuzzy matching code (lines 172-195 and 206-229). Extract to a helper function.

**Files affected:**
- `src/state.rs:172-195`
- `src/state.rs:206-229`

**Suggested refactor:**
```rust
fn fuzzy_filter_plugins(plugins: Vec<String>, search_string: &str) -> Vec<String> {
    if search_string.is_empty() {
        let mut sorted = plugins;
        sorted.sort();
        return sorted;
    }
    
    let matcher = SkimMatcherV2::default();
    let mut results: Vec<_> = plugins
        .iter()
        .filter_map(|item| {
            matcher
                .fuzzy_match(item, search_string)
                .map(|score| (item, score))
        })
        .collect();
    
    results.sort_by_key(|b| std::cmp::Reverse(b.1));
    results.into_iter().map(|(item, _)| item.to_string()).collect()
}
```

### 7. **Fix parameter passing**
Functions like `git_clone(plugin: &String)` should use `&str` instead of `&String` - more idiomatic and flexible.

**Files affected:**
- `src/plugins.rs:48` - `git_clone(plugin: &String, ...)`
- `src/plugins.rs:90` - `git_pull(plugin: &String)`

**Suggested change:**
```rust
pub async fn git_clone(plugin: &str, branch: Option<String>) -> io::Result<ExitStatus>
pub async fn git_pull(plugin: &str) -> io::Result<ExitStatus>
```

## **Architecture Improvements**

### 8. **Add more comprehensive tests**
Only `utils.rs` has tests. Consider adding tests for:
- Plugin installation/removal logic
- Git operations (with mocked commands)
- State management
- CLI command parsing

**Current test coverage:**
- ✅ `src/utils.rs` - 10 tests
- ❌ `src/plugins.rs` - 0 tests
- ❌ `src/state.rs` - 0 tests
- ❌ `src/cli.rs` - 0 tests
- ❌ `src/app.rs` - 0 tests

### 9. **Better async handling**
Some functions are async but don't need to be (e.g., `app::run()` just calls sync functions). Consider restructuring or removing unnecessary async.

**Files affected:**
- `src/app.rs:4` - `run()` is async but mostly calls sync code

### 10. **Type safety for paths**
Consider using newtype wrappers instead of raw `String` for plugin names/paths to prevent mixing them up.

**Suggested approach:**
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginName(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginPath(PathBuf);

impl PluginName {
    pub fn as_str(&self) -> &str {
        &self.0
    }
    
    pub fn to_path(&self) -> PluginPath {
        PluginPath(PathBuf::from(format_plugin_dir_name(&self.0)))
    }
}
```

## **Documentation**

### 11. **Add inline documentation**
Public APIs lack doc comments. Add `///` comments for public structs, enums, and functions.

**Priority files:**
- `src/state.rs` - `State` struct and public methods
- `src/plugins.rs` - `Plugin` struct and public functions
- `src/cli.rs` - `Cli` and `Commands` enums
- `src/app.rs` - Public functions

**Example:**
```rust
/// Represents the application state for the TUI.
///
/// Manages installed and available plugins, search state,
/// and the currently selected tab.
pub struct State {
    // ...
}
```

### 12. **Add CONTRIBUTING.md**
Help potential contributors understand the codebase structure and how to contribute.

**Suggested sections:**
- Project structure overview
- How to run tests
- How to add new plugins to the registry
- Code style guidelines
- PR process

## **Performance**

### 13. **Batch git operations more efficiently**
In `plugins.rs:197-219`, consider limiting concurrent git operations or adding a progress indicator for better UX.

**Files affected:**
- `src/plugins.rs:197-219` - `pull()` function

**Suggested improvements:**
- Add concurrency limit (e.g., max 5 concurrent git operations)
- Add progress feedback for long operations
- Consider using `futures::stream::StreamExt::buffer_unordered`

## Priority Recommendations

**Start with these in order:**

1. ✅ Fix `todo!()` macros (Critical - causes panics)
2. ✅ Remove invalid clippy dependency (Quick fix)
3. ✅ Replace `expect("REASON")` with descriptive messages (Better UX)
4. ⚠️ Add basic error propagation for plugin operations
5. 📚 Add inline documentation for public APIs
6. 🧪 Add tests for core plugin operations
