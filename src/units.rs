#![allow(dead_code)]

#[derive(Debug, Clone)]
pub enum UnitType {
    Unknown,
    Warrior(i16),
    Rider(i16),
    Archer(i16),
    Defender(i16),
    Knight(i16),
    Swordsman(i16),
    Catapult(i16),
    Giant(i16),
    Crab(i16),
    Tridention(i16),
    Polytaur(i16),
    Navalon(i16),
    Gaami(i16),
    MindBender(i16),
    DragonEgg(i16),
    DragonBaby(i16),
    DragonFire(i16),
    BattleSled(i16),
    IceFortress(i16),
    Mooni(i16),
    NatureBunny(i16),
}

impl UnitType {
    pub fn set_health(&mut self, health: i16) {
        match self {
            UnitType::Unknown => {}
            UnitType::Warrior(hp)
            | UnitType::Rider(hp)
            | UnitType::Archer(hp)
            | UnitType::Defender(hp)
            | UnitType::Knight(hp)
            | UnitType::Swordsman(hp)
            | UnitType::Catapult(hp)
            | UnitType::Giant(hp)
            | UnitType::Crab(hp)
            | UnitType::Tridention(hp)
            | UnitType::Polytaur(hp)
            | UnitType::Navalon(hp)
            | UnitType::Gaami(hp)
            | UnitType::MindBender(hp)
            | UnitType::DragonEgg(hp)
            | UnitType::DragonBaby(hp)
            | UnitType::DragonFire(hp)
            | UnitType::BattleSled(hp)
            | UnitType::IceFortress(hp)
            | UnitType::Mooni(hp)
            | UnitType::NatureBunny(hp) => {
                // edit the values in-place
                *hp = health.clone();
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Unit {
    pub unit_type: UnitType,
    pub health: i16,
    pub max_health: i16,
    pub vet: bool,
    pub can_vet: bool,
    pub attack: f32,
    pub defence: i16,
    pub defence_bonus: i16,
    pub ranged: bool,
    pub retaliation: bool,
}

impl Unit {
    pub fn is_dead(&self) -> bool {
        self.health <= 0
    }
}

impl Default for Unit {
    fn default() -> Self {
        Unit {
            unit_type: UnitType::Unknown,
            health: 0,
            max_health: 0,
            vet: false,
            can_vet: true,
            attack: 0.0,
            defence: 0,
            defence_bonus: 1,
            ranged: false,
            retaliation: true,
        }
    }
}

impl From<UnitType> for Unit {
    fn from(unit: UnitType) -> Self {
        match unit {
            UnitType::Warrior(hp) => Unit {
                unit_type: UnitType::Warrior(hp),
                health: hp,
                max_health: 10,
                vet: if hp < 11 { false } else { true },
                attack: 2.0,
                defence: 2,
                ..Default::default()
            },
            UnitType::Rider(hp) => Unit {
                unit_type: UnitType::Rider(hp),
                health: hp,
                max_health: 10,
                vet: if hp < 11 { false } else { true },
                attack: 2.0,
                defence: 1,
                ..Default::default()
            },
            UnitType::Archer(hp) => Unit {
                unit_type: UnitType::Archer(hp),
                health: hp,
                max_health: 10,
                vet: if hp < 11 { false } else { true },
                attack: 2.0,
                defence: 1,
                ranged: true,
                ..Default::default()
            },
            UnitType::Defender(hp) => Unit {
                unit_type: UnitType::Defender(hp),
                health: hp,
                max_health: 15,
                vet: if hp < 16 { false } else { true },
                attack: 1.0,
                defence: 3,
                ..Default::default()
            },
            UnitType::Knight(hp) => Unit {
                unit_type: UnitType::Knight(hp),
                health: hp,
                max_health: 15,
                vet: if hp < 16 { false } else { true },
                attack: 3.5,
                defence: 1,
                ..Default::default()
            },
            UnitType::Swordsman(hp) => Unit {
                unit_type: UnitType::Swordsman(hp),
                health: hp,
                max_health: 15,
                vet: if hp < 16 { false } else { true },
                attack: 3.0,
                defence: 3,
                ..Default::default()
            },
            UnitType::Catapult(hp) => Unit {
                unit_type: UnitType::Catapult(hp),
                health: hp,
                max_health: 10,
                vet: if hp < 11 { false } else { true },
                attack: 4.0,
                defence: 0,
                ranged: true,
                ..Default::default()
            },
            UnitType::Giant(hp) => Unit {
                unit_type: UnitType::Giant(hp),
                health: hp,
                max_health: 40,
                vet: false,
                attack: 5.0,
                defence: 4,
                can_vet: false,
                ..Default::default()
            },
            UnitType::Crab(hp) => Unit {
                unit_type: UnitType::Crab(hp),
                health: hp,
                max_health: 40,
                vet: false,
                attack: 4.0,
                defence: 4,
                can_vet: false,
                ..Default::default()
            },
            UnitType::Tridention(hp) => Unit {
                unit_type: UnitType::Tridention(hp),
                health: hp,
                max_health: 15,
                vet: if hp < 16 { false } else { true },
                attack: 3.0,
                defence: 1,
                ranged: true,
                ..Default::default()
            },
            UnitType::Navalon(hp) => Unit {
                unit_type: UnitType::Navalon(hp),
                health: hp,
                max_health: 30,
                attack: 4.0,
                defence: 4,
                can_vet: false,
                ..Default::default()
            },
            UnitType::Gaami(hp) => Unit {
                unit_type: UnitType::Gaami(hp),
                health: hp,
                max_health: 30,
                vet: false,
                attack: 4.0,
                defence: 4,
                can_vet: false,
                ..Default::default()
            },
            UnitType::Polytaur(hp) => Unit {
                unit_type: UnitType::Polytaur(hp),
                health: hp,
                max_health: 15,
                vet: if hp < 16 { false } else { true },
                attack: 3.0,
                defence: 1,
                ..Default::default()
            },
            UnitType::DragonBaby(hp) => Unit {
                unit_type: UnitType::DragonBaby(hp),
                health: hp,
                max_health: 15,
                can_vet: false,
                attack: 3.0,
                defence: 3,
                ..Default::default()
            },
            UnitType::DragonEgg(hp) => Unit {
                unit_type: UnitType::DragonEgg(hp),
                health: hp,
                max_health: 10,
                can_vet: false,
                attack: 0.0,
                defence: 2,
                retaliation: false,
                ..Default::default()
            },
            UnitType::DragonFire(hp) => Unit {
                unit_type: UnitType::DragonFire(hp),
                health: hp,
                max_health: 20,
                vet: false,
                attack: 4.0,
                defence: 3,
                ranged: true,
                ..Default::default()
            },
            UnitType::MindBender(hp) => Unit {
                unit_type: UnitType::MindBender(hp),
                health: hp,
                max_health: 10,
                can_vet: false,
                attack: 0.0,
                defence: 1,
                retaliation: false,
                ..Default::default()
            },
            UnitType::BattleSled(hp) => Unit {
                unit_type: UnitType::BattleSled(hp),
                health: hp,
                max_health: 15,
                vet: if hp < 16 { false } else { true },
                attack: 3.0,
                defence: 2,
                ..Default::default()
            },
            UnitType::IceFortress(hp) => Unit {
                unit_type: UnitType::IceFortress(hp),
                health: hp,
                max_health: 20,
                vet: if hp < 21 { false } else { true },
                attack: 4.0,
                defence: 3,
                ranged: true,
                ..Default::default()
            },
            UnitType::NatureBunny(hp) => Unit {
                unit_type: UnitType::NatureBunny(hp),
                health: hp,
                max_health: 20,
                can_vet: false,
                attack: 5.0,
                defence: 1,
                ..Default::default()
            },
            UnitType::Mooni(hp) => Unit {
                unit_type: UnitType::Mooni(hp),
                health: hp,
                max_health: 10,
                can_vet: false,
                attack: 0.0,
                defence: 2,
                ..Default::default()
            },

            UnitType::Unknown => {
                panic!("Unknown unit type");
            }
        }
    }
}
