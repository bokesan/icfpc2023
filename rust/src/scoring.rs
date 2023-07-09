use crate::geometry::{Point, point};
use crate::intersect::line_circle_intersect;
use crate::problem::{Attendee, Problem, Solution};

pub fn score(problem: &Problem, solution: &Solution, playing_together: bool) -> f64 {
    let closeness = closeness_factors(problem, &solution.placements, playing_together);
    problem.attendees.iter().map(|a| happiness(problem, a, solution, &closeness)).sum()
}

pub fn closeness_factors(problem: &Problem, placements: &Vec<Point<f64>>, playing_together: bool) -> Vec<f64> {
    let m = placements.len();
    let mut closeness = vec![1.0; m];
    if playing_together {
        for (i,p) in placements.iter().enumerate() {
            let mut sum = 0.0;
            for (j,q) in placements.iter().enumerate() {
                if i != j && problem.musicians[i] == problem.musicians[j] {
                    sum += 1.0 / (*p - *q).length();
                }
            }
            closeness[i] = 1.0 + sum;
        }
    }
    closeness
}

fn happiness(problem: &Problem, attendee: &Attendee, solution: &Solution, closeness: &Vec<f64>) -> f64 {
    (0..solution.placements.len())
        .map(|k| (solution.volume(k) * closeness[k] * impact(problem, attendee, &solution.placements, k)).ceil())
        .sum()
}

fn impact(problem: &Problem, attendee: &Attendee, placements: &Vec<Point<f64>>, k: usize) -> f64 {
    if is_blocked(problem, attendee, placements, k) {
        return 0.0
    }
    let p = placements[k];
    let dx = attendee.x - p.x;
    let dy = attendee.y - p.y;
    let d = (dx*dx + dy*dy).sqrt();
    let instrument = problem.musicians[k];
    (1000000.0 * attendee.tastes[instrument] / (d*d)).ceil()
}

pub fn is_blocked(problem: &Problem, attendee: &Attendee, placements: &Vec<Point<f64>>, musician_index: usize) -> bool {
    let a = point(attendee.x, attendee.y);
    let p = placements[musician_index];

    // check other musicians
    for (k1, p1) in placements.iter().enumerate() {
        if k1 != musician_index && line_circle_intersect(a, p, *p1, 5.0) {
            return true
        }
    }

    // check pillars
    for pil in &problem.pillars {
        let c = point(pil.center[0], pil.center[1]);
        if line_circle_intersect(a, p, c, pil.radius) {
            return true
        }
    }
    false
}