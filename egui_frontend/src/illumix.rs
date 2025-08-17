use eframe::{App, CreationContext, egui};
use fixture_lib::universe::Universe;
use web_sys::WebSocket;

use crate::{
    fader_page::{self, FaderPage},
    fixture_component_listener::{ListenerDatabase, SharedState},
    websocket::open_websocket,
};

#[derive(PartialEq)]
enum Tab {
    FaderPage,
    ColorPicker,
    MovingHeads,
}

pub struct PageInstances {
    fader_page: FaderPage,
}

impl PageInstances {
    fn new(ctx: &CreationContext) -> Self {
        Self {
            fader_page: FaderPage::new(ctx),
        }
    }
}

pub struct Illumix {
    active_tab: Tab,
    universe: SharedState<Universe>,
    listener_database: SharedState<ListenerDatabase>,
    web_socket: WebSocket,
    page_instances: PageInstances,
}

impl Illumix {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let universe = SharedState::new(Universe::new());
        let listener_database = SharedState::new(ListenerDatabase::new());
        let web_socket = open_websocket(universe.clone(), listener_database.clone());

        let page_instances = PageInstances {
            fader_page: FaderPage::new(&cc),
        };

        Illumix {
            active_tab: Tab::FaderPage,
            universe,
            listener_database,
            web_socket,
            page_instances,
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
                    .selectable_label(self.active_tab == Tab::ColorPicker, "Color Picker")
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
            Tab::FaderPage => {
                self.page_instances.fader_page.show(ctx);
            }
            Tab::ColorPicker => {
                ui.label("Here you can pick colors");
            }
            Tab::MovingHeads => {
                ui.label("Turn your head around");
            }
        });
    }
}
