use crate::helpers::*;
use discorec::{Dataset, Recommender, RecommenderBuilder};
use std::error::Error;
use std::fs::File;
use std::io::ErrorKind;
use std::path::Path;

fn fit_recommender(input: &Path, factors: u32, iterations: u32) -> Result<Recommender<String, String>, Box<dyn Error>> {
    let file = File::open(input).map_err(|e| -> Box<dyn Error> {
        if e.kind() == ErrorKind::NotFound {
            format!("File not found: {}", input.display()).into()
        } else {
            e.into()
        }
    })?;
    let mut rdr = csv::Reader::from_reader(file);

    let headers = rdr.headers()?;
    let explicit = !headers.iter().any(|r| r == "value");
    let value_header = if explicit { "rating" } else { "value" };

    let user_index = headers.iter().position(|r| r == "user_id").ok_or("Missing user_id column")?;
    let item_index = headers.iter().position(|r| r == "item_id").ok_or("Missing item_id column")?;
    let value_index = headers.iter().position(|r| r == value_header).ok_or("Missing rating/value column")?;

    let mut dataset = Dataset::new();
    for (i, result) in rdr.records().enumerate() {
        let record = result?;

        // safe to unwrap since csv::Reader checks for same number of columns as header
        dataset.push(
            record.get(user_index).unwrap().to_string(),
            record.get(item_index).unwrap().to_string(),
            // match CSV error: record 1 (line: 2, byte: 23): found record with 2 fields
            record.get(value_index).unwrap().parse::<f32>().map_err(|e| format!("Input error: record {} (line: {}, column: {}): {}", i + 1, i + 2, value_header, e))?,
        );
    }

    let bar = progress_bar(iterations as u64, "Training model", "{msg} {wide_bar} {percent}%");

    let cb = |_| {
        bar.inc(1);
    };

    let mut builder = RecommenderBuilder::new();
    builder
        .factors(factors)
        .iterations(iterations)
        .callback(cb);

    let recommender = if explicit {
        builder.fit_explicit(&dataset)
    } else {
        builder.fit_implicit(&dataset)
    };

    bar.finish();

    Ok(recommender)
}

pub fn user_recs(input: &Path, output: &Path, count: usize, factors: u32, iterations: u32, overwrite: bool) -> Result<(), Box<dyn Error>> {
    if !overwrite {
        check_exists(output)?;
    }

    let recommender = fit_recommender(input, factors, iterations)?;
    let mut user_ids = recommender.user_ids().clone();
    user_ids.sort_unstable();

    let mut wtr = create_csv(output, overwrite)?;
    wtr.write_record(["user_id", "recommended_item_id", "score"])?;

    let bar = progress_bar(user_ids.len() as u64, "Saving recs", "{msg} {wide_bar} {pos}/{len}");

    for user in user_ids.iter() {
        for (recommended_item, score) in recommender.user_recs(user, count) {
            wtr.write_record([user, recommended_item, &score.to_string()])?;
        }
        bar.inc(1);
    }

    wtr.flush()?;
    bar.finish();

    Ok(())
}

pub fn item_recs(input: &Path, output: &Path, count: usize, factors: u32, iterations: u32, overwrite: bool) -> Result<(), Box<dyn Error>> {
    if !overwrite {
        check_exists(output)?;
    }

    let recommender = fit_recommender(input, factors, iterations)?;
    let mut item_ids = recommender.item_ids().clone();
    item_ids.sort_unstable();

    let mut wtr = create_csv(output, overwrite)?;
    wtr.write_record(["item_id", "recommended_item_id", "score"])?;

    let bar = progress_bar(item_ids.len() as u64, "Saving recs", "{msg} {wide_bar} {pos}/{len}");

    for item in item_ids.iter() {
        for (recommended_item, score) in recommender.item_recs(item, count) {
            wtr.write_record([item, recommended_item, &score.to_string()])?;
        }
        bar.inc(1);
    }

    wtr.flush()?;
    bar.finish();

    Ok(())
}

pub fn similar_users(input: &Path, output: &Path, count: usize, factors: u32, iterations: u32, overwrite: bool) -> Result<(), Box<dyn Error>> {
    if !overwrite {
        check_exists(output)?;
    }

    let recommender = fit_recommender(input, factors, iterations)?;
    let mut user_ids = recommender.user_ids().clone();
    user_ids.sort_unstable();

    let mut wtr = create_csv(output, overwrite)?;
    wtr.write_record(["user_id", "similar_user_id", "score"])?;

    let bar = progress_bar(user_ids.len() as u64, "Saving users", "{msg} {wide_bar} {pos}/{len}");

    for user in user_ids.iter() {
        for (similar_user, score) in recommender.similar_users(user, count) {
            wtr.write_record([user, similar_user, &score.to_string()])?;
        }
        bar.inc(1);
    }

    wtr.flush()?;
    bar.finish();

    Ok(())
}
