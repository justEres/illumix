use eframe::egui::{self, CentralPanel, Context, Slider, Window};
use eframe::App;
use eframe::WebRunner;
use wasm_bindgen::JsCast;
use web_sys::window;

struct MyApp {
    slider_value: f32,
    counter: i32,
    show_window1: bool,
    show_window2: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            slider_value: 50.0,
            counter: 0,
            show_window1: true,
            show_window2: true,
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Egui WebAssembly Example");
            ui.label("Everything below is inside floating windows.");

            if ui.button("Toggle Window 1").clicked() {
                self.show_window1 = !self.show_window1;
            }
            if ui.button("Toggle Window 2").clicked() {
                self.show_window2 = !self.show_window2;
            }
        });

        if self.show_window1 {
            Window::new("Window 1: Slider & Counter").show(ctx, |ui| {
                ui.label("Move the slider:");
                ui.add(Slider::new(&mut self.slider_value, 0.0..=100.0).text("value"));
                ui.separator();
                if ui.button("Increment Counter").clicked() {
                    self.counter += 1;
                }
                ui.label(format!("Counter value: {}", self.counter));
            });
        }

        if self.show_window2 {
            Window::new("Window 2: More Buttons").show(ctx, |ui| {
                if ui.button("Reset Counter").clicked() {
                    self.counter = 0;
                }
                if ui.button("Center Slider").clicked() {
                    self.slider_value = 50.0;
                }
            });
        }
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
            .start(canvas, web_options, Box::new(|_cc| Ok(Box::new(MyApp::default()))))
            .await
            .expect("failed to start eframe");
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    panic!("This project only runs in the browser with wasm.");
}
