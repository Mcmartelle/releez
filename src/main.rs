extern crate checkboxer;
use clap::{App, Arg};
use colored::*;
use checkboxer::constants;
use checkboxer::out;
use tokio::runtime::Builder;

async fn run() {
    let matches = App::new(constants::APP_NAME)
        .version(constants::APP_VERSION)
        .version_short("v")
        .author(constants::APP_AUTHOR)
        .about("An utility tool to run through checklists.")
        .arg(
            Arg::with_name("config")
                .help(format!("The {} config file path", constants::CONFIG_FILE_NAME).as_str())
                .short("c")
                .long("config")
                .value_name("CONFIG_PATH")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("releaseVersion")
                .help("The next release version")
                .value_name("RELEASE_VERSION")
                .index(1)
                .required(false),
        )
        .get_matches();

    let config_file_path = matches
        .value_of("config")
        .unwrap_or(constants::DEFAULT_CONFIG_FILE_PATH);
    let release_version = matches
        .value_of("releaseVersion")
        .unwrap_or(constants::DEFAULT_RELEASE_VERSION);

    if let Err(err) = checkboxer::runner::run_release_checklist(config_file_path, release_version).await {
        out::print_err(format!("\n{} {}\n", "error:".red(), err)).await.unwrap();
    }
}

fn main() {
    let mut runtime = Builder::new()
        .basic_scheduler()
        .enable_all()
        .build()
        .expect("Couldn't create Tokio runtime");

    runtime.block_on(run());
}
