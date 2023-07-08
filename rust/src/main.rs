mod geometry;
mod problem;
mod intersect;
mod fixed_pos_solver;

use std::fs::File;
use regex::Regex;
use crate::fixed_pos_solver::solve_fixed;

use crate::problem::{Problem, Solution};

fn main() {
    let argv: Vec<String> = wild::args().collect();

	let mut verbose: bool = false;
	let mut files: Vec<String> = Vec::new();
	for f in &argv[1..argv.len()] {
		if f == "-v" {
			verbose = true;
		} else {
            files.push(f.to_string());
		}
	}

	if files.len() == 0 {
		println!("Usage: icfpc [-v] problem-file...");
		std::process::exit(1);
	}

	let pre = Regex::new(r"problem-([1-9][0-9]*)\.json$").unwrap();
	for f in files {
		let id: u32 = match pre.captures(&*f) {
			None => panic!("unable to get solution id"),
			Some(caps) => caps.get(1).unwrap().as_str().parse::<u32>().unwrap()
		};
		if verbose {
			println!("Processing problem {} ({}) ...", id, f);
		}
		let problem = Problem::from_file(&f).unwrap();
		println!("Problem {} loaded. Musicians: {}, attendees: {}", id, problem.musicians.len(), problem.attendees.len());
		let (score, placements) = solve_fixed(&problem);
		if score <= 0.0 {
			println!("No scoring solution found :-(");
		} else {
			let solution = Solution { placements };
			let ofn = format!("solution-{}-{}.json", id, score);
			let w = File::create(ofn).unwrap();
			let _ = serde_json::to_writer_pretty(w, &solution);
		}
	}
}
