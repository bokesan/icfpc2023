mod geometry;
mod problem;
mod intersect;
mod mutate_solver;
mod scoring;

use std::fs::File;
use regex::Regex;

use crate::problem::{Problem, Solution};

fn usage() {
	println!("Usage: icfpc [-v] [-t seconds] problem-file...");
	println!("   or: icfpc -score problem-file solution-file");
	std::process::exit(1);
}

fn load_problem(f: &String, verbose: bool) -> (u32, Problem) {
	let pre = Regex::new(r"problem-([1-9][0-9]*)\.json$").unwrap();
	let id: u32 = match pre.captures(&*f) {
		None => panic!("unable to get solution id"),
		Some(caps) => caps.get(1).unwrap().as_str().parse::<u32>().unwrap()
	};
	if verbose {
		println!("Processing problem {} ({}) ...", id, f);
	}
	let problem = Problem::from_file(&f).unwrap();
	(id, problem)
}

fn compute_score(pf: &String, sf: &String) {
	let (id, problem) = load_problem(pf, false);
	let solution = Solution::from_file(sf).unwrap();
	let score = scoring::score(&problem, &solution, id >= 56);
	println!("Problem: {}, score: {}", id, score);
}

fn main() {
    let argv: Vec<String> = wild::args().collect();

	let mut verbose: bool = false;
	// Solver timeout in seconds
	let mut time: u64 = 60;
	let mut files: Vec<String> = Vec::new();

	if argv.len() >= 2 && &argv[1] == "-score" {
		if argv.len() != 4 {
			usage();
		}
		compute_score(&argv[2], &argv[3]);
		return;
	}

	let mut i = 1;
	while i < argv.len() {
		let f = &argv[i];
		if f == "-v" {
			verbose = true;
		} else if f == "-t" {
			i += 1;
			time = argv[i].parse::<u64>().unwrap();
		} else {
			files.push(f.to_string());
		}
		i += 1;
	}

	if files.len() == 0 {
		usage();
	}

	println!("Time per problem: {} seconds.", time);
	for f in files {
		let (id, problem) = load_problem(&f, verbose);
		println!("Problem {} loaded. Musicians: {}, attendees: {}", id, problem.musicians.len(), problem.attendees.len());
		println!("Number of instruments: {}", problem.musicians.iter().max().unwrap());
		let (score, solution) = mutate_solver::optimize(&problem, id >= 56, time);
		let ref_score = scoring::score(&problem, &solution, id >= 56);
		if score == ref_score {
			println!("Score matches reference score: {}", score);
		} else {
			println!("Scores do not match. Solver: {}, reference: {}, {:.1}% off.",
					 score, ref_score, percent_off(ref_score, score));
		}
		if score <= 0.0 {
			println!("No scoring solution found :-(");
		} else {
			let ofn = format!("solution-{}-{}.json", id, ref_score);
			let w = File::create(ofn).unwrap();
			let _ = serde_json::to_writer_pretty(w, &solution);
		}
	}
}

fn percent_off(correct: f64, wrong: f64) -> f64 {
	let delta = if wrong > correct { wrong - correct } else { correct - wrong };
	100.0 * delta / correct
}
