use crate::{BG_WIDGET, BORDER};
use eframe::egui;
use egui::{CentralPanel, Stroke, TextEdit};

pub struct State {
    pub length: usize,
    pub output: String,
    pub use_lowercase: bool,
    pub use_uppercase: bool,
    pub use_digits: bool,
    pub use_special: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            length: 16,
            output: String::new(),
            use_lowercase: true,
            use_uppercase: true,
            use_digits: true,
            use_special: true,
        }
    }
}

#[derive(Clone)]
pub enum Msg {
    Generate,
    Copy,
    Back,
    ToggleLowercase,
    ToggleUppercase,
    ToggleDigits,
    ToggleSpecial,
    IncreaseLength,
    DecreaseLength,
}

pub fn update(msg: &Msg, state: &mut State, ctx: &egui::Context) {
    match msg {
        Msg::Generate => {
            state.output = crate::actions::generate_password(
                state.length,
                state.use_lowercase,
                state.use_uppercase,
                state.use_digits,
                state.use_special,
            );
        }
        Msg::Copy => {
            ctx.copy_text(state.output.clone());
        }
        Msg::Back => {}
        Msg::ToggleLowercase => state.use_lowercase = !state.use_lowercase,
        Msg::ToggleUppercase => state.use_uppercase = !state.use_uppercase,
        Msg::ToggleDigits => state.use_digits = !state.use_digits,
        Msg::ToggleSpecial => state.use_special = !state.use_special,
        Msg::IncreaseLength => {
            if state.length < 128 {
                state.length += 1;
            }
        }
        Msg::DecreaseLength => {
            if state.length > 4 {
                state.length -= 1;
            }
        }
    }
}

pub fn view(state: &mut State, ui: &mut egui::Ui) -> Vec<Msg> {
    let mut msgs = Vec::new();

    egui::Panel::top("top_bar").show_inside(ui, |ui| {
        ui.horizontal(|ui| {
            if ui
                .add(egui::Button::new("Back").stroke(Stroke::new(1.0_f32, BORDER)))
                .clicked()
            {
                msgs.push(Msg::Back);
            }
            ui.heading("Password Generate");
        });
    });

    CentralPanel::default().show_inside(ui, |ui| {
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.label("Length:");
            ui.add(
                egui::DragValue::new(&mut state.length)
                    .range(4..=128)
                    .custom_formatter(|n, _| format!("{n}")),
            );
        });

        ui.add_space(4.0);
        ui.label("Character sets:");
        ui.horizontal(|ui| {
            if ui
                .checkbox(&mut state.use_lowercase, "a-z")
                .clicked()
            {
                msgs.push(Msg::ToggleLowercase);
            }
            if ui
                .checkbox(&mut state.use_uppercase, "A-Z")
                .clicked()
            {
                msgs.push(Msg::ToggleUppercase);
            }
            if ui.checkbox(&mut state.use_digits, "0-9").clicked() {
                msgs.push(Msg::ToggleDigits);
            }
            if ui
                .checkbox(&mut state.use_special, "!@#$%^&*")
                .clicked()
            {
                msgs.push(Msg::ToggleSpecial);
            }
        });

        ui.add_space(8.0);
        if ui
            .add(
                egui::Button::new("Generate Password")
                    .fill(BG_WIDGET)
                    .stroke(Stroke::new(1.0_f32, BORDER)),
            )
            .clicked()
            || ui.input(|i| i.key_pressed(egui::Key::Enter) && i.modifiers.ctrl)
        {
            msgs.push(Msg::Generate);
        }

        ui.add_space(8.0);
        ui.label("Output:");
        if ui.input(|i| i.key_pressed(egui::Key::C) && i.modifiers.ctrl) {
            msgs.push(Msg::Copy);
        }
        ui.add(
            TextEdit::multiline(&mut state.output)
                .desired_rows(2)
                .interactive(false)
                .desired_width(f32::INFINITY)
                .margin(egui::vec2(8.0, 8.0)),
        );
    });

    if ui.input(|i| i.events.contains(&egui::Event::Copy)) {
        msgs.push(Msg::Copy);
    }
    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
        msgs.push(Msg::Back);
    }

    // Hotkeys for character sets
    if ui.input(|i| i.key_pressed(egui::Key::Num1) && i.modifiers.ctrl) {
        msgs.push(Msg::ToggleLowercase);
    }
    if ui.input(|i| i.key_pressed(egui::Key::Num2) && i.modifiers.ctrl) {
        msgs.push(Msg::ToggleUppercase);
    }
    if ui.input(|i| i.key_pressed(egui::Key::Num3) && i.modifiers.ctrl) {
        msgs.push(Msg::ToggleDigits);
    }
    if ui.input(|i| i.key_pressed(egui::Key::Num4) && i.modifiers.ctrl) {
        msgs.push(Msg::ToggleSpecial);
    }

    // Hotkeys for length adjustment
    if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
        msgs.push(Msg::IncreaseLength);
    }
    if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
        msgs.push(Msg::DecreaseLength);
    }

    msgs
}
