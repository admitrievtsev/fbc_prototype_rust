use std::vec::Vec;
mod analyser;
mod tester;

use analyser::Analyser;
use std::collections::HashMap;

fn main() {
    let mut analyser = Analyser::new("input.txt", "output.txt").unwrap();
    analyser.deduplication();
}
