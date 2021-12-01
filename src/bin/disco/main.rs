mod download;
mod helpers;
mod recs;

use download::*;
use recs::*;

use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "disco")]
enum Opt {
    UserRecs {
        #[structopt(parse(from_os_str))]
        input: PathBuf,

        #[structopt(parse(from_os_str))]
        output: PathBuf,

        #[structopt(long, default_value = "10")]
        count: usize,

        #[structopt(long, default_value = "8")]
        factors: u32,

        #[structopt(long, default_value = "20")]
        iterations: u32,

        #[structopt(long)]
        overwrite: bool,
    },
    ItemRecs {
        #[structopt(parse(from_os_str))]
        input: PathBuf,

        #[structopt(parse(from_os_str))]
        output: PathBuf,

        #[structopt(long, default_value = "10")]
        count: usize,

        #[structopt(long, default_value = "8")]
        factors: u32,

        #[structopt(long, default_value = "20")]
        iterations: u32,

        #[structopt(long)]
        overwrite: bool,
    },
    SimilarUsers {
        #[structopt(parse(from_os_str))]
        input: PathBuf,

        #[structopt(parse(from_os_str))]
        output: PathBuf,

        #[structopt(long, default_value = "10")]
        count: usize,

        #[structopt(long, default_value = "8")]
        factors: u32,

        #[structopt(long, default_value = "20")]
        iterations: u32,

        #[structopt(long)]
        overwrite: bool,
    },
    Download {
        #[structopt(possible_values(&Dataset::variants()))]
        dataset: Dataset,

        #[structopt(parse(from_os_str))]
        output: Option<PathBuf>,

        #[structopt(long)]
        overwrite: bool,
    },
}

fn main() {
    let opt = Opt::from_args();

    let res = match opt {
        Opt::UserRecs {
            input,
            output,
            count,
            factors,
            iterations,
            overwrite,
        } => user_recs(&input, &output, count, factors, iterations, overwrite),
        Opt::ItemRecs {
            input,
            output,
            count,
            factors,
            iterations,
            overwrite,
        } => item_recs(&input, &output, count, factors, iterations, overwrite),
        Opt::SimilarUsers {
            input,
            output,
            count,
            factors,
            iterations,
            overwrite,
        } => similar_users(&input, &output, count, factors, iterations, overwrite),
        Opt::Download {
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
