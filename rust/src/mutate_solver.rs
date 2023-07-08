use std::ops::Add;
use std::time::{Duration, Instant};
use rand::Rng;
use crate::geometry::{Point, point, vector};
use crate::intersect::line_circle_intersect;
use crate::problem::{Attendee, Problem};

fn make_positions(problem: &Problem) -> Vec<Point<f64>> {
    let n = problem.musicians.len();
    let mut positions = Vec::with_capacity(n);
    let max_rows = (problem.stage_height / 10.0).floor() as i32 - 1;
    let max_cols = (problem.stage_width / 10.0).floor() as i32 - 1;

    // r * c == n
    println!("stage size: {} x {}", problem.stage_width, problem.stage_height);
    let mut c = ((n as f64).sqrt() * problem.stage_width.sqrt() / problem.stage_height.sqrt()) as i32;
    let mut r = ((n as f64).sqrt() * problem.stage_height.sqrt() / problem.stage_width.sqrt()) as i32;
    println!("calc: {} x {} = {}", c, r, c*r);
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

fn is_blocked(problem: &Problem, placements: &Vec<Point<f64>>, musician_index: usize, attendee: &Attendee) -> bool {
    let attendee_pos = point(attendee.x, attendee.y);
    let musician_pos = placements[musician_index];
    for (i,p) in placements.iter().enumerate() {
        if i != musician_index && line_circle_intersect(attendee_pos, musician_pos, *p, 5.0) {
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

fn annotate_with_los(problem: &Problem, positions: &Vec<Point<f64>>) -> Vec<(Point<f64>, Vec<bool>)> {
    let mut result = Vec::with_capacity(positions.len());
    let na = problem.attendees.len();
    for (i, p) in positions.iter().enumerate() {
        let mut visible = Vec::with_capacity(na);
        for a in &problem.attendees {
            visible.push(!is_blocked(problem, positions, i, a));
        }
        result.push((*p, visible));
    }
    result
}

fn happiness1(att: &Attendee, mus: Point<f64>, instrument: usize) -> f64 {
    let dx = att.x - mus.x;
    let dy = att.y - mus.y;
    let d = (dx * dx + dy * dy).sqrt();
    (1000000.0 * att.tastes[instrument] / (d*d)).ceil()
}

fn happiness(attendee_index: usize, problem: &Problem, placements: &Vec<(Point<f64>, Vec<bool>)>) -> f64 {
    let mut sum = 0.0;
    let ms = &problem.musicians;
    let a = &problem.attendees[attendee_index];
    for (k, place) in placements.iter().enumerate() {
        if place.1[attendee_index] {
            let instrument = ms[k];
            sum = sum + happiness1(a, place.0, instrument as usize);
        }
    }
    sum
}

fn score(problem: &Problem, placements: &Vec<(Point<f64>, Vec<bool>)>) -> f64 {
    if placements.len() != problem.musicians.len() {
        panic!("Fatal error: wrong placements length. musicians: {}, placements: {}",
               problem.musicians.len(), placements.len());
    }
    let mut sum = 0.0;
    for ai in 0 .. problem.attendees.len() {
        sum = sum + happiness(ai, problem, placements);
    }
    sum
}

fn mutate(problem: &Problem, v: &Vec<(Point<f64>, Vec<bool>)>) -> Vec<(Point<f64>, Vec<bool>)> {
    let mut r = v.to_vec();
    let mut rng = rand::thread_rng();
    let n = v.len();
    let mut mutated = false;
    if rng.gen_range(0..2) == 0 {
        // flip two musicians
        let i1 = rng.gen_range(0..n);
        while !mutated {
            let i2 = rng.gen_range(0..n);
            if i1 != i2 {
                r[i1] = v[i2].clone();
                r[i2] = v[i1].clone();
                mutated = true;
            }
        }
    } else {
        // move musician
        let mut count = 100;
        while !mutated  && count > 0 {
            let i1 = rng.gen_range(0..n);
            let xoffs = rng.gen_range(0..11) as f64 - 5.0;
            let yoffs = rng.gen_range(0..11) as f64 - 5.0;
            let np = r[i1].0.add(vector(xoffs, yoffs));
            if on_stage(problem, &np) && distance_to_others_ok(&np, i1, &r) {
                r[i1] = (np, compute_los(problem, &r, i1));
                mutated = true;
            }
            count = count - 1;
        }
    }
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

pub fn solve(problem: &Problem) -> (f64, Vec<Point<f64>>) {
    let timeout = Duration::from_secs(100);
    let start = Instant::now();
    let r = make_positions(problem);
    let mut ar = annotate_with_los(problem, &r);
    let mut s = score(problem, &ar);
    println!("Initial score: {}", s);
    let mut perms: u64 = 1;
    while start.elapsed() < timeout {
        perms = perms + 1;
        let r2 = mutate(problem, &ar);
        let s2 = score(problem, &r2);
        if s2 > s {
            ar = r2;
            s = s2;
        }
    }
    println!("{} mutations tested. Final score: {}", perms, s);
    (s, ar.iter().map(|x| x.0).collect())
}
