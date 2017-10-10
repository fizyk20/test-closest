extern crate rand;
extern crate tiny_keccak;
extern crate rayon;

mod simulation;

use simulation::*;

fn main() {
    let runs = vec![
        (10, 25, 6, 10000, 200),
        (12, 30, 7, 10000, 200),
        (15, 37, 8, 10000, 200),
        (20, 50, 11, 50000, 200),
        (31, 76, 16, 200000, 200),
    ];

    for (g, n, q, times, tries) in runs {
        let sim = Simulation::new(g, n, q);
        let result = sim.run(times, tries, NodeAgeingSectionData::new);

        println!("Group size: {}, section size: {}, quorum: {}", g, n, q);
        println!("  Success rate:             {}%", result.success_rate);
        println!(
            "  Close group success rate: {}%",
            result.closest_success_rate
        );
        println!("  Avg number of tries:      {}", result.avg_tries);
        println!("");
        //    for (&tries, &num) in &result.tries_map {
        //        println!("{} tries: {} cases", tries, num);
        //    }
    }
}
