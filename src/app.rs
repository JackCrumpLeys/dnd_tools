use self::egui::Color32;
use eframe::egui;
use eframe::egui::Id;
use rand::Rng;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use winreg::enums::RegDisposition;
use crate::formulaic_dice_roll::{DiceRollEquationNode, parse_equation, tokenize_equation};

// use ::egui::*;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
/// All of the app data needed to provide app functionality.
///
/// Properties:
///
/// * `places`: Vec<Place> - This is a vector of Place structs. Each Place struct represents a place in
/// the world.
/// * `selected_place_index`: The index of the place that is currently selected.
/// * `open_place_windows_indexes`: This is a vector of indexes of the places that are currently open.
/// * `open_interface`: This is the interface that is currently open.
/// * `dice_windows`: All of the dice windows that are currently open.
/// * `id_next`: The next id to be used to allocate a window id.,
/// * `notes`: A vector of all the notes that have been created.
pub struct DndTool {
    places: Vec<Place>,
    selected_place_index: usize,
    open_place_windows_indexes: Vec<usize>,
    open_interface: Interface,
    // creating_creatcher: bool
    creature_creation_windows: Vec<CreatureMenu>,
    dice_windows: Vec<DiceMenu>,
    id_next: NextId,
    notes: Vec<Note>,
}

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
    amount: usize,
    raw_formula: String,
    formula: Option<Result<DiceRollEquationNode, String>>,
    id: usize,
    sort: bool,
    // dice_results: Vec<usize>,
    note: String,
    rolls: Vec<Vec<i64>>,
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
    id: usize,
    text: String,
    displayed: bool,
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
    name: String,
    creatures: Vec<Creature>,
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
    size: Size,
    danger: DangerRating,
    _type: String,
    lv: i32,
    hp: i32,
    strength: i32,
    speed: i32,
    int: i32,
    mana: i32,
    vit: i32,
    name: String,
    skills: Vec<Skill>,
    spells: Vec<Spell>,
    notes: Vec<String>,
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
    inner: Creature,
    id: usize,
    max_value: i32,
    selected_place_index: usize,
    editing: Option<(usize, usize)>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Ord, PartialEq, PartialOrd, Eq)]
/// A Skill that a creature has.
///
/// Properties:
///
/// * `name`: The name of the skill.
/// * `min_max`: a tuple with the minimum and maximum values.
pub struct Skill {
    name: String,
    min_max: (i32, i32),
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Ord, PartialEq, PartialOrd, Eq)]
/// A spell that a creature has.
///
/// Properties:
///
/// * `name`: The name of the spell.
/// * `min_max`: a tuple with the minimum and maximum values.
pub struct Spell {
    name: String,
    min_max: (i32, i32),
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
    fn randomize(max_value: i32) -> Creature {
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

impl Default for DndTool {
    fn default() -> Self {
        Self {
            places: vec![
                Place {
                    name: "Scorched Lands".to_string(),
                    creatures: vec![],
                },
                Place {
                    name: "Desolate Lands".to_string(),
                    creatures: vec![],
                },
                Place {
                    name: "Hollow".to_string(),
                    creatures: vec![],
                },
                Place {
                    name: "Grove".to_string(),
                    creatures: vec![],
                },
                Place {
                    name: "Province".to_string(),
                    creatures: vec![],
                },
            ],
            selected_place_index: 1,
            open_place_windows_indexes: vec![],
            open_interface: Interface::DiceRolling,
            creature_creation_windows: vec![],
            dice_windows: vec![],
            id_next: NextId::new(),
            notes: vec![],
        }
    }
}

impl DndTool {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for DndTool {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self {
            places,
            selected_place_index,
            open_place_windows_indexes,
            open_interface,
            dice_windows,
            id_next,
            notes,
            creature_creation_windows,
            ..
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
                ui.menu_button("Interface", |ui| {
                    if ui.button("Dice").clicked() {
                        *open_interface = Interface::DiceRolling;
                    }
                    if ui.button("Creature creation").clicked() {
                        *open_interface = Interface::CreatureCreation;
                    }
                });
            });
        });

        match open_interface {
            /// Render the DiceRolling interface
            Interface::DiceRolling => {
                egui::SidePanel::left("side_panel").show(ctx, |ui| {
                    // ui.heading("Side Panel");
                    //
                    // ui.horizontal(|ui| {
                    //     ui.label("Write something: ");
                    //     ui.text_edit_singleline(label);
                    // });
                    //
                    // ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
                    // if ui.button("Increment").clicked() {
                    //     *value += 1.0;
                    // }

                    // egui::ComboBox::from_label("place:")
                    //     .selected_text(format!("{}", places[*selected_place_index].name))
                    //     .show_ui(ui, |ui| {
                    //         for place_index in 0..places.len() {
                    //             ui.selectable_value(
                    //                 selected_place_index,
                    //                 place_index.clone(),
                    //                 places[place_index].name.clone(),
                    //             );
                    //         }
                    //     });
                    //
                    // if ui.button("open place window").clicked() {
                    //     open_place_windows_indexes.push(*selected_place_index)
                    // }

                    if ui.button("open dice window").clicked() {
                        dice_windows.push(DiceMenu {
                            amount: 1,
                            raw_formula: "1d20".to_string(),
                            formula: None,
                            id: id_next.next(),
                            note: String::new(),
                            rolls: vec![],
                            sort: false,
                        });
                    }

                    // if ui.button("create note").clicked() {
                    //     notes.push(Note { amount: 1, size: 20, id: *dice_id_next, dice_results: vec![] });
                    //     *dice_id_next += 1;
                    // }

                    // ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    //     ui.horizontal(|ui| {
                    //         ui.spacing_mut().item_spacing.x = 0.0;
                    //         ui.label("powered by ");
                    //         ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    //         ui.label(" and ");
                    //         ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/eframe");
                    //     });
                    // });
                });

                egui::CentralPanel::default().show(ctx, |ui| {
                    // The central panel the region left after adding TopPanels and SidePanels

                    ui.heading("DnD tools");
                    egui::warn_if_debug_build(ui);
                });
                open_place_windows_indexes.sort();
                open_place_windows_indexes.dedup();

                let mut dice_windows_to_remove = vec![];
                for (i, dice_window) in dice_windows.into_iter().enumerate() {
                    egui::Window::new(format!(
                        "Roll {}",
                        dice_window.raw_formula
                    ))
                    .id(Id::new(format!("{}dice", &dice_window.id)))
                    .show(ctx, |ui| {
                        if ui.button("close window").clicked() {
                            dice_windows_to_remove.push(i);
                        }

                        ui.horizontal(|ui| {
                            ui.label("note:");
                            ui.text_edit_singleline(&mut dice_window.note);
                        });

                        ui.horizontal(|ui| {
                            ui.label("amount of times to roll:");
                            ui.add(egui::DragValue::new(&mut dice_window.amount));
                            if ui.text_edit_singleline(&mut dice_window.raw_formula).changed() {
                                dice_window.parse_formula();
                            }
                        });
                        
                        if dice_window.formula.is_some() {
                            if let Err(err) = dice_window.formula.as_ref().unwrap() {
                                ui.label(egui::RichText::new(format!("error: {}", err)).size(20.0).underline());
                            }
                        }

                        ui.checkbox(&mut dice_window.sort, "sort list");

                        if ui.button("Roll dice").clicked() {
                            let mut dice_results = vec![];
                            for _ in 0..dice_window.amount {
                                dice_results.push(
                                    if let Some(f) = &dice_window.formula {
                                        if let Ok(formula) = f {
                                            formula.evaluate()
                                        } else {
                                            0
                                        }
                                    } else {
                                        0
                                    }
                                );
                            }
                            dice_window.rolls.push(dice_results);
                        }

                        egui::ScrollArea::vertical().show(ui, |ui| {
                            if !dice_window.rolls.is_empty() {
                                let current_roll: &mut Vec<i64> =
                                    &mut dice_window.rolls.last().unwrap().clone();

                                if dice_window.sort {
                                    current_roll.sort();
                                }

                                ui.text_edit_multiline(&mut format!(
                                    "{:?}, sum: {}",
                                    current_roll,
                                    (current_roll.iter().sum::<i64>())
                                ));

                                if dice_window.rolls.len() != 1 {
                                    ui.collapsing("history", |ui| {
                                        egui::ScrollArea::vertical().show(ui, |ui| {
                                            for mut roll in
                                                dice_window.rolls.clone().into_iter().rev()
                                            {
                                                if dice_window.sort {
                                                    roll.sort();
                                                }

                                                ui.text_edit_multiline(&mut format!(
                                                    "{:?}, sum: {}",
                                                    roll,
                                                    (roll.iter().sum::<i64>()
                                                        )
                                                ));
                                            }
                                        });
                                    });
                                }

                                if dice_window.rolls.len() > 50 as usize {
                                    dice_window.rolls.remove(0);
                                }
                            }
                        });
                    });
                }

                for window in dice_windows_to_remove {
                    dice_windows.remove(window);
                }
            }
            Interface::CreatureCreation => {
                egui::SidePanel::left("side_panel").show(ctx, |ui| {
                    // TODO: put anything here

                    egui::ComboBox::from_label("place:")
                        .selected_text(format!("{}", places[*selected_place_index].name))
                        .show_ui(ui, |ui| {
                            for place_index in 0..places.len() {
                                ui.selectable_value(
                                    selected_place_index,
                                    place_index.clone(),
                                    places[place_index].name.clone(),
                                );
                            }
                        });

                    if ui.button("open place window").clicked() {
                        open_place_windows_indexes.push(*selected_place_index)
                    }

                    if ui.button("open creature window").clicked() {
                        creature_creation_windows.push(CreatureMenu {
                            inner: Default::default(),
                            id: id_next.next(),
                            max_value: 100,
                            selected_place_index: *selected_place_index,
                            editing: None,
                        });
                    }

                    ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 0.0;
                            ui.label("powered by ");
                            ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                            ui.label(" and ");
                            ui.hyperlink_to(
                                "eframe",
                                "https://github.com/emilk/egui/tree/master/eframe",
                            );
                        });
                    });

                    let mut creature_creation_windows_to_remove: Vec<usize> = vec![];
                    for (i, mut window) in &mut creature_creation_windows.into_iter().enumerate() {
                        egui::Window::new(format!(
                            "Creature creation window. {}",
                            window.inner.name
                        ))
                        .id(Id::new(format!("{}place", &window.id)))
                        .show(ctx, |ui| {
                            // create a window with all of the fields required to make a creature and an option to randomize any of the values

                            if ui.button("close window").clicked() {
                                creature_creation_windows_to_remove.push(i);
                            }

                            let max_value = &mut window.max_value;
                            // slider for max value for the values
                            ui.add(
                                egui::Slider::new(max_value, 2..=*max_value + 500)
                                    .text("max value"),
                            );

                            if ui
                                .button("Randomize; WARNING: RESETS NAME NOTES AND OTHER THINGS")
                                .clicked()
                            {
                                window.inner = Creature::randomize(*max_value);
                            }

                            let size = &mut window.inner.size;
                            let danger = &mut window.inner.danger;
                            let _type = &mut window.inner._type;
                            let lv = &mut window.inner.lv;
                            let hp = &mut window.inner.hp;
                            let strength = &mut window.inner.strength;
                            let speed = &mut window.inner.speed;
                            let int = &mut window.inner.int;
                            let mana = &mut window.inner.mana;
                            let vit = &mut window.inner.vit;
                            let name = &mut window.inner.name;
                            let skills = &mut window.inner.skills;
                            let spells = &mut window.inner.spells;
                            let notes = &mut window.inner.notes;

                            ui.horizontal(|ui| {
                                ui.label("Size:");
                                ui.selectable_value(size, Size::Tiny, "Tiny");
                                ui.selectable_value(size, Size::Small, "Small");
                                ui.selectable_value(size, Size::Medium, "Medium");
                                ui.selectable_value(size, Size::Large, "Large");
                                ui.selectable_value(size, Size::Huge, "Huge");
                                ui.selectable_value(size, Size::Gargantuan, "Gargantuan");
                            });

                            ui.horizontal(|ui| {
                                ui.label("Danger:");
                                ui.selectable_value(danger, DangerRating::Easy, "Easy");
                                ui.selectable_value(danger, DangerRating::Normal, "Normal");
                                ui.selectable_value(danger, DangerRating::Hard, "Hard");
                                ui.selectable_value(danger, DangerRating::Difficult, "Difficult");
                                ui.selectable_value(danger, DangerRating::Dangerous, "Dangerous");
                            });

                            ui.horizontal(|ui| {
                                ui.label("Type:");
                                ui.text_edit_singleline(_type);
                            });

                            ui.horizontal(|ui| {
                                ui.label("Level:");
                                ui.add(egui::Slider::new(lv, 1..=*max_value).text("lv"));
                                if ui.button("Randomize").clicked() {
                                    *lv = rand::thread_rng().gen_range(1..=*max_value);
                                }
                            });

                            ui.horizontal(|ui| {
                                ui.label("Health points:");
                                ui.add(egui::Slider::new(hp, 1..=*max_value).text("hp"));
                                if ui.button("Randomize").clicked() {
                                    *hp = rand::thread_rng().gen_range(1..=*max_value);
                                }
                            });

                            ui.horizontal(|ui| {
                                ui.label("Strength:");
                                ui.add(
                                    egui::Slider::new(strength, 1..=*max_value).text("strength"),
                                );
                                if ui.button("Randomize").clicked() {
                                    *strength = rand::thread_rng().gen_range(1..=*max_value);
                                }
                            });

                            ui.horizontal(|ui| {
                                ui.label("Speed:");
                                ui.add(egui::Slider::new(speed, 1..=*max_value).text("speed"));
                                if ui.button("Randomize").clicked() {
                                    *speed = rand::thread_rng().gen_range(1..=*max_value);
                                }
                            });

                            ui.horizontal(|ui| {
                                ui.label("Intelligence:");
                                ui.add(egui::Slider::new(int, 1..=*max_value).text("int"));
                                if ui.button("Randomize").clicked() {
                                    *int = rand::thread_rng().gen_range(1..=*max_value);
                                }
                            });

                            ui.horizontal(|ui| {
                                ui.label("Mana:");
                                ui.add(egui::Slider::new(mana, 1..=*max_value).text("mana"));
                                if ui.button("Randomize").clicked() {
                                    *mana = rand::thread_rng().gen_range(1..=*max_value);
                                }
                            });

                            ui.horizontal(|ui| {
                                ui.label("Vitality:");
                                ui.add(egui::Slider::new(vit, 1..=*max_value).text("vit"));
                                if ui.button("Randomize").clicked() {
                                    *vit = rand::thread_rng().gen_range(1..=*max_value);
                                }
                            });

                            ui.horizontal(|ui| {
                                ui.label("Name:");
                                ui.text_edit_singleline(name);
                            });

                            let mut skills_to_remove: Vec<usize> = vec![];
                            ui.horizontal(|ui| {
                                if ui.button("add skill").clicked() {
                                    skills.push(Skill {
                                        name: String::from(""),
                                        min_max: (0, 0),
                                    });
                                }
                                ui.label("Skills:");
                                ui.vertical(|ui| {
                                    for (i, skill) in skills.iter_mut().enumerate() {
                                        // format like NAME_EDIT: MIN - MAX (randomize) (remove)
                                        ui.horizontal(|ui| {
                                            ui.text_edit_singleline(&mut skill.name);

                                            ui.add(
                                                egui::DragValue::new(&mut skill.min_max.0)
                                                    .speed(1)
                                                    .clamp_range(0..=*max_value),
                                            );
                                            ui.label("-");
                                            ui.add(
                                                egui::DragValue::new(&mut skill.min_max.1)
                                                    .speed(1)
                                                    .clamp_range(0..=*max_value),
                                            );

                                            if ui.button("remove").clicked() {
                                                skills_to_remove.push(i);
                                            }

                                            if ui.button("randomize").clicked() {
                                                skill.min_max.0 =
                                                    rand::thread_rng().gen_range(0..=*max_value);
                                                skill.min_max.1 =
                                                    rand::thread_rng().gen_range(0..=*max_value);
                                            }
                                        });
                                    }
                                });
                            });
                            for i in skills_to_remove.into_iter().rev() {
                                skills.remove(i);
                            }

                            let mut spells_to_remove: Vec<usize> = vec![];
                            ui.horizontal(|ui| {
                                if ui.button("add spell").clicked() {
                                    spells.push(Spell {
                                        name: String::from(""),
                                        min_max: (0, 0),
                                    });
                                }
                                ui.label("Spells:");
                                ui.vertical(|ui| {
                                    for (i, spell) in spells.iter_mut().enumerate() {
                                        ui.horizontal(|ui| {
                                            ui.text_edit_singleline(&mut spell.name);
                                            ui.add(
                                                egui::DragValue::new(&mut spell.min_max.0)
                                                    .speed(1)
                                                    .clamp_range(0..=*max_value),
                                            );
                                            ui.label("-");
                                            ui.add(
                                                egui::DragValue::new(&mut spell.min_max.1)
                                                    .speed(1)
                                                    .clamp_range(0..=*max_value),
                                            );

                                            if ui.button("remove").clicked() {
                                                spells_to_remove.push(i);
                                            }

                                            if ui.button("randomize").clicked() {
                                                spell.min_max.0 =
                                                    rand::thread_rng().gen_range(0..=*max_value);
                                                spell.min_max.1 =
                                                    rand::thread_rng().gen_range(0..=*max_value);
                                            }
                                        });
                                    }
                                });
                            });
                            for i in spells_to_remove.into_iter().rev() {
                                spells.remove(i);
                            }

                            let mut notes_to_remove: Vec<usize> = vec![];
                            ui.horizontal(|ui| {
                                if ui.button("add note").clicked() {
                                    notes.push(String::from(""));
                                }
                                ui.label("Notes:");
                                ui.vertical(|ui| {
                                    for (i, note) in notes.iter_mut().enumerate() {
                                        ui.horizontal(|ui| {
                                            ui.text_edit_singleline(note);
                                            if ui.button("remove").clicked() {
                                                notes_to_remove.push(i);
                                            }
                                        });
                                    }
                                });
                            });
                            for i in notes_to_remove.into_iter().rev() {
                                notes.remove(i);
                            }

                            if ui.button("save").clicked() {
                                let creature = Creature {
                                    name: name.clone(),
                                    size: size.clone(),
                                    danger: Default::default(),
                                    _type: _type.clone(),
                                    lv: lv.clone(),
                                    hp: hp.clone(),
                                    strength: strength.clone(),
                                    speed: speed.clone(),
                                    int: int.clone(),
                                    mana: mana.clone(),
                                    vit: vit.clone(),
                                    skills: skills.clone(),
                                    spells: spells.clone(),
                                    notes: notes.clone(),
                                };
                                if let Some((place_idx, creature_idx)) = window.editing {
                                    places[place_idx].creatures[creature_idx] = creature;
                                } else {
                                    places[*selected_place_index].creatures.push(creature);
                                }

                                creature_creation_windows_to_remove.push(i);
                            }
                        });
                    }

                    for window in creature_creation_windows_to_remove {
                        creature_creation_windows.remove(window);
                    }

                    // place display menu
                    open_place_windows_indexes.sort();
                    open_place_windows_indexes.dedup();

                    let mut place_windows_to_remove: Vec<usize> = vec![];
                    for open_place_window_index in open_place_windows_indexes.clone() {
                        egui::Window::new(places[open_place_window_index].name.clone())
                            .id(Id::new(format!("{}place", &open_place_window_index)))
                            .show(ctx, |ui| {
                                let place = &mut places[open_place_window_index];
                                let mut name = &mut place.name;
                                let mut creatures = &mut place.creatures;
                                if ui.button("close window").clicked() {
                                    place_windows_to_remove.push(open_place_window_index);
                                }
                                ui.horizontal(|ui| {
                                    ui.label("Name:");
                                    ui.text_edit_singleline(name);
                                });
                                let mut creatures_to_remove: Vec<usize> = vec![];

                                ui.collapsing("Creatures", |ui| {
                                    for (i, creature) in creatures.iter_mut().enumerate() {
                                        // basic information on creature
                                        ui.horizontal(|ui| {
                                            ui.label(format!(
                                                "{}: lvl {}",
                                                creature.name, creature.lv
                                            ));
                                            if ui.button("edit").clicked() {
                                                creature_creation_windows.push(CreatureMenu {
                                                    inner: creature.clone(),
                                                    id: id_next.next(),
                                                    max_value: 100,
                                                    selected_place_index: *selected_place_index,
                                                    editing: Some((open_place_window_index, i)),
                                                });
                                            }
                                        });
                                        // creature details
                                        egui::collapsing_header::CollapsingHeader::new("Details").id_source(i+1).show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.label("Name:");
                                                ui.text_edit_singleline(&mut creature.name);
                                            });

                                            let size = &mut creature.size;
                                            ui.horizontal(|ui| {
                                                ui.label("Size:");
                                                ui.selectable_value(size, Size::Tiny, "Tiny");
                                                ui.selectable_value(size, Size::Small, "Small");
                                                ui.selectable_value(size, Size::Medium, "Medium");
                                                ui.selectable_value(size, Size::Large, "Large");
                                                ui.selectable_value(size, Size::Huge, "Huge");
                                                ui.selectable_value(size, Size::Gargantuan, "Gargantuan");
                                            });

                                            ui.horizontal(|ui| {
                                                ui.label("Type:");
                                                ui.text_edit_singleline(&mut creature._type);
                                            });
                                            ui.horizontal(|ui| {
                                                ui.label("Level:");
                                                ui.add(
                                                    egui::DragValue::new(&mut creature.lv)
                                                        .speed(1)
                                                        .clamp_range(0..=100),
                                                );
                                            });
                                            ui.horizontal(|ui| {
                                                ui.label("HP:");
                                                ui.add(
                                                    egui::DragValue::new(&mut creature.hp)
                                                        .speed(1)
                                                        .clamp_range(0..=100),
                                                );
                                            });
                                            ui.horizontal(|ui| {
                                                ui.label("Strength:");
                                                ui.add(
                                                    egui::DragValue::new(&mut creature.strength)
                                                        .speed(1)
                                                        .clamp_range(0..=100),
                                                );
                                            });
                                            ui.horizontal(|ui| {
                                                ui.label("Speed:");
                                                ui.add(
                                                    egui::DragValue::new(&mut creature.speed)
                                                        .speed(1)
                                                        .clamp_range(0..=100),
                                                );
                                            });
                                            ui.horizontal(|ui| {
                                                ui.label("Intelligence:");
                                                ui.add(
                                                    egui::DragValue::new(&mut creature.int)
                                                        .speed(1)
                                                        .clamp_range(0..=100),
                                                );
                                            });
                                            ui.horizontal(|ui| {
                                                ui.label("Mana:");
                                                ui.add(
                                                    egui::DragValue::new(&mut creature.mana)
                                                        .speed(1)
                                                        .clamp_range(0..=100),
                                                );
                                            });
                                            ui.horizontal(|ui| {
                                                ui.label("Vitality:");
                                                ui.add(
                                                    egui::DragValue::new(&mut creature.vit)
                                                        .speed(1)
                                                        .clamp_range(0..=100),
                                                );
                                            });
                                            ui.collapsing("Skills", |ui| {
                                                for skill in creature.skills.iter() {
                                                    ui.horizontal(|ui| {
                                                        ui.label(format!(
                                                            "{}: {}-{}",
                                                            skill.name,
                                                            skill.min_max.0,
                                                            skill.min_max.1
                                                        ));
                                                    });
                                                }
                                            });

                                            ui.collapsing("Spells", |ui| {
                                                for spell in creature.spells.iter() {
                                                    ui.horizontal(|ui| {
                                                        ui.label(format!(
                                                            "{}: {}-{}",
                                                            spell.name,
                                                            spell.min_max.0,
                                                            spell.min_max.1
                                                        ));
                                                    });
                                                }
                                            });


                                        });
                                        if ui.button("remove").clicked() {
                                            creatures_to_remove.push(i);
                                        }
                                    }
                                });
                                if ui.button("add creature").clicked() {
                                    creature_creation_windows.push(CreatureMenu {
                                        inner: Default::default(),
                                        id: id_next.next(),
                                        max_value: 100,
                                        selected_place_index: *selected_place_index,
                                        editing: None,
                                    });
                                }
                                for c in creatures_to_remove {
                                    places[open_place_window_index].creatures.remove(c);
                                }
                            });
                    }
                    for window in place_windows_to_remove {
                        let index = open_place_windows_indexes
                            .iter()
                            .position(|x| *x == window)
                            .unwrap();
                        open_place_windows_indexes.remove(index);
                    }
                });
                egui::CentralPanel::default().show(ctx, |ui| {
                    // The central panel the region left after adding TopPanels and SidePanels

                    ui.heading("DnD tools");
                    egui::warn_if_debug_build(ui);
                });
            }
        }
    }
}
