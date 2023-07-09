mod geometry;
mod problem;
mod intersect;
mod mutate_solver;
mod scoring;

use std::fs::File;
use regex::Regex;

use crate::problem::{Problem, Solution};

fn usage() {
	println!("Usage: icfpc [-v] problem-file...");
	println!("   or: icfpc -score problem-file solution-file");
	std::process::exit(1);
}

fn load_problem(f: &String) -> (u32, Problem) {
	let pre = Regex::new(r"problem-([1-9][0-9]*)\.json$").unwrap();
	let id: u32 = match pre.captures(&*f) {
		None => panic!("unable to get solution id"),
		Some(caps) => caps.get(1).unwrap().as_str().parse::<u32>().unwrap()
	};
	let problem = Problem::from_file(&f).unwrap();
	(id, problem)
}

fn compute_score(pf: &String, sf: &String) {
	let (id, problem) = load_problem(pf);
	let solution = Solution::from_file(sf).unwrap();
	let score = scoring::score(&problem, &solution.placements, id >= 56);
	println!("Problem: {}, score: {}", id, score);
}

fn main() {
    let argv: Vec<String> = wild::args().collect();

	let mut verbose: bool = false;
	let mut files: Vec<String> = Vec::new();

	if argv.len() >= 2 && &argv[1] == "-score" {
		if argv.len() != 4 {
			usage();
		}
		compute_score(&argv[2], &argv[3]);
		return;
	}

	for f in &argv[1..argv.len()] {
		if f == "-v" {
			verbose = true;
		} else {
            files.push(f.to_string());
		}
	}

	if files.len() == 0 {
		usage();
	}

	for f in files {
		let (id, problem) = load_problem(&f);
		if verbose {
			println!("Processing problem {} ({}) ...", id, f);
		}
		println!("Problem {} loaded. Musicians: {}, attendees: {}", id, problem.musicians.len(), problem.attendees.len());
		println!("Number of instruments: {}", problem.musicians.iter().max().unwrap());
		let (score, placements) = mutate_solver::solve(&problem, id >= 56);
		let ref_score = scoring::score(&problem, &placements, id >= 56);
		if score == ref_score {
			println!("Score matches reference score: {}", score);
		} else {
			println!("Scores do not match. Solver: {}, reference: {}, {:.1}% off.",
					 score, ref_score, percent_off(ref_score, score));
		}
		if score <= 0.0 {
			println!("No scoring solution found :-(");
		} else {
			let solution = Solution { placements };
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
