use std::{fs::read_to_string, path::PathBuf};

use log::{error, info, trace};

use crate::constants::CONFIG_FILE_NAME;

const PYTHON_TEMPLATE_KEY: &str = "python";
const TYPESCRIPT_TEMPLATE_KEY: &str = "typescript";
const FSHARP_TEMPLATE_KEY: &str = "fsharp";
const GO_TEMPLATE_KEY: &str = "go";

const PYTHON_TEMPLATE_CONTENTS: &str = include_str!("../templates/python.toml");
const TYPESCRIPT_TEMPLATE_CONTENTS: &str = include_str!("../templates/typescript.toml");
const FSHARP_TEMPLATE_CONTENTS: &str = include_str!("../templates/fsharp.toml");
const GO_TEMPLATE_CONTENTS: &str = include_str!("../templates/go.toml");

const DEFAULT_FILE_CONTENTS: [(&str, &str); 5] = [
    (PYTHON_TEMPLATE_KEY, PYTHON_TEMPLATE_CONTENTS),
    (TYPESCRIPT_TEMPLATE_KEY, TYPESCRIPT_TEMPLATE_CONTENTS),
    (GO_TEMPLATE_KEY, GO_TEMPLATE_CONTENTS),
    (FSHARP_TEMPLATE_KEY, FSHARP_TEMPLATE_CONTENTS),
    (CONFIG_FILE_NAME, DEFAULT_CONFIG_CONTENTS)
];

const DEFAULT_CONFIG_CONTENTS: &str = include_str!("../default_config.toml");
const CONFIG_DIR: &str = ".config";

const APP_DIR: &str = "pbj";
const TEMPLATE_DIR: &str = "templates";

fn get_template_path(template: &str) -> PathBuf {
    let template = format!("{}.toml", template);
    let template = vec![TEMPLATE_DIR.to_string(), template];
    PathBuf::from_iter(template.iter())
}

pub fn get_default_file_contents(key: &str) -> Option<&str> {
    return DEFAULT_FILE_CONTENTS
        .iter()
        .find(|&&(cand, _)| cand == key)
        .map(|(_, content)| *content);
}

pub fn read_file(file: &PathBuf) -> Option<String> {
    let dot_config_path = dirs::home_dir()?.join(CONFIG_DIR).join(APP_DIR).join(&file);

    let local_os_config_path = dirs::config_local_dir()?.join(APP_DIR).join(&file);

    let os_config_path = dirs::config_dir()?.join(APP_DIR).join(&file);

    if let Some(contents) = read_to_string(&dot_config_path).ok() {
        print_loading(&dot_config_path);
        return Some(contents);
    }

    if let Some(contents) = read_to_string(&local_os_config_path).ok() {
        print_loading(&local_os_config_path);
        return Some(contents);
    }

    if let Some(contents) = read_to_string(&os_config_path).ok() {
        print_loading(&os_config_path);
        return Some(contents);
    }

    None
}

fn print_loading(path: &PathBuf) {
    info!("loading file from path {:?}", path);
}

pub fn get_template(template_name: &str) -> Option<String> {
    let template = get_template_path(template_name);
    if let Some(contents) = read_file(&template) {
        return Some(contents);
    } else {
        trace!(
            "template not found at path: \"{:?}\" looking for default contents...",
            template
        )
    }
    if let Some(contents) = get_default_file_contents(&template_name) {
        return Some(contents.to_string());
    } else {
        error!("default contents not found for template {}.", template_name)
    }
    None
}
