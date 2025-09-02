use std::fmt::format;

use eframe::egui::{
    self, Button, CentralPanel, Color32, ColorImage, Context, Slider, TextureHandle, Ui, UiBuilder,
    Vec2, Window,
};
use eframe::egui::{Pos2, Rect};
use eframe::{CreationContext, Frame};
use fixture_lib::fixture::{Dimmer, FixtureComponent};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::info;

use crate::fader_page::fader::Fader;
use crate::fixture_component_listener::{ChangeEventManager, ListenerDatabase, SharedState};

use crate::fader_page::ui_auto_scalling::AutoScaller;

#[path = "ui_helper/ui_auto_scalling.rs"]
mod ui_auto_scalling;

mod fader;

pub struct FaderPage {
    fader_list: SharedState<[fader::Fader; 24]>,

    //Styling
    panel_resolution: Vec2,
    change_event_manager: SharedState<ChangeEventManager>,
    listener_database: SharedState<ListenerDatabase>,
    ui_auto_scaller: ui_auto_scalling::AutoScaller,
    rect: Rect,

    group_select: bool,
    first_selected: Option<u8>,
    second_selection: Option<u8>,
    selection_save: Option<Vec<bool>>,
    test_save: Vec<bool>,
}

impl FaderPage {
    pub fn new(
        ctx: &CreationContext,
        change_event_manager: SharedState<ChangeEventManager>,
        listener_database: SharedState<ListenerDatabase>,
    ) -> Self {
        let panel_resolution = Vec2 { x: 1000., y: 800. };

        let fader_list: SharedState<[fader::Fader; 24]> =
            SharedState::new(std::array::from_fn(|_| fader::Fader::new(None, None)));

        let rect = Rect::from_min_size(Pos2::ZERO, egui::vec2(0.0, 0.0));

        let mut fp = Self {
            fader_list,
            panel_resolution,
            ui_auto_scaller: AutoScaller::new(),
            rect,
            change_event_manager,
            listener_database,

            group_select: false,
            first_selected: None,
            second_selection: None,
            selection_save: None,
            test_save: vec![false;24],
        };
        fp.add_listeners();
        fp
    }

    pub fn add_listeners(&mut self) {
        let mut listener_database = self.listener_database.borrow_mut();
        for i in 0..self.fader_list.borrow().len() {
            let fader_list = self.fader_list.clone();
            listener_database.add_listener(
                i as u8,
                0,
                Box::new(move |fc| match fc {
                    FixtureComponent::Dimmer(d) => {
                        fader_list.borrow_mut()[i].fader_value = d.intensity;
                        //web_sys::console::log_1(&"updated fader intensity".into());
                    }
                    _ => {}
                }),
            );
        }
        web_sys::console::log_1(&"created listeners".into());
    }

    pub fn show(&mut self, ctx: &egui::Context) {
            egui::CentralPanel::default().show(ctx, |ui| {
                ctx.request_repaint();
                self.rect = ui.max_rect();
                self.panel_resolution = Vec2 {
                    x: self.rect.max.x / 100.,
                    y: self.rect.max.y / 100.,
                };

                let mut style = (*ctx.style()).clone();
                style.spacing.slider_width = (self.ui_auto_scaller.get_cell_size().y); // Wider slider
                style.spacing.interact_size.y = (self.ui_auto_scaller.get_cell_size().x / 1.75); // Taller handle
                style.visuals.handle_shape = egui::style::HandleShape::Rect { aspect_ratio: 1.5 };

                ui.set_style(style.clone());
                
                self.draw_slider_bank(ui);

                

                self.draw_ctrl_buttons(ui)
            });
        
    }

    fn draw_slider_bank(&mut self, ui: &mut Ui) {

        for i in 0..24 {
            

            let local = egui::Rect::from_min_size(self.ui_auto_scaller.get_rect(self.rect, true, i as u8).min + 
                                                        Vec2{x: self.ui_auto_scaller.get_cell_size().x / 4., y:0.}, egui::vec2(20.0, 20.0));

            let response = ui.put(
                local,
                egui::Slider::new(&mut self.fader_list.borrow_mut()[i].fader_value, 0..=255)
                    .show_value(false)
                    .orientation(egui::SliderOrientation::Vertical),
            );

            if response.changed() {
                self.fader_list.borrow_mut()[i].fader_value_changed();
                self.change_event_manager.borrow_mut().create_event(
                    i as u8,
                    0,
                    fixture_lib::fixture::FixtureComponent::Dimmer(Dimmer {
                        intensity: self.fader_list.borrow()[i].fader_value,
                    }),
                );
            }

            let local = egui::Rect::from_min_size(
                self.ui_auto_scaller.get_rect(self.rect, false, i as u8).min + 
                Vec2{x:self.ui_auto_scaller.get_cell_size().x / 4., y: 0.},
                egui::vec2(self.ui_auto_scaller.get_cell_size().x / 2., self.ui_auto_scaller.get_cell_size().x / 2.),
            );

            if self.fader_list.borrow()[i].is_selected == false
                && ui.put(local, Button::new(format!("{}", i + 1))).clicked()
            {
                self.fader_list.borrow_mut()[i].is_selected = true;
            } else if self.fader_list.borrow()[i].is_selected == true
                && ui.put(local, Button::new(format!("{}", i + 1))).clicked()
            {
                self.fader_list.borrow_mut()[i].is_selected = false;
            }


            if self.group_select{
                match self.selection_save{
                    None => {    
                    let mut selection_save = Vec::new();    
                    for k in 0..24{
                        
                        selection_save.push(self.fader_list.borrow_mut()[k].is_selected);
                        
                        
                        self.fader_list.borrow_mut()[k].is_selected = false;
                    }
                    self.selection_save = Some(selection_save);
                },
                    Some(_) => {}
                }
            }
            

            
            
            

            if self.fader_list.borrow()[i].is_selected == true {
                let local = egui::Rect::from_min_size(
                    self.ui_auto_scaller.get_rect(self.rect, false, i as u8).min + 
                    Vec2{x:self.ui_auto_scaller.get_cell_size().x / 2. -self.ui_auto_scaller.get_cell_size().x / 24. ,y: 0.},
                    egui::vec2(self.ui_auto_scaller.get_cell_size().x / 12., self.ui_auto_scaller.get_cell_size().x / 12.),
                );
    
                ui.allocate_rect(local, egui::Sense::hover());
    
                
                let color = Color32::from_rgb(255, 200, 120);
                ui.painter().rect_filled(
                    local, 5.0,   // corner radius
                    color, // background color
                );

                if self.group_select && self.first_selected != None{
                    //ui.label(format!("{}",(self.first_selected.unwrap())));
                    self.second_selection = Some(i as u8);
                    
                    if self.second_selection.unwrap() < self.first_selected.unwrap(){
                        for k in self.second_selection.unwrap()..=self.first_selected .unwrap(){
                            self.test_save[k as usize] = true;
                            self.fader_list.borrow_mut()[k as usize].is_selected = true;
                            
                        }
                    }else{
                        for k in self.first_selected.unwrap()..=self.second_selection.unwrap(){
                            
                            self.fader_list.borrow_mut()[k as usize].is_selected = true;
                            
                        }
                    }
                    let selection_save = self.selection_save.as_ref().unwrap();

                    match self.selection_save{
                        None => {},
                        Some(_) => {for k in 0..24{
                            if selection_save[k] == true{
                                
                                self.fader_list.borrow_mut()[k].is_selected = true;
                            }
                            
                        }
                        self.group_select = false;
                        self.selection_save = None;
                        self.first_selected = None;
                        self.second_selection = None;}
                    }
                    
                    

                }else if self.group_select{
                    
                    self.first_selected = Some(i as u8);
                    self.fader_list.borrow_mut()[i].is_selected = false;
                    

                    
                }
                
            }

            
        }
    }

    fn draw_ctrl_buttons(&mut self, ui: &mut Ui){
        let local = self.ui_auto_scaller.get_ctrl_button(0, self.rect);
        if ui.put(local, Button::new("All")).clicked(){
            self.ctrl_button_trigger(0);
        }

        let local = self.ui_auto_scaller.get_ctrl_button(1, self.rect);
        if ui.put(local, Button::new("Config")).clicked(){
            self.ctrl_button_trigger(1);
        }

        let local = self.ui_auto_scaller.get_ctrl_button(2, self.rect);
        if ui.put(local, Button::new("Clear")).clicked(){
            self.ctrl_button_trigger(2);
        }

        let local = self.ui_auto_scaller.get_ctrl_button(3, self.rect);
        if self.group_select{
            if ui.put(local, Button::new("Group Select").fill(Color32::DARK_RED)).clicked(){
                self.ctrl_button_trigger(3);
            }
        }else{
            if ui.put(local, Button::new("Group Select")).clicked(){
                self.ctrl_button_trigger(3);
            }
        }
        
    }

    fn ctrl_button_trigger(&mut self, button_id: u8){
        match button_id{
            0 => for i in 0..24{
                self.fader_list.borrow_mut()[i].is_selected = true;
            },
            1 => return,
            2 => for i in 0..24{
                self.fader_list.borrow_mut()[i].is_selected = false;
            },
            3 => if self.group_select{
                self.group_select = !self.group_select;
                self.first_selected =None;
                self.second_selection =None;
            }else{
                self.group_select = !self.group_select;
            },
            _ => return
        }
    }
}
