use std::fmt::{Display, Formatter};
use rand::Rng;
use crate::formulaic_dice_roll::{DiceRollEquationNode, parse_equation, tokenize_equation};

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Ord, PartialEq, PartialOrd, Eq)]
pub struct NextId {
    id: usize,
}

impl NextId {
    pub fn new() -> Self {
        Self { id: 0 }
    }

    pub fn next(&mut self) -> usize {
        self.id += 1;
        self.id
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Ord, PartialEq, PartialOrd, Eq)]
/// Everything needed to render a dice menu windows.
///
/// Properties:
///
/// * `amount`: The number of dice to roll.
/// * `size`: The size of the dice.
/// * `id`: The id of the menu. This is used to identify the menu when it is displayed.
/// * `sort`: bool - This is a boolean value that determines whether or not the dice results are sorted.
/// * `modifier`: The modifier to add to the roll.
/// * `note`: This is a string that will be displayed with the results of the roll.
/// * `rolls`: a roll history
pub struct DiceMenu {
    pub amount: usize,
    pub raw_formula: String,
    pub formula: Option<Result<DiceRollEquationNode, String>>,
    pub id: usize,
    pub sort: bool,
    // dice_results: Vec<usize>,
    pub note: String,
    pub rolls: Vec<Vec<i64>>,
}

impl DiceMenu {
    pub fn parse_formula(&mut self) -> Result<(), String> {
        self.formula = Some(parse_equation(&tokenize_equation(&self.raw_formula)?));
        Ok(())
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Ord, PartialEq, PartialOrd, Eq)]
/// A note menu.
///
/// Properties:
///
/// * `id`: The id of the note.
/// * `text`: The text of the note.
/// * `displayed`: This is a boolean value that indicates whether the note is displayed or not.
pub struct Note {
    pub id: usize,
    pub text: String,
    pub displayed: bool,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Ord, PartialEq, PartialOrd, Eq)]
pub enum Interface {
    DiceRolling,
    CreatureCreation,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Ord, PartialEq, PartialOrd, Eq)]
/// A `Place` is a place that contains a `Vec` of `Creature`s that can be contained in that place.
///
/// Properties:
///
/// * `name`: A String that holds the name of the place.
/// * `creatures`: A vector of Creature objects.
pub struct Place {
    pub(crate) name: String,
    pub(crate) creatures: Vec<Creature>,
}

#[derive(
    serde::Deserialize, serde::Serialize, Debug, Clone, Ord, PartialEq, PartialOrd, Eq, Default,
)]
/// `Creature` is a struct with 10 fields, one of which is a vector of `Skill`s and another of which is
/// a vector of `Spell`s. this contains all the information that is needed for a creature
///
/// Properties:
///
/// * `size`: The size of the creature.
/// * `_type`: The type of creature. This is used to determine what kind of creature it is.
/// * `lv`: Level
/// * `hp`: Health points.
/// * `strength`: How much damage the creature can do with physical attacks.
/// * `speed`: How fast the creature is.
/// * `int`: Intelligence
/// * `mana`: The amount of mana the creature has.
/// * `vit`: Vitality, or health.
/// * `name`: The name of the creature.
/// * `skills`: A vector of Skills that the creature has.
/// * `spells`: A vector of spells that the creature can cast.
pub struct Creature {
    pub size: Size,
    pub danger: DangerRating,
    pub _type: String,
    pub lv: i32,
    pub hp: i32,
    pub strength: i32,
    pub speed: i32,
    pub int: i32,
    pub mana: i32,
    pub vit: i32,
    pub name: String,
    pub skills: Vec<Skill>,
    pub spells: Vec<Spell>,
    pub notes: Vec<String>,
}

#[derive(
serde::Deserialize, serde::Serialize, Debug, Clone, Ord, PartialEq, PartialOrd, Eq, Default,
)]
pub enum DangerRating {
    Easy,
    #[default]
    Normal,
    Hard,
    Dangerous,
    Difficult,
}

impl DangerRating {
    pub fn randomize() -> Self {
        let mut rng = rand::thread_rng();
        let rand_num = rng.gen_range(0..5);
        match rand_num {
            0 => Self::Easy,
            1 => Self::Normal,
            2 => Self::Hard,
            3 => Self::Dangerous,
            4 => Self::Difficult,
            _ => panic!("Invalid random number"),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Ord, PartialEq, PartialOrd, Eq)]
pub struct CreatureMenu {
    pub inner: Creature,
    pub id: usize,
    pub max_value: i32,
    pub selected_place_index: usize,
    pub editing: Option<(usize, usize)>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Ord, PartialEq, PartialOrd, Eq)]
/// A Skill that a creature has.
///
/// Properties:
///
/// * `name`: The name of the skill.
/// * `min_max`: a tuple with the minimum and maximum values.
pub struct Skill {
    pub name: String,
    pub min_max: (i32, i32),
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Ord, PartialEq, PartialOrd, Eq)]
/// A spell that a creature has.
///
/// Properties:
///
/// * `name`: The name of the spell.
/// * `min_max`: a tuple with the minimum and maximum values.
pub struct Spell {
    pub name: String,
    pub min_max: (i32, i32),
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Ord, PartialEq, PartialOrd, Eq, Clone)]
/// The size of a Creature.
pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
}

impl Creature {
    pub(crate) fn randomize(max_value: i32) -> Creature {
        let mut rng = rand::thread_rng();
        Creature {
            size: Size::randomize(),
            danger: DangerRating::randomize(),
            _type: String::from("Humanoid"),
            lv: rng.gen_range(1..=max_value),
            hp: rng.gen_range(1..=max_value),
            strength: rng.gen_range(1..=max_value),
            speed: rng.gen_range(1..=max_value),
            int: rng.gen_range(1..=max_value),
            mana: rng.gen_range(1..=max_value),
            vit: rng.gen_range(1..=max_value),
            name: String::from(""),
            skills: vec![],
            spells: vec![],
            notes: vec![],
        }
    }
    // ut in the level range - it then takes the values above 1 and uses it as "points" to add onto the other stats randomly. 1 point = to one value change. So it does all this when I press randomize for the level. I just would need to set the maximum value for the stats first.
    pub(crate) fn randomise_based_on_lvl(&mut self, max_value: i32)  {
        let mut rng = rand::thread_rng();
        let mut points = self.lv.clone();
        let mut stats = vec![
            &mut self.hp,
            &mut self.strength,
            &mut self.speed,
            &mut self.int,
            &mut self.mana,
            &mut self.vit,
        ];
        for stat in stats.iter_mut() {
            **stat = 1;
        }
        while points > 0 {
            let stat = rng.gen_range(0..stats.len());
            if stats[stat] == &max_value {
                if stats.iter().all(|x| *x == &max_value) {
                    break;
                }
                continue;
            }
            *stats[stat] += 1;
            points -= 1;
        }
    }
}

impl Size {
    fn randomize() -> Size {
        let mut rng = rand::thread_rng();
        let size = rng.gen_range(0..6);

        match size {
            0 => Size::Tiny,
            1 => Size::Small,
            2 => Size::Medium,
            3 => Size::Large,
            4 => Size::Huge,
            5 => Size::Gargantuan,
            _ => panic!("No size found"),
        }
    }
}

impl Default for Size {
    fn default() -> Self {
        Self::Medium
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string());
        Ok(())
    }
}
