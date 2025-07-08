use eframe::egui::{Color32, Pos2, Rect};
use eframe::egui::{self, CentralPanel, ColorImage, Context, Slider, TextureHandle, Vec2, Window,Ui, Button};
use egui::{ Frame, Margin};
use wasm_bindgen::convert::IntoWasmAbi;

pub struct FaderPage{
    i: f32,
    y_offset: f32,
    fader_selected: [bool; 32],
    button_pressed: [bool; 32],
    fader_manual_override: [bool; 32],
    fader_value: [f32; 32],
    group_select: bool,
    counter: u16,

}

impl FaderPage{
    pub fn new(ctx: &egui::Context) -> Self {
        Self{
            i: 0.,
            y_offset: 0.,
            fader_selected: [false; 32],
            button_pressed: [false; 32],
            fader_manual_override: [false; 32],
            fader_value: [0.; 32],
            group_select: false,
            counter: 0,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        ctx.request_repaint();
        egui::CentralPanel::default().show(ctx, |ui|{


            /// Styling
            let mut style = (*ctx.style()).clone();
            style.spacing.slider_width = 275.0; // Wider slider
            style.spacing.interact_size.y = 40.0; // Taller handle
            style.visuals.handle_shape = egui::style::HandleShape::Rect { aspect_ratio: 1.5 };
            
            ctx.set_style(style);

            self.counter = 0;

            self.y_offset = 0.;
            for y in 0..4{
                if y > 1{
                    self.y_offset = 370.;
                }
                for x in 0..8{
                    if y == 3 || y == 1{
                        self.draw_slider_bank(ui, Vec2 { x: (60. * x as f32) + 560., y: self.y_offset});
                    }else{
                        self.draw_slider_bank(ui, Vec2 { x: 60. * x as f32, y: self.y_offset});
                    }
                    self.counter +=1
                }
            }    

            self.draw_slider(ui, Vec2 { x: (60. * 9.) + 560., y: 200.});
            if self.draw_button(ui, Vec2 { x: (60. * 9.) + 560., y: 500.}, "Reset".into()){
                self.fader_selected =  [false; 32];
            }
            if self.draw_button(ui, Vec2 { x: (60. * 9.) + 560., y: 550.}, "GR Sel".into()){
                self.group_select = !self.group_select;
            }
            
        });
    }

    fn draw_slider(&mut self, ui: &mut Ui, offset: Vec2){
        let widget_rect = 
            Rect::from_min_size(ui.min_rect().min + offset, Vec2 { x: 20., y: 20.});
        ui.put(widget_rect,
        egui::Slider::new(&mut self.i, 0.0..=100.0).show_value(false)
            .orientation(egui::SliderOrientation::Vertical)
            //.handle_shape(egui::style::HandleShape::Rect { aspect_ratio: 1.5 })
        );
        //let button_rect = 
        //Rect::from_min_size(ui.min_rect().min + offset + Vec2 { x: 0., y: 290.}, Vec2 { x: 40., y: 20.});
    }

    fn draw_button(&mut self, ui: &mut Ui, offset: Vec2,name : String) -> bool {
        let widget_rect = 
            Rect::from_min_size(ui.min_rect().min + offset, Vec2 { x: 40., y: 40.});
        let button =ui.put(widget_rect, Button::new(name));
        if button.clicked(){
            self.draw_frame("".into(), ui, Vec2 { x: 10., y: 10. }, offset + Vec2 { x: 15., y: 292.5});
        }

        return button.clicked();
    }


    fn draw_slider_bank(&mut self, ui: &mut Ui, offset: Vec2){
        let widget_rect = 
            Rect::from_min_size(ui.min_rect().min + offset, Vec2 { x: 20., y: 20.});
            ui.put(widget_rect,
                egui::Slider::new(&mut self.fader_value[self.counter as usize], 0.0..=100.0).show_value(false)
                    .orientation(egui::SliderOrientation::Vertical)
                    //.handle_shape(egui::style::HandleShape::Rect { aspect_ratio: 1.5 })
            );
        let button_rect = 
        Rect::from_min_size(ui.min_rect().min + offset + Vec2 { x: 0., y: 290.}, Vec2 { x: 40., y: 20.});
        /* if ui.put(button_rect, Button::new("name")).t{
            ui.put(button_rect, Button::new("test"));
        } */


        // Style the button based on state
        
        let mut button = Button::new(format!("{}", self.counter + 1));
        



        

        let index = self.counter as usize;

        // === Handle manual selection button ===
        if ui.put(button_rect, button).clicked() {
            // Flip the current selection state
            self.fader_selected[index] = !self.fader_selected[index];
        
            // Manually control it now
            self.fader_manual_override[index] = true;
        
            // Update button to match the selection
            self.button_pressed[index] = self.fader_selected[index];
        }
        
        if self.fader_selected[self.counter as usize] {
            // Apply red fill when clicked
            /* button = button.fill(Color32::DARK_RED); */
            self.draw_frame("".into(), ui, Vec2 { x: 10., y: 10. }, offset + Vec2 { x: 15., y: 292.5});
            if self.group_select{
                self.fader_value[self.counter as usize] = self.i;
            }
            
        }

        // === Auto-selection fallback ===
        if !self.fader_manual_override[index] {
            self.fader_selected[index] = self.fader_value[index] > 0.0;
        }

        // === Optional: Exit manual mode if fader returns to 0 and button not pressed ===
        if self.fader_value[index] == 0.0
            && self.fader_manual_override[index]
            && !self.button_pressed[index]
        {
            self.fader_manual_override[index] = false;
        }

        if self.fader_value[index] == 0.0
            && self.fader_manual_override[index]
            && !self.button_pressed[index]
        {
            self.fader_manual_override[index] = false;
        }

        
        
    }

    fn draw_frame(&self, name: String, ui: &mut Ui, size: Vec2, offset: Vec2) {
        let pos = ui.min_rect().min + offset;
        let rect = Rect::from_min_size(pos, size);
        ui.allocate_rect(rect, egui::Sense::hover()); // Reserve space

        // Paint background manually
        ui.painter().rect_filled(
            rect,
            5.0,                          // corner radius
            Color32::from_rgb(255, 200, 120), // background color
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