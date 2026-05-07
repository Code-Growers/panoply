use crate::{BG_WIDGET, BORDER};
use eframe::egui;
use egui::{CentralPanel, Stroke, TextEdit};

pub struct State {
    pub output: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            output: String::new(),
        }
    }
}

#[derive(Clone)]
pub enum Msg {
    Generate,
    Copy,
    Back,
}

pub fn update(msg: &Msg, state: &mut State, ctx: &egui::Context) {
    match msg {
        Msg::Generate => {
            state.output = uuid::Uuid::new_v4().to_string();
        }
        Msg::Copy => {
            ctx.copy_text(state.output.clone());
        }
        Msg::Back => {}
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
            ui.heading("UUID Generate");
        });
    });

    CentralPanel::default().show_inside(ui, |ui| {
        ui.add_space(8.0);
        if ui
            .add(
                egui::Button::new("Generate UUID v4")
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

    msgs
}
