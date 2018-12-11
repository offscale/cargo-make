//! # env
//!
//! Sets up the env vars before running the tasks.
//!

pub(crate) mod crateinfo;
mod gitinfo;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

use crate::command;
use crate::types::{
    CliArgs, Config, CrateInfo, EnvInfo, EnvValue, EnvValueInfo, GitInfo, PackageInfo, Step, Task,
    Workspace,
};
use indexmap::IndexMap;
use rust_info;
use rust_info::types::{RustChannel, RustInfo};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

fn evaluate_env_value(env_value: &EnvValueInfo) -> String {
    match command::run_script_get_output(&env_value.script, None, &vec![], true) {
        Ok(output) => {
            let exit_code = output.0;
            let stdout = output.1;

            command::validate_exit_code(exit_code);

            if exit_code == 0 {
                let mut lines: Vec<&str> = stdout.split("\n").collect();
                lines.retain(|&line| line.len() > 0);

                if lines.len() > 0 {
                    let line = lines[lines.len() - 1].to_string();

                    let line_str = str::replace(&line, "\r", "");

                    line_str.to_string()
                } else {
                    "".to_string()
                }
            } else {
                "".to_string()
            }
        }
        _ => "".to_string(),
    }
}

fn expand_value(value: &str) -> String {
    let mut value_string = value.to_string();
    match value_string.find("${") {
        Some(_) => {
            for (existing_key, existing_value) in env::vars() {
                let mut key_pattern = "${".to_string();
                key_pattern.push_str(&existing_key);
                key_pattern.push_str("}");

                value_string = str::replace(&value_string, &key_pattern, &existing_value);
            }

            value_string
        }
        None => value_string,
    }
}

fn evaluate_and_set_env(key: &str, value: &str) {
    let env_value = expand_value(&value);

    env::set_var(&key, &env_value);
}

fn set_env_for_info(key: &str, env_value: &EnvValueInfo) {
    let value = evaluate_env_value(&env_value);

    evaluate_and_set_env(&key, &value);
}

/// Updates the env based on the provided data
pub(crate) fn set_env(env: IndexMap<String, EnvValue>) {
    debug!("Setting Up Env.");

    for (key, env_value) in &env {
        debug!("Setting env: {} = {:#?}", &key, &env_value);

        match *env_value {
            EnvValue::Value(ref value) => evaluate_and_set_env(&key, value),
            EnvValue::Info(ref info) => set_env_for_info(&key, info),
        };
    }
}

/// Updates the env for the current execution based on the descriptor.
fn initialize_env(config: &Config) {
    info!("Setting Up Env.");

    set_env(config.env.clone());
}

fn setup_env_for_crate() -> CrateInfo {
    let crate_info = crateinfo::load();
    let crate_info_clone = crate_info.clone();

    let package_info = crate_info.package.unwrap_or(PackageInfo::new());

    if package_info.name.is_some() {
        let crate_name = package_info.name.unwrap();
        env::set_var("CARGO_MAKE_CRATE_NAME", &crate_name);

        let crate_fs_name = str::replace(&crate_name, "-", "_");
        env::set_var("CARGO_MAKE_CRATE_FS_NAME", &crate_fs_name);
    }

    if package_info.version.is_some() {
        env::set_var("CARGO_MAKE_CRATE_VERSION", &package_info.version.unwrap());
    }

    if package_info.description.is_some() {
        env::set_var(
            "CARGO_MAKE_CRATE_DESCRIPTION",
            &package_info.description.unwrap(),
        );
    }

    if package_info.license.is_some() {
        env::set_var("CARGO_MAKE_CRATE_LICENSE", &package_info.license.unwrap());
    }

    if package_info.documentation.is_some() {
        env::set_var(
            "CARGO_MAKE_CRATE_DOCUMENTATION",
            &package_info.documentation.unwrap(),
        );
    }

    if package_info.homepage.is_some() {
        env::set_var("CARGO_MAKE_CRATE_HOMEPAGE", &package_info.homepage.unwrap());
    }

    if package_info.repository.is_some() {
        env::set_var(
            "CARGO_MAKE_CRATE_REPOSITORY",
            &package_info.repository.unwrap(),
        );
    }

    let has_dependencies = match crate_info.dependencies {
        Some(ref dependencies) => dependencies.len() > 0,
        None => crate_info.workspace.is_some(),
    };

    let has_dependencies_var_value = if has_dependencies { "TRUE" } else { "FALSE" };
    env::set_var(
        "CARGO_MAKE_CRATE_HAS_DEPENDENCIES",
        has_dependencies_var_value,
    );

    let is_workspace_var_value = if crate_info.workspace.is_none() {
        "FALSE"
    } else {
        "TRUE"
    };
    env::set_var("CARGO_MAKE_CRATE_IS_WORKSPACE", is_workspace_var_value);

    let workspace = crate_info.workspace.unwrap_or(Workspace::new());
    let members = workspace.members.unwrap_or(vec![]);
    let members_string = members.join(",");
    env::set_var("CARGO_MAKE_CRATE_WORKSPACE_MEMBERS", &members_string);

    // check if Cargo.lock file exists in working directory
    let lock_file = Path::new("Cargo.lock");
    let lock_file_exists_var_value = if lock_file.exists() { "TRUE" } else { "FALSE" };
    env::set_var(
        "CARGO_MAKE_CRATE_LOCK_FILE_EXISTS",
        lock_file_exists_var_value,
    );

    crate_info_clone
}

fn setup_env_for_git_repo() -> GitInfo {
    let git_info = gitinfo::load();
    let git_info_clone = git_info.clone();

    if git_info.branch.is_some() {
        env::set_var("CARGO_MAKE_GIT_BRANCH", &git_info.branch.unwrap());
    }

    if git_info.user_name.is_some() {
        env::set_var("CARGO_MAKE_GIT_USER_NAME", &git_info.user_name.unwrap());
    }

    if git_info.user_email.is_some() {
        env::set_var("CARGO_MAKE_GIT_USER_EMAIL", &git_info.user_email.unwrap());
    }

    git_info_clone
}

fn setup_env_for_rust() -> RustInfo {
    let rustinfo = rust_info::get();
    let rust_info_clone = rustinfo.clone();

    if rustinfo.version.is_some() {
        env::set_var("CARGO_MAKE_RUST_VERSION", &rustinfo.version.unwrap());
    }

    if rustinfo.channel.is_some() {
        let channel_option = rustinfo.channel.unwrap();

        let channel = match channel_option {
            RustChannel::Stable => "stable",
            RustChannel::Beta => "beta",
            RustChannel::Nightly => "nightly",
        };

        env::set_var("CARGO_MAKE_RUST_CHANNEL", channel.to_string());
    }

    env::set_var(
        "CARGO_MAKE_RUST_TARGET_ARCH",
        &rustinfo.target_arch.unwrap_or("unknown".to_string()),
    );
    env::set_var(
        "CARGO_MAKE_RUST_TARGET_ENV",
        &rustinfo.target_env.unwrap_or("unknown".to_string()),
    );
    env::set_var(
        "CARGO_MAKE_RUST_TARGET_OS",
        &rustinfo.target_os.unwrap_or("unknown".to_string()),
    );
    env::set_var(
        "CARGO_MAKE_RUST_TARGET_POINTER_WIDTH",
        &rustinfo
            .target_pointer_width
            .unwrap_or("unknown".to_string()),
    );
    env::set_var(
        "CARGO_MAKE_RUST_TARGET_VENDOR",
        &rustinfo.target_vendor.unwrap_or("unknown".to_string()),
    );

    rust_info_clone
}

/// Sets up the env before the tasks execution.
pub(crate) fn setup_env(cli_args: &CliArgs, config: &Config, task: &str) -> EnvInfo {
    env::set_var("CARGO_MAKE", "true");
    env::set_var("CARGO_MAKE_TASK", &task);

    env::set_var("CARGO_MAKE_COMMAND", &cli_args.command);

    let task_arguments = match cli_args.arguments {
        Some(ref args) => {
            if args.len() == 0 {
                "".to_string()
            } else {
                args.join(";")
            }
        }
        None => "".to_string(),
    };
    env::set_var("CARGO_MAKE_TASK_ARGS", &task_arguments);

    // load crate info
    let crate_info = setup_env_for_crate();

    // load git info
    let git_info = setup_env_for_git_repo();

    // load rust info
    let rust_info = setup_env_for_rust();

    // load env vars
    initialize_env(config);

    EnvInfo {
        rust_info,
        crate_info,
        git_info,
    }
}

fn remove_unc_prefix(directory_path_buf: &PathBuf) -> PathBuf {
    let mut path_str = directory_path_buf.to_str().unwrap_or(".");

    let prefix = r"\\?\";
    if path_str.starts_with(prefix) {
        path_str = &path_str[prefix.len()..];
        PathBuf::from(path_str)
    } else {
        directory_path_buf.clone()
    }
}

pub(crate) fn setup_cwd(cwd: Option<&str>) {
    let directory = cwd.unwrap_or(".");

    debug!("Changing working directory to: {}", &directory);

    let mut directory_path_buf = PathBuf::from(&directory);
    directory_path_buf = directory_path_buf
        .canonicalize()
        .unwrap_or(directory_path_buf);

    // remove UNC path for windows
    if cfg!(windows) {
        directory_path_buf = remove_unc_prefix(&directory_path_buf);
    }

    let directory_path = directory_path_buf.as_path();

    match env::set_current_dir(&directory_path) {
        Err(error) => error!(
            "Unable to set current working directory to: {} {:#?}",
            &directory, error
        ),
        _ => {
            env::set_var("CARGO_MAKE_WORKING_DIRECTORY", directory_path);

            debug!("Working directory changed to: {}", &directory);
        }
    }
}

pub(crate) fn get_env(key: &str, default_value: &str) -> String {
    match env::var(key) {
        Ok(value) => value.to_string(),
        _ => default_value.to_string(),
    }
}

pub(crate) fn get_env_as_bool(key: &str, default_value: bool) -> bool {
    let default_str = if default_value { "true" } else { "false" };

    let mut value = get_env(key, default_str);
    value = value.to_lowercase();

    if value == "true" || value == "yes" || value == "1" {
        true
    } else {
        false
    }
}

pub(crate) fn parse_env_file(env_file: Option<String>) -> Option<Vec<String>> {
    match env_file {
        Some(file_name) => {
            let file_path = if file_name.starts_with(".") {
                let base_path = get_env("CARGO_MAKE_WORKING_DIRECTORY", ".");
                Path::new(&base_path).join(file_name)
            } else {
                Path::new(&file_name).to_path_buf()
            };

            if file_path.exists() {
                debug!("Opening env file: {:#?}", &file_path);
                let mut file = match File::open(&file_path) {
                    Ok(value) => value,
                    Err(error) => panic!(
                        "Unable to open env file: {} error: {}",
                        file_path.to_str().unwrap_or(""),
                        error
                    ),
                };

                let mut env_content = String::new();
                file.read_to_string(&mut env_content).unwrap();

                let mut env: Vec<String> = vec![];

                let lines: Vec<&str> = env_content.split('\n').collect();

                for mut line in lines {
                    line = line.trim();

                    if !line.starts_with("#") {
                        env.push(line.to_string());
                    }
                }

                Some(env)
            } else {
                None
            }
        }
        None => None,
    }
}

fn get_project_root_for_path(directory: &PathBuf) -> Option<String> {
    let file_path = Path::new(directory).join("Cargo.toml");

    if file_path.exists() {
        match directory.to_str() {
            Some(directory_string) => Some(directory_string.to_string()),
            _ => None,
        }
    } else {
        match directory.parent() {
            Some(parent_directory) => {
                let parent_directory_path = parent_directory.to_path_buf();
                get_project_root_for_path(&parent_directory_path)
            }
            None => None,
        }
    }
}

pub(crate) fn get_project_root() -> Option<String> {
    match env::current_dir() {
        Ok(directory) => get_project_root_for_path(&directory),
        _ => None,
    }
}

fn expand_env_for_arguments(task: &mut Task) {
    //update args by replacing any env vars
    let updated_args = match task.args {
        Some(ref args) => {
            let mut expanded_args = vec![];

            let task_args_str = get_env("CARGO_MAKE_TASK_ARGS", "").to_string();
            let task_args: Vec<String> = if task_args_str.len() == 0 {
                vec![]
            } else {
                let args_str: Vec<&str> = task_args_str.split(";").collect();
                args_str.iter().map(|item| item.to_string()).collect()
            };

            for index in 0..args.len() {
                if args[index].contains("${@}") {
                    if task_args_str.len() > 0 {
                        if args[index] == "${@}" {
                            for arg_index in 0..task_args.len() {
                                expanded_args.push(task_args[arg_index].clone());
                            }
                        } else {
                            for arg_index in 0..task_args.len() {
                                let value_string =
                                    str::replace(&args[index], "${@}", &task_args[arg_index]);
                                expanded_args.push(value_string);
                            }
                        }
                    }
                } else {
                    expanded_args.push(args[index].clone());
                }
            }

            for index in 0..expanded_args.len() {
                expanded_args[index] = expand_value(&expanded_args[index]);
            }

            Some(expanded_args)
        }
        None => None,
    };

    task.args = updated_args;
}

pub(crate) fn expand_env(step: &Step) -> Step {
    //clone data before modify
    let mut config = step.config.clone();

    //update command by replacing any env vars
    match config.command {
        Some(value) => {
            config.command = Some(expand_value(&value));
        }
        None => {}
    };

    //update args by replacing any env vars
    expand_env_for_arguments(&mut config);

    Step {
        name: step.name.clone(),
        config,
    }
}
