use std::ops::Add;
use std::time::{Duration, Instant};

use rand::Rng;

use crate::geometry::{Point, point, vector};
use crate::intersect::line_circle_intersect;
use crate::problem::{Attendee, Problem, Solution};
use crate::scoring;
use crate::scoring::closeness_factors;

fn make_positions(problem: &Problem) -> Vec<Point<f64>> {
    let n = problem.musicians.len();
    let mut positions = Vec::with_capacity(n);
    let max_rows = (problem.stage_height / 10.0).floor() as i32 - 1;
    let max_cols = (problem.stage_width / 10.0).floor() as i32 - 1;

    println!("stage size: {} x {}", problem.stage_width, problem.stage_height);
    let mut c = ((n as f64).sqrt() * problem.stage_width.sqrt() / problem.stage_height.sqrt()) as i32;
    let mut r = ((n as f64).sqrt() * problem.stage_height.sqrt() / problem.stage_width.sqrt()) as i32;
    // println!("calc: {} x {} = {}", c, r, c*r);
    if c == 0 {
        c = 1;
        r = n as i32;
    } else if r == 0 {
        r = 1;
        c = n as i32;
    }
    while c < max_cols && c * r < n as i32 {
        c = c + 1
    }
    while r < max_rows && c * r < n as i32 {
        r = r + 1
    }

    if c * r < n as i32 {
        panic!("Stage too small - bailing out.");
    }
    println!("Placing {} musicians in {}x{} grid.", n, c, r);
    let blx = problem.stage_bottom_left[0];
    let bly = problem.stage_bottom_left[1];
        let x_step = problem.stage_width / (c + 1) as f64;
        let y_step = problem.stage_height / (r + 1) as f64;
        if x_step < 10.0 || y_step < 10.0 {
            panic!("step size too small (wide): {}, {}", x_step, y_step);
        }
        let mut y = y_step;
        for _r in 0..r {
            let mut x = x_step;
            for _c in 0..c {
                let p = point(x + blx, y + bly);
                if !on_stage(problem, &p) {
                    panic!("not on stage: {}", p);
                }
                positions.push(p);
                if positions.len() == n {
                    return positions;
                }
                x = x + x_step;
            }
            y = y + y_step;
        }
    positions
}

fn on_stage(problem: &Problem, p: &Point<f64>) -> bool {
    let blx = problem.stage_bottom_left[0];
    let bly = problem.stage_bottom_left[1];
    p.x >= blx + 10.0
        && p.y >= bly + 10.0
        && p.x <= problem.stage_width + blx - 10.0
        && p.y <= problem.stage_height + bly - 10.0
}

fn is_blocked2(problem: &Problem, placements: &Vec<(Point<f64>, Vec<bool>)>, musician_index: usize, attendee: &Attendee) -> bool {
    let attendee_pos = point(attendee.x, attendee.y);
    let musician_pos = placements[musician_index].0;
    for (i,p) in placements.iter().enumerate() {
        if i != musician_index && line_circle_intersect(attendee_pos, musician_pos, p.0, 5.0) {
            return true
        }
    }
    // check pillars
    for pil in &problem.pillars {
        let c = point(pil.center[0], pil.center[1]);
        if line_circle_intersect(attendee_pos, musician_pos, c, pil.radius) {
            return true
        }
    }
    false
}

fn compute_los(problem: &Problem, positions: &Vec<(Point<f64>, Vec<bool>)>, i: usize) -> Vec<bool> {
    let na = problem.attendees.len();
    let mut visible = Vec::with_capacity(na);
    for a in &problem.attendees {
        visible.push(!is_blocked2(problem, positions, i, a));
    }
    visible
}

fn recompute_los(problem: &Problem, ann: &mut Vec<(Point<f64>, Vec<bool>)>) {
    for i in 0..ann.len() {
        ann[i] = (ann[i].0, compute_los(problem, ann, i));
    }
}

fn annotate_with_los(problem: &Problem, positions: &Vec<Point<f64>>) -> Vec<(Point<f64>, Vec<bool>)> {
    let mut result = Vec::with_capacity(positions.len());
    let na = problem.attendees.len();
    for (i, p) in positions.iter().enumerate() {
        let mut visible = Vec::with_capacity(na);
        for a in &problem.attendees {
            visible.push(!scoring::is_blocked(problem, a, positions, i));
        }
        result.push((*p, visible));
    }
    result
}

fn impact(att: &Attendee, mus: Point<f64>, instrument: usize) -> f64 {
    let dx = att.x - mus.x;
    let dy = att.y - mus.y;
    let d = (dx * dx + dy * dy).sqrt();
    (1000000.0 * att.tastes[instrument] / (d*d)).ceil()
}

fn happiness(attendee_index: usize, problem: &Problem, placements: &Vec<(Point<f64>, Vec<bool>)>, closeness: &Vec<f64>) -> f64 {
    let mut sum = 0.0;
    let ms = &problem.musicians;
    let a = &problem.attendees[attendee_index];
    for (k, place) in placements.iter().enumerate() {
        if place.1[attendee_index] {
            let instrument = ms[k];
            sum += (closeness[k] * impact(a, place.0, instrument)).ceil();
        }
    }
    sum
}

fn score(problem: &Problem, placements: &Vec<(Point<f64>, Vec<bool>)>, playing_together: bool) -> f64 {
    if placements.len() != problem.musicians.len() {
        panic!("Fatal error: wrong placements length. musicians: {}, placements: {}",
               problem.musicians.len(), placements.len());
    }
    let closeness = closeness_factors(problem, &placements.iter().map(|e| e.0).collect(), playing_together);
    let mut sum = 0.0;
    for ai in 0 .. problem.attendees.len() {
        sum += happiness(ai, problem, placements, &closeness);
    }
    sum
}

fn mutation_swap(problem: &Problem, placements: &mut Vec<(Point<f64>, Vec<bool>)>) -> bool {
    let mut rng = rand::thread_rng();
    let n= placements.len();
    let i1 = rng.gen_range(0..n);
    let instrument1 = problem.musicians[i1];
    for _k in 0..100 {
        let i2 = rng.gen_range(0..n);
        if i1 != i2 && instrument1 != problem.musicians[i2] {
            placements.swap(i1, i2);
            // println!("  mutation: swapped {} and {}", i1, i2);
            return true;
        }
    }
    false
}

// move a musician up to 5 units in any direction
fn mutation_move(problem: &Problem, placements: &mut Vec<(Point<f64>, Vec<bool>)>) -> bool {
    let max_move_dist = (problem.stage_width.max(problem.stage_height) as usize / 2).max(2);
    let lim = 2 * max_move_dist + 1;
    let mut rng = rand::thread_rng();
    let n = placements.len();
    for _i in 0..100 {
        let i = rng.gen_range(0..n);
        let x_offs = (rng.gen_range(0..lim) - max_move_dist) as f64;
        let y_offs = (rng.gen_range(0..lim) - max_move_dist) as f64;
        let np = placements[i].0.add(vector(x_offs, y_offs));
        if on_stage(problem, &np) && distance_to_others_ok(&np, i, &placements) {
            placements[i] = (np, Vec::new());
            recompute_los(problem, placements);
            // println!("  mutation: moved {} by {},{}", i, x_offs, y_offs);
            return true
        }
    }
    false
}

fn mutate(problem: &Problem, v: &Vec<(Point<f64>, Vec<bool>)>, swap_enabled: bool) -> Vec<(Point<f64>, Vec<bool>)> {
    let mut r = v.to_vec();
    let mut rng = rand::thread_rng();
    let mut c = rng.gen_range(0..20);

    if c == 0 || !swap_enabled {
        // mit moves ist der score total grütze
        if mutation_move(problem, &mut r) {
            return r;
        }
        c -= 1;
    }
    mutation_swap(problem, &mut r);
    r
}

fn pt_distance_squared(p: &Point<f64>, q: &Point<f64>) -> f64 {
    let dx = p.x - q.x;
    let dy = p.y - q.y;
    dx * dx + dy * dy
}

fn distance_to_others_ok(p: &Point<f64>, i: usize, pts: &Vec<(Point<f64>, Vec<bool>)>) -> bool {
    for (k,e) in pts.iter().enumerate() {
        if k != i && pt_distance_squared(&p, &e.0) < 100.0 {
            return false
        }
    }
    true
}

pub fn solve(problem: &Problem, playing_together: bool, max_time_seconds: u64) -> (f64, Solution) {
    let verify = false;
    let timeout = Duration::from_secs(max_time_seconds);
    let start = Instant::now();
    let r = make_positions(problem);
    let mut ar = annotate_with_los(problem, &r);
    let mut s = score(problem, &ar, playing_together);
    let ref_score = scoring::score(problem, &Solution { placements: r, volumes: None }, playing_together);
    if s != ref_score {
        panic!("Bug in solver score computation. Solver: {}, reference: {}", s, ref_score);
    }
    println!("Initial score: {}", s);
    let swap_enabled = problem.musicians.iter().any(|i| *i != 0);
    let mut perms: u64 = 1;
    while start.elapsed() < timeout {
        perms = perms + 1;
        let r2 = mutate(problem, &ar, swap_enabled);
        let s2 = score(problem, &r2, playing_together);
        if verify {
            let p2 = r2.iter().map(|e| e.0).collect();
            let ref_s2 = scoring::score(problem, &Solution{placements: p2, volumes: None}, playing_together);
            if s2 != ref_s2 {
                println!("    verify: score wrong. calc={}, ref={}", s2, ref_s2);
            }
        }
        if s2 > s {
            // println!("    good one! Score improved from {} to {}", s, s2);
            ar = r2;
            s = s2;
        }
    }
    println!("{} mutations tested. Final score: {}", perms, s);
    let volumes = vec![10.0; problem.musicians.len()];
    (10.0 * s, Solution { placements: ar.iter().map(|x| x.0).collect(), volumes: Some(volumes) })
}
