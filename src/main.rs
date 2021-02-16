#![cfg_attr(test, feature(test))]

mod units;
mod engine;
mod tests;

use std::time::*;
use crate::units::{Unit, UnitType};

#[cfg(test)]
mod benches {
    extern crate test;
    use self::test::{Bencher};
    use super::{engine, Unit, UnitType};

    #[bench]
    fn test_optim(b: &mut Bencher) {
        let attackers = vec![Unit::from(UnitType::Defender(15)); 5];
        let def = Unit::from(UnitType::Defender(15));
        b.iter(
            || engine::optim(
                &attackers,
                &def
            )
        );
    }
}

fn main() {
    for i in 1..6 {
        let count = 10;
        let mut total = Duration::new(0, 0);
        for _ in 0..count {
            let unit = Unit::from(UnitType::Defender(15));

            let attackers: Vec<Unit> = vec![
                unit; i
            ];

            // let mut _attacker = Unit::from(UnitType::Defender(15));

            let defender = Unit::from(UnitType::Defender(15));
            let start = SystemTime::now();
            engine::optim(&attackers, &defender);
            total += SystemTime::now().duration_since(start).unwrap();
        }
        println!("{:?} iterations of optim, {} attackers; took {:?}", count, i, total / count)
    }
}
