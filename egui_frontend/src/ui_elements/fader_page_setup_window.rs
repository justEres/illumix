use eframe::egui::{pos2, Color32, Pos2, Rect};
use eframe::egui::{self, CentralPanel, ColorImage, Context, Slider, TextureHandle, Vec2, Window,Ui, Button, DragValue};
use egui::{ Frame, Margin};
use wasm_bindgen::convert::IntoWasmAbi;


pub struct FaderPageSetupWindow{
    selected_fader: u8,
    test: i32,
}

impl FaderPageSetupWindow{
    pub fn new(ctx: &egui::Context) -> Self{
        Self{
            selected_fader: 1,
            test: 1
        }
    }

    pub fn show(&mut self, ctx: &egui::Context){
        Window::new("Setup Fader Page").show(ctx, |ui|{
            
            if self.selected_fader < 32{
                if self.draw_button(ui, Vec2 { x: 0., y: 0.}, "⬆".into(), None){
                    self.selected_fader += 1;
                }
            }
            

            self.draw_frame(format!("{}",self.selected_fader), ui, Vec2 { x: 40., y: 40. }, Vec2 { x: 0., y: 60. }, Color32::DARK_GRAY);
            if self.selected_fader > 1{
                if self.draw_button(ui, Vec2 { x: 0., y: 120.}, "⬇".into(), None){
                    self.selected_fader -= 1;
                }
    
            }
            
        });
    }






    fn draw_button(&mut self, ui: &mut Ui, offset: Vec2,name : String,  mut pressed: Option<bool>) -> bool {
        let widget_rect = 
            Rect::from_min_size(ui.min_rect().min + offset, Vec2 { x: 40., y: 40.});
        let button =ui.put(widget_rect, Button::new(name));
        if pressed != None{
            if button.clicked(){
                pressed = Some(!pressed.unwrap());
                
            }
            if pressed.unwrap(){
                self.draw_frame("".into(), ui, Vec2 { x: 10., y: 10. }, offset + Vec2 { x: 15., y: 0.}, Color32::from_rgb(255, 200, 120));
            }
        }
        

        return button.clicked();
    }


    fn draw_frame(&self, name: String, ui: &mut Ui, size: Vec2, offset: Vec2, color: Color32) {
        let pos = ui.min_rect().min + offset;
        let rect = Rect::from_min_size(pos, size);
        ui.allocate_rect(rect, egui::Sense::hover()); // Reserve space

        // Paint background manually
        ui.painter().rect_filled(
            rect,
            5.0,                          // corner radius
            color, // background color
        );

        // Paint centered text
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            name,
            egui::TextStyle::Button.resolve(ui.style()),
            Color32::WHITE,
        );
    }
}