use clap::Parser;
use tmuxedo::cli::{Cli, Commands};

// Helper to simulate running the app with arguments
fn parse_args(args: Vec<&str>) -> Result<Cli, clap::Error> {
    Cli::try_parse_from(args)
}

#[test]
fn test_no_args_is_valid() {
    // Running just `my-app` should be valid (Commands is Option<>)
    let result = parse_args(vec!["my-app"]);
    assert!(result.is_ok());
    let cli = result.unwrap();
    assert_eq!(cli.tui, false);
    assert_eq!(cli.update, false);
    assert_eq!(cli.commands, None);
}

#[test]
fn test_tui_flag() {
    let result = parse_args(vec!["my-app", "--tui"]);
    assert!(result.is_ok());
    let cli = result.unwrap();
    assert!(cli.tui);
    assert!(!cli.update);
}

#[test]
fn test_update_flag() {
    let result = parse_args(vec!["my-app", "--update"]);
    assert!(result.is_ok());
    let cli = result.unwrap();
    assert!(!cli.tui);
    assert!(cli.update);
}

#[test]
fn test_conflict_tui_and_update() {
    // Fails because tui conflicts with update
    let result = parse_args(vec!["my-app", "--tui", "--update"]);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().kind(),
        clap::error::ErrorKind::ArgumentConflict
    );
}

#[test]
fn test_global_conflict_tui_and_subcommand() {
    // Fails because of #[command(args_conflicting_with_subcommands = true)]
    let result = parse_args(vec!["my-app", "--tui", "install", "--path", "."]);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().kind(),
        clap::error::ErrorKind::ArgumentConflict
    );
}

#[test]
fn test_global_conflict_update_and_subcommand() {
    let result = parse_args(vec!["my-app", "--update", "install", "--path", "."]);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().kind(),
        clap::error::ErrorKind::ArgumentConflict
    );
}

#[test]
fn test_install_valid() {
    let result = parse_args(vec!["my-app", "install", "--path", "/tmp"]);
    assert!(result.is_ok());

    match result.unwrap().commands {
        Some(Commands::Install { path, branch }) => {
            assert_eq!(path, "/tmp");
            assert_eq!(branch, None);
        }
        _ => panic!("Expected Install command"),
    }
}

#[test]
fn test_install_with_branch() {
    let result = parse_args(vec![
        "my-app", "install", "--path", "/tmp", "--branch", "dev",
    ]);
    assert!(result.is_ok());

    match result.unwrap().commands {
        Some(Commands::Install { path, branch }) => {
            assert_eq!(path, "/tmp");
            assert_eq!(branch, Some("dev".to_string()));
        }
        _ => panic!("Expected Install command"),
    }
}

#[test]
fn test_install_missing_required_path() {
    // Path is mandatory, so this must fail
    let result = parse_args(vec!["my-app", "install"]);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().kind(),
        clap::error::ErrorKind::MissingRequiredArgument
    );
}

#[test]
fn test_branch_requires_path_enforcement() {
    // Even if path wasn't mandatory by itself, `branch` requires `path`.
    // Since `path` IS mandatory, this fails with MissingRequiredArgument.
    let result = parse_args(vec!["my-app", "install", "--branch", "main"]);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().kind(),
        clap::error::ErrorKind::MissingRequiredArgument
    );
}
