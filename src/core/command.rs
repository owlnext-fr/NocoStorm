use indicatif::{style, ParallelProgressIterator};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;

use super::{
    context::{Context, Initialized},
    csv,
};
use ::csv::StringRecord;
use colored::Colorize;
use eyre::{bail, Result};
use simple_log::{error, info};

/// Run the command.
///
/// This is the main entrypoint for this CLI command.
pub fn run(context: &Context<Initialized>) -> Result<()> {
    let csv_file = context.args.csv_file.clone();

    if false == csv_file.exists() {
        bail!("CSV file does not exist: {:?}", csv_file);
    }

    let mut csv_reader = csv::parse_csv(
        csv_file,
        context.args.csv_separator.unwrap(),
        context.args.csv_quote_separator.unwrap(),
        context.args.use_windows_format.unwrap(),
    )?;

    let headers = csv_reader.headers()?.clone();
    let headers: Vec<String> = headers.iter().map(|x| x.to_string()).collect();

    let mut chunks: Vec<Vec<StringRecord>> = Vec::new();
    let mut treated = 0;
    for record in csv_reader.records() {
        if chunks.is_empty()
            || chunks.last().unwrap().len() >= context.args.chunk_size.unwrap() as usize
        {
            chunks.push(Vec::new());
        }

        if let Err(e) = record {
            error!("Error while reading CSV record: {:?}", e);
            continue;
        }

        let r = record.unwrap();

        chunks.last_mut().unwrap().push(r);
        treated += 1;
    }

    info!(
        "CSV file split into {} chunks ({} lines)",
        chunks.len(),
        treated
    );

    // using rayon here, but could use tokio with a scoped runtime.
    info!("Initializing rayon library...");
    rayon::ThreadPoolBuilder::new()
        .num_threads(context.args.parallel_jobs.unwrap_or(4) as usize)
        .build_global()
        .unwrap();

    info!("Uploading data to NocoDB...");

    println!("");

    for (field, value) in context.args.to_map() {
        println!("{}: {}", field.bold().white(), value.cyan());
    }

    println!("");

    let progress_size: u64 = chunks.len() as u64;

    chunks
        .par_iter()
        .progress_count(progress_size)
        .with_style(
            style::ProgressStyle::default_bar()
                .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")?,
        )
        .try_for_each(|chunk| {
            let payload: Vec<HashMap<String, String>> = chunk
                .iter()
                .map(|record| {
                    let mut map: HashMap<String, String> = HashMap::new();
                    map.insert("UUID".to_owned(), uuid::Uuid::new_v4().to_string());

                    for (i, header) in headers.iter().enumerate() {
                        map.insert(header.clone(), record.get(i).unwrap().to_string());
                    }

                    map
                })
                .collect();

            let res = context
                .nocodb
                .insert_bulk(context.args.nocodb_table_id.as_ref().unwrap(), payload);

            if let Err(e) = res {
                bail!("{:?}", e);
            }

            Ok(())
        })?;

    Ok(())
}
