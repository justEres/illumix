use eframe::App;
use eframe::WebRunner;
use eframe::egui::Color32;
use eframe::egui::{self, CentralPanel, ColorImage, Context, Slider, TextureHandle, Vec2, Window};
use futures_util::StreamExt;
use gloo_net::websocket::Message;
use gloo_net::websocket::futures::WebSocket;
use wasm_bindgen::JsCast;
use web_sys::window;
#[path = "ui_elements/color_picker.rs"]
mod color_picker;
use color_picker::ColorPickerWindow;

#[path = "ui_elements/fixture_manager.rs"]
mod fixture_manager;
use fixture_manager::FixtureManager;

struct MyApp {
    color_picker: ColorPickerWindow,
    fixture_manager: FixtureManager,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let width = 400;
        let height = 255;
        let strip = color_picker::generate_color_strip_image(400, 255); // width x height
        let texture = cc
            .egui_ctx
            .load_texture("hue_strip", strip, egui::TextureOptions::LINEAR);
        Self {
            color_picker: ColorPickerWindow::new(&cc.egui_ctx),
            fixture_manager: FixtureManager::new(&cc.egui_ctx),
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        //self.color_picker.show(ctx);
        self.fixture_manager.show(ctx);

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
