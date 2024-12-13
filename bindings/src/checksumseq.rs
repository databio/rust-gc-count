use pyo3::prelude::*;

use crate::models::PyChecksumResult;
use rust_gc_count::checksum::process_sequence;
use rust_gc_count::checksum::process_str;
use rust_gc_count::checksum::get_file_reader;
use seq_io::fasta::Reader;

use std::io::prelude::Read;

#[pyfunction]
/// Computes the checksum for each sequence in a given FASTA file.
///
/// # Arguments
///
/// * `file` - A string representing the path to the FASTA file.
/// * `verbose` - An optional boolean to enable verbose output.
///
/// # Returns
///
/// A vector of `PyChecksumResult` containing the checksum results for each sequence.
pub fn checksum(file: String, verbose: Option<bool>) -> Vec<PyChecksumResult> {
    let mut results = Vec::new();
    // let mut reader = Reader::from_path(file).expect("Cannot find file to read from");
    // Allow either .fa or .fa.gz
    let read = get_file_reader(&file);
    let mut reader = Reader::new(read);
    let verbose = verbose.unwrap_or(false);

    while let Some(record) = reader.next() {
        let record = record.expect("Error found when retrieving next record");
        let result = process_sequence(record, verbose);

        results.push(PyChecksumResult::from(result));
    }

    results
}

#[pyfunction]
/// Computes the checksum for a given sequence string.
///
/// # Arguments
///
/// * `readable` - A string slice representing the sequence.
///
/// # Returns
///
/// A `PyChecksumResult` containing the checksum result for the sequence.
pub fn checksum_from_str(readable: &str) -> PyChecksumResult {
	// let mut results = Vec::new();
	// let fasta = b">id\nSEQUENCE";
    let dummy_str = "SEQUENCE";
	// let mut reader = Reader::new(dummy_str.as_bytes());
	// let result;
    let result = process_str(readable);
    return PyChecksumResult::from(result);

    // if let Some(record) = reader.next() {
    //     let record = record.expect("Error found when retrieving next record");
    // } else {
    //     panic!("No record found in the provided string");
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum_from_str() {
        let result = checksum_from_str("SEQUENCE");
        // print the result to stdout
        println!("{:?}", result.sha512);
        assert!(result.sha512 == "XizFQiF5qny4EgPOz3mMaBpxcKOktRbM")
        // Add assertions to verify the result
        // assert!(result.is_some());
    }
}
