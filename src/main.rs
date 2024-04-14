mod analyser;
mod tester;

use analyser::Analyser;

fn main() {
    let mut analyser = Analyser::new();
    analyser.deduplicate("input.txt", "output.txt");
}
