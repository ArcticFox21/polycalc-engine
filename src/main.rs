#![cfg_attr(test, feature(test))]

mod engine;
mod units;

use crate::units::{Unit, UnitType};
use std::time::*;
use std::process::exit;
use crate::engine::Solution;

#[cfg(test)]
mod benches {
    extern crate test;
    use self::test::Bencher;
    use super::{engine, Unit, UnitType};

    #[bench]
    fn test_optim(b: &mut Bencher) {
        let attackers = &[Unit::from(UnitType::Defender(15)); 5];
        let def = Unit::from(UnitType::Defender(15));
        b.iter(|| engine::optim(&attackers, &def));
    }
}


fn test_calc() {
    let attackers = vec![
        Unit::from(UnitType::Warrior(9)),
        Unit::from(UnitType::Defender(13)),
        Unit::from(UnitType::Gaami(17)),
        Unit::from(UnitType::Navalon(21)),
        Unit::from(UnitType::Archer(11)),
    ];
    let defender = Unit::from(UnitType::Giant(40));

    let actual_solution = engine::calc(&attackers.as_slice(), &defender);

    println!("{:?}", actual_solution);
}


fn test_optim() {
    let attackers = vec![
        Unit::from(UnitType::Warrior(9)),
        Unit::from(UnitType::Defender(13)),
        Unit::from(UnitType::Gaami(17)),
        Unit::from(UnitType::Navalon(21)),
        Unit::from(UnitType::Archer(11)),
    ];

    let defender = Unit::from(UnitType::Giant(40));

    let actual_solution = engine::optim(&attackers.as_slice(), &defender);

    let expected_solution = engine::OptimSolution {
        attackerlist: vec![
            Unit::from(UnitType::Warrior(0)),
            Unit::from(UnitType::Gaami(6)),
            Unit::from(UnitType::Navalon(12)),
            Unit::from(UnitType::Archer(11)),
            Unit::from(UnitType::Defender(1)),
        ],
        defender: Unit::from(UnitType::Giant(15)),
    };

    println!("Expected: {:?}\nActual: {:?}", expected_solution, actual_solution);
}

fn main() {
    test_calc();

    exit(0);

    for i in 1..10 {
        let count = 10;
        let mut total = Duration::new(0, 0);
        for _ in 0..count {
            let unit = Unit::from(UnitType::Defender(15));

            let attackers = vec![unit; i];

            // let mut _attacker = Unit::from(UnitType::Defender(15));

            let defender = Unit::from(UnitType::Defender(15));
            let start = SystemTime::now();
            engine::optim(&attackers.as_slice(), &defender);
            total += SystemTime::now().duration_since(start).unwrap();
        }
        println!(
            "{:?} iterations of optim, {} attackers; took {:?}",
            count,
            i,
            total / count
        )
    }
}

// ActualSolution { attackerlist: [Unit { unit_type: Warrior(0), health: 0, max_health: 10, vet: false, can_vet: true, attack: 2.0, defence: 2, defence_bonus: 1, ranged: false, retaliation: true }, Unit { unit_type: Gaami(6), health: 6, max_health: 30, vet: false, can_vet: false, attack: 4.0, defence: 4, defence_bonus: 1, ranged: false, retaliation: true }, Unit { unit_type: Navalon(12), health: 12, max_health: 30, vet: false, can_vet: false, attack: 4.0, defence: 4, defence_bonus: 1, ranged: false, retaliation: true }, Unit { unit_type: Archer(11), health: 11, max_health: 10, vet: true, can_vet: true, attack: 2.0, defence: 1, defence_bonus: 1, ranged: true, retaliation: true }, Unit { unit_type: Defender(1), health: 1, max_health: 15, vet: false, can_vet: true, attack: 1.0, defence: 3, defence_bonus: 1, ranged: false, retaliation: true }], defender: Unit { unit_type: Giant(15), health: 15, max_health: 40, vet: false, can_vet: false, attack: 5.0, defence: 4, defence_bonus: 1, ranged: false, retaliation: true } }
