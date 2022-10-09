mod download;
mod helpers;
mod recs;

use download::*;
use recs::*;

use clap::{ColorChoice, Parser, Subcommand};
use clap::builder::{PossibleValuesParser, TypedValueParser};
use std::path::PathBuf;
use std::process;

#[derive(Debug, Parser)]
#[command(name = "disco", version, color = ColorChoice::Never)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    UserRecs {
        #[arg(value_parser)]
        input: PathBuf,

        #[arg(value_parser)]
        output: PathBuf,

        #[arg(long, default_value_t = 10)]
        count: usize,

        #[arg(long, default_value_t = 8)]
        factors: u32,

        #[arg(long, default_value_t = 20)]
        iterations: u32,

        #[arg(long)]
        overwrite: bool,
    },
    ItemRecs {
        #[arg(value_parser)]
        input: PathBuf,

        #[arg(value_parser)]
        output: PathBuf,

        #[arg(long, default_value_t = 10)]
        count: usize,

        #[arg(long, default_value_t = 8)]
        factors: u32,

        #[arg(long, default_value_t = 20)]
        iterations: u32,

        #[arg(long)]
        overwrite: bool,
    },
    SimilarUsers {
        #[arg(value_parser)]
        input: PathBuf,

        #[arg(value_parser)]
        output: PathBuf,

        #[arg(long, default_value_t = 10)]
        count: usize,

        #[arg(long, default_value_t = 8)]
        factors: u32,

        #[arg(long, default_value_t = 20)]
        iterations: u32,

        #[arg(long)]
        overwrite: bool,
    },
    Download {
        #[arg(value_parser = PossibleValuesParser::new(Dataset::variants()).map(|s| s.parse::<Dataset>().unwrap()))]
        dataset: Dataset,

        #[arg(value_parser)]
        output: Option<PathBuf>,

        #[arg(long)]
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
