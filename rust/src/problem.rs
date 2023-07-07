use std::fs::File;
use std::io::BufReader;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use crate::geometry::Point;

#[derive(Deserialize, Debug)]
pub struct Attendee {
    pub x: f64,
    pub y: f64,
    pub tastes: Vec<f64>,
}

#[derive(Deserialize, Debug)]
pub struct Problem {
    pub room_width: f64,
    pub room_height: f64,
    pub stage_width: f64,
    pub stage_height: f64,
    pub stage_bottom_left: Vec<f64>,
    pub musicians: Vec<u32>,
    pub attendees: Vec<Attendee>,
}

#[derive(Serialize, Debug)]
pub struct Solution {
    pub placements: Vec<Point<f64>>,
}

impl Problem {

    pub fn from_file(f: &String) -> Result<Problem> {
        let file = File::open(f).unwrap();
        let reader = BufReader::new(file);
        let s: String = serde_json::from_reader(reader)?;
        let p= serde_json::from_str(&*s);
        p
    }

}