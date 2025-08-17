use std::fmt::format;
use std::path::Component;
use std::sync::Arc;

use eframe::App;
use eframe::WebRunner;
use eframe::egui::mutex::Mutex;
use eframe::egui::{self, Context, Window};
use fixture_lib::fixture::FixtureComponent;

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

mod illumix;

#[path = "ui_elements/fader_page.rs"]
mod fader_page;
#[path = "ui_elements/fixture_component_ui.rs"]
mod fixture_component_ui;
use fader_page::FaderPage;

mod fixture_component_listener;

use fixture_lib::universe::Universe;
use web_sys::WebSocket;

use crate::fader_page::Fader;
use crate::fixture_component_listener::ListenerDatabase;
use crate::fixture_component_listener::SharedState;

use crate::websocket::open_websocket;

struct MyApp {
    color_picker: ColorPickerWindow,
    fixture_manager: FixtureManager,

    universe: SharedState<Universe>,
    websocket: WebSocket,
    fader_page: FaderPage,
    universe_modified: bool,
    listener_database: SharedState<ListenerDatabase>, //fader: FaderPage::Fader,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let width = 400;
        let height = 255;
        let strip = color_picker::generate_color_strip_image(400, 255); // width x height
        let texture = cc
            .egui_ctx
            .load_texture("hue_strip", strip, egui::TextureOptions::LINEAR);

        let universe = SharedState::new(Universe::new());
        let listener_database = SharedState::new(ListenerDatabase::new());

        let ws = open_websocket(universe.clone(), listener_database.clone());
        let app = Self {
            universe: universe.clone(),
            websocket: ws,
            color_picker: ColorPickerWindow::new(&cc.egui_ctx),
            fixture_manager: FixtureManager::new(&cc.egui_ctx),
            //universe_window: UniverseWindow::new(&cc.egui_ctx, universe.clone()),
            fader_page: FaderPage::new(&cc.egui_ctx),
            listener_database,
            //fader: self::fader_page,
            universe_modified: false,
        };

        return app;
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        self.fader_page.show(ctx);

        for i in 0..32 {
            if self.fader_page.fader[i].id != None {
                let mut uni = self.universe.borrow_mut();

                let mut fixture_test =
                    match uni.get_fixture_by_id_mut(self.fader_page.fader[i].id.unwrap()) {
                        Some(test) => test,
                        None => {
                            continue;
                        }
                    };

                for c in &mut fixture_test.components.iter_mut() {
                    match c {
                        FixtureComponent::Dimmer(d) => {
                            d.intensity = self.fader_page.fader[i].fader_value;
                        }
                        _ => {}
                    }
                }
            }
        }

        let color = self.color_picker.selected_color;
        Window::new("Test").show(ctx, |ui| {
            for i in 0..32 {
                ui.label(format!("{}", self.fader_page.fader[i].id.unwrap_or(0)));
            }
        });

        /* if self.universe.lock().modified {
            let uni = self.universe.lock().universe.export_to_json();
            self.websocket.send_with_str(&uni);
            self.universe.lock().modified = false;
        } */

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
        use crate::illumix::Illumix;

        runner
            .start(
                canvas,
                web_options,
                Box::new(|_cc| Ok(Box::new(Illumix::new(_cc)))),
            )
            .await
            .expect("failed to start eframe");
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    panic!("This project only runs in the browser with wasm.");
}
