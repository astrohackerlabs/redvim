use std::{fs, process::Command};

#[cfg(unix)]
#[tokio::test]
async fn detach_collision_suggests_redvim_without_starting_an_editor() {
    // Only an inert socket fixture: no detached editor process or live session.
    let root = tempfile::tempdir_in("/tmp").unwrap();
    let directory = root.path().join("astrohacker/redvim/run");
    let _fixture = red::headless::bind_session(&directory, "brand").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_redvim"))
        .arg("--detach=brand")
        .current_dir(root.path())
        .env("HOME", root.path())
        .env("XDG_CONFIG_HOME", root.path())
        .env_remove("REDVIM_RUNTIME")
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("redvim --attach brand"));
}

#[test]
fn branded_cli_forwarding_runtime_guidance_and_safe_errors() {
    let root = tempfile::tempdir().unwrap();
    let run = |args: &[&str]| {
        Command::new(env!("CARGO_BIN_EXE_redvim"))
            .args(args)
            .current_dir(root.path())
            .env("HOME", root.path())
            .env("XDG_CONFIG_HOME", root.path())
            .env("XDG_DATA_HOME", root.path().join("data"))
            .env("XDG_CACHE_HOME", root.path().join("cache"))
            .env_remove("REDVIM_RUNTIME")
            .output()
            .unwrap()
    };
    for (args, expected) in [
        (vec!["husk", "--help"], "Usage: redvim husk"),
        (vec!["--runtime-files"], "redvim --eject"),
    ] {
        let output = run(&args);
        assert!(output.status.success(), "{:?}", output);
        assert!(String::from_utf8_lossy(&output.stdout).contains(expected));
    }
    let output = run(&["husk", "--not-an-option"]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("redvim husk"));
    for args in [
        vec!["--attach", "branding-nonexistent-session"],
        vec![
            "language",
            "trust",
            "/nonexistent-redvim-branding-parser.so",
        ],
    ] {
        let output = run(&args);
        assert!(!output.status.success());
        assert!(!output.stderr.is_empty());
    }
    let template = red::assets::starter_config();
    assert!(template.contains("configuration file for RedVim"));
    assert!(template.contains("redvim language trust ~/.local/share/nvim/site/parser/buildspec.so"));
}

#[test]
fn redvim_identity_and_configuration_are_independent_of_red() {
    let root = tempfile::tempdir().unwrap();
    fs::create_dir(root.path().join("red")).unwrap();
    fs::create_dir_all(root.path().join("astrohacker/redvim")).unwrap();
    let old = root.path().join("red/config.toml");
    fs::write(&old, "this is deliberately invalid TOML").unwrap();
    for (args, expected) in [
        (
            vec!["--version"],
            concat!("redvim ", env!("CARGO_PKG_VERSION")),
        ),
        (vec!["--help"], "Usage: redvim"),
        (vec!["--check-config"], "config ok"),
        (vec!["--self-check"], "language nu: bundled highlighting ok"),
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_redvim"))
            .args(args)
            .current_dir(root.path())
            .env("XDG_CONFIG_HOME", root.path())
            .env("HOME", root.path())
            .env("RED_RUNTIME", root.path().join("missing-runtime"))
            .env_remove("REDVIM_RUNTIME")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(String::from_utf8_lossy(&output.stdout).contains(expected));
    }
    assert_eq!(
        fs::read_to_string(old).unwrap(),
        "this is deliberately invalid TOML"
    );
}
