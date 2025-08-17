use eframe::{
    App,
    egui::{self, Label, Window},
};

use crate::fader_page::FaderPage;

#[derive(PartialEq)]
enum Tab {
    FaderPage,
    ColorPicker,
    MovingHeads,
}

pub struct Illumix {
    active_tab: Tab,
}

impl Illumix {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Illumix {
            active_tab: Tab::FaderPage,
        }
    }
}

impl App for Illumix {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("tab_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Illumix");
                if ui
                    .selectable_label(self.active_tab == Tab::FaderPage, "Fader Page")
                    .clicked()
                {
                    self.active_tab = Tab::FaderPage;
                }
                if ui
                    .selectable_label(self.active_tab == Tab::ColorPicker, "Fader Page")
                    .clicked()
                {
                    self.active_tab = Tab::ColorPicker;
                }
                if ui
                    .selectable_label(self.active_tab == Tab::MovingHeads, "Moving Heads")
                    .clicked()
                {
                    self.active_tab = Tab::MovingHeads;
                }
            });
        });

        egui::SidePanel::left("left_sidebar").show(ctx, |ui| {
            ui.heading("Left Sidebar");
        });

        egui::SidePanel::right("right_sidebar").show(ctx, |ui| {
            ui.heading("Right Sidebar");
        });

        egui::CentralPanel::default().show(ctx, |ui| match self.active_tab {
            Tab::FaderPage => ui.label("Welcome to the Fader Page"),
            Tab::ColorPicker => ui.label("Here you can pick colors"),
            Tab::MovingHeads => ui.label("Turn your head around"),
        });
    }
}
