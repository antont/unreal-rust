mod generator;
mod parse_api;

use std::{error::Error, path::PathBuf};

use clap::{Parser, Subcommand};

use crate::parse_api::Api;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    // /// Optional name to operate on
    // name: Option<String>,
    //
    // /// Sets a custom config file
    // #[arg(short, long, value_name = "FILE")]
    // config: Option<PathBuf>,
    //
    // /// Turn debugging information on
    // #[arg(short, long, action = clap::ArgAction::Count)]
    // debug: u8,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Generate {
        /// lists test values
        #[arg(short, long)]
        path: PathBuf,
        #[arg(short, long)]
        out_dir: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // // You can check the value provided by positional arguments, or option arguments
    // if let Some(name) = cli.name.as_deref() {
    //     println!("Value for name: {name}");
    // }
    //
    // if let Some(config_path) = cli.config.as_deref() {
    //     println!("Value for config: {}", config_path.display());
    // }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    // match cli.debug {
    //     0 => println!("Debug mode is off"),
    //     1 => println!("Debug mode is kind of on"),
    //     2 => println!("Debug mode is on"),
    //     _ => println!("Don't be crazy"),
    // }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Generate { path, out_dir }) => {
            let raw_json = std::fs::read_to_string(path)?;
            // TODO: From some reason unreal has this in the json and serde is unhappy
            let content = raw_json.trim_start_matches('\u{feff}');
            let api: Api = serde_json::de::from_str::<Api>(content)?;
            generator::generate_crate(&api, out_dir)?
        }
        None => {}
    }

    Ok(())

}
