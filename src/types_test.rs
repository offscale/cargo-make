use super::*;

#[test]
fn cli_args_new() {
    let cli_args = CliArgs::new();

    assert_eq!(cli_args.command, "");
    assert_eq!(cli_args.build_file, "Makefile.toml");
    assert_eq!(cli_args.task, "default");
    assert_eq!(cli_args.log_level, "info");
    assert!(cli_args.cwd.is_none());
    assert!(cli_args.env.is_none());
    assert!(cli_args.env_file.is_none());
    assert!(!cli_args.disable_workspace);
    assert!(!cli_args.disable_on_error);
    assert!(!cli_args.disable_check_for_updates);
    assert!(!cli_args.print_only);
    assert!(!cli_args.list_all_steps);
    assert!(!cli_args.experimental);
    assert!(cli_args.arguments.is_none());
}

#[test]
fn global_config_new() {
    let global_config = GlobalConfig::new();

    assert!(global_config.file_name.is_none());
    assert!(global_config.log_level.is_none());
    assert!(global_config.default_task_name.is_none());
    assert!(global_config.update_check_minimum_interval.is_none());
    assert!(!global_config.search_project_root.unwrap());
}

#[test]
fn cache_new() {
    let cache = Cache::new();

    assert!(cache.file_name.is_none());
    assert!(cache.last_update_check.is_none());
}

#[test]
fn install_crate_info_eq_same_all() {
    let first = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: "--help".to_string(),
        rustup_component_name: Some("component".to_string()),
    };
    let second = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: "--help".to_string(),
        rustup_component_name: Some("component".to_string()),
    };

    assert_eq!(first, second);
}

#[test]
fn install_crate_info_eq_same_no_component() {
    let first = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: "--help".to_string(),
        rustup_component_name: None,
    };
    let second = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: "--help".to_string(),
        rustup_component_name: None,
    };

    assert_eq!(first, second);
}

#[test]
fn install_crate_info_eq_different_crate_name() {
    let first = InstallCrateInfo {
        crate_name: "test1".to_string(),
        binary: "bin".to_string(),
        test_arg: "--help".to_string(),
        rustup_component_name: None,
    };
    let second = InstallCrateInfo {
        crate_name: "test2".to_string(),
        binary: "bin".to_string(),
        test_arg: "--help".to_string(),
        rustup_component_name: None,
    };

    assert!(first != second);
}

#[test]
fn install_crate_info_eq_different_binary() {
    let first = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin1".to_string(),
        test_arg: "--help".to_string(),
        rustup_component_name: None,
    };
    let second = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin2".to_string(),
        test_arg: "--help".to_string(),
        rustup_component_name: None,
    };

    assert!(first != second);
}

#[test]
fn install_crate_info_eq_different_test_arg() {
    let first = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: "--help1".to_string(),
        rustup_component_name: None,
    };
    let second = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: "--help2".to_string(),
        rustup_component_name: None,
    };

    assert!(first != second);
}

#[test]
fn install_crate_info_eq_different_component_type() {
    let first = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: "--help".to_string(),
        rustup_component_name: Some("value".to_string()),
    };
    let second = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: "--help".to_string(),
        rustup_component_name: None,
    };

    assert!(first != second);
}

#[test]
fn install_crate_info_eq_different_component_value() {
    let first = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: "--help".to_string(),
        rustup_component_name: Some("value1".to_string()),
    };
    let second = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: "--help".to_string(),
        rustup_component_name: Some("value2".to_string()),
    };

    assert!(first != second);
}

#[test]
fn install_rustup_component_info_eq_same_all() {
    let first = InstallRustupComponentInfo {
        rustup_component_name: "component".to_string(),
        binary: Some("bin".to_string()),
        test_arg: Some("--help".to_string()),
    };
    let second = InstallRustupComponentInfo {
        rustup_component_name: "component".to_string(),
        binary: Some("bin".to_string()),
        test_arg: Some("--help".to_string()),
    };

    assert_eq!(first, second);
}

#[test]
fn install_rustup_component_info_eq_same_no_binary() {
    let first = InstallRustupComponentInfo {
        rustup_component_name: "component".to_string(),
        binary: None,
        test_arg: Some("--help".to_string()),
    };
    let second = InstallRustupComponentInfo {
        rustup_component_name: "component".to_string(),
        binary: None,
        test_arg: Some("--help".to_string()),
    };

    assert_eq!(first, second);
}

#[test]
fn install_rustup_component_info_eq_same_no_test_arg() {
    let first = InstallRustupComponentInfo {
        rustup_component_name: "component".to_string(),
        binary: Some("bin".to_string()),
        test_arg: None,
    };
    let second = InstallRustupComponentInfo {
        rustup_component_name: "component".to_string(),
        binary: Some("bin".to_string()),
        test_arg: None,
    };

    assert_eq!(first, second);
}

#[test]
fn install_rustup_component_info_eq_different_component() {
    let first = InstallRustupComponentInfo {
        rustup_component_name: "component1".to_string(),
        binary: Some("bin".to_string()),
        test_arg: Some("--help".to_string()),
    };
    let second = InstallRustupComponentInfo {
        rustup_component_name: "component2".to_string(),
        binary: Some("bin".to_string()),
        test_arg: Some("--help".to_string()),
    };

    assert!(first != second);
}

#[test]
fn install_rustup_component_info_eq_different_binary() {
    let first = InstallRustupComponentInfo {
        rustup_component_name: "component".to_string(),
        binary: Some("bin1".to_string()),
        test_arg: Some("--help".to_string()),
    };
    let second = InstallRustupComponentInfo {
        rustup_component_name: "component".to_string(),
        binary: Some("bin2".to_string()),
        test_arg: Some("--help".to_string()),
    };

    assert!(first != second);
}

#[test]
fn install_rustup_component_info_eq_different_binary_type() {
    let first = InstallRustupComponentInfo {
        rustup_component_name: "component".to_string(),
        binary: Some("bin".to_string()),
        test_arg: Some("--help".to_string()),
    };
    let second = InstallRustupComponentInfo {
        rustup_component_name: "component".to_string(),
        binary: None,
        test_arg: Some("--help".to_string()),
    };

    assert!(first != second);
}

#[test]
fn install_rustup_component_info_eq_different_test_arg() {
    let first = InstallRustupComponentInfo {
        rustup_component_name: "component".to_string(),
        binary: Some("bin".to_string()),
        test_arg: Some("--hel1p".to_string()),
    };
    let second = InstallRustupComponentInfo {
        rustup_component_name: "component".to_string(),
        binary: Some("bin".to_string()),
        test_arg: Some("--help2".to_string()),
    };

    assert!(first != second);
}

#[test]
fn install_rustup_component_info_eq_different_test_arg_type() {
    let first = InstallRustupComponentInfo {
        rustup_component_name: "component".to_string(),
        binary: Some("bin".to_string()),
        test_arg: None,
    };
    let second = InstallRustupComponentInfo {
        rustup_component_name: "component".to_string(),
        binary: None,
        test_arg: Some("--help".to_string()),
    };

    assert!(first != second);
}

#[test]
fn install_crate_eq_same_value() {
    let first = InstallCrate::Value("crate".to_string());
    let second = InstallCrate::Value("crate".to_string());

    assert_eq!(first, second);
}

#[test]
fn install_crate_eq_same_info() {
    let info = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: "--help".to_string(),
        rustup_component_name: Some("value".to_string()),
    };
    let first = InstallCrate::CrateInfo(info.clone());
    let second = InstallCrate::CrateInfo(info.clone());

    assert_eq!(first, second);
}

#[test]
fn install_crate_eq_different_value() {
    let first = InstallCrate::Value("crate1".to_string());
    let second = InstallCrate::Value("crate2".to_string());

    assert!(first != second);
}

#[test]
fn install_crate_eq_different_crate_info() {
    let first = InstallCrate::CrateInfo(InstallCrateInfo {
        crate_name: "test1".to_string(),
        binary: "bin".to_string(),
        test_arg: "--help".to_string(),
        rustup_component_name: Some("value".to_string()),
    });
    let second = InstallCrate::CrateInfo(InstallCrateInfo {
        crate_name: "test2".to_string(),
        binary: "bin".to_string(),
        test_arg: "--help".to_string(),
        rustup_component_name: Some("value".to_string()),
    });

    assert!(first != second);
}

#[test]
fn install_crate_eq_different_rustup_component_info() {
    let first = InstallCrate::RustupComponentInfo(InstallRustupComponentInfo {
        rustup_component_name: "component1".to_string(),
        binary: Some("bin".to_string()),
        test_arg: Some("--help".to_string()),
    });
    let second = InstallCrate::RustupComponentInfo(InstallRustupComponentInfo {
        rustup_component_name: "component2".to_string(),
        binary: Some("bin".to_string()),
        test_arg: Some("--help".to_string()),
    });

    assert!(first != second);
}

#[test]
fn task_new() {
    let task = Task::new();

    assert!(task.clear.is_none());
    assert!(task.install_crate.is_none());
    assert!(task.install_crate_args.is_none());
    assert!(task.command.is_none());
    assert!(task.disabled.is_none());
    assert!(task.private.is_none());
    assert!(task.condition.is_none());
    assert!(task.condition_script.is_none());
    assert!(task.description.is_none());
    assert!(task.category.is_none());
    assert!(task.workspace.is_none());
    assert!(task.force.is_none());
    assert!(task.env.is_none());
    assert!(task.cwd.is_none());
    assert!(task.alias.is_none());
    assert!(task.linux_alias.is_none());
    assert!(task.windows_alias.is_none());
    assert!(task.mac_alias.is_none());
    assert!(task.install_script.is_none());
    assert!(task.args.is_none());
    assert!(task.script.is_none());
    assert!(task.script_runner.is_none());
    assert!(task.script_extension.is_none());
    assert!(task.run_task.is_none());
    assert!(task.dependencies.is_none());
    assert!(task.toolchain.is_none());
    assert!(task.linux.is_none());
    assert!(task.windows.is_none());
    assert!(task.mac.is_none());
}

#[test]
fn external_config_new() {
    let config = ExternalConfig::new();

    assert!(config.extend.is_none());
    assert!(config.config.is_none());
    assert!(config.env.is_none());
    assert!(config.tasks.is_none());
}

#[test]
fn task_is_force_none() {
    let task = Task::new();
    assert!(!task.is_force());
}

#[test]
fn task_is_force_false() {
    let mut task = Task::new();
    task.force = Some(false);
    assert!(!task.is_force());
}

#[test]
fn task_is_force_true() {
    let mut task = Task::new();
    task.force = Some(true);
    assert!(task.is_force());
}

#[test]
fn task_extend_both_have_misc_data() {
    let mut base = Task::new();
    base.install_crate = Some(InstallCrate::Value("my crate1".to_string()));
    base.command = Some("test1".to_string());
    base.disabled = Some(false);
    base.private = Some(false);
    base.script = Some(vec!["1".to_string(), "2".to_string()]);

    let extended = Task {
        clear: Some(false),
        install_crate: Some(InstallCrate::Value("my crate2".to_string())),
        command: None,
        description: None,
        category: None,
        workspace: None,
        disabled: Some(true),
        private: Some(true),
        condition: None,
        condition_script: None,
        force: Some(true),
        env: Some(IndexMap::new()),
        cwd: None,
        alias: Some("alias2".to_string()),
        linux_alias: None,
        windows_alias: None,
        mac_alias: None,
        install_crate_args: None,
        install_script: None,
        args: None,
        script: None,
        script_runner: None,
        script_extension: None,
        run_task: None,
        dependencies: None,
        toolchain: None,
        linux: None,
        windows: None,
        mac: None,
    };

    base.extend(&extended);

    assert!(!base.clear.unwrap());
    assert!(base.install_crate.is_some());
    assert!(base.command.is_some());
    assert!(base.description.is_none());
    assert!(base.category.is_none());
    assert!(base.workspace.is_none());
    assert!(base.disabled.is_some());
    assert!(base.private.is_some());
    assert!(base.condition.is_none());
    assert!(base.condition_script.is_none());
    assert!(base.force.is_some());
    assert!(base.env.is_some());
    assert!(base.cwd.is_none());
    assert!(base.alias.is_some());
    assert!(base.linux_alias.is_none());
    assert!(base.windows_alias.is_none());
    assert!(base.mac_alias.is_none());
    assert!(base.install_crate_args.is_none());
    assert!(base.install_script.is_none());
    assert!(base.script_runner.is_none());
    assert!(base.script_extension.is_none());
    assert!(base.run_task.is_none());
    assert!(base.args.is_none());
    assert!(base.script.is_some());
    assert!(base.dependencies.is_none());
    assert!(base.toolchain.is_none());
    assert!(base.linux.is_none());
    assert!(base.windows.is_none());
    assert!(base.mac.is_none());

    assert_eq!(
        base.install_crate.unwrap(),
        InstallCrate::Value("my crate2".to_string())
    );
    assert_eq!(base.command.unwrap(), "test1");
    assert!(base.disabled.unwrap());
    assert!(base.private.unwrap());
    assert!(base.force.unwrap());
    assert_eq!(base.env.unwrap().len(), 0);
    assert_eq!(base.alias.unwrap(), "alias2");
    assert_eq!(base.script.unwrap().len(), 2);
}

#[test]
fn task_extend_extended_have_all_fields() {
    let mut base = Task {
        clear: Some(true),
        install_crate: Some(InstallCrate::Value("my crate1".to_string())),
        command: Some("test1".to_string()),
        description: None,
        category: None,
        workspace: None,
        disabled: Some(false),
        private: Some(true),
        condition: None,
        condition_script: None,
        force: Some(true),
        env: Some(IndexMap::new()),
        cwd: None,
        alias: None,
        linux_alias: None,
        windows_alias: None,
        mac_alias: None,
        install_crate_args: None,
        install_script: None,
        args: None,
        script: Some(vec!["1".to_string(), "2".to_string()]),
        script_runner: Some("sh1".to_string()),
        script_extension: Some("ext1".to_string()),
        run_task: Some("task1".to_string()),
        dependencies: None,
        toolchain: None,
        linux: None,
        windows: None,
        mac: None,
    };

    let mut env = IndexMap::new();
    env.insert("test".to_string(), EnvValue::Value("value".to_string()));
    let extended = Task {
        clear: Some(false),
        install_crate: Some(InstallCrate::Value("my crate2".to_string())),
        install_crate_args: Some(vec!["c1".to_string(), "c2".to_string()]),
        command: Some("test2".to_string()),
        description: Some("description".to_string()),
        category: Some("category".to_string()),
        workspace: Some(true),
        disabled: Some(true),
        private: Some(false),
        condition: Some(TaskCondition {
            platforms: Some(vec!["linux".to_string(), "mac".to_string()]),
            channels: Some(vec!["nightly".to_string(), "stable".to_string()]),
            env_set: None,
            env_not_set: None,
            env: None,
            rust_version: None,
        }),
        condition_script: Some(vec!["exit 0".to_string()]),
        force: Some(false),
        env: Some(env.clone()),
        cwd: Some("cwd".to_string()),
        alias: Some("alias2".to_string()),
        linux_alias: Some("linux".to_string()),
        windows_alias: Some("windows".to_string()),
        mac_alias: Some("mac".to_string()),
        install_script: Some(vec!["i1".to_string(), "i2".to_string()]),
        args: Some(vec!["a1".to_string(), "a2".to_string()]),
        script: Some(vec!["1".to_string(), "2".to_string(), "3".to_string()]),
        script_runner: Some("sh2".to_string()),
        script_extension: Some("ext2".to_string()),
        run_task: Some("task2".to_string()),
        dependencies: Some(vec!["A".to_string()]),
        toolchain: Some("toolchain".to_string()),
        linux: Some(PlatformOverrideTask {
            clear: Some(true),
            install_crate: Some(InstallCrate::Value("my crate2".to_string())),
            install_crate_args: Some(vec!["c1".to_string(), "c2".to_string()]),
            command: Some("test2".to_string()),
            disabled: Some(true),
            private: Some(false),
            condition: Some(TaskCondition {
                platforms: Some(vec!["linux".to_string(), "mac".to_string()]),
                channels: Some(vec!["nightly".to_string(), "stable".to_string()]),
                env_set: None,
                env_not_set: None,
                env: None,
                rust_version: None,
            }),
            condition_script: Some(vec!["exit 0".to_string()]),
            force: Some(true),
            env: Some(env.clone()),
            cwd: Some("cwd".to_string()),
            install_script: Some(vec!["i1".to_string(), "i2".to_string()]),
            args: Some(vec!["a1".to_string(), "a2".to_string()]),
            script: Some(vec!["1".to_string(), "2".to_string(), "3".to_string()]),
            script_runner: Some("sh3".to_string()),
            script_extension: Some("ext3".to_string()),
            run_task: Some("task3".to_string()),
            dependencies: Some(vec!["A".to_string()]),
            toolchain: Some("toolchain".to_string()),
        }),
        windows: Some(PlatformOverrideTask {
            clear: Some(false),
            install_crate: Some(InstallCrate::Value("my crate2".to_string())),
            install_crate_args: Some(vec!["c1".to_string(), "c2".to_string()]),
            command: Some("test2".to_string()),
            disabled: Some(true),
            private: Some(false),
            condition: Some(TaskCondition {
                platforms: Some(vec!["linux".to_string(), "mac".to_string()]),
                channels: Some(vec!["nightly".to_string(), "stable".to_string()]),
                env_set: None,
                env_not_set: None,
                env: None,
                rust_version: None,
            }),
            condition_script: Some(vec!["exit 0".to_string()]),
            force: Some(true),
            env: Some(env.clone()),
            cwd: Some("cwd".to_string()),
            install_script: Some(vec!["i1".to_string(), "i2".to_string()]),
            args: Some(vec!["a1".to_string(), "a2".to_string()]),
            script: Some(vec!["1".to_string(), "2".to_string(), "3".to_string()]),
            script_runner: Some("sh3".to_string()),
            script_extension: Some("ext3".to_string()),
            run_task: Some("task3".to_string()),
            dependencies: Some(vec!["A".to_string()]),
            toolchain: Some("toolchain".to_string()),
        }),
        mac: Some(PlatformOverrideTask {
            clear: None,
            install_crate: Some(InstallCrate::Value("my crate2".to_string())),
            install_crate_args: Some(vec!["c1".to_string(), "c2".to_string()]),
            command: Some("test2".to_string()),
            disabled: Some(true),
            private: Some(false),
            condition: Some(TaskCondition {
                platforms: Some(vec!["linux".to_string(), "mac".to_string()]),
                channels: Some(vec!["nightly".to_string(), "stable".to_string()]),
                env_set: None,
                env_not_set: None,
                env: None,
                rust_version: None,
            }),
            condition_script: Some(vec!["exit 0".to_string()]),
            force: Some(true),
            env: Some(env.clone()),
            cwd: Some("cwd".to_string()),
            install_script: Some(vec!["i1".to_string(), "i2".to_string()]),
            args: Some(vec!["a1".to_string(), "a2".to_string()]),
            script: Some(vec!["1".to_string(), "2".to_string(), "3".to_string()]),
            script_runner: Some("sh3".to_string()),
            script_extension: Some("ext3".to_string()),
            run_task: Some("task3".to_string()),
            dependencies: Some(vec!["A".to_string()]),
            toolchain: Some("toolchain".to_string()),
        }),
    };

    base.extend(&extended);

    assert!(!base.clear.unwrap());
    assert!(base.install_crate.is_some());
    assert!(base.install_crate_args.is_some());
    assert!(base.command.is_some());
    assert!(base.description.is_some());
    assert!(base.category.is_some());
    assert!(base.workspace.is_some());
    assert!(base.disabled.is_some());
    assert!(base.private.is_some());
    assert!(base.condition.is_some());
    assert!(base.condition_script.is_some());
    assert!(base.force.is_some());
    assert!(base.env.is_some());
    assert!(base.cwd.is_some());
    assert!(base.alias.is_some());
    assert!(base.linux_alias.is_some());
    assert!(base.windows_alias.is_some());
    assert!(base.mac_alias.is_some());
    assert!(base.install_script.is_some());
    assert!(base.args.is_some());
    assert!(base.script.is_some());
    assert!(base.script_runner.is_some());
    assert!(base.script_extension.is_some());
    assert!(base.run_task.is_some());
    assert!(base.dependencies.is_some());
    assert!(base.toolchain.is_some());
    assert!(base.linux.is_some());
    assert!(base.windows.is_some());
    assert!(base.mac.is_some());

    assert_eq!(
        base.install_crate.unwrap(),
        InstallCrate::Value("my crate2".to_string())
    );
    assert_eq!(base.install_crate_args.unwrap().len(), 2);
    assert_eq!(base.command.unwrap(), "test2");
    assert_eq!(base.description.unwrap(), "description");
    assert_eq!(base.category.unwrap(), "category");
    assert!(base.workspace.unwrap());
    assert!(base.disabled.unwrap());
    assert!(!base.private.unwrap());
    assert_eq!(base.condition_script.unwrap().len(), 1);
    assert!(!base.force.unwrap());
    assert_eq!(base.env.unwrap().len(), 1);
    assert_eq!(base.cwd.unwrap(), "cwd".to_string());
    assert_eq!(base.alias.unwrap(), "alias2");
    assert_eq!(base.linux_alias.unwrap(), "linux");
    assert_eq!(base.windows_alias.unwrap(), "windows");
    assert_eq!(base.mac_alias.unwrap(), "mac");
    assert_eq!(base.install_script.unwrap().len(), 2);
    assert_eq!(base.args.unwrap().len(), 2);
    assert_eq!(base.script.unwrap().len(), 3);
    assert_eq!(base.script_runner.unwrap(), "sh2");
    assert_eq!(base.script_extension.unwrap(), "ext2");
    assert_eq!(base.run_task.unwrap(), "task2");
    assert_eq!(base.dependencies.unwrap().len(), 1);
    assert_eq!(base.toolchain.unwrap(), "toolchain");
    assert!(base.linux.unwrap().clear.unwrap());
    assert!(!base.windows.unwrap().clear.unwrap());
    assert!(base.mac.unwrap().clear.is_none());

    let condition = base.condition.unwrap();
    assert_eq!(condition.platforms.unwrap().len(), 2);
    assert_eq!(condition.channels.unwrap().len(), 2);
}

#[test]
fn task_extend_clear_with_no_data() {
    let env = IndexMap::new();
    let mut base = Task {
        clear: Some(false),
        install_crate: Some(InstallCrate::Value("my crate2".to_string())),
        install_crate_args: Some(vec!["c1".to_string(), "c2".to_string()]),
        command: Some("test2".to_string()),
        description: Some("description".to_string()),
        category: Some("category".to_string()),
        workspace: Some(false),
        disabled: Some(true),
        private: Some(false),
        condition: Some(TaskCondition {
            platforms: Some(vec!["linux".to_string(), "mac".to_string()]),
            channels: Some(vec!["nightly".to_string(), "stable".to_string()]),
            env_set: None,
            env_not_set: None,
            env: None,
            rust_version: None,
        }),
        condition_script: Some(vec!["exit 0".to_string()]),
        force: Some(false),
        env: Some(env.clone()),
        cwd: Some("cwd".to_string()),
        alias: Some("alias2".to_string()),
        linux_alias: Some("linux".to_string()),
        windows_alias: Some("windows".to_string()),
        mac_alias: Some("mac".to_string()),
        install_script: Some(vec!["i1".to_string(), "i2".to_string()]),
        args: Some(vec!["a1".to_string(), "a2".to_string()]),
        script: Some(vec!["1".to_string(), "2".to_string(), "3".to_string()]),
        script_runner: Some("sh2".to_string()),
        script_extension: Some("ext2".to_string()),
        run_task: Some("task2".to_string()),
        dependencies: Some(vec!["A".to_string()]),
        toolchain: Some("toolchain".to_string()),
        linux: Some(PlatformOverrideTask {
            clear: Some(true),
            install_crate: Some(InstallCrate::Value("my crate2".to_string())),
            install_crate_args: Some(vec!["c1".to_string(), "c2".to_string()]),
            command: Some("test2".to_string()),
            disabled: Some(true),
            private: Some(false),
            condition: Some(TaskCondition {
                platforms: Some(vec!["linux".to_string(), "mac".to_string()]),
                channels: Some(vec!["nightly".to_string(), "stable".to_string()]),
                env_set: None,
                env_not_set: None,
                env: None,
                rust_version: None,
            }),
            condition_script: Some(vec!["exit 0".to_string()]),
            force: Some(true),
            env: Some(env.clone()),
            cwd: Some("cwd".to_string()),
            install_script: Some(vec!["i1".to_string(), "i2".to_string()]),
            args: Some(vec!["a1".to_string(), "a2".to_string()]),
            script: Some(vec!["1".to_string(), "2".to_string(), "3".to_string()]),
            script_runner: Some("sh3".to_string()),
            script_extension: Some("ext3".to_string()),
            run_task: Some("task3".to_string()),
            dependencies: Some(vec!["A".to_string()]),
            toolchain: Some("toolchain".to_string()),
        }),
        windows: Some(PlatformOverrideTask {
            clear: Some(false),
            install_crate: Some(InstallCrate::Value("my crate2".to_string())),
            install_crate_args: Some(vec!["c1".to_string(), "c2".to_string()]),
            command: Some("test2".to_string()),
            disabled: Some(true),
            private: Some(false),
            condition: Some(TaskCondition {
                platforms: Some(vec!["linux".to_string(), "mac".to_string()]),
                channels: Some(vec!["nightly".to_string(), "stable".to_string()]),
                env_set: None,
                env_not_set: None,
                env: None,
                rust_version: None,
            }),
            condition_script: Some(vec!["exit 0".to_string()]),
            force: Some(true),
            env: Some(env.clone()),
            cwd: Some("cwd".to_string()),
            install_script: Some(vec!["i1".to_string(), "i2".to_string()]),
            args: Some(vec!["a1".to_string(), "a2".to_string()]),
            script: Some(vec!["1".to_string(), "2".to_string(), "3".to_string()]),
            script_runner: Some("sh3".to_string()),
            script_extension: Some("ext3".to_string()),
            run_task: Some("task3".to_string()),
            dependencies: Some(vec!["A".to_string()]),
            toolchain: Some("toolchain".to_string()),
        }),
        mac: Some(PlatformOverrideTask {
            clear: None,
            install_crate: Some(InstallCrate::Value("my crate2".to_string())),
            install_crate_args: Some(vec!["c1".to_string(), "c2".to_string()]),
            command: Some("test2".to_string()),
            disabled: Some(true),
            private: Some(false),
            condition: Some(TaskCondition {
                platforms: Some(vec!["linux".to_string(), "mac".to_string()]),
                channels: Some(vec!["nightly".to_string(), "stable".to_string()]),
                env_set: None,
                env_not_set: None,
                env: None,
                rust_version: None,
            }),
            condition_script: Some(vec!["exit 0".to_string()]),
            force: Some(true),
            env: Some(env.clone()),
            cwd: Some("cwd".to_string()),
            install_script: Some(vec!["i1".to_string(), "i2".to_string()]),
            args: Some(vec!["a1".to_string(), "a2".to_string()]),
            script: Some(vec!["1".to_string(), "2".to_string(), "3".to_string()]),
            script_runner: Some("sh3".to_string()),
            script_extension: Some("ext3".to_string()),
            run_task: Some("task3".to_string()),
            dependencies: Some(vec!["A".to_string()]),
            toolchain: Some("toolchain".to_string()),
        }),
    };

    let mut extended = Task::new();
    extended.clear = Some(true);

    base.extend(&extended);

    assert!(base.clear.unwrap());
    assert!(base.install_crate.is_none());
    assert!(base.command.is_none());
    assert!(base.description.is_none());
    assert!(base.category.is_none());
    assert!(base.workspace.is_none());
    assert!(base.disabled.is_none());
    assert!(base.private.is_none());
    assert!(base.condition.is_none());
    assert!(base.condition_script.is_none());
    assert!(base.force.is_none());
    assert!(base.env.is_none());
    assert!(base.cwd.is_none());
    assert!(base.alias.is_none());
    assert!(base.linux_alias.is_none());
    assert!(base.windows_alias.is_none());
    assert!(base.mac_alias.is_none());
    assert!(base.install_crate_args.is_none());
    assert!(base.install_script.is_none());
    assert!(base.script_runner.is_none());
    assert!(base.script_extension.is_none());
    assert!(base.run_task.is_none());
    assert!(base.args.is_none());
    assert!(base.script.is_none());
    assert!(base.dependencies.is_none());
    assert!(base.toolchain.is_none());
    assert!(base.linux.is_none());
    assert!(base.windows.is_none());
    assert!(base.mac.is_none());
}

#[test]
fn task_extend_clear_with_all_data() {
    let mut base = Task::new();

    let env = IndexMap::new();
    let extended = Task {
        clear: Some(true),
        install_crate: Some(InstallCrate::Value("my crate2".to_string())),
        install_crate_args: Some(vec!["c1".to_string(), "c2".to_string()]),
        command: Some("test2".to_string()),
        description: Some("description".to_string()),
        category: Some("category".to_string()),
        workspace: Some(true),
        disabled: Some(true),
        private: Some(false),
        condition: Some(TaskCondition {
            platforms: Some(vec!["linux".to_string(), "mac".to_string()]),
            channels: Some(vec!["nightly".to_string(), "stable".to_string()]),
            env_set: None,
            env_not_set: None,
            env: None,
            rust_version: None,
        }),
        condition_script: Some(vec!["exit 0".to_string()]),
        force: Some(false),
        env: Some(env.clone()),
        cwd: Some("cwd".to_string()),
        alias: Some("alias2".to_string()),
        linux_alias: Some("linux".to_string()),
        windows_alias: Some("windows".to_string()),
        mac_alias: Some("mac".to_string()),
        install_script: Some(vec!["i1".to_string(), "i2".to_string()]),
        args: Some(vec!["a1".to_string(), "a2".to_string()]),
        script: Some(vec!["1".to_string(), "2".to_string(), "3".to_string()]),
        script_runner: Some("sh2".to_string()),
        script_extension: Some("ext2".to_string()),
        run_task: Some("task2".to_string()),
        dependencies: Some(vec!["A".to_string()]),
        toolchain: Some("toolchain".to_string()),
        linux: Some(PlatformOverrideTask {
            clear: Some(true),
            install_crate: Some(InstallCrate::Value("my crate2".to_string())),
            install_crate_args: Some(vec!["c1".to_string(), "c2".to_string()]),
            command: Some("test2".to_string()),
            disabled: Some(true),
            private: Some(false),
            condition: Some(TaskCondition {
                platforms: Some(vec!["linux".to_string(), "mac".to_string()]),
                channels: Some(vec!["nightly".to_string(), "stable".to_string()]),
                env_set: None,
                env_not_set: None,
                env: None,
                rust_version: None,
            }),
            condition_script: Some(vec!["exit 0".to_string()]),
            force: Some(true),
            env: Some(env.clone()),
            cwd: Some("cwd".to_string()),
            install_script: Some(vec!["i1".to_string(), "i2".to_string()]),
            args: Some(vec!["a1".to_string(), "a2".to_string()]),
            script: Some(vec!["1".to_string(), "2".to_string(), "3".to_string()]),
            script_runner: Some("sh3".to_string()),
            script_extension: Some("ext3".to_string()),
            run_task: Some("task3".to_string()),
            dependencies: Some(vec!["A".to_string()]),
            toolchain: Some("toolchain".to_string()),
        }),
        windows: Some(PlatformOverrideTask {
            clear: Some(false),
            install_crate: Some(InstallCrate::Value("my crate2".to_string())),
            install_crate_args: Some(vec!["c1".to_string(), "c2".to_string()]),
            command: Some("test2".to_string()),
            disabled: Some(true),
            private: Some(false),
            condition: Some(TaskCondition {
                platforms: Some(vec!["linux".to_string(), "mac".to_string()]),
                channels: Some(vec!["nightly".to_string(), "stable".to_string()]),
                env_set: None,
                env_not_set: None,
                env: None,
                rust_version: None,
            }),
            condition_script: Some(vec!["exit 0".to_string()]),
            force: Some(true),
            env: Some(env.clone()),
            cwd: Some("cwd".to_string()),
            install_script: Some(vec!["i1".to_string(), "i2".to_string()]),
            args: Some(vec!["a1".to_string(), "a2".to_string()]),
            script: Some(vec!["1".to_string(), "2".to_string(), "3".to_string()]),
            script_runner: Some("sh3".to_string()),
            script_extension: Some("ext3".to_string()),
            run_task: Some("task3".to_string()),
            dependencies: Some(vec!["A".to_string()]),
            toolchain: Some("toolchain".to_string()),
        }),
        mac: Some(PlatformOverrideTask {
            clear: None,
            install_crate: Some(InstallCrate::Value("my crate2".to_string())),
            install_crate_args: Some(vec!["c1".to_string(), "c2".to_string()]),
            command: Some("test2".to_string()),
            disabled: Some(true),
            private: Some(false),
            condition: Some(TaskCondition {
                platforms: Some(vec!["linux".to_string(), "mac".to_string()]),
                channels: Some(vec!["nightly".to_string(), "stable".to_string()]),
                env_set: None,
                env_not_set: None,
                env: None,
                rust_version: None,
            }),
            condition_script: Some(vec!["exit 0".to_string()]),
            force: Some(true),
            env: Some(env.clone()),
            cwd: Some("cwd".to_string()),
            install_script: Some(vec!["i1".to_string(), "i2".to_string()]),
            args: Some(vec!["a1".to_string(), "a2".to_string()]),
            script: Some(vec!["1".to_string(), "2".to_string(), "3".to_string()]),
            script_runner: Some("sh3".to_string()),
            script_extension: Some("ext3".to_string()),
            run_task: Some("task3".to_string()),
            dependencies: Some(vec!["A".to_string()]),
            toolchain: Some("toolchain".to_string()),
        }),
    };

    base.extend(&extended);

    assert!(base.clear.unwrap());
    assert!(base.install_crate.is_some());
    assert!(base.command.is_some());
    assert!(base.description.is_some());
    assert!(base.category.is_some());
    assert!(base.workspace.is_some());
    assert!(base.disabled.is_some());
    assert!(base.private.is_some());
    assert!(base.condition.is_some());
    assert!(base.condition_script.is_some());
    assert!(base.force.is_some());
    assert!(base.env.is_some());
    assert!(base.cwd.is_some());
    assert!(base.alias.is_some());
    assert!(base.linux_alias.is_some());
    assert!(base.windows_alias.is_some());
    assert!(base.mac_alias.is_some());
    assert!(base.install_crate_args.is_some());
    assert!(base.install_script.is_some());
    assert!(base.script_runner.is_some());
    assert!(base.script_extension.is_some());
    assert!(base.run_task.is_some());
    assert!(base.args.is_some());
    assert!(base.script.is_some());
    assert!(base.dependencies.is_some());
    assert!(base.toolchain.is_some());
    assert!(base.linux.is_some());
    assert!(base.windows.is_some());
    assert!(base.mac.is_some());
}

#[test]
fn task_get_alias_all_none() {
    let task = Task::new();

    let alias = task.get_alias();
    assert!(alias.is_none());
}

#[test]
fn task_get_alias_common_defined() {
    let mut task = Task::new();
    task.alias = Some("other".to_string());

    let alias = task.get_alias();
    assert_eq!(alias.unwrap(), "other");
}

#[test]
fn task_get_alias_platform_defined() {
    let mut task = Task::new();
    task.alias = Some("other".to_string());
    task.linux_alias = Some("linux".to_string());
    task.windows_alias = Some("windows".to_string());
    task.mac_alias = Some("mac".to_string());

    let alias = task.get_alias();
    if cfg!(windows) {
        assert_eq!(alias.unwrap(), "windows");
    } else if cfg!(target_os = "macos") || cfg!(target_os = "ios") {
        assert_eq!(alias.unwrap(), "mac");
    } else {
        assert_eq!(alias.unwrap(), "linux");
    };
}

#[test]
fn task_get_normalized_task_undefined() {
    let mut task = Task {
        clear: Some(false),
        alias: Some("alias".to_string()),
        linux_alias: Some("linux".to_string()),
        windows_alias: Some("windows".to_string()),
        mac_alias: Some("mac".to_string()),
        install_crate: Some(InstallCrate::Value("install_crate".to_string())),
        install_crate_args: None,
        command: Some("command".to_string()),
        disabled: Some(false),
        private: Some(true),
        condition: None,
        condition_script: None,
        force: None,
        env: None,
        cwd: None,
        install_script: Some(vec!["A".to_string(), "B".to_string(), "C".to_string()]),
        args: Some(vec!["1".to_string(), "2".to_string()]),
        script: Some(vec!["a".to_string(), "b".to_string()]),
        script_runner: Some("sh1".to_string()),
        script_extension: Some("ext1".to_string()),
        run_task: Some("task1".to_string()),
        dependencies: Some(vec!["1".to_string()]),
        toolchain: Some("toolchain2".to_string()),
        description: Some("description".to_string()),
        category: Some("category".to_string()),
        workspace: Some(false),
        linux: None,
        windows: None,
        mac: None,
    };

    let normalized_task = task.get_normalized_task();

    assert!(!normalized_task.clear.unwrap());
    assert!(normalized_task.install_crate.is_some());
    assert!(normalized_task.install_crate_args.is_none());
    assert!(normalized_task.command.is_some());
    assert!(normalized_task.disabled.is_some());
    assert!(normalized_task.private.is_some());
    assert!(normalized_task.condition.is_none());
    assert!(normalized_task.condition_script.is_none());
    assert!(normalized_task.force.is_none());
    assert!(normalized_task.env.is_none());
    assert!(normalized_task.cwd.is_none());
    assert!(normalized_task.alias.is_some());
    assert!(normalized_task.linux_alias.is_some());
    assert!(normalized_task.windows_alias.is_some());
    assert!(normalized_task.mac_alias.is_some());
    assert!(normalized_task.install_script.is_some());
    assert!(normalized_task.args.is_some());
    assert!(normalized_task.script.is_some());
    assert!(normalized_task.script_runner.is_some());
    assert!(normalized_task.script_extension.is_some());
    assert!(normalized_task.run_task.is_some());
    assert!(normalized_task.dependencies.is_some());
    assert!(normalized_task.toolchain.is_some());
    assert!(normalized_task.description.is_some());
    assert!(normalized_task.category.is_some());
    assert!(normalized_task.workspace.is_some());
    assert!(normalized_task.linux.is_none());
    assert!(normalized_task.windows.is_none());
    assert!(normalized_task.mac.is_none());

    assert_eq!(
        normalized_task.install_crate.unwrap(),
        InstallCrate::Value("install_crate".to_string())
    );
    assert_eq!(normalized_task.command.unwrap(), "command");
    assert_eq!(normalized_task.description.unwrap(), "description");
    assert_eq!(normalized_task.category.unwrap(), "category");
    assert!(!normalized_task.workspace.unwrap());
    assert!(!normalized_task.disabled.unwrap());
    assert!(normalized_task.private.unwrap());
    assert!(!normalized_task.force.unwrap_or(false));
    assert_eq!(normalized_task.alias.unwrap(), "alias");
    assert_eq!(normalized_task.linux_alias.unwrap(), "linux");
    assert_eq!(normalized_task.windows_alias.unwrap(), "windows");
    assert_eq!(normalized_task.mac_alias.unwrap(), "mac");
    assert_eq!(normalized_task.install_script.unwrap().len(), 3);
    assert_eq!(normalized_task.args.unwrap().len(), 2);
    assert_eq!(normalized_task.script.unwrap().len(), 2);
    assert_eq!(normalized_task.script_runner.unwrap(), "sh1");
    assert_eq!(normalized_task.script_extension.unwrap(), "ext1");
    assert_eq!(normalized_task.run_task.unwrap(), "task1");
    assert_eq!(normalized_task.dependencies.unwrap().len(), 1);
    assert_eq!(normalized_task.toolchain.unwrap(), "toolchain2");
}

#[test]
#[cfg(target_os = "linux")]
fn task_get_normalized_task_with_override_no_clear() {
    let mut env = IndexMap::new();
    env.insert("test".to_string(), EnvValue::Value("value".to_string()));

    let mut task = Task {
        clear: Some(true),
        alias: Some("bad".to_string()),
        linux_alias: Some("bad".to_string()),
        windows_alias: Some("bad".to_string()),
        mac_alias: Some("bad".to_string()),
        install_crate: Some(InstallCrate::Value("install_crate".to_string())),
        install_crate_args: Some(vec!["c1".to_string(), "c2".to_string()]),
        command: Some("command".to_string()),
        description: Some("description".to_string()),
        category: Some("category".to_string()),
        workspace: Some(true),
        disabled: Some(false),
        private: Some(true),
        condition: Some(TaskCondition {
            platforms: Some(vec!["linux".to_string(), "mac".to_string()]),
            channels: Some(vec!["nightly".to_string(), "stable".to_string()]),
            env_set: None,
            env_not_set: None,
            env: None,
            rust_version: None,
        }),
        condition_script: Some(vec!["exit 0".to_string()]),
        force: Some(false),
        env: Some(IndexMap::new()),
        cwd: Some("cwd".to_string()),
        install_script: Some(vec!["A".to_string(), "B".to_string(), "C".to_string()]),
        args: Some(vec!["1".to_string(), "2".to_string()]),
        script: Some(vec!["a".to_string(), "b".to_string()]),
        script_runner: Some("sh1".to_string()),
        script_extension: Some("ext1".to_string()),
        run_task: Some("task1".to_string()),
        dependencies: Some(vec!["1".to_string()]),
        toolchain: Some("toolchain1".to_string()),
        linux: Some(PlatformOverrideTask {
            clear: None,
            install_crate: Some(InstallCrate::Value("linux_crate".to_string())),
            install_crate_args: Some(vec!["c1".to_string(), "c2".to_string(), "c3".to_string()]),
            command: Some("linux_command".to_string()),
            disabled: Some(true),
            private: Some(false),
            condition: Some(TaskCondition {
                platforms: Some(vec!["linux".to_string(), "mac".to_string()]),
                channels: Some(vec!["nightly".to_string(), "stable".to_string()]),
                env_set: None,
                env_not_set: None,
                env: None,
                rust_version: None,
            }),
            condition_script: Some(vec!["exit 0".to_string()]),
            force: Some(true),
            env: Some(env),
            cwd: Some("cwd2".to_string()),
            install_script: Some(vec![
                "A".to_string(),
                "B".to_string(),
                "C".to_string(),
                "D".to_string(),
            ]),
            args: Some(vec!["1".to_string(), "2".to_string(), "3".to_string()]),
            script: Some(vec!["a".to_string(), "b".to_string(), "c".to_string()]),
            script_runner: Some("sh2".to_string()),
            script_extension: Some("ext2".to_string()),
            run_task: Some("task2".to_string()),
            dependencies: Some(vec!["1".to_string(), "2".to_string()]),
            toolchain: Some("toolchain2".to_string()),
        }),
        windows: None,
        mac: None,
    };

    let normalized_task = task.get_normalized_task();

    assert!(normalized_task.clear.unwrap());
    assert!(normalized_task.install_crate.is_some());
    assert!(normalized_task.install_crate_args.is_some());
    assert!(normalized_task.command.is_some());
    assert!(normalized_task.description.is_some());
    assert!(normalized_task.category.is_some());
    assert!(normalized_task.workspace.is_some());
    assert!(normalized_task.disabled.is_some());
    assert!(normalized_task.private.is_some());
    assert!(normalized_task.condition.is_some());
    assert!(normalized_task.condition_script.is_some());
    assert!(normalized_task.force.is_some());
    assert!(normalized_task.env.is_some());
    assert!(normalized_task.cwd.is_some());
    assert!(normalized_task.alias.is_none());
    assert!(normalized_task.linux_alias.is_none());
    assert!(normalized_task.windows_alias.is_none());
    assert!(normalized_task.mac_alias.is_none());
    assert!(normalized_task.install_script.is_some());
    assert!(normalized_task.args.is_some());
    assert!(normalized_task.script.is_some());
    assert!(normalized_task.script_runner.is_some());
    assert!(normalized_task.script_extension.is_some());
    assert!(normalized_task.run_task.is_some());
    assert!(normalized_task.dependencies.is_some());
    assert!(normalized_task.toolchain.is_some());
    assert!(normalized_task.linux.is_none());
    assert!(normalized_task.windows.is_none());
    assert!(normalized_task.mac.is_none());

    assert_eq!(
        normalized_task.install_crate.unwrap(),
        InstallCrate::Value("linux_crate".to_string())
    );
    assert_eq!(normalized_task.install_crate_args.unwrap().len(), 3);
    assert_eq!(normalized_task.command.unwrap(), "linux_command");
    assert_eq!(normalized_task.description.unwrap(), "description");
    assert_eq!(normalized_task.category.unwrap(), "category");
    assert!(normalized_task.workspace.unwrap());
    assert!(normalized_task.disabled.unwrap());
    assert!(!normalized_task.private.unwrap());
    assert_eq!(normalized_task.condition_script.unwrap().len(), 1);
    assert!(normalized_task.force.unwrap());
    assert_eq!(normalized_task.env.unwrap().len(), 1);
    assert_eq!(normalized_task.cwd.unwrap(), "cwd2".to_string());
    assert_eq!(normalized_task.install_script.unwrap().len(), 4);
    assert_eq!(normalized_task.args.unwrap().len(), 3);
    assert_eq!(normalized_task.script.unwrap().len(), 3);
    assert_eq!(normalized_task.script_runner.unwrap(), "sh2");
    assert_eq!(normalized_task.script_extension.unwrap(), "ext2");
    assert_eq!(normalized_task.run_task.unwrap(), "task2");
    assert_eq!(normalized_task.dependencies.unwrap().len(), 2);
    assert_eq!(normalized_task.toolchain.unwrap(), "toolchain2");

    let condition = normalized_task.condition.unwrap();
    assert_eq!(condition.platforms.unwrap().len(), 2);
    assert_eq!(condition.channels.unwrap().len(), 2);
}

#[test]
#[cfg(target_os = "linux")]
fn task_get_normalized_task_with_override_clear_false() {
    let mut env = IndexMap::new();
    env.insert("test".to_string(), EnvValue::Value("value".to_string()));

    let mut task = Task {
        clear: Some(true),
        alias: Some("bad".to_string()),
        linux_alias: Some("bad".to_string()),
        windows_alias: Some("bad".to_string()),
        mac_alias: Some("bad".to_string()),
        install_crate: Some(InstallCrate::Value("install_crate".to_string())),
        install_crate_args: Some(vec!["c1".to_string(), "c2".to_string()]),
        command: Some("command".to_string()),
        description: Some("description".to_string()),
        category: Some("category".to_string()),
        workspace: Some(true),
        disabled: Some(false),
        private: Some(true),
        condition: Some(TaskCondition {
            platforms: Some(vec!["linux".to_string(), "mac".to_string()]),
            channels: Some(vec!["nightly".to_string(), "stable".to_string()]),
            env_set: None,
            env_not_set: None,
            env: None,
            rust_version: None,
        }),
        condition_script: Some(vec!["exit 0".to_string()]),
        force: Some(false),
        env: Some(IndexMap::new()),
        cwd: Some("cwd".to_string()),
        install_script: Some(vec!["A".to_string(), "B".to_string(), "C".to_string()]),
        args: Some(vec!["1".to_string(), "2".to_string()]),
        script: Some(vec!["a".to_string(), "b".to_string()]),
        script_runner: Some("sh1".to_string()),
        script_extension: Some("ext1".to_string()),
        run_task: Some("task1".to_string()),
        dependencies: Some(vec!["1".to_string()]),
        toolchain: Some("toolchain1".to_string()),
        linux: Some(PlatformOverrideTask {
            clear: Some(false),
            install_crate: Some(InstallCrate::Value("linux_crate".to_string())),
            command: Some("linux_command".to_string()),
            disabled: Some(true),
            private: Some(false),
            condition: Some(TaskCondition {
                platforms: Some(vec!["linux".to_string()]),
                channels: Some(vec![
                    "nightly".to_string(),
                    "stable".to_string(),
                    "beta".to_string(),
                ]),
                env_set: None,
                env_not_set: None,
                env: None,
                rust_version: None,
            }),
            condition_script: Some(vec!["echo test".to_string(), "exit 1".to_string()]),
            force: Some(true),
            env: Some(env),
            cwd: Some("cwd2".to_string()),
            install_crate_args: Some(vec!["c1".to_string(), "c2".to_string(), "c3".to_string()]),
            install_script: Some(vec![
                "A".to_string(),
                "B".to_string(),
                "C".to_string(),
                "D".to_string(),
            ]),
            args: Some(vec!["1".to_string(), "2".to_string(), "3".to_string()]),
            script: Some(vec!["a".to_string(), "b".to_string(), "c".to_string()]),
            script_runner: Some("sh2".to_string()),
            script_extension: Some("ext2".to_string()),
            run_task: Some("task2".to_string()),
            dependencies: Some(vec!["1".to_string(), "2".to_string()]),
            toolchain: Some("toolchain2".to_string()),
        }),
        windows: None,
        mac: None,
    };

    let normalized_task = task.get_normalized_task();

    assert!(normalized_task.clear.unwrap());
    assert!(normalized_task.install_crate.is_some());
    assert!(normalized_task.command.is_some());
    assert!(normalized_task.description.is_some());
    assert!(normalized_task.category.is_some());
    assert!(normalized_task.workspace.is_some());
    assert!(normalized_task.disabled.is_some());
    assert!(normalized_task.private.is_some());
    assert!(normalized_task.condition.is_some());
    assert!(normalized_task.condition_script.is_some());
    assert!(normalized_task.force.is_some());
    assert!(normalized_task.env.is_some());
    assert!(normalized_task.cwd.is_some());
    assert!(normalized_task.alias.is_none());
    assert!(normalized_task.linux_alias.is_none());
    assert!(normalized_task.windows_alias.is_none());
    assert!(normalized_task.mac_alias.is_none());
    assert!(normalized_task.install_crate_args.is_some());
    assert!(normalized_task.install_script.is_some());
    assert!(normalized_task.args.is_some());
    assert!(normalized_task.script.is_some());
    assert!(normalized_task.script_runner.is_some());
    assert!(normalized_task.script_extension.is_some());
    assert!(normalized_task.run_task.is_some());
    assert!(normalized_task.dependencies.is_some());
    assert!(normalized_task.toolchain.is_some());
    assert!(normalized_task.linux.is_none());
    assert!(normalized_task.windows.is_none());
    assert!(normalized_task.mac.is_none());

    assert_eq!(
        normalized_task.install_crate.unwrap(),
        InstallCrate::Value("linux_crate".to_string())
    );
    assert_eq!(normalized_task.command.unwrap(), "linux_command");
    assert_eq!(normalized_task.description.unwrap(), "description");
    assert_eq!(normalized_task.category.unwrap(), "category");
    assert!(normalized_task.workspace.unwrap());
    assert!(normalized_task.disabled.unwrap());
    assert!(!normalized_task.private.unwrap());
    assert_eq!(normalized_task.condition_script.unwrap().len(), 2);
    assert!(normalized_task.force.unwrap());
    assert_eq!(normalized_task.env.unwrap().len(), 1);
    assert_eq!(normalized_task.cwd.unwrap(), "cwd2".to_string());
    assert_eq!(normalized_task.install_crate_args.unwrap().len(), 3);
    assert_eq!(normalized_task.install_script.unwrap().len(), 4);
    assert_eq!(normalized_task.args.unwrap().len(), 3);
    assert_eq!(normalized_task.script.unwrap().len(), 3);
    assert_eq!(normalized_task.script_runner.unwrap(), "sh2");
    assert_eq!(normalized_task.script_extension.unwrap(), "ext2");
    assert_eq!(normalized_task.run_task.unwrap(), "task2");
    assert_eq!(normalized_task.dependencies.unwrap().len(), 2);
    assert_eq!(normalized_task.toolchain.unwrap(), "toolchain2");

    let condition = normalized_task.condition.unwrap();
    assert_eq!(condition.platforms.unwrap().len(), 1);
    assert_eq!(condition.channels.unwrap().len(), 3);
}

#[test]
#[cfg(target_os = "linux")]
fn task_get_normalized_task_with_override_clear_false_partial_override() {
    let mut task = Task {
        clear: Some(true),
        alias: Some("bad".to_string()),
        linux_alias: Some("bad".to_string()),
        windows_alias: Some("bad".to_string()),
        mac_alias: Some("bad".to_string()),
        install_crate: Some(InstallCrate::Value("install_crate".to_string())),
        install_crate_args: Some(vec!["c1".to_string(), "c2".to_string()]),
        command: Some("command".to_string()),
        disabled: Some(false),
        private: Some(true),
        condition: Some(TaskCondition {
            platforms: Some(vec!["linux".to_string(), "mac".to_string()]),
            channels: Some(vec!["nightly".to_string(), "stable".to_string()]),
            env_set: None,
            env_not_set: None,
            env: None,
            rust_version: None,
        }),
        condition_script: Some(vec!["exit 0".to_string()]),
        force: Some(false),
        env: Some(IndexMap::new()),
        cwd: Some("cwd".to_string()),
        install_script: Some(vec!["A".to_string(), "B".to_string(), "C".to_string()]),
        args: Some(vec!["1".to_string(), "2".to_string()]),
        script: Some(vec!["a".to_string(), "b".to_string()]),
        script_runner: Some("sh1".to_string()),
        script_extension: Some("ext1".to_string()),
        run_task: Some("task1".to_string()),
        dependencies: Some(vec!["1".to_string()]),
        toolchain: Some("toolchain1".to_string()),
        description: None,
        category: None,
        workspace: None,
        linux: Some(PlatformOverrideTask {
            clear: Some(false),
            install_crate: None,
            install_crate_args: None,
            command: None,
            disabled: None,
            private: None,
            condition: None,
            condition_script: None,
            force: None,
            env: None,
            cwd: None,
            install_script: None,
            args: None,
            script: None,
            script_runner: None,
            script_extension: None,
            run_task: None,
            dependencies: None,
            toolchain: None,
        }),
        windows: None,
        mac: None,
    };

    let normalized_task = task.get_normalized_task();

    assert!(normalized_task.clear.unwrap());
    assert!(normalized_task.install_crate.is_some());
    assert!(normalized_task.install_crate_args.is_some());
    assert!(normalized_task.command.is_some());
    assert!(normalized_task.disabled.is_some());
    assert!(normalized_task.private.is_some());
    assert!(normalized_task.condition.is_some());
    assert!(normalized_task.condition_script.is_some());
    assert!(normalized_task.force.is_some());
    assert!(normalized_task.env.is_some());
    assert!(normalized_task.cwd.is_some());
    assert!(normalized_task.alias.is_none());
    assert!(normalized_task.linux_alias.is_none());
    assert!(normalized_task.windows_alias.is_none());
    assert!(normalized_task.mac_alias.is_none());
    assert!(normalized_task.install_script.is_some());
    assert!(normalized_task.args.is_some());
    assert!(normalized_task.script.is_some());
    assert!(normalized_task.script_runner.is_some());
    assert!(normalized_task.script_extension.is_some());
    assert!(normalized_task.run_task.is_some());
    assert!(normalized_task.dependencies.is_some());
    assert!(normalized_task.toolchain.is_some());
    assert!(normalized_task.description.is_none());
    assert!(normalized_task.category.is_none());
    assert!(normalized_task.workspace.is_none());
    assert!(normalized_task.linux.is_none());
    assert!(normalized_task.windows.is_none());
    assert!(normalized_task.mac.is_none());

    assert_eq!(
        normalized_task.install_crate.unwrap(),
        InstallCrate::Value("install_crate".to_string())
    );
    assert_eq!(normalized_task.command.unwrap(), "command");
    assert!(!normalized_task.disabled.unwrap());
    assert!(normalized_task.private.unwrap());
    assert!(!normalized_task.force.unwrap());
    assert_eq!(normalized_task.env.unwrap().len(), 0);
    assert_eq!(normalized_task.cwd.unwrap(), "cwd".to_string());
    assert_eq!(normalized_task.install_crate_args.unwrap().len(), 2);
    assert_eq!(normalized_task.install_script.unwrap().len(), 3);
    assert_eq!(normalized_task.args.unwrap().len(), 2);
    assert_eq!(normalized_task.script.unwrap().len(), 2);
    assert_eq!(normalized_task.script_runner.unwrap(), "sh1");
    assert_eq!(normalized_task.script_extension.unwrap(), "ext1");
    assert_eq!(normalized_task.run_task.unwrap(), "task1");
    assert_eq!(normalized_task.dependencies.unwrap().len(), 1);
    assert_eq!(normalized_task.toolchain.unwrap(), "toolchain1");
}

#[test]
#[cfg(target_os = "linux")]
fn task_get_normalized_task_with_override_clear_true() {
    let mut task = Task {
        clear: Some(true),
        alias: Some("bad".to_string()),
        linux_alias: Some("bad".to_string()),
        windows_alias: Some("bad".to_string()),
        mac_alias: Some("bad".to_string()),
        install_crate: Some(InstallCrate::Value("install_crate".to_string())),
        install_crate_args: Some(vec!["c1".to_string(), "c2".to_string()]),
        command: Some("command".to_string()),
        disabled: Some(false),
        private: Some(true),
        condition: Some(TaskCondition {
            platforms: Some(vec!["linux".to_string(), "mac".to_string()]),
            channels: Some(vec!["nightly".to_string(), "stable".to_string()]),
            env_set: None,
            env_not_set: None,
            env: None,
            rust_version: None,
        }),
        condition_script: Some(vec!["exit 0".to_string()]),
        force: Some(false),
        env: Some(IndexMap::new()),
        cwd: Some("cwd".to_string()),
        install_script: Some(vec!["A".to_string(), "B".to_string(), "C".to_string()]),
        args: Some(vec!["1".to_string(), "2".to_string()]),
        script: Some(vec!["a".to_string(), "b".to_string()]),
        script_runner: Some("sh1".to_string()),
        script_extension: Some("ext1".to_string()),
        run_task: Some("task1".to_string()),
        dependencies: Some(vec!["1".to_string()]),
        toolchain: Some("toolchain1".to_string()),
        description: Some("description".to_string()),
        category: Some("category".to_string()),
        workspace: Some(false),
        linux: Some(PlatformOverrideTask {
            clear: Some(true),
            install_crate: Some(InstallCrate::Value("linux_crate".to_string())),
            install_crate_args: None,
            command: None,
            disabled: None,
            private: None,
            condition: None,
            condition_script: None,
            force: None,
            env: None,
            cwd: None,
            install_script: None,
            args: None,
            script: None,
            script_runner: None,
            script_extension: None,
            run_task: None,
            dependencies: None,
            toolchain: None,
        }),
        windows: None,
        mac: None,
    };

    let normalized_task = task.get_normalized_task();

    assert!(normalized_task.clear.unwrap());
    assert!(normalized_task.install_crate.is_some());
    assert!(normalized_task.install_crate_args.is_none());
    assert!(normalized_task.command.is_none());
    assert!(normalized_task.disabled.is_none());
    assert!(normalized_task.private.is_none());
    assert!(normalized_task.condition.is_none());
    assert!(normalized_task.condition_script.is_none());
    assert!(normalized_task.force.is_none());
    assert!(normalized_task.env.is_none());
    assert!(normalized_task.cwd.is_none());
    assert!(normalized_task.alias.is_none());
    assert!(normalized_task.linux_alias.is_none());
    assert!(normalized_task.windows_alias.is_none());
    assert!(normalized_task.mac_alias.is_none());
    assert!(normalized_task.install_script.is_none());
    assert!(normalized_task.args.is_none());
    assert!(normalized_task.script.is_none());
    assert!(normalized_task.script_runner.is_none());
    assert!(normalized_task.script_extension.is_none());
    assert!(normalized_task.run_task.is_none());
    assert!(normalized_task.dependencies.is_none());
    assert!(normalized_task.toolchain.is_none());
    assert!(normalized_task.description.is_some());
    assert!(normalized_task.category.is_some());
    assert!(normalized_task.workspace.is_some());
    assert!(normalized_task.linux.is_none());
    assert!(normalized_task.windows.is_none());
    assert!(normalized_task.mac.is_none());

    assert_eq!(
        normalized_task.install_crate.unwrap(),
        InstallCrate::Value("linux_crate".to_string())
    );
    assert_eq!(normalized_task.description.unwrap(), "description");
    assert_eq!(normalized_task.category.unwrap(), "category");
}

#[test]
fn task_is_valid_all_none() {
    let task = Task::new();

    assert!(task.is_valid());
}

#[test]
fn task_is_valid_only_run_task() {
    let mut task = Task::new();
    task.run_task = Some("test".to_string());

    assert!(task.is_valid());
}

#[test]
fn task_is_valid_only_command() {
    let mut task = Task::new();
    task.command = Some("test".to_string());

    assert!(task.is_valid());
}

#[test]
fn task_is_valid_only_script() {
    let mut task = Task::new();
    task.script = Some(vec!["test".to_string()]);

    assert!(task.is_valid());
}

#[test]
fn task_is_valid_both_run_task_and_command() {
    let mut task = Task::new();
    task.run_task = Some("test".to_string());
    task.command = Some("test".to_string());

    assert!(!task.is_valid());
}

#[test]
fn task_is_valid_both_run_task_and_script() {
    let mut task = Task::new();
    task.run_task = Some("test".to_string());
    task.script = Some(vec!["test".to_string()]);

    assert!(!task.is_valid());
}

#[test]
fn task_is_valid_both_command_and_script() {
    let mut task = Task::new();
    task.command = Some("test".to_string());
    task.script = Some(vec!["test".to_string()]);

    assert!(!task.is_valid());
}

#[test]
fn config_section_new() {
    let config = ConfigSection::new();

    assert!(config.skip_core_tasks.is_none());
    assert!(config.init_task.is_none());
    assert!(config.end_task.is_none());
    assert!(config.on_error_task.is_none());
    assert!(config.load_script.is_none());
    assert!(config.linux_load_script.is_none());
    assert!(config.windows_load_script.is_none());
    assert!(config.mac_load_script.is_none());
}

#[test]
fn config_section_extend_all_values() {
    let mut base = ConfigSection::new();
    let mut extended = ConfigSection::new();

    base.skip_core_tasks = Some(true);
    base.init_task = Some("base_init".to_string());
    base.end_task = Some("base_end".to_string());
    base.on_error_task = Some("base_err".to_string());
    base.load_script = Some(vec!["base_info".to_string()]);
    base.linux_load_script = Some(vec!["linux".to_string(), "base_info".to_string()]);
    base.windows_load_script = Some(vec!["windows".to_string(), "base_info".to_string()]);
    base.mac_load_script = Some(vec!["mac".to_string(), "base_info".to_string()]);

    extended.skip_core_tasks = Some(false);
    extended.init_task = Some("extended_init".to_string());
    extended.end_task = Some("extended_end".to_string());
    extended.on_error_task = Some("extended_err".to_string());
    extended.load_script = Some(vec!["extended_info".to_string(), "arg2".to_string()]);
    extended.linux_load_script = Some(vec!["extended_info".to_string()]);
    extended.windows_load_script = Some(vec!["extended_info".to_string()]);
    extended.mac_load_script = Some(vec!["extended_info".to_string()]);

    base.extend(&mut extended);

    assert!(!base.skip_core_tasks.unwrap());
    assert_eq!(base.init_task.unwrap(), "extended_init".to_string());
    assert_eq!(base.end_task.unwrap(), "extended_end".to_string());
    assert_eq!(base.on_error_task.unwrap(), "extended_err".to_string());
    assert_eq!(base.load_script.unwrap().len(), 2);
    assert_eq!(base.linux_load_script.unwrap().len(), 1);
    assert_eq!(base.windows_load_script.unwrap().len(), 1);
    assert_eq!(base.mac_load_script.unwrap().len(), 1);
}

#[test]
fn config_section_extend_no_values() {
    let mut base = ConfigSection::new();
    let mut extended = ConfigSection::new();

    base.skip_core_tasks = Some(true);
    base.init_task = Some("base_init".to_string());
    base.end_task = Some("base_end".to_string());
    base.on_error_task = Some("base_err".to_string());
    base.load_script = Some(vec!["base_info".to_string(), "arg2".to_string()]);
    base.linux_load_script = Some(vec!["linux".to_string(), "base_info".to_string()]);
    base.windows_load_script = Some(vec!["windows".to_string(), "base_info".to_string()]);
    base.mac_load_script = Some(vec!["mac".to_string(), "base_info".to_string()]);

    base.extend(&mut extended);

    assert!(base.skip_core_tasks.unwrap());
    assert_eq!(base.init_task.unwrap(), "base_init".to_string());
    assert_eq!(base.end_task.unwrap(), "base_end".to_string());
    assert_eq!(base.on_error_task.unwrap(), "base_err".to_string());
    assert_eq!(base.load_script.unwrap().len(), 2);
    assert_eq!(base.linux_load_script.unwrap().len(), 2);
    assert_eq!(base.windows_load_script.unwrap().len(), 2);
    assert_eq!(base.mac_load_script.unwrap().len(), 2);
}

#[test]
fn config_section_extend_some_values() {
    let mut base = ConfigSection::new();
    let mut extended = ConfigSection::new();

    base.skip_core_tasks = Some(true);
    base.init_task = Some("base_init".to_string());
    base.end_task = Some("base_end".to_string());
    base.on_error_task = Some("base_err".to_string());
    base.load_script = Some(vec!["base_info".to_string(), "arg2".to_string()]);
    base.linux_load_script = Some(vec!["linux".to_string(), "base_info".to_string()]);
    base.windows_load_script = Some(vec!["windows".to_string(), "base_info".to_string()]);
    base.mac_load_script = Some(vec!["mac".to_string(), "base_info".to_string()]);

    extended.skip_core_tasks = Some(false);
    extended.init_task = Some("extended_init".to_string());

    base.extend(&mut extended);

    assert!(!base.skip_core_tasks.unwrap());
    assert_eq!(base.init_task.unwrap(), "extended_init".to_string());
    assert_eq!(base.end_task.unwrap(), "base_end".to_string());
    assert_eq!(base.on_error_task.unwrap(), "base_err".to_string());
    assert_eq!(base.load_script.unwrap().len(), 2);
    assert_eq!(base.linux_load_script.unwrap().len(), 2);
    assert_eq!(base.windows_load_script.unwrap().len(), 2);
    assert_eq!(base.mac_load_script.unwrap().len(), 2);
}

#[test]
fn config_section_get_get_load_script_all_none() {
    let config = ConfigSection::new();

    let load_script = config.get_load_script();
    assert!(load_script.is_none());
}

#[test]
fn config_section_get_get_load_script_platform_none() {
    let mut config = ConfigSection::new();
    config.load_script = Some(vec!["exit 0".to_string()]);

    let load_script = config.get_load_script();
    assert!(load_script.is_some());
}

#[test]
fn config_section_get_get_load_script_platform_some() {
    let mut config = ConfigSection::new();
    config.linux_load_script = Some(vec!["exit 0".to_string()]);
    config.windows_load_script = Some(vec!["exit 0".to_string()]);
    config.mac_load_script = Some(vec!["exit 0".to_string()]);

    let load_script = config.get_load_script();
    assert!(load_script.is_some());
}

#[test]
fn config_section_get_get_load_script_all_defined() {
    let mut config = ConfigSection::new();
    config.load_script = Some(vec!["base".to_string(), "0".to_string()]);
    config.linux_load_script = Some(vec!["linux".to_string()]);
    config.windows_load_script = Some(vec!["windows".to_string()]);
    config.mac_load_script = Some(vec!["mac".to_string()]);

    let load_script = config.get_load_script();
    assert!(load_script.is_some());

    let script = load_script.unwrap();
    assert_eq!(script.len(), 1);
    assert_eq!(script[0], get_platform_name());
}

#[test]
fn workspace_new() {
    let workspace = Workspace::new();

    assert!(workspace.members.is_none());
}

#[test]
fn git_info_new() {
    let git_info = GitInfo::new();

    assert!(git_info.branch.is_none());
    assert!(git_info.user_name.is_none());
    assert!(git_info.user_email.is_none());
}
