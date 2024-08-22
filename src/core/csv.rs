use std::{fs::File, path::PathBuf};

use csv::{Reader, ReaderBuilder};
use eyre::Result;

use encoding_rs::{UTF_8, WINDOWS_1252};
use encoding_rs_io::{DecodeReaderBytes, DecodeReaderBytesBuilder};

/// Parses a CSV file into a CSV reader for later use.
pub fn parse_csv(
    path: PathBuf,
    separator: char,
    quote_separator: char,
    use_windows_format: bool,
) -> Result<Reader<DecodeReaderBytes<File, Vec<u8>>>> {
    let encoding = if use_windows_format {
        Some(WINDOWS_1252)
    } else {
        Some(UTF_8)
    };

    let file = DecodeReaderBytesBuilder::new()
        .encoding(encoding)
        .build(std::fs::File::open(path).unwrap());

    let rdr = ReaderBuilder::new()
        .delimiter(separator as u8)
        .quote(quote_separator as u8)
        .from_reader(file);

    Ok(rdr)
}
