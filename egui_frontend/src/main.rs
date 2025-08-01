use std::fmt::format;
use std::path::Component;
use std::sync::Arc;

use eframe::App;
use eframe::WebRunner;
use eframe::egui::Button;
use eframe::egui::Color32;
use eframe::egui::mutex::Mutex;
use eframe::egui::scroll_area::State;
use eframe::egui::{self, CentralPanel, ColorImage, Context, Slider, TextureHandle, Vec2, Window};
use fixture_lib::fixture::FixtureComponent;
use fixture_lib::universe;
use futures_util::StreamExt;

use wasm_bindgen::JsCast;
use wasm_bindgen::convert::IntoWasmAbi;
use wasm_bindgen::prelude::Closure;
use web_sys::js_sys;
use web_sys::window;
#[path = "ui_elements/color_picker.rs"]
mod color_picker;
use color_picker::ColorPickerWindow;

#[path = "ui_elements/fixture_manager.rs"]
mod fixture_manager;
use fixture_manager::FixtureManager;
mod websocket;

#[path = "ui_elements/universe_window.rs"]
mod universe_window;

#[path = "ui_elements/fader_page.rs"]
mod fader_page;
#[path = "ui_elements/fixture_component_ui.rs"]
mod fixture_component_ui;
use fader_page::FaderPage;

use fixture_lib::universe::Universe;
use web_sys::ErrorEvent;
use web_sys::Event;
use web_sys::MessageEvent;
use web_sys::WebSocket;

use crate::fader_page::Fader;
use crate::universe_window::UniverseWindow;
use crate::websocket::open_websocket;

struct UniverseState {
    pub universe: Universe,
    pub modified: bool,
}

struct MyApp {
    color_picker: ColorPickerWindow,
    fixture_manager: FixtureManager,
    universe_window: UniverseWindow,
    universe: Arc<Mutex<UniverseState>>,
    websocket: WebSocket,
    fader_page: FaderPage,
    universe_modified: bool,
    //fader: FaderPage::Fader,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let width = 400;
        let height = 255;
        let strip = color_picker::generate_color_strip_image(400, 255); // width x height
        let texture = cc
            .egui_ctx
            .load_texture("hue_strip", strip, egui::TextureOptions::LINEAR);

        let universe = Arc::new(Mutex::new(UniverseState {
            universe: Universe::new(),
            modified: false,
        }));

        let ws = open_websocket(universe.clone());
        let app = Self {
            universe: universe.clone(),
            websocket: ws,
            color_picker: ColorPickerWindow::new(&cc.egui_ctx),
            fixture_manager: FixtureManager::new(&cc.egui_ctx),
            universe_window: UniverseWindow::new(&cc.egui_ctx, universe.clone()),
            fader_page: FaderPage::new(&cc.egui_ctx),
            //fader: self::fader_page,
            universe_modified: false,
        };

        return app;
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        //self.color_picker.show(ctx);
        //self.fixture_manager.show(ctx);
        self.universe_window.show(ctx);

        self.universe_window
            .sync_fixtures_with_color_picker(&self.color_picker);
        //self.color_picker.show(ctx);
        //self.fixture_manager.show(ctx);
        self.fader_page.show(ctx);

        for i in 0..32 {
            if self.fader_page.fader[i].id != None {
                //self.universe.lock().universe.get_fixture_by_id(self.fader_page.fader[i].id);
                let mut uni = self.universe.lock();

                let mut fixture_test = match uni
                    .universe
                    .get_fixture_by_id_mut(self.fader_page.fader[i].id.unwrap())
                {
                    Some(test) => test,
                    None => {
                        continue;
                    }
                };

                for c in &mut fixture_test.components.iter_mut() {
                    match c {
                        FixtureComponent::Dimmer(d) => {
                            self.universe_modified = true; // Wichtig: Steht hier weil ein Error gesagt hat uni kann nicht zwei mal geborrowed werden.KP
                            d.intensity = self.fader_page.fader[i].fader_value;
                        }
                        _ => {}
                    }
                }
                uni.modified = self.universe_modified;
            }
        }

        let color = self.color_picker.selected_color;
        Window::new("Test").show(ctx, |ui| {
            for i in 0..32 {
                ui.label(format!("{}", self.fader_page.fader[i].id.unwrap_or(0)));
            }
            /* let mut uni = self.universe.lock();

            let mut fixture_test = match uni.universe.get_fixture_by_id_mut(self.fader_page.fader[0].id.unwrap()) {
                Some(test) => {test},
                None => {return},
            };
            ui.label(format!("{:?}", fixture_test.components));
            for c in &mut fixture_test.components.iter_mut(){
                match c {
                    FixtureComponent::Dimmer(d) => {
                        ui.label(format!("{}", d.intensity));
                        d.intensity = 100;
                        ui.label(format!("{}", d.intensity));
                    }
                    _ => {}
                }
            } */
        });

        if self.universe.lock().modified {
            let uni = self.universe.lock().universe.export_to_json();
            self.websocket.send_with_str(&uni);
            self.universe.lock().modified = false;
        }

        let color = self.color_picker.selected_color;
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    let runner = WebRunner::new();
    let web_options = eframe::WebOptions::default();

    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("the_canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    wasm_bindgen_futures::spawn_local(async move {
        runner
            .start(
                canvas,
                web_options,
                Box::new(|_cc| Ok(Box::new(MyApp::new(_cc)))),
            )
            .await
            .expect("failed to start eframe");
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    panic!("This project only runs in the browser with wasm.");
}
