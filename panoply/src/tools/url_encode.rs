use crate::{BG_WIDGET, BORDER};
use eframe::egui;
use egui::{CentralPanel, Stroke, TextEdit};

pub struct State {
    pub input: String,
    pub output: String,
    pub request_focus: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            request_focus: true,
        }
    }
}

#[derive(Clone)]
pub enum Msg {
    Run,
    Copy,
    Back,
}

pub fn update(msg: &Msg, state: &mut State, ctx: &egui::Context) {
    match msg {
        Msg::Run => {
            state.output = crate::actions::url_encode(&state.input).unwrap_or_else(|e| e);
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
            ui.heading("URL Encode");
        });
    });

    CentralPanel::default().show_inside(ui, |ui| {
        ui.add_space(8.0);
        ui.label("Input:");
        let input = ui.add(
            TextEdit::multiline(&mut state.input)
                .desired_rows(6)
                .desired_width(f32::INFINITY)
                .margin(egui::vec2(8.0, 8.0)),
        );
        if state.request_focus {
            state.request_focus = false;
            input.request_focus();
        }
        ui.add_space(8.0);

        if ui
            .add(
                egui::Button::new("Run")
                    .fill(BG_WIDGET)
                    .stroke(Stroke::new(1.0_f32, BORDER)),
            )
            .clicked()
            || ui.input(|i| i.key_pressed(egui::Key::Enter) && i.modifiers.ctrl)
        {
            msgs.push(Msg::Run);
        }

        ui.add_space(8.0);
        ui.label("Output:");
        ui.add(
            TextEdit::multiline(&mut state.output)
                .desired_rows(6)
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
