use crate::helpers::*;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::convert::TryInto;
use std::error::Error;
use std::fmt::Write;
use std::io::{self, BufRead, BufReader, Cursor, Read};
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Debug)]
pub enum Dataset {
    Movielens100k,
    Movielens1m,
    Movielens25m,
}

impl Dataset {
    pub fn variants() -> [&'static str; 3] {
        ["movielens-100k", "movielens-1m", "movielens-25m"]
    }
}

impl FromStr for Dataset {
    type Err = String;

    fn from_str(s: &str) -> Result<Dataset, Self::Err> {
        match s {
            "movielens-100k" => Ok(Dataset::Movielens100k),
            "movielens-1m" => Ok(Dataset::Movielens1m),
            "movielens-25m" => Ok(Dataset::Movielens25m),
            // not shown since possible_values used
            _ => Err(format!("Invalid dataset: {}", s)),
        }
    }
}

impl ToString for Dataset {
    fn to_string(&self) -> String {
        match self {
            Dataset::Movielens100k => "movielens-100k".to_string(),
            Dataset::Movielens1m => "movielens-1m".to_string(),
            Dataset::Movielens25m => "movielens-25m".to_string(),
        }
    }
}

fn sha256(contents: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(contents);
    let result = hasher.finalize();

    let mut s = String::with_capacity(result.len() * 2);
    for b in result {
        write!(&mut s, "{:02x}", b).unwrap();
    }

    s
}

fn download_file(url: &str, expected_hash: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut response = reqwest::blocking::get(url)?.error_for_status()?;
    let content_length = response.content_length().unwrap();
    let mut contents = Vec::with_capacity(content_length.try_into()?);

    let bar = progress_bar(content_length, "Downloading", "{msg} {wide_bar} {percent}%");

    io::copy(&mut response, &mut bar.wrap_write(&mut contents))?;

    bar.finish();

    let hash = sha256(&contents);
    if hash != expected_hash {
        return Err(format!("Bad hash: {}", hash).into());
    }
    Ok(contents)
}

fn download_movielens_100k(output: &Path, overwrite: bool) -> Result<(), Box<dyn Error>> {
    let mut movies = HashMap::new();

    let archive_data = download_file(
        "https://files.grouplens.org/datasets/movielens/ml-100k.zip",
        "50d2a982c66986937beb9ffb3aa76efe955bf3d5c6b761f4e3a7cd717c6a3229",
    )?;
    let cursor = Cursor::new(archive_data);
    let mut archive = zip::ZipArchive::new(cursor)?;

    // make borrow checker happy
    {
        let mut movies_data = archive.by_name("ml-100k/u.item")?;

        // remove invalid UTF-8 bytes
        let mut buf = Vec::new();
        movies_data.read_to_end(&mut buf)?;
        let movies_data = String::from_utf8_lossy(&buf);

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'|')
            .from_reader(movies_data.as_bytes());
        for result in rdr.records() {
            let record = result?;
            let id = record.get(0).unwrap().to_string();
            let title = record.get(1).unwrap().to_string();
            movies.insert(id, title);
        }
    }

    let file = create_file(output, overwrite)?;
    let mut wtr = csv::Writer::from_writer(file);
    wtr.write_record(&["user_id", "item_id", "rating"])?;

    let ratings_data = archive.by_name("ml-100k/u.data")?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_reader(ratings_data);
    for result in rdr.records() {
        let record = result?;
        let user_id = record.get(0).unwrap().to_string();
        let item_id = record.get(1).unwrap().to_string();
        let rating = record.get(2).unwrap().to_string();
        wtr.write_record(&[user_id, movies.get(&item_id).unwrap().to_string(), rating])?;
    }

    wtr.flush()?;

    Ok(())
}

fn download_movielens_1m(output: &Path, overwrite: bool) -> Result<(), Box<dyn Error>> {
    let mut movies = HashMap::new();

    let archive_data = download_file(
        "https://files.grouplens.org/datasets/movielens/ml-1m.zip",
        "a6898adb50b9ca05aa231689da44c217cb524e7ebd39d264c56e2832f2c54e20",
    )?;
    let cursor = Cursor::new(archive_data);
    let mut archive = zip::ZipArchive::new(cursor)?;

    // make borrow checker happy
    {
        let mut movies_data = archive.by_name("ml-1m/movies.dat")?;

        // remove invalid UTF-8 bytes
        let mut buf = Vec::new();
        movies_data.read_to_end(&mut buf)?;
        let movies_data = String::from_utf8_lossy(&buf);

        let rdr = BufReader::new(movies_data.as_bytes());
        for result in rdr.lines() {
            let line = result?;
            let mut parts = line.split("::");
            let id = parts.next().unwrap().to_string();
            let title = parts.next().unwrap().to_string();
            movies.insert(id, title);
        }
    }

    let file = create_file(output, overwrite)?;
    let mut wtr = csv::Writer::from_writer(file);
    wtr.write_record(&["user_id", "item_id", "rating"])?;

    let ratings_data = archive.by_name("ml-1m/ratings.dat")?;
    let rdr = BufReader::new(ratings_data);
    for result in rdr.lines() {
        let line = result?;
        let mut parts = line.split("::");
        let user_id = parts.next().unwrap().to_string();
        let item_id = parts.next().unwrap().to_string();
        let rating = parts.next().unwrap().to_string();
        wtr.write_record(&[user_id, movies.get(&item_id).unwrap().to_string(), rating])?;
    }

    wtr.flush()?;

    Ok(())
}

fn download_movielens_25m(output: &Path, overwrite: bool) -> Result<(), Box<dyn Error>> {
    let mut movies = HashMap::new();

    let archive_data = download_file(
        "https://files.grouplens.org/datasets/movielens/ml-25m.zip",
        "8b21cfb7eb1706b4ec0aac894368d90acf26ebdfb6aced3ebd4ad5bd1eb9c6aa",
    )?;
    let cursor = Cursor::new(archive_data);
    let mut archive = zip::ZipArchive::new(cursor)?;

    // make borrow checker happy
    {
        let movies_data = archive.by_name("ml-25m/movies.csv")?;

        let mut rdr = csv::ReaderBuilder::new()
            .from_reader(movies_data);
        for result in rdr.records() {
            let record = result?;
            let id = record.get(0).unwrap().to_string();
            let title = record.get(1).unwrap().to_string();
            movies.insert(id, title);
        }
    }

    let file = create_file(output, overwrite)?;
    let mut wtr = csv::Writer::from_writer(file);
    wtr.write_record(&["user_id", "item_id", "rating"])?;

    // show processing progress since it takes a while
    let bar = progress_bar(25000095, "Processing", "{msg} {wide_bar} {percent}%");
    bar.set_draw_delta(250000);

    let ratings_data = archive.by_name("ml-25m/ratings.csv")?;
    let mut rdr = csv::ReaderBuilder::new()
        .from_reader(ratings_data);
    for result in rdr.records() {
        let record = result?;
        let user_id = record.get(0).unwrap().to_string();
        let item_id = record.get(1).unwrap().to_string();
        let rating = record.get(2).unwrap().to_string();
        wtr.write_record(&[user_id, movies.get(&item_id).unwrap().to_string(), rating])?;
        bar.inc(1);
    }

    wtr.flush()?;
    bar.finish();

    Ok(())
}

pub fn download(dataset: Dataset, output: Option<PathBuf>, overwrite: bool) -> Result<(), Box<dyn Error>> {
    let output = output.unwrap_or_else(|| {
        let mut default_output = PathBuf::from(&dataset.to_string());
        default_output.set_extension("csv");
        default_output
    });
    if !overwrite {
        check_exists(&output)?;
    }

    let usage_url = match dataset {
        Dataset::Movielens100k => "https://files.grouplens.org/datasets/movielens/ml-100k-README.txt",
        Dataset::Movielens1m => "https://files.grouplens.org/datasets/movielens/ml-1m-README.txt",
        Dataset::Movielens25m => "https://files.grouplens.org/datasets/movielens/ml-25m-README.html",
    };
    eprintln!("For dataset usage info, see {}", usage_url);

    let res = match dataset {
        Dataset::Movielens100k => download_movielens_100k(&output, overwrite),
        Dataset::Movielens1m => download_movielens_1m(&output, overwrite),
        Dataset::Movielens25m => download_movielens_25m(&output, overwrite),
    };
    if res.is_ok() {
        eprintln!("Saved to {}", output.display());
    }
    res
}
