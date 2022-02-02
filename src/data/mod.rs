use rand::{thread_rng, Rng};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Race {
    pub name: String,
    pub icons: Vec<String>,
    pub faction: String,
    pub classes: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct WoWClass {
    pub name: String,
    pub icon: String,
    pub specs: Vec<String>,
}

pub fn get_classes() -> HashMap<&'static str, WoWClass> {
    HashMap::from([
        (
            "warrior",
            WoWClass {
                name: String::from("Warrior"),
                icon: String::from("0px 0px"),
                specs: vec![
                    String::from("Arms"),
                    String::from("Fury"),
                    String::from("Protection"),
                ],
            },
        ),
        (
            "mage",
            WoWClass {
                name: String::from("Mage"),
                icon: String::from("-75px -75px"),
                specs: vec![
                    String::from("Arcane"),
                    String::from("Frost"),
                    String::from("Fire"),
                ],
            },
        ),
        (
            "deathKnight",
            WoWClass {
                name: String::from("Death Knight"),
                icon: String::from("-380px -75px"),
                specs: vec![
                    String::from("Blood"),
                    String::from("Frost"),
                    String::from("Unholy"),
                ],
            },
        ),
        (
            "demonHunter",
            WoWClass {
                name: String::from("Demon Hunter"),
                icon: String::from("-300px -75px"),
                specs: vec![String::from("Havoc"), String::from("Vengeance")],
            },
        ),
        (
            "druid",
            WoWClass {
                name: String::from("Druid"),
                icon: String::from("-225px -75px"),
                specs: vec![
                    String::from("Balance"),
                    String::from("Feral"),
                    String::from("Guardian"),
                    String::from("Restoration"),
                ],
            },
        ),
        (
            "hunter",
            WoWClass {
                name: String::from("Hunter"),
                icon: String::from("-150px -75px"),
                specs: vec![
                    String::from("Beast Mastery"),
                    String::from("Marksmanship"),
                    String::from("Survival"),
                ],
            },
        ),
        (
            "monk",
            WoWClass {
                name: String::from("Monk"),
                icon: String::from("0px -75px"),
                specs: vec![
                    String::from("Windwalker"),
                    String::from("Brewmaster"),
                    String::from("Mistweaver"),
                ],
            },
        ),
        (
            "paladin",
            WoWClass {
                name: String::from("Paladin"),
                icon: String::from("-383px 0px"),
                specs: vec![
                    String::from("Protection"),
                    String::from("Holy"),
                    String::from("Retribution"),
                ],
            },
        ),
        (
            "priest",
            WoWClass {
                name: String::from("Priest"),
                icon: String::from("-300px 0px"),
                specs: vec![
                    String::from("Holy"),
                    String::from("Shadow"),
                    String::from("Discipline"),
                ],
            },
        ),
        (
            "rogue",
            WoWClass {
                name: String::from("Rogue"),
                icon: String::from("-225px 0px"),
                specs: vec![
                    String::from("Subtlety"),
                    String::from("Assassination"),
                    String::from("Outlaw"),
                ],
            },
        ),
        (
            "shaman",
            WoWClass {
                name: String::from("Shaman"),
                icon: String::from("-150px 0px"),
                specs: vec![
                    String::from("Elemental"),
                    String::from("Enhancement"),
                    String::from("Restoration"),
                ],
            },
        ),
        (
            "warlock",
            WoWClass {
                name: String::from("Warlock"),
                icon: String::from("-75px 0px"),
                specs: vec![
                    String::from("Demonology"),
                    String::from("Destruction"),
                    String::from("Affliction"),
                ],
            },
        ),
    ])
}

pub fn get_races() -> HashMap<&'static str, Race> {
    HashMap::from([
        (
            "darkIronDwarf",
            Race {
                name: String::from("Dark Iron Dwarf"),
                icons: vec![String::from("-485px -160px"), String::from("-560px -160px")],
                faction: String::from("Alliance"),
                classes: vec![
                    String::from("deathKnight"),
                    String::from("hunter"),
                    String::from("mage"),
                    String::from("monk"),
                    String::from("paladin"),
                    String::from("priest"),
                    String::from("rogue"),
                    String::from("shaman"),
                    String::from("warlock"),
                    String::from("warrior"),
                ],
            },
        ),
        (
            "draenei",
            Race {
                name: String::from("Draenei"),
                icons: vec![String::from("0px -240px"), String::from("-80px -240px")],
                faction: String::from("Alliance"),
                classes: vec![
                    String::from("deathKnight"),
                    String::from("hunter"),
                    String::from("mage"),
                    String::from("monk"),
                    String::from("priest"),
                    String::from("paladin"),
                    String::from("shaman"),
                    String::from("warrior"),
                ],
            },
        ),
        (
            "dwarf",
            Race {
                name: String::from("Dwarf"),
                icons: vec![String::from("-161px -240px"), String::from("-238px -240px")],
                faction: String::from("Alliance"),
                classes: vec![
                    String::from("deathKnight"),
                    String::from("hunter"),
                    String::from("mage"),
                    String::from("monk"),
                    String::from("paladin"),
                    String::from("priest"),
                    String::from("rogue"),
                    String::from("shaman"),
                    String::from("warlock"),
                    String::from("warrior"),
                ],
            },
        ),
        (
            "gnome",
            Race {
                name: String::from("Gnome"),
                icons: vec![String::from("-320px -240px"), String::from("-400px -240px")],
                faction: String::from("Alliance"),
                classes: vec![
                    String::from("deathKnight"),
                    String::from("hunter"),
                    String::from("mage"),
                    String::from("monk"),
                    String::from("priest"),
                    String::from("rogue"),
                    String::from("warlock"),
                    String::from("warrior"),
                ],
            },
        ),
        (
            "human",
            Race {
                name: String::from("Human"),
                icons: vec![String::from("-480px -240px"), String::from("-558px -240px")],
                faction: String::from("Alliance"),
                classes: vec![
                    String::from("deathKnight"),
                    String::from("hunter"),
                    String::from("mage"),
                    String::from("monk"),
                    String::from("paladin"),
                    String::from("priest"),
                    String::from("rogue"),
                    String::from("warlock"),
                    String::from("warrior"),
                ],
            },
        ),
        (
            "kulTiran",
            Race {
                name: String::from("Kul Tiran"),
                icons: vec![String::from("0px -320px"), String::from("-80px -320px")],
                faction: String::from("Alliance"),
                classes: vec![
                    String::from("deathKnight"),
                    String::from("druid"),
                    String::from("hunter"),
                    String::from("mage"),
                    String::from("monk"),
                    String::from("priest"),
                    String::from("rogue"),
                    String::from("shaman"),
                    String::from("warrior"),
                ],
            },
        ),
        (
            "lightforgedDraenei",
            Race {
                name: String::from("Lightforged Draenei"),
                icons: vec![String::from("-161px -320px"), String::from("-238px -320px")],
                faction: String::from("Alliance"),
                classes: vec![
                    String::from("deathKnight"),
                    String::from("hunter"),
                    String::from("mage"),
                    String::from("paladin"),
                    String::from("priest"),
                    String::from("warrior"),
                ],
            },
        ),
        (
            "mechagnome",
            Race {
                name: String::from("Mechagnome"),
                icons: vec![String::from("-320px -320px"), String::from("-400px -320px")],
                faction: String::from("Alliance"),
                classes: vec![
                    String::from("deathKnight"),
                    String::from("hunter"),
                    String::from("mage"),
                    String::from("monk"),
                    String::from("priest"),
                    String::from("rogue"),
                    String::from("warlock"),
                    String::from("warrior"),
                ],
            },
        ),
        (
            "nightElf",
            Race {
                name: String::from("Night Elf"),
                icons: vec![String::from("0px -400px"), String::from("-80px -400px")],
                faction: String::from("Alliance"),
                classes: vec![
                    String::from("deathKnight"),
                    String::from("demonHunter"),
                    String::from("druid"),
                    String::from("hunter"),
                    String::from("mage"),
                    String::from("monk"),
                    String::from("priest"),
                    String::from("rogue"),
                    String::from("warrior"),
                ],
            },
        ),
        (
            "voidElf",
            Race {
                name: String::from("Void Elf"),
                icons: vec![String::from("0px -160px"), String::from("-80px -160px")],
                faction: String::from("Alliance"),
                classes: vec![
                    String::from("deathKnight"),
                    String::from("hunter"),
                    String::from("mage"),
                    String::from("monk"),
                    String::from("priest"),
                    String::from("rogue"),
                    String::from("warlock"),
                    String::from("warrior"),
                ],
            },
        ),
        (
            "worgen",
            Race {
                name: String::from("Worgen"),
                icons: vec![String::from("-160px -400px"), String::from("-240px -400px")],
                faction: String::from("Alliance"),
                classes: vec![
                    String::from("deathKnight"),
                    String::from("druid"),
                    String::from("hunter"),
                    String::from("mage"),
                    String::from("priest"),
                    String::from("rogue"),
                    String::from("warlock"),
                    String::from("warrior"),
                ],
            },
        ),
        (
            "bloodElf",
            Race {
                name: String::from("Blood Elf"),
                icons: vec![String::from("0px 0px"), String::from("-76px 0px")],
                faction: String::from("Horde"),
                classes: vec![
                    String::from("deathKnight"),
                    String::from("demonHunter"),
                    String::from("hunter"),
                    String::from("mage"),
                    String::from("monk"),
                    String::from("paladin"),
                    String::from("priest"),
                    String::from("rogue"),
                    String::from("warlock"),
                    String::from("warrior"),
                ],
            },
        ),
        (
            "goblin",
            Race {
                name: String::from("Goblin"),
                icons: vec![String::from("-161px 0px"), String::from("-238px 0px")],
                faction: String::from("Horde"),
                classes: vec![
                    String::from("deathKnight"),
                    String::from("hunter"),
                    String::from("mage"),
                    String::from("priest"),
                    String::from("rogue"),
                    String::from("shaman"),
                    String::from("warlock"),
                    String::from("warrior"),
                ],
            },
        ),
        (
            "highmountainTauren",
            Race {
                name: String::from("Highmountain Tauren"),
                icons: vec![String::from("-320px 0px"), String::from("-400px 0px")],
                faction: String::from("Horde"),
                classes: vec![
                    String::from("deathKnight"),
                    String::from("druid"),
                    String::from("hunter"),
                    String::from("mage"),
                    String::from("shaman"),
                    String::from("warrior"),
                ],
            },
        ),
        (
            "magharOrc",
            Race {
                name: String::from("Mag'har Orc"),
                icons: vec![String::from("-478px 0px"), String::from("-556px 0px")],
                faction: String::from("Horde"),
                classes: vec![
                    String::from("deathKnight"),
                    String::from("hunter"),
                    String::from("mage"),
                    String::from("monk"),
                    String::from("priest"),
                    String::from("rogue"),
                    String::from("shaman"),
                    String::from("warrior"),
                ],
            },
        ),
        (
            "nightborne",
            Race {
                name: String::from("Nightborne"),
                icons: vec![String::from("-480px -320px"), String::from("-558px -320px")],
                faction: String::from("Horde"),
                classes: vec![
                    String::from("deathKnight"),
                    String::from("hunter"),
                    String::from("mage"),
                    String::from("monk"),
                    String::from("priest"),
                    String::from("rogue"),
                    String::from("warlock"),
                    String::from("warrior"),
                ],
            },
        ),
        (
            "orc",
            Race {
                name: String::from("Orc"),
                icons: vec![String::from("0px -80px"), String::from("-80px -80px")],
                faction: String::from("Horde"),
                classes: vec![
                    String::from("deathKnight"),
                    String::from("hunter"),
                    String::from("mage"),
                    String::from("monk"),
                    String::from("rogue"),
                    String::from("shaman"),
                    String::from("warlock"),
                    String::from("warrior"),
                ],
            },
        ),
        (
            "tauren",
            Race {
                name: String::from("Tauren"),
                icons: vec![String::from("-161px -80px"), String::from("-238px -80px")],
                faction: String::from("Horde"),
                classes: vec![
                    String::from("deathKnight"),
                    String::from("druid"),
                    String::from("hunter"),
                    String::from("monk"),
                    String::from("paladin"),
                    String::from("priest"),
                    String::from("shaman"),
                    String::from("warrior"),
                ],
            },
        ),
        (
            "troll",
            Race {
                name: String::from("Troll"),
                icons: vec![String::from("-320px -80px"), String::from("-400px -80px")],
                faction: String::from("Horde"),
                classes: vec![
                    String::from("deathKnight"),
                    String::from("druid"),
                    String::from("hunter"),
                    String::from("mage"),
                    String::from("monk"),
                    String::from("priest"),
                    String::from("rogue"),
                    String::from("shaman"),
                    String::from("warlock"),
                    String::from("warrior"),
                ],
            },
        ),
        (
            "undead",
            Race {
                name: String::from("Undead"),
                icons: vec![String::from("-478px -80px"), String::from("-558px -80px")],
                faction: String::from("Horde"),
                classes: vec![
                    String::from("deathKnight"),
                    String::from("hunter"),
                    String::from("mage"),
                    String::from("monk"),
                    String::from("priest"),
                    String::from("rogue"),
                    String::from("warlock"),
                    String::from("warrior"),
                ],
            },
        ),
        (
            "vulpera",
            Race {
                name: String::from("Vulpera"),
                icons: vec![String::from("-160px -160px"), String::from("-242px -160px")],
                faction: String::from("Horde"),
                classes: vec![
                    String::from("deathKnight"),
                    String::from("hunter"),
                    String::from("mage"),
                    String::from("monk"),
                    String::from("priest"),
                    String::from("rogue"),
                    String::from("shaman"),
                    String::from("warlock"),
                    String::from("warrior"),
                ],
            },
        ),
        (
            "zandalariTroll",
            Race {
                name: String::from("Zandalari Troll"),
                icons: vec![String::from("-320px -160px"), String::from("-400px -160px")],
                faction: String::from("Horde"),
                classes: vec![
                    String::from("deathKnight"),
                    String::from("druid"),
                    String::from("hunter"),
                    String::from("mage"),
                    String::from("monk"),
                    String::from("paladin"),
                    String::from("priest"),
                    String::from("rogue"),
                    String::from("shaman"),
                    String::from("warrior"),
                ],
            },
        ),
        (
            "pandaren",
            Race {
                name: String::from("Pandaren"),
                icons: vec![String::from("-320px -400px"), String::from("-400px -400px")],
                faction: String::from("Both"),
                classes: vec![
                    String::from("deathKnight"),
                    String::from("hunter"),
                    String::from("mage"),
                    String::from("monk"),
                    String::from("priest"),
                    String::from("rogue"),
                    String::from("shaman"),
                    String::from("warrior"),
                ],
            },
        ),
    ])
}

#[allow(dead_code)]
pub fn get_races_keys() -> Vec<&'static str> {
    get_races().keys().cloned().collect()
}

pub fn get_random_race() -> Race {
    let vec = get_races().values().cloned().collect::<Vec<Race>>();

    let mut rng = thread_rng();
    let index = rng.gen_range(0..vec.len());

    vec[index].clone()
}

pub fn get_random_icon(race: &Race) -> String {
    let mut rng = thread_rng();
    let index = rng.gen_range(0..race.icons.len());

    race.icons[index].clone()
}

pub fn get_random_class(race: &Race) -> WoWClass {
    let mut rng = thread_rng();
    let index = rng.gen_range(0..race.classes.len());

    get_classes().get(&*race.classes[index]).unwrap().clone()
}

pub fn get_random_spec(class: &WoWClass) -> String {
    let mut rng = thread_rng();
    let index = rng.gen_range(0..class.specs.len());

    class.specs[index].clone()
}
