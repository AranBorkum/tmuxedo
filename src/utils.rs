use std::io::{self, Write};
use std::process::Stdio;
use std::{
    fs::{self, OpenOptions},
    path::PathBuf,
};

pub fn format_plugin_dir_name(dir: &str) -> String {
    dir.replace("/", "_")
}

pub fn ensure_dir_exists(path: &PathBuf) {
    match fs::create_dir_all(path) {
        Ok(_) => {}
        Err(e) => eprintln!("Error creating directory: {e}"),
    }
}

pub fn ensure_file_exists(path: &PathBuf, content: Vec<&str>) -> io::Result<()> {
    if !path.exists() {
        let mut file = OpenOptions::new().create(true).append(true).open(path)?;

        for line in content {
            writeln!(file, "{}", String::from(line))?;
        }
    }

    Ok(())
}

pub fn open_fuzzy_picker(prompt: &str, values: Vec<String>) -> io::Result<Option<String>> {
    let mut child = std::process::Command::new("fzf")
        .arg(format!("--prompt={}> ", prompt))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(values.join("\n").as_bytes())?;
    }

    let output = child.wait_with_output()?;
    if !output.status.success() {
        return Ok(None);
    }

    let selected = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if selected.is_empty() {
        Ok(None)
    } else {
        Ok(Some(selected))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_plugin_dir_name_replaces_slashes() {
        assert_eq!(format_plugin_dir_name("user/repo"), "user_repo");
        assert_eq!(format_plugin_dir_name("org/user/repo"), "org_user_repo");
    }

    #[test]
    fn test_format_plugin_dir_name_handles_no_slashes() {
        assert_eq!(format_plugin_dir_name("simple"), "simple");
    }

    #[test]
    fn test_format_plugin_dir_name_handles_multiple_slashes() {
        assert_eq!(format_plugin_dir_name("a/b/c/d/e"), "a_b_c_d_e");
        assert_eq!(format_plugin_dir_name("//"), "__");
    }

    #[test]
    fn test_ensure_dir_exists_creates_directory() {
        let temp_dir = std::env::temp_dir().join("tmuxedo_test_dir");

        let _ = fs::remove_dir_all(&temp_dir);

        ensure_dir_exists(&temp_dir);

        assert!(temp_dir.exists());
        assert!(temp_dir.is_dir());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_ensure_dir_exists_handles_nested_directories() {
        let temp_dir = std::env::temp_dir().join("tmuxedo_test_nested/a/b/c");

        let _ = fs::remove_dir_all(std::env::temp_dir().join("tmuxedo_test_nested"));

        ensure_dir_exists(&temp_dir);

        assert!(temp_dir.exists());
        assert!(temp_dir.is_dir());

        let _ = fs::remove_dir_all(std::env::temp_dir().join("tmuxedo_test_nested"));
    }

    #[test]
    fn test_ensure_dir_exists_idempotent() {
        let temp_dir = std::env::temp_dir().join("tmuxedo_test_idempotent");

        let _ = fs::remove_dir_all(&temp_dir);

        ensure_dir_exists(&temp_dir);
        ensure_dir_exists(&temp_dir);

        assert!(temp_dir.exists());
        assert!(temp_dir.is_dir());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_ensure_file_exists_creates_file_with_content() {
        let temp_file = std::env::temp_dir().join("tmuxedo_test_file.txt");

        let _ = fs::remove_file(&temp_file);

        let content = vec!["line 1", "line 2", "line 3"];
        ensure_file_exists(&temp_file, content).unwrap();

        assert!(temp_file.exists());
        assert!(temp_file.is_file());

        let file_content = fs::read_to_string(&temp_file).unwrap();
        assert_eq!(file_content, "line 1\nline 2\nline 3\n");

        let _ = fs::remove_file(&temp_file);
    }

    #[test]
    fn test_ensure_file_exists_with_empty_content() {
        let temp_file = std::env::temp_dir().join("tmuxedo_test_empty.txt");

        let _ = fs::remove_file(&temp_file);

        let content: Vec<&str> = vec![];
        ensure_file_exists(&temp_file, content).unwrap();

        assert!(temp_file.exists());
        assert!(temp_file.is_file());

        let file_content = fs::read_to_string(&temp_file).unwrap();
        assert_eq!(file_content, "");

        let _ = fs::remove_file(&temp_file);
    }

    #[test]
    fn test_ensure_file_exists_does_not_overwrite() {
        let temp_file = std::env::temp_dir().join("tmuxedo_test_no_overwrite.txt");

        let _ = fs::remove_file(&temp_file);

        fs::write(&temp_file, "original content\n").unwrap();

        let new_content = vec!["new line 1", "new line 2"];
        ensure_file_exists(&temp_file, new_content).unwrap();

        let file_content = fs::read_to_string(&temp_file).unwrap();
        assert_eq!(file_content, "original content\n");

        let _ = fs::remove_file(&temp_file);
    }

    #[test]
    fn test_ensure_file_exists_fails_for_nonexistent_parent_dir() {
        let temp_file = PathBuf::from("/nonexistent/directory/that/does/not/exist/file.txt");

        let content = vec!["line 1"];
        let result = ensure_file_exists(&temp_file, content);

        assert!(result.is_err());
    }
}
