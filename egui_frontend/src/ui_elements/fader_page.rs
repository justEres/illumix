use eframe::egui::{Color32, Pos2, Rect};
use eframe::egui::{self, CentralPanel, ColorImage, Context, Slider, TextureHandle, Vec2, Window,Ui, Button};
use egui::{ Frame, Margin};

pub struct FaderPage{
    i: f32,
    y_offset: f32,
    button_clicked: [bool; 32],
    fader_value: [f32; 32],
    counter: u16,

}

impl FaderPage{
    pub fn new(ctx: &egui::Context) -> Self {
        Self{
            i: 0.,
            y_offset: 0.,
            button_clicked: [
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,

            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,

            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,

            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            ],
            fader_value: [
                0.,
                0.,
                0.,
                0.,
                0.,
                0.,
                0.,
                0.,

                0.,
                0.,
                0.,
                0.,
                0.,
                0.,
                0.,
                0.,

                0.,
                0.,
                0.,
                0.,
                0.,
                0.,
                0.,
                0.,

                0.,
                0.,
                0.,
                0.,
                0.,
                0.,
                0.,
                0.,
            ],
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
                        self.draw_slider(ui, Vec2 { x: (60. * x as f32) + 560., y: self.y_offset});
                    }else{
                        self.draw_slider(ui, Vec2 { x: 60. * x as f32, y: self.y_offset});
                    }
                    self.counter +=1
                }
            }    
            
        });
    }

    fn draw_slider(&mut self, ui: &mut Ui, offset: Vec2){
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

        if self.button_clicked[self.counter as usize] {
            // Apply red fill when clicked
            button = button.fill(Color32::DARK_RED);
        }

        // Handle button press
        if ui.put(button_rect, button).clicked() {
            self.button_clicked[self.counter as usize] = !self.button_clicked[self.counter as usize];
        }
        
    }

}