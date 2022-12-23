use std::time::Instant;

fn main() {
    let start = Instant::now();
    aoc22::run();
    println!("Finished in {:?}", start.elapsed());
}
