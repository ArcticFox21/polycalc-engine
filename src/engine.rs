#![allow(dead_code)]

use crate::units::Unit;
use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Clone, Default)]
pub struct Solution {
    pub attacker_health: i16,
    pub attacker_health_loss: i16,
    pub attacker_death_count: i16,
    pub defender_health: i16,
    pub defender_health_loss: i16,
    pub attacker_list: Vec<Unit>,
    pub defender: Unit,
}

pub struct OptimSolution {
    pub attackerlist: Vec<Unit>,
    pub defender: Unit,
}

pub fn attack(attacker: &mut Unit, defender: &mut Unit, solution: &mut Solution) {
    // If attacker has no attack, return.
    if attacker.attack <= 0.0 {
        return;
    }

    // Can't kill something if if's already dead...
    // plus, using negative numbers in the calculations seriously messes them up.
    if defender.health <= 0 {
        return;
    }

    // calculate attack force and defence force
    let attack_force = attacker.attack * (attacker.health as f32) / (attacker.max_health as f32);
    let defence_force: f32 = (defender.defence as f32 * defender.health as f32
        / defender.max_health as f32
        * defender.defence_bonus as f32) as f32;

    // calculate total_damage.
    let total_damage: f32 = attack_force + (defence_force as f32);

    // apply damage to the defender, and add to the total defender health loss.
    let damage = (attack_force / total_damage * attacker.attack * 4.5).floor() as i16;
    solution.defender_health_loss = solution.defender_health_loss + damage;
    defender.health = defender.health - damage;

    // only calculate damage to the attacker if retaliation should occur
    if attacker.retaliation {
        // apply damage to the attacker, add to the total attacker health loss
        let attacker_damage = defence_force as f32 / total_damage * defender.defence as f32 * 4.5;
        solution.attacker_health_loss =
            solution.attacker_health_loss + attacker_damage.ceil() as i16;
        attacker.health = attacker.health - attacker_damage.ceil() as i16;

        if attacker.health <= 0 {
            solution.attacker_death_count += 1;
        }
    }

    solution.attacker_list.push(attacker.clone());
    solution.defender = defender.clone();
}

pub fn calc(attackers: &Vec<&Unit>, def: &Unit) -> Solution {
    let mut solution = Solution {
        attacker_health_loss: 0,
        defender_health: def.health.clone(),
        defender_health_loss: 0,
        attacker_health: attackers.iter().map(|x| x.health).sum(),
        attacker_death_count: 0,
        attacker_list: Vec::new(),
        defender: def.clone(),
    };

    let mut defender = def.clone();

    for &attacker in attackers {
        let mut unit = Unit { ..attacker.clone() };
        attack(&mut unit, &mut defender, &mut solution);
    }

    solution
}

#[inline]
pub fn optim(attackers: &Vec<Unit>, defender: &Unit) -> OptimSolution {
    if attackers.len() > 10 {
        panic!("Too many units!");
    }

    let mut solutions = attackers
        .iter()
        .permutations(attackers.len())
        .map(|x| calc(&Vec::from(x), defender))
        .sorted_by(|a, b| {
            // 1. Order by defender health loss
            return if a.defender_health_loss > b.defender_health_loss {
                Ordering::Less
            } else if a.defender_health_loss == b.defender_health_loss {
                // 2. Order by attacker death count
                if a.attacker_death_count > b.attacker_death_count {
                    // If more attackers die than in the other solution, this solution is less awesome than the other
                    Ordering::Less
                } else if a.attacker_death_count == b.attacker_death_count {
                    // 3. Order by attacker health loss
                    if a.attacker_health_loss < b.attacker_health_loss {
                        Ordering::Less
                    } else if a.attacker_health_loss == b.attacker_health_loss {
                        // 4. Order by number of attackers required.
                        if a.attacker_list.len() < b.attacker_list.len() {
                            Ordering::Less
                        } else if a.attacker_list.len() == b.attacker_list.len() {
                            Ordering::Equal
                        } else {
                            Ordering::Greater
                        }
                    } else {
                        Ordering::Greater
                    }
                } else {
                    Ordering::Greater
                }
            } else {
                Ordering::Greater
            };
        });

    let best_solution: Solution = solutions.next().unwrap();

    OptimSolution {
        attackerlist: best_solution.attacker_list,
        defender: best_solution.defender,
    }
}
