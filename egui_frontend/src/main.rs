#[cfg(target_arch = "wasm32")]
use eframe::WebRunner;
use eframe::egui::{self, Context, Window};
use wasm_bindgen::JsCast;
use web_sys::window;

#[path = "ui_elements/color_picker.rs"]
mod color_picker;
#[path = "ui_elements/fader_page.rs"]
mod fader_page;
#[path = "ui_elements/fixture_component_ui.rs"]
mod fixture_component_ui;
#[path = "ui_elements/fixture_manager.rs"]
mod fixture_manager;

#[path = "ui_elements/visual_page.rs"]
mod visual_page;

#[path = "ui_elements/right_sidebar.rs"]
mod right_sidebar;
#[path = "ui_elements/ui_helper/ui_auto_scalling.rs"]
mod ui_auto_scalling;

mod fixture_component_listener;
mod illumix;
mod websocket;

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
