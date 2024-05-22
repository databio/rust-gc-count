use pyo3::prelude::*;

use crate::models::PyChecksumResult;
use rust_gc_count::checksum::process_sequence;
use rust_gc_count::checksum::get_file_reader;
use seq_io::fasta::Reader;

#[pyfunction]
pub fn checksum(file: String, verbose: Option<bool>) -> Vec<PyChecksumResult> {
    let mut results = Vec::new();
    // let mut reader = Reader::from_path(file).expect("Cannot find file to read from");
    // Allow either .fa or .fa.gz
    let read = get_file_reader(&file);
    let mut reader = Reader::new(read);


    let verbose = verbose.unwrap_or(false);

    while let Some(record) = reader.next() {
        let record = record.expect("Error found when retriving next record");
        let result = process_sequence(record, verbose);

        results.push(PyChecksumResult::from(result));
    }

    results
}
