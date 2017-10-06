extern crate rand;
extern crate tiny_keccak;
extern crate rayon;

mod simulation;

use simulation::*;

fn main() {
    let runs = vec![
        (10, 25, 6, 5000, 200),
        (12, 30, 7, 5000, 200),
        (15, 37, 8, 5000, 200),
        (20, 50, 11, 10000, 400),
        (31, 76, 16, 20000, 500),
    ];

    for (g, n, q, times, tries) in runs {
        let sim = Simulation::new(g, n, q);
        let result = sim.run(times, tries);

        println!("Group size: {}, section size: {}, quorum: {}", g, n, q);
        println!("  Success rate: {}%", result.success_rate);
        println!("  Avg number of tries: {}", result.avg_tries);
        println!("");
        //    for (&tries, &num) in &result.tries_map {
        //        println!("{} tries: {} cases", tries, num);
        //    }
    }
}
