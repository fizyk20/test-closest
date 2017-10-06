extern crate rand;
extern crate tiny_keccak;

mod simulation;

use simulation::*;

fn main() {
    let sim = Simulation::new(10, 25, 6);
    let result = sim.run(5000, 200);

    println!("Success rate: {}%", result.success_rate);
    println!("Avg number of tries: {}", result.avg_tries);
    for (&tries, &num) in &result.tries_map {
        println!("{} tries: {} cases", tries, num);
    }
}
