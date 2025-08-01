use eframe::egui::Color32;
use eframe::egui::{self, CentralPanel, ColorImage, Context, Slider, TextureHandle, Vec2, Window};
use eframe::emath::Numeric;

pub struct ColorPickerWindow {
    pub selected_color: egui::Color32,
    pub texture: egui::TextureHandle,
    pub width: usize,
    pub height: usize,
    pub anchored: bool,
}

impl ColorPickerWindow {
    pub fn new(ctx: &egui::Context) -> Self {
        let width = 400;
        let height = 255;
        let image = generate_color_strip_image(width, height);
        let texture = ctx.load_texture("color_picker", image, egui::TextureOptions::LINEAR);

        Self {
            selected_color: egui::Color32::WHITE,
            texture,
            width,
            height,
            anchored: true,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        egui::Window::new("Color Picker").movable(self.anchored).show(ctx, |ui| {
            ui.heading("Color Picker");

            let image_size = self.texture.size_vec2();
            let (rect, response) = ui.allocate_exact_size(image_size, egui::Sense::click());

            /* draw_texture_with_circle(ui, self.texture.id(), egui::vec2(self.width as f32, self.height as f32)); */
            ui.painter().image(
                self.texture.id(),
                rect,
                egui::Rect::from_min_max(
                    egui::Pos2::new(0.0, 0.0),
                    egui::Pos2::new(1.0, 1.0),
                ),
                egui::Color32::WHITE,
            );
            if (ui.input(|i| i.pointer.primary_down())) {

                if let Some(click_pos) = ui.input(|i| i.pointer.latest_pos()) {
                    let relative_x = (click_pos.x - rect.min.x) / rect.width();
                    let relative_y = (click_pos.y - rect.min.y) / rect.height();

                    let pixel_x = (relative_x * self.width as f32).floor() as usize;
                    let pixel_y = (relative_y * self.height as f32).floor() as usize;
                    if !(pixel_y > self.height || pixel_y <= 0 || pixel_x > self.width || pixel_x <= 0){
                        self.anchored = false;
                        self.selected_color = hue_at_pixel(pixel_x, self.width, 255, pixel_y as u8);
                    }


                }
            }else {
                self.anchored = true;
            }

            ui.separator();
            ui.colored_label(self.selected_color, "Selected Color");
            ui.label(format!(
                "Hex: #{:02X}{:02X}{:02X}",
                self.selected_color.r(),
                self.selected_color.g(),
                self.selected_color.b()
            ));
      ui.checkbox(&mut self.anchored, "Window Movable")
        });
    }
}

/* fn draw_texture_with_circle(ui: &mut egui::Ui, texture_id: egui::TextureId, texture_size: egui::Vec2) {
    // Allocate space for the image and get response
    let (response, painter) = ui.allocate_painter(texture_size, egui::Sense::click());

    // Draw the texture
    ui.painter().image(
        texture_id,
        response.rect,
        egui::Rect::from_min_max(
            egui::Pos2::new(0.0, 0.0),
            egui::Pos2::new(1.0, 1.0),
        ),
        egui::Color32::WHITE,
    );

    // Draw the circle over the texture
    let circle_center = response.rect.center() + Vec2{x:200.0,y:0.0}; // or pick any point in response.rect
    let radius = 10.0;

    painter.circle_stroke(
        circle_center,
        radius,
        egui::Stroke::new(3.0, egui::Color32::RED),
    );
} */

pub fn generate_color_strip_image(width: usize, height: usize) -> ColorImage {
    let mut pixels = Vec::with_capacity(width * height);
    let mut pixel_height: u8 = 255;
    let mut zeropixel_height: u8 = 0;

    for y in 0..height {
        for x in 0..width {
            let color = hue_at_pixel(x, width, pixel_height, zeropixel_height);

            pixels.push(color);
        }

        //pixel_height -= 1;
        zeropixel_height += 1;
    }

    ColorImage {
        size: [width, height],
        pixels,
    }
}

pub fn hue_at_pixel(x: usize, width: usize, pixel_height: u8, zeropixel_height: u8) -> Color32 {
    let t = x as f32 / (width - 1) as f32;

    let color = match t {
        t if t <= 1.0 / 6.0 => lerp_rgb(
            (pixel_height, zeropixel_height, zeropixel_height),
            (pixel_height, pixel_height, zeropixel_height),
            t * 6.0,
        ),
        t if t <= 2.0 / 6.0 => lerp_rgb(
            (pixel_height, pixel_height, zeropixel_height),
            (zeropixel_height, pixel_height, zeropixel_height),
            (t - 1.0 / 6.0) * 6.0,
        ),
        t if t <= 3.0 / 6.0 => lerp_rgb(
            (zeropixel_height, pixel_height, zeropixel_height),
            (zeropixel_height, pixel_height, pixel_height),
            (t - 2.0 / 6.0) * 6.0,
        ),
        t if t <= 4.0 / 6.0 => lerp_rgb(
            (zeropixel_height, pixel_height, pixel_height),
            (zeropixel_height, zeropixel_height, pixel_height),
            (t - 3.0 / 6.0) * 6.0,
        ),
        t if t <= 5.0 / 6.0 => lerp_rgb(
            (zeropixel_height, zeropixel_height, pixel_height),
            (pixel_height, zeropixel_height, pixel_height),
            (t - 4.0 / 6.0) * 6.0,
        ),
        _ => lerp_rgb(
            (pixel_height, zeropixel_height, pixel_height),
            (pixel_height, zeropixel_height, zeropixel_height),
            (t - 5.0 / 6.0) * 6.0,
        ),
    };
    return color;
}

/// Linear interpolation between two RGB tuples
pub fn lerp_rgb(from: (u8, u8, u8), to: (u8, u8, u8), t: f32) -> Color32 {
    let lerp = |a, b| a as f32 + (b as f32 - a as f32) * t;
    Color32::from_rgb(
        lerp(from.0, to.0) as u8,
        lerp(from.1, to.1) as u8,
        lerp(from.2, to.2) as u8,
    )
}

pub fn color_picker() {}
