use std::fs::File;
use std::io::{BufReader, BufWriter};

use csv::{ReaderBuilder, Writer};

fn main() {
    let source_file = File::open("customer.csv").unwrap();
    let out_file = File::create("out.csv").unwrap();

    let reader = BufReader::new(source_file);
    let writer = BufWriter::new(out_file);

    let mut builder = ReaderBuilder::new();
    // Magic values for this weird csv (found via hex editor)
    builder.terminator(csv::Terminator::Any(10));

    // allow escaping by using "\"
    builder.escape(Some(b'\\'));

    let mut csv_reader = builder.from_reader(reader);

    let mut csv_writer = Writer::from_writer(writer);

    for record in csv_reader.byte_records() {
        let record = record.unwrap();

        let row = record
            .iter()
            .map(|value| match String::from_utf8(value.to_vec()) {
                /* Parse as utf8, if it is not valid utf8 convert bytes to hex string */
                Ok(string) => string,
                Err(_) => hex::encode(value),
            });

        csv_writer.write_record(row).unwrap();
    }
}
