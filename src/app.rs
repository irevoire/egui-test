use egui::{
    color_picker::{color_edit_button_rgba, Alpha},
    Color32, Pos2, Rounding, Sense, Ui, Vec2,
};
use rand::{Rng, SeedableRng};

pub struct Maze {
    /// Seed used to generate the maze
    seed: u64,
    /// Number of opened walls
    opened_walls: usize,
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
            opened_walls: 0,
            wall_color: Color32::GOLD,
            path_color: Color32::TRANSPARENT,
            dimensions: (15, 15),
            size: 30,
        }
    }

    pub fn configuration(&mut self, ui: &mut Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.label("Wall:");
            let mut rgba = self.wall_color.into();
            color_edit_button_rgba(ui, &mut rgba, Alpha::Opaque);
            self.wall_color = rgba.into();

            ui.label("Path:");
            let mut rgba = self.path_color.into();
            color_edit_button_rgba(ui, &mut rgba, Alpha::Opaque);
            self.path_color = rgba.into();

            ui.separator();

            ui.label("Rng:");
            ui.add(egui::DragValue::new(&mut self.seed).speed(1));
            if ui.add(egui::Button::new("regenerate")).clicked() {
                let mut rng = rand::thread_rng();
                self.seed = rng.gen_range(0..9999);
            };
            ui.label("Opened walls:");
            ui.add(egui::DragValue::new(&mut self.opened_walls).speed(1));

            ui.separator();

            ui.label("Cell size:");
            ui.add(egui::DragValue::new(&mut self.size).speed(1));
            ui.label("Maze width:");
            ui.add(egui::DragValue::new(&mut self.dimensions.0).speed(1));
            ui.label("Maze height:");
            ui.add(egui::DragValue::new(&mut self.dimensions.1).speed(1));
        })
        .response
    }

    pub fn draw_on(&mut self, ui: &mut Ui) {
        let mut rng = rand::rngs::StdRng::seed_from_u64(self.seed);
        let mut window = window_rs::WindowBuffer::new(self.dimensions.0, self.dimensions.1);
        maze::MazeConfig {
            path_color: u32::from_ne_bytes(self.path_color.to_array()),
            wall_color: u32::from_ne_bytes(self.wall_color.to_array()),
            open_walls: self.opened_walls,
        }
        .generate(&mut window, &mut rng);

        egui::ScrollArea::both().show(ui, |ui| {
            let (response, painter) = ui.allocate_painter(
                Vec2::new(
                    (self.size * window.width()) as f32,
                    (self.size * window.height()) as f32,
                ),
                Sense::hover(),
            );

            let base_position = response.rect.left_top().to_vec2();
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

                    let rect = rect.translate(base_position);

                    let [r, g, b, a] = color.to_ne_bytes();
                    painter.rect_filled(
                        rect,
                        Rounding::ZERO,
                        Color32::from_rgba_premultiplied(r, g, b, a),
                    );
                }
            }
        });
    }
}

impl eframe::App for Maze {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("Config").show(ctx, |ui| {
            self.configuration(ui);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_on(ui);
        });
    }
}
