use eframe::egui::Color32;
use eframe::egui::{self, CentralPanel, ColorImage, Context, Slider, TextureHandle, Vec2, Window};
use egui::{Frame, Ui, Button, Rect, Margin};

pub struct FixtureManager{
    
    pub length: u8,
    pub height: u8,
    pub fixture: [Fixture; 12],
    
}

struct Fixture{
    is_used: bool,
    name: Option<String>,
    fixture_length: Option<u16>,
    fixture_start: Option<u16>,
    fixture_end: Option<u16>,
    
}


impl FixtureManager{
    pub fn new(ctx: &egui::Context) -> Self {
        let length: u8 = 6;
        let height: u8 = 2;
        Self {
            length,
            height,
            fixture: [
            Fixture{
                is_used: false,
                name: None,
                fixture_length: None,
                fixture_start: None,
                fixture_end: None,},
            Fixture{
                is_used: false,
                name: None,
                fixture_length: None,
                fixture_start: None,
                fixture_end: None,},
            Fixture{
                is_used: true,
                name: Some("Hello".into()),
                fixture_length: None,
                fixture_start: None,
                fixture_end: None,},
            Fixture{
                is_used: true,
                name: Some("Hello".into()),
                fixture_length: None,
                fixture_start: None,
                fixture_end: None,},
            Fixture{
                    is_used: false,
                    name: None,
                    fixture_length: None,
                    fixture_start: None,
                    fixture_end: None,},
            Fixture{
                    is_used: false,
                    name: None,
                    fixture_length: None,
                    fixture_start: None,
                    fixture_end: None,},
            Fixture{
                    is_used: true,
                    name: Some("Hallo".into()),
                    fixture_length: Some(2),
                    fixture_start: Some(6),
                    fixture_end: Some(7),},
            Fixture{
                    is_used: true,
                    name: Some("Hello".into()),
                    fixture_length: Some(2),
                    fixture_start: Some(6),
                    fixture_end: Some(7),},
            Fixture{
                is_used: true,
                name: Some("Hello".into()),
                fixture_length: None,
                fixture_start: None,
                fixture_end: None,},
            Fixture{
                is_used: true,
                    name: Some("Hello".into()),
                    fixture_length: Some(3),
                    fixture_start: Some(9),
                    fixture_end: Some(11),},
            Fixture{
                is_used: true,
                    name: Some("Hello".into()),
                    fixture_length: Some(3),
                    fixture_start: Some(9),
                    fixture_end: Some(11),},
            Fixture{
                is_used: true,
                    name: Some("Hello".into()),
                    fixture_length: Some(3),
                    fixture_start: Some(9),
                    fixture_end: Some(11),},
                    ],
        }
    }   
    pub fn show(&mut self, ctx: &egui::Context){
        egui::Window::new("Fixture Manager").resizable(true).show(ctx, |ui|{

            /* let image_size = self.texture.show to get ize_vec2();
            let (rect, response) = ui.allocate_exact_size(image_size, egui::Sense::click());


            ui.painter().image(
                self.texture.id(),
                rect,
                egui::Rect::from_min_max(
                    egui::Pos2::new(0.0, 0.0),
                    egui::Pos2::new(1.0, 1.0),
                ),
                egui::Color32::WHITE,
            ); */

            //ui.label("Hello World!");
            
            let mut i:u16 = 0;
            for y in 0..self.height{
                
                for x in 0..self.length{
                    
                    if self.fixture.get(i as usize).unwrap().is_used == false{
                        self.draw_button((i + 1).to_string() ,ui, Vec2{x: 50., y: 50.}, Vec2{x: x as f32 * 55., y: y as f32 * 55.});
                    }else {
                        if let Some(fixture) = self.fixture.get(i as usize) {
                            // fixture is &Fixture
                            
                            if let Some(name) = fixture.name.as_ref() {
                                if fixture.fixture_length == None{
                                    self.draw_frame( name.to_string() ,ui, Vec2 { x: 50., y: 50. },Vec2{x: x as f32 * 55.,y: y as f32 * 55.});
                                } else{
                                    let length_of_fixture = fixture.fixture_length.unwrap();
                                    if fixture.fixture_start.unwrap() == i{
                                        self.draw_frame( name.to_string() ,ui, Vec2 { x: ((50 * length_of_fixture) + ((length_of_fixture - 1) * 5)) as f32, y: 50. },Vec2{x: x as f32 * 55.,y: y as f32 * 55.});
                                    }
                                    
                                }
                                
                            }
                        }
                        
                    }
                    i += 1; 
                }
            }


            //self.draw_frame("".into(), ui, 25, 25);
            
        });
    }

    fn draw_frame(&self, name: String, ui: &mut Ui, size: Vec2, offset: Vec2) {
        let pos = ui.min_rect().min + offset;
        let rect = Rect::from_min_size(pos, size);
        ui.allocate_rect(rect, egui::Sense::hover()); // Reserve space
    
        // Paint background manually
        ui.painter().rect_filled(
            rect,
            5.0, // corner radius
            Color32::from_rgb(255, 0, 0), // background color
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

    fn draw_button(&self,name: String, ui: &mut Ui, size: Vec2, offset: Vec2) {
        let widget_rect = Rect::from_min_size(  
            ui.min_rect().min + offset,
            size,
        );
        ui.put(widget_rect, Button::new(name));
    }
}




fn grid_generator(width: usize, height:usize)-> ColorImage{
    let mut pixels = Vec::with_capacity(width * height);


    for y in 0..height{
        for x in 0..width{
            if x%50 == 0|| y%50 == 0{
                pixels.push(Color32::GREEN);
            }else{

                pixels.push(Color32::BLACK);
            }
        }
    }

    ColorImage {
        size: [width, height],
        pixels,
    }
}