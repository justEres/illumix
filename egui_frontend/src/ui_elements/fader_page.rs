use eframe::egui::{self, Button, Ui, Vec2};
use eframe::egui::{Color32, Rect};

mod fader_page_setup_window;
use fader_page_setup_window::FaderPageSetupWindow;

pub struct FaderPage {
    group_value: u8,
    y_offset: f32,

    button_pressed: [bool; 32],
    fader_manual_override: [bool; 32],

    group_select: bool,
    counter: u8,
    setup_toggle: bool,

    pub fader: Vec<Fader>,

    setup_window: FaderPageSetupWindow,

    selection_save: Vec<bool>,
    fader_value_save: Vec<u8>,

    screen_size: Vec2,
}
#[derive(Clone)]
pub struct Fader {
    pub id: Option<u8>,
    pub fader_value: u8,
    pub fader_selected: bool,
}

impl FaderPage {
    pub fn new(ctx: &egui::Context) -> Self {
        let mut fader = Vec::new();
        let mut selection_save = Vec::new();
        let mut fader_value_save = Vec::new();

        for i in 0..=31 {
            fader.push(Fader {
                id: None,
                fader_value: 0,
                fader_selected: false,
            });
        }
        for i in 0..=31 {
            selection_save.push(false);
            fader_value_save.push(0);
        }

        Self {
            group_value: 0,
            y_offset: 0.,

            button_pressed: [false; 32],
            fader_manual_override: [false; 32],

            setup_toggle: false,

            group_select: false,
            counter: 0,
            fader,
            setup_window: FaderPageSetupWindow::new(ctx),

            selection_save,
            fader_value_save,

            screen_size: Vec2 { x: 0., y: 0. },
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        //ctx.set_pixels_per_point(2.);

        egui::CentralPanel::default().show(ctx, |ui| {
            self.screen_size = ui.available_size();

            /// Styling
            let mut style = (*ctx.style()).clone();
            style.spacing.slider_width = ((self.screen_size.y / 100.) * 37.5); // Wider slider
            style.spacing.interact_size.y = ((self.screen_size.y / 100.) * 6.); // Taller handle
            //style.spacing.interact_size.x = ((self.screen_size.x / 100.) * 4.);
            style.visuals.handle_shape = egui::style::HandleShape::Rect { aspect_ratio: 1.5 };

            ctx.set_style(style);

            self.counter = 0;

            self.y_offset = 0.;
            for y in 0..4 {
                if y > 1 {
                    self.y_offset = ((self.screen_size.y / 100.) * 50.);
                }
                for x in 0..8 {
                    if y == 3 || y == 1 {
                        match self.fader[self.counter as usize].id {
                            Some(T) => {
                                self.draw_slider_bank(
                                    ui,
                                    Vec2 {
                                        x: (((self.screen_size.x / 100.) * (5.5 * x as f32))
                                            + ((self.screen_size.x / 100.) * 50.)),
                                        y: self.y_offset,
                                    },
                                    true,
                                );
                            }
                            None => {
                                self.draw_slider_bank(
                                    ui,
                                    Vec2 {
                                        x: (((self.screen_size.x / 100.) * (5.5 * x as f32))
                                            + ((self.screen_size.x / 100.) * 50.)),
                                        y: self.y_offset,
                                    },
                                    false,
                                );
                            }
                        }
                    } else {
                        match self.fader[self.counter as usize].id {
                            Some(T) => {
                                self.draw_slider_bank(
                                    ui,
                                    Vec2 {
                                        x: ((self.screen_size.x / 100.) * 5.5) * x as f32,
                                        y: self.y_offset,
                                    },
                                    true,
                                );
                            }
                            None => {
                                self.draw_slider_bank(
                                    ui,
                                    Vec2 {
                                        x: ((self.screen_size.x / 100.) * 5.5) * x as f32,
                                        y: self.y_offset,
                                    },
                                    false,
                                );
                            }
                        }
                    }
                    self.counter += 1
                }
            }

            self.draw_slider_group(
                ui,
                Vec2 {
                    x: (((self.screen_size.x / 100.) * (5.5 * 9.))
                        + ((self.screen_size.x / 100.) * 45.)),
                    y: (self.screen_size.y / 100.) * 25.,
                },
            );
            if self.draw_button(
                ui,
                Vec2 {
                    x: (((self.screen_size.x / 100.) * (5.5 * 9.))
                        + ((self.screen_size.x / 100.) * 45.)),
                    y: (self.screen_size.y / 100.) * 65.,
                },
                "Reset   Sel".into(),
                None,
            ) {
                for i in 0..32 {
                    self.fader[i].fader_selected = false;
                }
            }
            if self.draw_button(
                ui,
                Vec2 {
                    x: (((self.screen_size.x / 100.) * (5.5 * 9.))
                        + ((self.screen_size.x / 100.) * 45.)),
                    y: (self.screen_size.y / 100.) * 72.5,
                },
                "GR Sel".into(),
                Some(self.group_select),
            ) {
                self.group_select = !self.group_select;
            }
            if self.draw_button(
                ui,
                Vec2 {
                    x: (((self.screen_size.x / 100.) * (5.5 * 9.))
                        + ((self.screen_size.x / 100.) * 45.)),
                    y: (self.screen_size.y / 100.) * 17.5,
                },
                "Clear".into(),
                None,
            ) {
                for i in 0..32 {
                    self.fader[i].fader_selected = false;
                }
                self.button_pressed = [false; 32];
                self.fader_manual_override = [false; 32];
                for i in 0..32 {
                    self.fader[i].fader_value = 0;
                }
                self.group_select = false;
                self.group_value = 0;
            }
            if self.draw_button(
                ui,
                Vec2 {
                    x: (((self.screen_size.x / 100.) * (5.5 * 9.))
                        + ((self.screen_size.x / 100.) * 45.)),
                    y: (self.screen_size.y / 100.) * 80.,
                },
                "Setup".into(),
                Some(self.setup_toggle),
            ) {
                self.setup_toggle = !self.setup_toggle;
                if self.setup_toggle {
                    for i in 0..=31 {
                        self.selection_save[i] = self.fader[i].fader_selected;
                        self.fader_value_save[i] = self.fader[i].fader_value;
                    }
                } else {
                    for i in 0..=31 {
                        self.fader[i].fader_selected = self.selection_save[i];
                        self.fader[i].fader_value = self.fader_value_save[i];
                    }
                }
            }

            if self.setup_toggle {
                self.setup_window.show(ctx, &mut self.fader);
                self.group_select = false;
            }
        });
    }

    fn draw_slider_group(&mut self, ui: &mut Ui, offset: Vec2) {
        let widget_rect = Rect::from_min_size(ui.min_rect().min + offset, Vec2 { x: 20., y: 20. });
        ui.put(
            widget_rect,
            egui::Slider::new(&mut self.group_value, 0..=255)
                .show_value(false)
                .orientation(egui::SliderOrientation::Vertical), //.handle_shape(egui::style::HandleShape::Rect { aspect_ratio: 1.5 })
        );
        //let button_rect =
        //Rect::from_min_size(ui.min_rect().min + offset + Vec2 { x: 0., y: 290.}, Vec2 { x: 40., y: 20.});
    }

    fn draw_button(
        &mut self,
        ui: &mut Ui,
        offset: Vec2,
        name: String,
        mut pressed: Option<bool>,
    ) -> bool {
        let widget_rect = Rect::from_min_size(
            ui.min_rect().min + offset,
            Vec2 {
                x: ((self.screen_size.y / 100.) * 6.),
                y: ((self.screen_size.y / 100.) * (((self.screen_size.x / 100.) * 3.) / 100.)),
            },
        );
        let button = ui.put(widget_rect, Button::new(name));
        if pressed != None {
            if button.clicked() {
                pressed = Some(!pressed.unwrap());
            }
            if pressed.unwrap() {
                self.draw_frame(
                    "".into(),
                    ui,
                    Vec2 {
                        x: ((self.screen_size.y / 100.) * 1.),
                        y: ((self.screen_size.y / 100.)
                            * (((self.screen_size.y / 100.) * 10.) / 100.)),
                    },
                    offset
                        + Vec2 {
                            x: ((self.screen_size.y / 100.) * 2.5),
                            y: ((self.screen_size.y / 100.) * 0.5),
                        },
                    Color32::from_rgb(255, 200, 120),
                );
            }
        }

        return button.clicked();
    }

    fn draw_slider_bank(&mut self, ui: &mut Ui, offset: Vec2, patched: bool) {
        let widget_rect = Rect::from_min_size(ui.min_rect().min + offset, Vec2 { x: 20., y: 20. });
        ui.put(
            widget_rect,
            egui::Slider::new(&mut self.fader[self.counter as usize].fader_value, 0..=255)
                .show_value(false)
                .orientation(egui::SliderOrientation::Vertical), //.handle_shape(egui::style::HandleShape::Rect { aspect_ratio: 1.5 })
        );

        let button_rect = Rect::from_min_size(
            ui.min_rect().min
                + offset
                + Vec2 {
                    x: 0.,
                    y: ((self.screen_size.y / 100.) * 40.),
                },
            Vec2 {
                x: ((self.screen_size.y / 100.) * 6.),
                y: ((self.screen_size.y / 100.) * (((self.screen_size.x / 100.) * 3.) / 100.)),
            },
        );
        /* if ui.put(button_rect, Button::new("name")).t{
            ui.put(button_rect, Button::new("test"));
        } */

        // Style the button based on state

        let button = Button::new(format!("{}", self.counter + 1));

        let index = self.counter as usize;

        // === Handle manual selection button ===
        if ui.put(button_rect, button).clicked() {
            // Flip the current selection state
            self.fader[index].fader_selected = !self.fader[index].fader_selected;

            // Manually control it now
            self.fader_manual_override[index] = true;

            // Update button to match the selection
            self.button_pressed[index] = self.fader[index].fader_selected;
        }

        if self.fader[self.counter as usize].fader_selected {
            // Apply red fill when clicked
            /* button = button.fill(Color32::DARK_RED); */
            if self.setup_toggle || !patched {
                if !patched {
                    self.draw_frame(
                        "".into(),
                        ui,
                        Vec2 {
                            x: ((self.screen_size.y / 100.) * 1.),
                            y: ((self.screen_size.y / 100.)
                                * (((self.screen_size.y / 100.) * 10.) / 100.)),
                        },
                        offset
                            + Vec2 {
                                x: ((self.screen_size.y / 100.) * 2.5),
                                y: ((self.screen_size.y / 100.) * 40.5),
                            },
                        Color32::from_rgb(255, 0, 0),
                    );
                } else {
                    self.draw_frame(
                        "".into(),
                        ui,
                        Vec2 {
                            x: ((self.screen_size.y / 100.) * 1.),
                            y: ((self.screen_size.y / 100.)
                                * (((self.screen_size.y / 100.) * 10.) / 100.)),
                        },
                        offset
                            + Vec2 {
                                x: ((self.screen_size.y / 100.) * 2.5),
                                y: ((self.screen_size.y / 100.) * 40.5),
                            },
                        Color32::from_rgb(255, 200, 120),
                    );
                }
            } else {
                self.draw_frame(
                    "".into(),
                    ui,
                    Vec2 {
                        x: ((self.screen_size.y / 100.) * 1.),
                        y: ((self.screen_size.y / 100.)
                            * (((self.screen_size.y / 100.) * 10.) / 100.)),
                    },
                    offset
                        + Vec2 {
                            x: ((self.screen_size.y / 100.) * 2.5),
                            y: ((self.screen_size.y / 100.) * 40.5),
                        },
                    Color32::from_rgb(255, 200, 120),
                );
            }

            if self.group_select {
                self.fader[self.counter as usize].fader_value = self.group_value as u8;
            }
        }

        // === Auto-selection fallback ===
        if !self.fader_manual_override[index] {
            self.fader[index].fader_selected = self.fader[index].fader_value > 0;
        }

        // === Optional: Exit manual mode if fader returns to 0 and button not pressed ===
        if self.fader[index].fader_value == 0
            && self.fader_manual_override[index]
            && !self.button_pressed[index]
        {
            self.fader_manual_override[index] = false;
        }

        if self.fader[index].fader_value == 0
            && self.fader_manual_override[index]
            && !self.button_pressed[index]
        {
            self.fader_manual_override[index] = false;
        }
    }

    fn draw_frame(&self, name: String, ui: &mut Ui, size: Vec2, offset: Vec2, color: Color32) {
        let pos = ui.min_rect().min + offset;
        let rect = Rect::from_min_size(pos, size);
        ui.allocate_rect(rect, egui::Sense::hover()); // Reserve space

        // Paint background manually

        ui.painter().rect_filled(
            rect,
            5.0,                                                // corner radius
            Color32::from_rgb(color.r(), color.g(), color.b()), // background color
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
