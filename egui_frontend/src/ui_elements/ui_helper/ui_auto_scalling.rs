use eframe::egui::Color32;
use eframe::egui::{self, CentralPanel, ColorImage, Context, Slider, TextureHandle, Vec2, Window, Rect, Pos2};
use eframe::emath::Numeric;

pub struct AutoScaller{
    rect: Rect,
    panel_resolution: Vec2,

}

impl AutoScaller{
    pub fn new() -> Self{
        Self{
            rect: Rect::from_min_max(Pos2::ZERO, egui::pos2(0.0, 0.0)),
            panel_resolution: Vec2 { x: 0., y: 0. },
        }
    }
    pub fn get_rect(&mut self, rect: Rect, is_fader: bool, fader_id: u8) -> Rect{
        self.rect = rect;
        self.panel_resolution = Vec2 { x: self.rect.max.x - self.rect.min.x + 16., y: self.rect.max.y  - self.rect.min.y + 16.};

        let id = fader_id;

        let mut min = 0.;
        let mut max = 0.;

        let mut bank_id = 0;

        let mut reserved_panel_space;

        if is_fader{
            match fader_id {
                0..=5   => reserved_panel_space = Rect::from_min_max(
                            Pos2 { x: 0., y: 0. }, 
                            Pos2{x: self.panel_resolution.x / 2. - self.panel_resolution.x / 13., y: self.panel_resolution.y / 2. - self.panel_resolution.y / 10. }
                            ),
                6..=11  => reserved_panel_space = Rect::from_min_max(
                            Pos2 { x: self.panel_resolution.x / 2. - self.panel_resolution.x / 26., y: 0.},
                            Pos2{x: self.panel_resolution.x / 2. - self.panel_resolution.x / 13., y: self.panel_resolution.y / 2. - self.panel_resolution.y / 10. }
                            ),
                12..=17 => reserved_panel_space = Rect::from_min_max(
                            Pos2 { x: 0., y: self.panel_resolution.y / 2. }, 
                            Pos2{x: self.panel_resolution.x / 2. - self.panel_resolution.x / 13., y: self.panel_resolution.y / 2. - self.panel_resolution.y / 10. }
                            ),
                18..=24 => reserved_panel_space = Rect::from_min_max(
                            Pos2 { x: self.panel_resolution.x / 2. - self.panel_resolution.x / 26., y: self.panel_resolution.y / 2.}, 
                            Pos2{x: self.panel_resolution.x / 2. - self.panel_resolution.x / 13., y: self.panel_resolution.y / 2. - self.panel_resolution.y / 10. }
                            ),
                _ => reserved_panel_space = Rect::from_min_size(Pos2::ZERO, egui::vec2(0.0, 0.0))
                
            }

            return self.calculate_fader_grid_pos(reserved_panel_space, id);
        }else{
            match fader_id {
                0..=5   => reserved_panel_space = Rect::from_min_max(
                            Pos2{x: 0., y: self.panel_resolution.y / 2. - self.panel_resolution.y / 10. }, 
                            Pos2{x: self.panel_resolution.x / 2. - self.panel_resolution.x / 13., y: self.panel_resolution.y / 10. }
                            ),
                6..=11  => reserved_panel_space = Rect::from_min_max(
                            Pos2{x: self.panel_resolution.x / 2. - self.panel_resolution.x / 26., y: self.panel_resolution.y / 2. - self.panel_resolution.y / 10. }, 
                            Pos2{x: self.panel_resolution.x / 2. - self.panel_resolution.x / 13., y: self.panel_resolution.y / 10. }
                            ),
                12..=17 => reserved_panel_space = Rect::from_min_max(
                            Pos2{x: 0., y: self.panel_resolution.y - self.panel_resolution.y / 10.}, 
                            Pos2{x: self.panel_resolution.x / 2. - self.panel_resolution.x / 13., y: self.panel_resolution.y / 10. }
                            ),
                18..=24 => reserved_panel_space = Rect::from_min_max(
                            Pos2{x: self.panel_resolution.x / 2. - self.panel_resolution.x / 26., y: self.panel_resolution.y - self.panel_resolution.y / 10.}, 
                            Pos2{x: self.panel_resolution.x / 2. - self.panel_resolution.x / 13., y: self.panel_resolution.y / 10. }
                            ),
                _ => reserved_panel_space = Rect::from_min_size(Pos2::ZERO, egui::vec2(0.0, 0.0))
                
            }

            return self.calculate_button_grid_pos(reserved_panel_space, id)
        }

    }

    pub fn get_ctrl_button(&mut self, fader_id: u8, rect: Rect) -> Rect{
        self.rect = rect;
        let panel_resolution = Vec2 { x: self.rect.max.x, y: self.rect.max.y  - self.rect.min.y + 16.};
        let button_starting_min = (panel_resolution.x - panel_resolution.x / 13. + panel_resolution.x / 65.);

        match fader_id {
            0 => return Rect::from_min_max(
                Pos2 { x: button_starting_min, y: panel_resolution.y / 2.},
                Pos2 { x: button_starting_min + self.get_cell_size().x / 2., y: panel_resolution.y/ 2. + (button_starting_min - (button_starting_min + self.get_cell_size().x / 2.))}
            ),
            1 => return Rect::from_min_max(
                Pos2 { x: button_starting_min, y: panel_resolution.y / 2. + ((panel_resolution.y/2.) / 4.) },
                Pos2 { x: button_starting_min + self.get_cell_size().x / 2., y: panel_resolution.y/ 2. + ((panel_resolution.y/2.) / 4.) + (button_starting_min - (button_starting_min + self.get_cell_size().x / 2.))}
            ),
            2 => return Rect::from_min_max(
                Pos2 { x: button_starting_min, y: panel_resolution.y / 2. + ((panel_resolution.y/2.) / 4.) * 2.},
                Pos2 { x: button_starting_min + self.get_cell_size().x / 2., y: panel_resolution.y/ 2. + ((panel_resolution.y/2.) / 4.) *2. + (button_starting_min - (button_starting_min + self.get_cell_size().x / 2.))}
            ),
            3 => return Rect::from_min_max(
                Pos2 { x: button_starting_min, y: panel_resolution.y / 2. + ((panel_resolution.y/2.) / 4.) * 3.},
                Pos2 { x: button_starting_min + self.get_cell_size().x / 2., y: panel_resolution.y/ 2. + ((panel_resolution.y/2.) / 4.) *3. + (button_starting_min - (button_starting_min + self.get_cell_size().x / 2.))}
            ),
            _ => return Rect::from_min_max(
                Pos2 { x: (self.panel_resolution.x / 2. + self.panel_resolution.x / 2. - self.panel_resolution.x / 26.) + ((self.rect.max.x - (self.panel_resolution.x / 2. + self.panel_resolution.x / 2. - self.panel_resolution.x / 26.))  / 4.), y: self.panel_resolution.y / 2. },
                Pos2 { x: self.rect.max.x -((self.rect.max.x - (self.panel_resolution.x / 2. + self.panel_resolution.x / 2. - self.panel_resolution.x / 26.))  / 4.), y: self.rect.max.y }
            )
    
        }
        
    }

    fn calculate_fader_grid_pos(&mut self,reserved_panel_space: Rect, fader_id: u8) -> Rect{
        let grid_size_x = 6;
        let grid_size_y = 1;

        let minimal_cell_size_x: f32 = 25.;
        let minimal_cell_size_y: f32 = 100.;

        let mut offset = Vec2 { x: 0., y: 0. };

        let mut cell_length_x = reserved_panel_space.max.x / grid_size_x as f32;

            /* if minimal_cell_size_x > cell_length_x{
                let mut counter: f32 = 0.;
                while minimal_cell_size_x > cell_length_x{
                    counter += 1.;
                    if (counter) == grid_size_x as f32{
                        break;
                    }
                    cell_length_x=  reserved_panel_space.max.x / (grid_size_x as f32 - counter);
                }
            } */
            let mut cell_length_y = reserved_panel_space.max.y / grid_size_y as f32;

            /* if minimal_cell_size_y > cell_length_y{
                let mut counter: f32 = 0.;
                while minimal_cell_size_y > cell_length_y{
                    counter += 1.;
                    if (counter) == grid_size_y as f32{
                        break;
                    }
                    cell_length_y=  reserved_panel_space.max.y / (grid_size_y as f32 - counter);
                }
            } */

            
            

        let size = Vec2 { x: cell_length_x, y: cell_length_y };

        

        match fader_id{
            0..=5   => offset = Vec2 { x: cell_length_x * (fader_id as f32) + reserved_panel_space.min.x + (cell_length_x / 4.), y: cell_length_y * 0 as f32 + reserved_panel_space.min.y},
            6..=11  => offset = Vec2 { x: cell_length_x * (fader_id as f32 - 6.) + reserved_panel_space.min.x + (cell_length_x / 4.), y: cell_length_y * 0 as f32 + reserved_panel_space.min.y},
            12..=17 => offset = Vec2 { x: cell_length_x * (fader_id as f32 - 12.) + reserved_panel_space.min.x + (cell_length_x / 4.), y: cell_length_y * 0 as f32 + reserved_panel_space.min.y},
            18..=24 => offset = Vec2 { x: cell_length_x * (fader_id as f32 - 18.) + reserved_panel_space.min.x + (cell_length_x / 4.), y: cell_length_y * 0 as f32 + reserved_panel_space.min.y},
            _ => offset = Vec2 { x: 0., y: 0. },
        }
        
        


        return Rect::from_min_size(self.rect.min /* + Vec2{x:-8.,y:-8.} */ + offset, size);
    }

    fn calculate_button_grid_pos(&mut self,reserved_panel_space: Rect, fader_id: u8) -> Rect{
        let grid_size_x = 6;
        let grid_size_y = 1;

        let minimal_cell_size_x: f32 = 25.;
        let minimal_cell_size_y: f32 = 100.;

        let mut offset = Vec2 { x: 0., y: 0. };

        let mut cell_length_x = reserved_panel_space.max.x / grid_size_x as f32;

            /* if minimal_cell_size_x > cell_length_x{
                let mut counter: f32 = 0.;
                while minimal_cell_size_x > cell_length_x{
                    counter += 1.;
                    if (counter) == grid_size_x as f32{
                        break;
                    }
                    cell_length_x=  reserved_panel_space.max.x / (grid_size_x as f32 - counter);
                }
            } */
            let mut cell_length_y = reserved_panel_space.max.y / grid_size_y as f32;

            /* if minimal_cell_size_y > cell_length_y{
                let mut counter: f32 = 0.;
                while minimal_cell_size_y > cell_length_y{
                    counter += 1.;
                    if (counter) == grid_size_y as f32{
                        break;
                    }
                    cell_length_y=  reserved_panel_space.max.y / (grid_size_y as f32 - counter);
                }
            } */

            
            

        let size = Vec2 { x: cell_length_x/2., y: cell_length_y/ 2. };

        

        match fader_id{
            0..=5   => offset = Vec2 { x: cell_length_x * (fader_id as f32) + reserved_panel_space.min.x + (cell_length_x / 3.5), y: cell_length_y * 0 as f32 + reserved_panel_space.min.y},
            6..=11  => offset = Vec2 { x: cell_length_x * (fader_id as f32 - 6.) + reserved_panel_space.min.x + (cell_length_x / 3.5), y: cell_length_y * 0 as f32 + reserved_panel_space.min.y},
            12..=17 => offset = Vec2 { x: cell_length_x * (fader_id as f32 - 12.) + reserved_panel_space.min.x + (cell_length_x / 3.5), y: cell_length_y * 0 as f32 + reserved_panel_space.min.y},
            18..=24 => offset = Vec2 { x: cell_length_x * (fader_id as f32 - 18.) + reserved_panel_space.min.x + (cell_length_x / 3.5), y: cell_length_y * 0 as f32 + reserved_panel_space.min.y},
            _ => offset = Vec2 { x: 0., y: 0. },
        }
        
        


        return Rect::from_min_size(self.rect.min + Vec2{x:0.,y:8.} + offset, size);
    }

    pub fn get_cell_size(&mut self) -> Vec2{
        
        let reserved_panel_space_max = Pos2{x: self.panel_resolution.x / 2. - self.panel_resolution.x / 13., y: self.panel_resolution.y / 2. - self.panel_resolution.y / 10. };

        Vec2 { x: reserved_panel_space_max.x / 6., y: reserved_panel_space_max.y }
    }
    
}