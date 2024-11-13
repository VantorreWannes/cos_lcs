#![cfg(test)]

use csv::{Reader, WriterBuilder};
use rand::{distributions::Uniform, prelude::Distribution, thread_rng};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io;

#[derive(Debug, PartialEq, Clone, Copy, Default, PartialOrd, Serialize, Deserialize)]
pub struct ComparedLcsStats {
    input_size: usize,
    alphabet_size: u8,
    approximate_output_length: usize,
    correct_output_length: usize,
    approximation_factor: f64,
}

impl ComparedLcsStats {
    pub fn new(
        input_size: usize,
        alphabet_size: u8,
        approximate_output_length: usize,
        correct_output_length: usize,
    ) -> Self {
        Self {
            input_size,
            alphabet_size,
            approximate_output_length,
            correct_output_length,
            approximation_factor: Self::accuracy(approximate_output_length, correct_output_length),
        }
    }

    pub fn accuracy(approximate_output_length: usize, correct_output_length: usize) -> f64 {
        if correct_output_length == 0 {
            0.0
        } else {
            approximate_output_length as f64 / correct_output_length as f64
        }
    }
}

pub fn serialize_stats<W: io::Write>(
    stats: &[ComparedLcsStats],
    writer: W,
) -> Result<(), Box<dyn Error>> {
    let mut wtr = WriterBuilder::new().has_headers(true).from_writer(writer);

    for stat in stats {
        wtr.serialize(stat)?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn deserialize_stats<R: io::Read>(reader: R) -> Result<Vec<ComparedLcsStats>, Box<dyn Error>> {
    let mut rdr = Reader::from_reader(reader);
    let mut stats = Vec::new();

    for result in rdr.deserialize() {
        let record: ComparedLcsStats = result?;
        stats.push(record);
    }
    Ok(stats)
}

/// Helper function to generate random source and target data of given length and alphabet size
pub fn generate_random_data(length: usize, alphabet_size: u8) -> (Vec<u8>, Vec<u8>) {
    let mut rng = thread_rng();
    let die = Uniform::from(0..=alphabet_size);
    let source: Vec<u8> = die.sample_iter(&mut rng).take(length).collect();
    let target: Vec<u8> = die.sample_iter(&mut rng).take(length).collect();
    (source, target)
}
