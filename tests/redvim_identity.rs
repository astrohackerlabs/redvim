use std::{fs, process::Command};

#[test]
fn redvim_identity_and_configuration_are_independent_of_red() {
    let root = tempfile::tempdir().unwrap();
    fs::create_dir(root.path().join("red")).unwrap();
    fs::create_dir(root.path().join("redvim")).unwrap();
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
