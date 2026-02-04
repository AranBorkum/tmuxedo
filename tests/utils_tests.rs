use tmuxedo::utils::format_plugin_dir_name;

#[test]
fn test_replaces_single_slash() {
    let input = "user/repo";
    let expected = "user_repo";
    assert_eq!(format_plugin_dir_name(input), expected);
}

#[test]
fn test_replaces_multiple_slashes() {
    let input = "github.com/user/repo";
    let expected = "github.com_user_repo";
    assert_eq!(format_plugin_dir_name(input), expected);
}

#[test]
fn test_no_slashes_remains_unchanged() {
    let input = "simple-plugin";
    let expected = "simple-plugin";
    assert_eq!(format_plugin_dir_name(input), expected);
}

#[test]
fn test_handles_trailing_slash() {
    // Important edge case for directory paths
    let input = "user/repo/";
    let expected = "user_repo_";
    assert_eq!(format_plugin_dir_name(input), expected);
}

#[test]
fn test_handles_empty_string() {
    let input = "";
    let expected = "";
    assert_eq!(format_plugin_dir_name(input), expected);
}
