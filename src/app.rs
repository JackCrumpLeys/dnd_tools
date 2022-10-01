use std::collections::HashSet;
use std::ops::Deref;
use egui_inspect::derive::Inspect;
use eframe::{egui};
use eframe::egui::Id;
use self::egui::Color32;
use rand::Rng;
use egui_inspect::inspect;
// use ::egui::*;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct DndTool {
    places: Vec<Place>,
    selected_place_index: usize,
    open_place_windows_indexes: Vec<usize>,
    open_interface: Interface,
    // creating_creatcher: bool
    dice_windows: Vec<DiceMenu>,
    dice_id_next: usize
}

#[derive(serde::Deserialize, serde::Serialize, Inspect, Debug, Clone, Ord, PartialEq, PartialOrd, Eq)]
pub struct DiceMenu {
    amount: i32,
    size: i32,
    id: usize,
    dice_results: Vec<usize>
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Ord, PartialEq, PartialOrd, Eq)]
pub enum Interface {
    CreatureCreation,
    MapTool,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Ord, PartialEq, PartialOrd, Eq)]
pub struct Place {
    name: String,
    creatures: Vec<Creature>,
}

#[derive(serde::Deserialize, serde::Serialize, Inspect, Debug, Clone, Ord, PartialEq, PartialOrd, Eq)]
pub struct Creature {
    size: Size,
    _type: String,
    lv:i32,
    hp: i32,
    strength: i32,
    speed: i32,
    int: i32,
    mana: i32,
    vit:i32,
    name: String,
    skills: Vec<Skill>,
    spells: Vec<Spell>
}

#[derive(serde::Deserialize, serde::Serialize, Inspect, Debug, Clone, Ord, PartialEq, PartialOrd, Eq)]
pub struct Skill {
    name: String,
    min_max: (i32, i32)
}

#[derive(serde::Deserialize, serde::Serialize, Inspect, Debug, Clone, Ord, PartialEq, PartialOrd, Eq)]
pub struct Spell {
    name: String,
    min_max: (i32, i32)
}

#[derive(serde::Deserialize, serde::Serialize, Inspect, Debug, Ord, PartialEq, PartialOrd, Eq, Clone)]
pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
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
            open_interface: Interface::CreatureCreation,
            dice_windows: vec![],
            dice_id_next: 0
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
        let Self { places, selected_place_index, open_place_windows_indexes, open_interface, dice_windows, dice_id_next } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

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

            egui::ComboBox::from_label("place:")
                .selected_text(format!("{}", places[*selected_place_index].name))
                .show_ui(ui, |ui| {
                    for place_index in 0..places.len() {
                        ui.selectable_value(selected_place_index, place_index.clone(), places[place_index].name.clone());
                    }
                });

            if ui.button("open place window").clicked() {
                open_place_windows_indexes.push(*selected_place_index)
            }

            if ui.button("open dice window").clicked() {
                dice_windows.push(DiceMenu { amount: 1, size: 20, id: *dice_id_next, dice_results: vec![] });
                *dice_id_next += 1;
            }



            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/eframe");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanels and SidePanels

            ui.heading("DnD tools");
            egui::warn_if_debug_build(ui);
        });
        open_place_windows_indexes.sort();
        open_place_windows_indexes.dedup();

        let mut windows_to_remove:Vec<usize> = vec![];
        for open_place_window_index in open_place_windows_indexes.clone() {
            egui::Window::new(places[open_place_window_index].name.clone()).id(Id::new(&open_place_window_index)).show(ctx, |ui| {
                let place = &mut places[open_place_window_index];
                let mut name = &mut place.name;
                let mut creatures = &mut place.creatures;
                inspect!(
                    ui,
                    name,
                    creatures
                );
                if ui.button("close window").clicked() {
                    windows_to_remove.push(open_place_window_index);
                }
            });
        }

        for dice_window in dice_windows {
            egui::Window::new(format!("Roll {}d{} dice", dice_window.amount, dice_window.size)).id(Id::new(dice_window.id)).show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(&mut dice_window.amount).speed(0.1));
                    ui.label("d");
                    ui.add(egui::DragValue::new(&mut dice_window.size).speed(0.5));
                });

                if ui.button("Roll dice").clicked() {
                    dice_window.dice_results.drain(..);
                    for _ in 0..dice_window.amount {
                        dice_window.dice_results.push(rand::thread_rng().gen_range(1..=dice_window.size) as usize);
                    }
                }

                if !dice_window.dice_results.is_empty() {
                    ui.text_edit_multiline(&mut format!("{:?}", dice_window.dice_results));
                }

            });
        }

        for window in windows_to_remove{
            let index = open_place_windows_indexes.iter().position(|x| *x == window).unwrap();
            open_place_windows_indexes.remove(index);
        }

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}
