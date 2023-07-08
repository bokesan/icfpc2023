use crate::geometry::{Point, point};
use crate::intersect::line_circle_intersect;
use crate::problem::{Attendee, Problem};

pub fn score(problem: &Problem, placements: &Vec<Point<f64>>) -> f64 {
    problem.attendees.iter().map(|a| happiness(problem, a, placements)).sum()
}

fn happiness(problem: &Problem, attendee: &Attendee, placements: &Vec<Point<f64>>) -> f64 {
    (0..placements.len()).map(|k| impact(problem, attendee, placements, k)).sum()
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

fn is_blocked(problem: &Problem, attendee: &Attendee, placements: &Vec<Point<f64>>, k: usize) -> bool {
    let a = point(attendee.x, attendee.y);
    let p = placements[k];

    // check other musicians
    for (k1, p1) in placements.iter().enumerate() {
        if k1 != k && line_circle_intersect(a, p, *p1, 5.0) {
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