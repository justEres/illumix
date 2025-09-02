use eframe::egui::{self, Button, DragValue, Ui, Vec2, Window, vec2};
use eframe::egui::{Color32, Label, Rect};

use crate::fader_page::Fader;

pub struct FaderPageSetupWindow {
    selected_fader: u8,
    fader_patch: i8,
    test: i32,
    patch_view: bool,
    screen_size: Vec2,
}


impl FaderPageSetupWindow{
    pub fn new(ctx: &egui::Context) -> Self{
        Self{
            selected_fader: 1,
            fader_patch: 0,
            test: 1,
            patch_view: false,
            screen_size: Vec2 { x: 0., y: 0. }
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, fader: &mut Vec<Fader>){
        Window::new("Setup Fader Page").show(ctx, |ui|{

            self.screen_size = ui.available_size();

            let mut style = (*ctx.style()).clone();
            style.spacing.slider_width = ((self.screen_size.y / 100.) * 20.); // Wider slider
            style.spacing.interact_size.y = ((self.screen_size.y / 100.) * 10.); // Taller handle
            //style.spacing.interact_size.x = ((self.screen_size.x / 100.) * 4.);
            style.visuals.handle_shape = egui::style::HandleShape::Rect { aspect_ratio: 1.5 };
            
            ctx.set_style(style);
            
            
            
            if self.selected_fader < 32{
                if self.draw_button(ui, Vec2 { x: 0., y: 0.}, "⬆".into(), None){
                    self.selected_fader += 1;
                }
            }

            self.draw_frame(
                format!("{}", self.selected_fader),
                ui,
                Vec2 { x: 40., y: 40. },
                Vec2 { x: 0., y: 60. },
                Color32::DARK_GRAY,
            );

            if self.selected_fader > 1 {
                if self.draw_button(ui, Vec2 { x: 0., y: 120. }, "⬇".into(), None) {
                    self.selected_fader -= 1;
                }
            }

            match fader[self.selected_fader as usize - 1].id {
                Some(T) => {
                    self.fader_patch = fader[self.selected_fader as usize - 1].id.unwrap() as i8;
                }
                None => self.fader_patch = -1,
            }

            self.draw_drag_value(ui, Vec2 { x: 60., y: 60. }, "Fixture ID: ".into());

            match self.fader_patch {
                -1 => {
                    fader[self.selected_fader as usize - 1].id = None;
                }
                _ => {
                    fader[self.selected_fader as usize - 1].id = Some(self.fader_patch as u8);
                }
            }

            if self.draw_button(ui, Vec2 { x: 50., y: 120. }, "View Patch".into(), None) {
                self.patch_view = !self.patch_view;
            }

            if self.patch_view == false {
                for i in 0..32 {
                    fader[i].fader_value = 0;
                    if i == (self.selected_fader as usize - 1) {
                        fader[i].fader_selected = true;
                    } else {
                        fader[i].fader_selected = false;
                    }
                }
            } else {
                for i in 0..32 {
                    fader[i].fader_value = 0;
                    fader[i].fader_selected = true;
                }
            }
        });
    }

    fn draw_drag_value(&mut self, ui: &mut Ui, offset: Vec2, name: String) {
        let rect = Rect::from_min_size(ui.min_rect().min + offset, vec2(20.0, 10.0));
        let label_rect = Rect::from_min_size(
            ui.min_rect().min + (offset - vec2(10., 30.)),
            vec2(40.0, 10.0),
        );
        ui.put(label_rect, Label::new(name));
        ui.put(
            rect,
            DragValue::new(&mut self.fader_patch).clamp_range(-1..=32),
        );
    }

    fn draw_button(
        &mut self,
        ui: &mut Ui,
        offset: Vec2,
        name: String,
        mut pressed: Option<bool>,
    ) -> bool {
        let widget_rect = Rect::from_min_size(ui.min_rect().min + offset, Vec2 { x: 40., y: 40. });
        let button = ui.put(widget_rect, Button::new(name));
        if pressed != None {
            if button.clicked() {
                pressed = Some(!pressed.unwrap());
            }
            if pressed.unwrap() {
                self.draw_frame(
                    "".into(),
                    ui,
                    Vec2 { x: 10., y: 10. },
                    offset + Vec2 { x: 15., y: 0. },
                    Color32::from_rgb(255, 200, 120),
                );
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
            rect, 5.0,   // corner radius
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
