use std::time::{Duration, Instant};
use rand::Rng;
use crate::geometry::{Point, point};
use crate::intersect::line_circle_intersect;
use crate::problem::{Attendee, Problem};

fn make_positions(problem: &Problem, mut rows: u32) -> Vec<Point<f64>> {
    let n = problem.musicians.len();
    let mut positions = Vec::with_capacity(n);
    let wide = problem.stage_width >= problem.stage_height;
    let max_rows: u32;
    let max_per_row: u32;
    if wide {
        max_rows = problem.stage_width as u32 / 10 - 1;
        max_per_row = problem.stage_height as u32 / 10 - 1;
    } else {
        max_rows = problem.stage_height as u32 / 10 - 1;
        max_per_row = problem.stage_width as u32 / 10 - 1;
    }
    if rows > max_rows {
        println!("Stage too small: reducing rows from {} to {}.", rows, max_rows);
        rows = max_rows;
    }
    let mut n_per_row = (n as u32 + rows - 1) / rows;
    while n_per_row > max_per_row {
        if rows >= max_rows {
            panic!("Stage too small - bailing out.");
        }
        rows = rows + 1;
        n_per_row = (n as u32 + rows - 1) / rows;
    }
    println!("Placing {} musicians in {} rows: {} per row.", n, rows, n_per_row);
    if n_per_row * rows > n as u32 {
        println!("  (Last row only {})", n as u32 - n_per_row * (rows - 1));
    }
    let xoffs = problem.stage_bottom_left[0];
    let yoffs = problem.stage_bottom_left[1];
    if wide {
        let x_step = problem.stage_width / (n_per_row + 1) as f64;
        let y_step = problem.stage_height / (rows + 1) as f64;
        let mut y = y_step;
        for _r in 0..rows {
            let mut x = x_step;
            for _c in 0..n_per_row {
                positions.push(point(x+xoffs, y+yoffs));
                if positions.len() == n {
                    return positions;
                }
                x = x + x_step;
            }
            y = y + y_step;
        }
    } else {
        let x_step = problem.stage_width / (rows + 1) as f64;
        let y_step = problem.stage_height / (n_per_row + 1) as f64;
        let mut x = x_step;
        for _c in 0..rows {
            let mut y = y_step;
            for _r in 0..n_per_row {
                positions.push(point(x+xoffs, y+yoffs));
                if positions.len() == n {
                    return positions;
                }
                y = y + y_step;
            }
            x = x + x_step;
        }
    }
    positions
}

fn is_blocked(placements: &Vec<Point<f64>>, musician_index: usize, attendee: &Attendee) -> bool {
    let attendee_pos = point(attendee.x, attendee.y);
    let musician_pos = placements[musician_index];
    for (i,p) in placements.iter().enumerate() {
        if i != musician_index && line_circle_intersect(attendee_pos, musician_pos, *p, 5.0) {
            return true
        }
    }
    false
}

fn annotate_with_los(problem: &Problem, positions: &Vec<Point<f64>>) -> Vec<(Point<f64>, Vec<bool>)> {
    let mut result = Vec::with_capacity(positions.len());
    let na = problem.attendees.len();
    for (i, p) in positions.iter().enumerate() {
        let mut visible = Vec::with_capacity(na);
        for a in &problem.attendees {
            visible.push(!is_blocked(positions, i, a));
        }
        result.push((*p, visible));
    }
    result
}

fn happiness1(att: &Attendee, mus: Point<f64>, instrument: usize) -> f64 {
    let dx = att.x - mus.x;
    let dy = att.y - mus.y;
    let d2 = dx * dx + dy * dy;
    (1000000.0 * att.tastes[instrument] / d2).ceil()
}

fn happiness(attendee_index: usize, problem: &Problem, ann_placements: &Vec<(Point<f64>, Vec<bool>)>) -> f64 {
    let mut sum = 0.0;
    let ms = &problem.musicians;
    let a = &problem.attendees[attendee_index];
    for (k, place) in ann_placements.iter().enumerate() {
        if place.1[attendee_index] {
            let instrument = ms[k];
            sum = sum + happiness1(a, place.0, instrument as usize);
        }
    }
    sum
}

fn score(problem: &Problem, ann_placements: &Vec<(Point<f64>, Vec<bool>)>) -> f64 {
    if ann_placements.len() != problem.musicians.len() {
        panic!("Fatal error: wrong placements length. musicians: {}, placements: {}",
               problem.musicians.len(), ann_placements.len());
    }
    let mut sum = 0.0;
    for ai in 0 .. problem.attendees.len() {
        sum = sum + happiness(ai, problem, ann_placements);
    }
    sum
}

fn mutate(v: &Vec<(Point<f64>, Vec<bool>)>) -> Vec<(Point<f64>, Vec<bool>)> {
    let mut r = v.to_vec();
    let mut rng = rand::thread_rng();
    let n = v.len();
    let i1 = rng.gen_range(0..n);
    let i2 = rng.gen_range(0..n);
    if i1 != i2 {
        r[i1] = v[i2].clone();
        r[i2] = v[i1].clone();
    }
    r
}

pub fn solve_fixed(problem: &Problem) -> Vec<Point<f64>> {
    let timeout = Duration::from_secs(120);
    let start = Instant::now();
    let r = make_positions(problem, 2);
    println!("Precomputing line-of-sound...");
    let mut ar = annotate_with_los(problem, &r);
    let mut s = score(problem, &ar);
    println!("Initial score: {}", s);
    let mut perms: u64 = 1;
    while start.elapsed() < timeout {
        perms = perms + 1;
        let r2 = mutate(&ar);
        let s2 = score(problem, &r2);
        if s2 > s {
            ar = r2;
            s = s2;
        }
    }
    println!("{} mutations tested. Final score: {}", perms, s);
    ar.iter().map(|x| x.0).collect()
}