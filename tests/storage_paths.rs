use std::{fs, path::Path, process::Command};

fn command(home: &Path, xdg: Option<&Path>, cwd: &Path) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_redvim"));
    command
        .env("HOME", home)
        .env_remove("XDG_CONFIG_HOME")
        .env_remove("REDVIM_RUNTIME")
        .env_remove("RED_RUNTIME")
        .current_dir(cwd);
    if let Some(xdg) = xdg {
        command.env("XDG_CONFIG_HOME", xdg);
    }
    command
}

#[test]
fn config_location_matrix_reads_only_the_namespaced_root() {
    for mode in ["unset", "empty", "override", "spaces"] {
        let temp = tempfile::tempdir().unwrap();
        let home = temp.path().join("home");
        let xdg = match mode {
            "unset" => None,
            "empty" => Some(Path::new("").to_path_buf()),
            "override" => Some(temp.path().join("xdg")),
            _ => Some(temp.path().join("xdg with spaces")),
        };
        let base = xdg
            .as_ref()
            .filter(|p| !p.as_os_str().is_empty())
            .cloned()
            .unwrap_or_else(|| home.join(".config"));
        let root = base.join("astrohacker/redvim");
        fs::create_dir_all(&root).unwrap();
        for old in [
            base.join("red"),
            base.join("redvim"),
            home.join(".config/redvim"),
        ] {
            fs::create_dir_all(&old).unwrap();
            fs::write(old.join("config.toml"), "invalid = [").unwrap();
        }
        let config = root.join("config.toml");
        fs::write(&config, "relative_line_numbers = true\n").unwrap();
        let output = command(&home, xdg.as_deref(), temp.path())
            .arg("--check-config")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{mode}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        fs::write(&config, "invalid = [").unwrap();
        let output = command(&home, xdg.as_deref(), temp.path())
            .arg("--check-config")
            .output()
            .unwrap();
        assert!(!output.status.success(), "{mode}");
        assert!(String::from_utf8_lossy(&output.stdout).contains(&config.display().to_string()));
        fs::write(&config, "relative_line_numbers = true\n").unwrap();
        // Loading a configured custom theme proves runtime overrides use this root too.
        let output = command(&home, xdg.as_deref(), temp.path())
            .args(["--eject", "themes/mocha.json"])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(root.join("themes/mocha.json").is_file());
        for old in [base.join("red"), base.join("redvim")] {
            assert_eq!(
                fs::read_to_string(old.join("config.toml")).unwrap(),
                "invalid = ["
            );
            assert_eq!(fs::read_dir(old).unwrap().count(), 1);
        }
        assert!(!temp.path().join("red.log").exists());
        assert!(!temp.path().join("redvim.log").exists());
    }
}

// Run the actual macro in a child so its global OnceCell and environment are isolated.
#[test]
fn fallback_log_child() {
    if std::env::var_os("REDVIM_TEST_LOG_CHILD").is_none() {
        return;
    }
    red::log!("fallback storage probe");
}

#[test]
fn fallback_log_is_namespaced_and_failure_is_nonfatal() {
    for blocked in [false, true] {
        let temp = tempfile::tempdir().unwrap();
        let base = temp.path().join("config");
        fs::create_dir(&base).unwrap();
        if blocked {
            fs::write(base.join("astrohacker"), "not a directory").unwrap();
        }
        let output = Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "fallback_log_child", "--nocapture"])
            .env("REDVIM_TEST_LOG_CHILD", "1")
            .env("XDG_CONFIG_HOME", &base)
            .env("HOME", temp.path())
            .current_dir(temp.path())
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let log = base.join("astrohacker/redvim/redvim.log");
        assert_eq!(log.exists(), !blocked);
        if !blocked {
            assert!(fs::read_to_string(log)
                .unwrap()
                .contains("fallback storage probe"));
        }
        assert!(!temp.path().join("red.log").exists());
        assert!(!temp.path().join("redvim.log").exists());
    }
}

#[tokio::test]
async fn storage_components_child() {
    if std::env::var_os("REDVIM_TEST_STORAGE_CHILD").is_none() {
        return;
    }
    use red::{
        config::Config,
        plugin::package::{PluginId, PluginPackageManager},
        preferences::PreferencesStore,
    };
    let root = Config::config_dir();
    fs::create_dir_all(root.join("state/plugins")).unwrap();
    fs::write(
        root.join("state/plugins/session_restore.json"),
        r#"{"latest":{"marker":"namespace"}}"#,
    )
    .unwrap();
    let mut preferences = PreferencesStore::load(Config::path("preferences.json"));
    assert_eq!(
        preferences
            .plugin_storage("session_restore", "latest")
            .unwrap()["marker"],
        "namespace"
    );
    preferences.record_search("namespace search").unwrap();
    assert!(PreferencesStore::load(Config::path("preferences.json"))
        .search_history()
        .contains(&"namespace search".to_string()));

    let package = root.join("fixture-package");
    fs::create_dir(&package).unwrap();
    fs::write(
        package.join("red-plugin.toml"),
        r#"
schema_version = 1
[plugin]
id = "namespace-probe"
name = "Namespace probe"
version = "1.0.0"
red_api = "*"
[languages.probe]
extensions = ["probe"]
[languages.probe.grammar]
builtin = "rust"
"#,
    )
    .unwrap();
    let manager = PluginPackageManager::new(&root);
    manager.install_path(&package).await.unwrap();
    assert_eq!(manager.list().unwrap()[0].id.as_str(), "namespace-probe");
    assert!(root.join("plugins/namespace-probe").is_dir());
    let data = manager.data_dir(&PluginId::parse("namespace-probe").unwrap());
    fs::create_dir_all(&data).unwrap();
    fs::write(data.join("value"), "persisted").unwrap();
    assert_eq!(
        fs::read_to_string(root.join("plugin-data/namespace-probe/value")).unwrap(),
        "persisted"
    );

    // Trust and stage real bytes through the production loader; this deliberately
    // invalid library must be quarantined after staging, never executed.
    let grammar = root.join("probe.so");
    fs::write(&grammar, b"not a shared library").unwrap();
    let trust = red::language::GrammarTrustStore::new(&root);
    let digest = trust.trust_path(&grammar).unwrap();
    assert!(root.join("trusted-grammars.json").is_file());
    let text = format!("[languages.external_probe]\nextensions = [\"probe2\"]\n[languages.external_probe.grammar]\npath = {:?}\nsymbol = \"tree_sitter_probe\"\n", grammar);
    let config = root.join("probe.toml");
    fs::write(&config, text).unwrap();
    let mut loaded = Config::load_user_file(&config, &[]).unwrap();
    red::language::finalize_language_configuration(&mut loaded, &root).unwrap();
    assert!(!loaded.diagnostics.is_empty());
    assert_eq!(
        fs::read(root.join(format!("grammar-cache/{digest}.so"))).unwrap(),
        b"not a shared library"
    );
    trust.revoke_path(&grammar).unwrap();
}

#[test]
fn storage_components_use_the_resolved_root() {
    let temp = tempfile::tempdir().unwrap();
    let output = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "storage_components_child", "--nocapture"])
        .env("REDVIM_TEST_STORAGE_CHILD", "1")
        .env("XDG_CONFIG_HOME", temp.path())
        .env("HOME", temp.path())
        .env_remove("REDVIM_RUNTIME")
        .current_dir(temp.path())
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!temp.path().join("redvim").exists());
    assert!(!temp.path().join("red").exists());
}

#[test]
fn configured_log_paths_remain_explicit() {
    for absolute in [false, true] {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("astrohacker/redvim");
        fs::create_dir_all(&root).unwrap();
        let log = if absolute {
            temp.path().join("chosen.log")
        } else {
            root.join("chosen.log")
        };
        let setting = if absolute {
            log.clone()
        } else {
            "chosen.log".into()
        };
        fs::write(
            root.join("config.toml"),
            format!("log_file = {:?}\n", setting),
        )
        .unwrap();
        let output = command(temp.path(), Some(temp.path()), temp.path())
            .arg("--check-config")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert!(log.is_file());
        assert!(!root.join("redvim.log").exists());
        // An unwritable sink is reported and disabled, without a fallback in CWD.
        fs::remove_file(&log).unwrap();
        fs::create_dir(&log).unwrap();
        let output = command(temp.path(), Some(temp.path()), temp.path())
            .arg("--check-config")
            .output()
            .unwrap();
        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stdout).contains("CFG303"));
        assert!(!temp.path().join("red.log").exists());
    }
}

#[cfg(unix)]
#[tokio::test]
async fn detached_owner_uses_namespaced_ipc_and_cleans_up() {
    use std::time::{Duration, Instant};
    // macOS Unix sockets have a small path limit; avoid the long system TMPDIR.
    let temp = tempfile::Builder::new()
        .prefix("rv-")
        .tempdir_in("/tmp")
        .unwrap();
    let root = temp.path().join("astrohacker/redvim");
    fs::create_dir_all(&root).unwrap();
    fs::write(root.join("config.toml"), "disable_ai = true\nfetch_release_notes = false\nshow_whats_new = false\n[lsp]\nenabled = false\n").unwrap();
    struct Owner(std::process::Child);
    impl Drop for Owner {
        fn drop(&mut self) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
    let mut owner = Owner(
        command(temp.path(), Some(temp.path()), temp.path())
            .args(["--core-session", "probe"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .unwrap(),
    );
    let run = root.join("run");
    let deadline = Instant::now() + Duration::from_secs(15);
    while !run.join("probe.pid").exists() {
        assert!(Instant::now() < deadline, "owner startup timed out");
        assert!(owner.0.try_wait().unwrap().is_none(), "owner exited early");
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
    let client = red::headless::connect_session(&run, "probe", None, (80, 24))
        .await
        .unwrap();
    assert!(run.join("probe.sock").exists());
    assert!(run.join("probe.token").exists());
    drop(client);
    let stopped = command(temp.path(), Some(temp.path()), temp.path())
        .args(["--stop", "probe"])
        .output()
        .unwrap();
    assert!(
        stopped.status.success(),
        "{}",
        String::from_utf8_lossy(&stopped.stderr)
    );
    while owner.0.try_wait().unwrap().is_none() {
        assert!(Instant::now() < deadline, "owner shutdown timed out");
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
    for suffix in ["sock", "token", "pid"] {
        assert!(!run.join(format!("probe.{suffix}")).exists());
    }
    assert!(root.join("redvim.log").exists());
    assert!(root.join("sessions").is_dir());
    assert!(!temp.path().join("redvim").exists());

    let long = root.join("a".repeat(110)).join("run");
    let error = match red::headless::bind_session(&long, "probe") {
        Ok(_) => panic!("overlong socket path was accepted"),
        Err(error) => error.to_string(),
    };
    assert!(error.contains("socket"), "{error}");
    assert!(!long.join("probe.token").exists());
    assert!(!long.join("probe.pid").exists());
}
