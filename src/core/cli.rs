use clap::Parser;

use std::{collections::BTreeMap, path::PathBuf};

#[derive(Parser, Debug, Clone)]
#[command(
    version,
    about,
    long_about = "⚡ Lightning fast, multi-threaded tool to push CSV data into NocoDB tables"
)]
/// CLI arguments & options representation.
pub struct Cli {
    #[arg(value_name = "FILE", help = "path to CSV file tu upload to NocoDB")]
    /// Path to the CSV file to upload to NocoDB.
    pub csv_file: PathBuf,

    #[arg(
        short = 'b',
        long,
        value_name = "BASE_URL",
        help = "NocoDB server base URL"
    )]
    /// NocoDB server base URL.
    pub nocodb_base_url: Option<String>,

    #[arg(short = 'k', long, value_name = "API_TOKEN", help = "NocoDB API token")]
    /// NocoDB API token.
    pub nocodb_api_token: Option<String>,

    #[arg(
        short = 't',
        long,
        value_name = "TABLE_ID",
        help = "NocoDB table identifier"
    )]
    /// NocoDB table identifier.
    pub nocodb_table_id: Option<String>,

    #[arg(
        short = 'j',
        long,
        value_name = "NB_THREADS",
        help = "Number of parallel threads used to upload data",
        default_value = "4"
    )]
    /// Number of parallel threads used to upload data.
    pub parallel_jobs: Option<i8>,

    #[arg(
        short = 'c',
        long,
        value_name = "SIZE",
        help = "Number of CSV rows to upload in a single request",
        default_value = "1000"
    )]
    /// Number of CSV rows to upload in a single request.
    pub chunk_size: Option<i32>,

    #[arg(
        short = 's',
        long,
        value_name = "SEPARATOR",
        help = "CSV separator character",
        default_value = ","
    )]
    /// CSV separator character.
    pub csv_separator: Option<char>,

    #[arg(
        short = 'u',
        long,
        value_name = "SEPARATOR",
        help = "CSV quote separator",
        default_value = "\""
    )]
    /// CSV quote separator.
    pub csv_quote_separator: Option<char>,

    #[arg(
        short = 'w',
        long,
        help = "Either to use UTF-8 or ISO 8859-1 encoding for CSV file",
        default_value = "false"
    )]
    /// Either to use UTF-8 or ISO 8859-1 encoding for CSV file.
    pub use_windows_format: Option<bool>,

    #[command(flatten)]
    /// Verbose mode flag.
    pub verbose: clap_verbosity_flag::Verbosity,
}

impl Cli {
    /// Convert the CLI arguments to a map.
    ///
    /// Useful for logging and debugging.
    pub fn to_map(&self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();

        map.insert(
            "csv_file".to_string(),
            self.csv_file.to_string_lossy().to_string(),
        );
        map.insert(
            "nocodb_base_url".to_string(),
            self.nocodb_base_url.clone().unwrap_or_default(),
        );
        map.insert(
            "nocodb_api_token".to_string(),
            self.nocodb_api_token.clone().unwrap_or_default(),
        );
        map.insert(
            "nocodb_table_id".to_string(),
            self.nocodb_table_id.clone().unwrap_or_default(),
        );
        map.insert(
            "parallel_jobs".to_string(),
            self.parallel_jobs.unwrap_or_default().to_string(),
        );
        map.insert(
            "chunk_size".to_string(),
            self.chunk_size.unwrap_or_default().to_string(),
        );
        map.insert(
            "csv_separator".to_string(),
            self.csv_separator.unwrap_or_default().to_string(),
        );
        map.insert(
            "csv_quote_separator".to_string(),
            self.csv_quote_separator.unwrap_or_default().to_string(),
        );
        map.insert(
            "use_windows_format".to_string(),
            self.use_windows_format.unwrap_or_default().to_string(),
        );

        map
    }
}
