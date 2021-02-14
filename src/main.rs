#![allow(dead_code)]

mod units;
mod engine;

use std::time::*;
use crate::units::{Unit, UnitType};

fn main() {
    for i in 1..6 {
        let count = 10;
        let mut total = Duration::new(0, 0);
        for _ in 0..count {
            let unit = Unit::from(UnitType::Defender(15));

            let attackers: Vec<Unit> = vec![
                unit; i
            ];

            let mut _attacker = Unit::from(UnitType::Defender(15));

            let defender = Unit::from(UnitType::Defender(15));
            let start = SystemTime::now();
            engine::optim(&attackers, &defender);
            total += SystemTime::now().duration_since(start).unwrap();
        }
        println!("{:?} iterations of optim, {} attackers; took {:?}", count, i, total / count)
    }
}
