use std::fmt;

use clap::ValueEnum;

// Multizip for zipping arbitary number of iterators
pub struct Multizip<T>(pub Vec<T>);

impl<T> Iterator for Multizip<T>
where
    T: Iterator,
{
    type Item = Vec<T::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.iter_mut().map(Iterator::next).collect()
    }
}

// Calculate hamming distance
pub fn hamming_distance(a: u64, b: u64) -> u32 {
    (a ^ b).count_ones()
}

// Algorithm enum for CLI
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[value(rename_all = "lower")]
pub enum Algorithm {
    AHash,
    DHash,
}

impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Algorithm::AHash => "aHash",
            Algorithm::DHash => "dHash",
        };
        write!(f, "{}", s)
    }
}
