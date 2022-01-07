pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
pub const APP_AUTHOR: &'static str = "Rousan Ali <hello@rousan.io>, Matthew Martelle <mcmartelle@gmail.com>";

pub const CONFIG_FILE_NAME: &'static str = "checklist.yml";
pub const DEFAULT_CONFIG_FILE_PATH: &'static str = "./checklist.yml";
pub const DEFAULT_RELEASE_VERSION: &'static str = "0.1.0";
pub const HALT_CONFIG_FILE_PREFIX: &'static str = ".halt.";

pub const VAR_NAME_VERSION: &'static str = "VERSION";

pub const BOOL_POSSIBLE_TRUTHY_INPUTS: [&'static str; 2] = ["yes", "y"];
pub const BOOL_POSSIBLE_FALSY_INPUTS: [&'static str; 2] = ["no", "n"];

pub const MANUAL_TASK_QUIT_COMMAND: &'static str = "q";
