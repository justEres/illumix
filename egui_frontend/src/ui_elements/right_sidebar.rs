use eframe::{egui::{self, Button, Color32, Pos2, Slider, Vec2}, CreationContext};

use crate::fixture_component_listener::SharedState;

#[path = "ui_helper/ui_auto_scalling.rs"]
mod ui_auto_scalling;

pub struct RightSidebar{
    pub master_fader: SharedState<MasterFader>,
}

pub struct MasterFader{
    pub master_fader: u8,
    pub master_control: bool,
}

impl MasterFader{
    pub fn new()-> Self{
        Self{
            master_fader: 0,
            master_control: false,
        }
    }
}

impl RightSidebar{
    pub fn new(ctx: &CreationContext, master_fader: SharedState<MasterFader>) -> Self{

        let ui_auto_scaller = ui_auto_scalling::AutoScaller::new();

        Self{
            master_fader,    
        }
    }

    pub fn show(&mut self, ctx: &egui::Context){
        egui::SidePanel::right("Right Sidebar").resizable(false).show(ctx, |ui|{

            ui.set_min_width(150.);
            let mut style = (*ctx.style()).clone();
            style.spacing.slider_width = (ui.max_rect().max.y / 2.5); // Wider slider
            style.spacing.interact_size.y = (75. / 1.5); // Taller handle
            style.visuals.handle_shape = egui::style::HandleShape::Rect { aspect_ratio: 1.5 };

            ui.set_style(style.clone());

            let local = egui::Rect::from_min_size(
                Pos2{x:100.,y:100.},
                egui::vec2(20.0, 20.0),
            );
            let mut test = 0;
            let mut master_overwrite_responds = false;
            ui.vertical(|ui|{
                ui.add_space(ui.max_rect().max.y / 2. - 10.);
                ui.horizontal(|ui|{
                    ui.add_space((100./2.));
                    ui.add(Slider::new(&mut self.master_fader.borrow_mut().master_fader,0..=255).orientation(egui::SliderOrientation::Vertical).show_value(false));
                });
                ui.add_space(10.);
                ui.horizontal(|ui|{
                    ui.add_space(50.);
                    if self.master_fader.borrow().master_control{
                        master_overwrite_responds = ui.add(Button::new("MOver").fill(Color32::DARK_RED)).clicked();
                    }else{

                        master_overwrite_responds = ui.add(Button::new("MOver")).clicked();
                    }
                });
            });

            if master_overwrite_responds{
                let master_control_state =!self.master_fader.borrow().master_control;
                self.master_fader.borrow_mut().master_control = master_control_state;
            }



        });
    }
}