use egui::{
    color_picker::{color_edit_button_rgba, Alpha},
    Color32, Pos2, Rgba, Rounding, Sense, Ui,
};
use rand::{Rng, SeedableRng};

pub struct Maze {
    /// Seed used to generate the maze
    seed: u64,
    /// Should we enclose the maze before generating it
    enclosed: bool,
    /// Color of the walls
    wall_color: Color32,
    /// Color of the paths
    path_color: Color32,
    /// Dimensions in number of cells
    dimensions: (usize, usize),
    /// Size of every square of the path
    size: usize,
}

impl Maze {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        // we're going to keep the range of possible initial seed small so it renders well on screen
        let seed = rng.gen_range(0..9999);

        Maze {
            seed,
            wall_color: Color32::GOLD,
            path_color: Color32::TRANSPARENT,
            dimensions: (15, 15),
            size: 30,
            enclosed: false,
        }
    }

    pub fn configuration(&mut self, ui: &mut Ui) -> egui::Response {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Wall:");
                let mut rgba = self.wall_color.into();
                color_edit_button_rgba(ui, &mut rgba, Alpha::Opaque);
                self.wall_color = rgba.into();

                ui.label("Path:");
                let mut rgba = self.path_color.into();
                color_edit_button_rgba(ui, &mut rgba, Alpha::Opaque);
                self.path_color = rgba.into();
            });
            ui.label("Rng:");
            ui.add(egui::DragValue::new(&mut self.seed).speed(1));
            if ui.add(egui::Button::new("regenerate")).clicked() {
                let mut rng = rand::thread_rng();
                self.seed = rng.gen_range(0..9999);
            };

            ui.label("Cell size:");
            ui.add(egui::DragValue::new(&mut self.size).speed(1));

            ui.label("Maze width:");
            ui.add(egui::DragValue::new(&mut self.dimensions.0).speed(1));
            ui.label("Maze height:");
            ui.add(egui::DragValue::new(&mut self.dimensions.1).speed(1));
        })
        .response
    }

    pub fn draw_on(&mut self, ui: &mut Ui) -> egui::Response {
        let mut rng = rand::rngs::StdRng::seed_from_u64(self.seed);
        let mut window = window_rs::WindowBuffer::new(self.dimensions.0, self.dimensions.1);
        maze::MazeConfig {
            path_color: u32::from_ne_bytes(self.path_color.to_array()),
            wall_color: u32::from_ne_bytes(self.wall_color.to_array()),
        }
        .generate(&mut window, &mut rng);

        let (response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

        ui.set_min_width((window.width() * self.size) as f32);
        ui.set_min_height((window.height() * self.size) as f32);

        for x in 0..window.width() {
            for y in 0..window.height() {
                let color = window[(x, y)];

                let rect = egui::Rect {
                    min: Pos2 {
                        x: x as f32 * self.size as f32,
                        y: y as f32 * self.size as f32,
                    },
                    max: Pos2 {
                        x: (x + 1) as f32 * self.size as f32,
                        y: (y + 1) as f32 * self.size as f32,
                    },
                };
                // println!("drawing rectangle {rect:?}");
                let [r, g, b, a] = color.to_ne_bytes();
                painter.rect_filled(
                    rect,
                    Rounding::ZERO,
                    Color32::from_rgba_premultiplied(r, g, b, a),
                );
            }
        }

        response
    }
}

impl eframe::App for Maze {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                self.configuration(ui);
                self.draw_on(ui);
            });
        });
    }
}
