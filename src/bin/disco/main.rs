mod download;
mod helpers;
mod recs;

use download::*;
use recs::*;

use clap::{AppSettings, ColorChoice, Parser, Subcommand};
use std::path::PathBuf;
use std::process;

#[derive(Debug, Parser)]
#[clap(name = "disco", version, color = ColorChoice::Never)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    UserRecs {
        #[clap(parse(from_os_str))]
        input: PathBuf,

        #[clap(parse(from_os_str))]
        output: PathBuf,

        #[clap(long, default_value = "10")]
        count: usize,

        #[clap(long, default_value = "8")]
        factors: u32,

        #[clap(long, default_value = "20")]
        iterations: u32,

        #[clap(long)]
        overwrite: bool,
    },
    ItemRecs {
        #[clap(parse(from_os_str))]
        input: PathBuf,

        #[clap(parse(from_os_str))]
        output: PathBuf,

        #[clap(long, default_value = "10")]
        count: usize,

        #[clap(long, default_value = "8")]
        factors: u32,

        #[clap(long, default_value = "20")]
        iterations: u32,

        #[clap(long)]
        overwrite: bool,
    },
    SimilarUsers {
        #[clap(parse(from_os_str))]
        input: PathBuf,

        #[clap(parse(from_os_str))]
        output: PathBuf,

        #[clap(long, default_value = "10")]
        count: usize,

        #[clap(long, default_value = "8")]
        factors: u32,

        #[clap(long, default_value = "20")]
        iterations: u32,

        #[clap(long)]
        overwrite: bool,
    },
    Download {
        #[clap(possible_values(Dataset::variants()))]
        dataset: Dataset,

        #[clap(parse(from_os_str))]
        output: Option<PathBuf>,

        #[clap(long)]
        overwrite: bool,
    },
}

fn main() {
    let args = Args::parse();

    let res = match args.command {
        Commands::UserRecs {
            input,
            output,
            count,
            factors,
            iterations,
            overwrite,
        } => user_recs(&input, &output, count, factors, iterations, overwrite),
        Commands::ItemRecs {
            input,
            output,
            count,
            factors,
            iterations,
            overwrite,
        } => item_recs(&input, &output, count, factors, iterations, overwrite),
        Commands::SimilarUsers {
            input,
            output,
            count,
            factors,
            iterations,
            overwrite,
        } => similar_users(&input, &output, count, factors, iterations, overwrite),
        Commands::Download {
            dataset,
            output,
            overwrite,
        } => download(dataset, output, overwrite),
    };

    if let Err(err) = res {
        eprintln!("{}", err);
        process::exit(1);
    }
}
