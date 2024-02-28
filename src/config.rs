#![allow(dead_code)]
use std::fs;

use serde::Deserialize;

type Path = String;
const PROJECT_NAME_REPLACEMENT: &str = "$PROJECT_NAME";

#[derive(Debug, Deserialize)]
pub struct TestDrivenConfig {
    pub language: Language,
    pub project: Project,
    pub code: Code,
    pub config: Vec<FileSpec>,
}

#[derive(Debug, Deserialize)]
pub struct Language {
    pub binary: String,
    pub version: String,
    pub name: String,
    pub file_extension: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    pub dependencies: Vec<String>,
    pub dev_dependencies: Vec<String>,
    pub tool: ProjectTool,
}

#[derive(Debug, Deserialize)]
pub struct ProjectTool {
    pub binary: String,
    pub initializes_in_project_directory: bool,
    pub commands: ProjectToolCommands,
}

#[derive(Debug, Deserialize)]
pub struct ProjectToolCommands {
    pub initialize: Vec<String>,
    pub add_development_dependency: Vec<String>,
    pub add_dependency: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Code {
    pub directories: CodeDirectories,
    pub source: Vec<FileSpec>,
    pub test: Vec<FileSpec>,
}

#[derive(Debug, Deserialize)]
pub struct CodeDirectories {
    pub source: Path,
    pub test: Path,
}

#[derive(Debug, Deserialize)]
pub struct FileSpec {
    pub contents: String,
    pub file: Path,
    pub variant: Option<String>,
}

pub fn load_configuration(project_name: &str, language: &str) -> TestDrivenConfig {
    let path = format!("./templates/{language}.toml");
    let toml_file = fs::read_to_string(path).expect("this to work").replace(PROJECT_NAME_REPLACEMENT, &project_name);
    toml::from_str(&toml_file).expect("bad toml")
}
