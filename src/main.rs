extern crate rand;
extern crate tiny_keccak;
extern crate rayon;
extern crate clap;

mod simulation;

use simulation::*;
use clap::{App, Arg};

fn main() {
    let matches = App::new("test-closest")
        .about(
            "This tool calculates the chance of successful attack on a relay group by \
                simulating various strategies of relay group selection and quorum calculation",
        )
        .arg(
            Arg::with_name("strategy")
                .short("s")
                .long("strategy")
                .value_name("STRATEGY")
                .possible_values(
                    &["basic", "node_ageing_quorum50", "node_ageing_index_weights"],
                )
                .required(true)
                .help("The strategy to use"),
        )
        .get_matches();
    let strategy = matches.value_of("strategy").unwrap_or("basic");
    let runs = vec![
        (10, 25, 6, 10000, 200),
        (11, 28, 6, 10000, 200),
        (12, 30, 7, 10000, 200),
        (13, 33, 7, 10000, 200),
        (14, 35, 8, 10000, 200),
        (15, 37, 8, 10000, 200),
        (17, 43, 9, 10000, 200),
        (20, 50, 11, 50000, 200),
        (31, 76, 16, 200000, 200),
    ];

    for (g, n, q, times, tries) in runs {
        let sim = Simulation::new(g, n, q);
        let result = match strategy {
            "basic" => sim.run(times, tries, BasicSectionData::new),
            "node_ageing_quorum50" => sim.run(times, tries, NodeAgeingSectionData::new),
            "node_ageing_index_weights" => sim.run(times, tries, NodeAgeingIndexWeights::new),
            _ => unreachable!(),
        };

        println!("Group size: {}, section size: {}, quorum: {}", g, n, q);
        println!("  Forge success rate:       {}%", result.success_rate);
        println!("  Stall success rate:       {}%", result.stall_rate);
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
