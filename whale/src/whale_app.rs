use std::collections::HashMap;
use eframe::{egui, App};
use crate::chess_parts::{piece_from_u8, Board, Piece};

fn print_board(board: Board){
    println!("  +---+---+---+---+---+---+---+---+");
    for i in 0..8 {
        print!("{} |", 8 - i);
        for j in 0..8 {
            let cell = board.cells[i * 8 + j];
            if cell == 0 {
                print!("   |");
            } else {
                let (piece, color) = piece_from_u8(cell);
                let symbol = match piece {
                    Piece::Pawn => 'P',
                    Piece::Bishop => 'B',
                    Piece::Rook => 'R',
                    Piece::Knight => 'N',
                    Piece::Queen => 'Q',
                    Piece::King => 'K',
                };
                let display_char = if color.into() { symbol } else { symbol.to_ascii_lowercase() };
                print!(" {} |", display_char);
            }
        }
        println!("\n  +---+---+---+---+---+---+---+---+");
    }
    println!("    a   b   c   d   e   f   g   h");
}

pub(crate) struct WhaleApp {
    board: Board,
    image_bytes: Vec<(&'static str, &'static [u8])>,
    textures: HashMap<&'static str, egui::TextureHandle>,
}

impl WhaleApp {
    pub(crate) fn new() -> Self {
        Self {
            board: Board::default(),
            image_bytes: vec![
                ("white_pawn", include_bytes!("assets/white-pawn.png")),
                ("black_pawn", include_bytes!("assets/black-pawn.png")),
                ("white_rook", include_bytes!("assets/white-rook.png")),
                ("black_rook", include_bytes!("assets/black-rook.png")),
                ("white_knight", include_bytes!("assets/white-knight.png")),
                ("black_knight", include_bytes!("assets/black-knight.png")),
                ("white_bishop", include_bytes!("assets/white-bishop.png")),
                ("black_bishop", include_bytes!("assets/black-bishop.png")),
                ("white_queen", include_bytes!("assets/white-queen.png")),
                ("black_queen", include_bytes!("assets/black-queen.png")),
                ("white_king", include_bytes!("assets/white-king.png")),
                ("black_king", include_bytes!("assets/black-king.png")),
            ],
            textures: HashMap::new(),
        }
    }
}

impl App for WhaleApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.textures.is_empty() {
            for (name, bytes) in &self.image_bytes {
                let img = image::load_from_memory(bytes).unwrap().to_rgba8();
                let size = [img.width() as usize, img.height() as usize];
                let tex = ctx.load_texture(
                    *name,
                    egui::ColorImage::from_rgba_unmultiplied(size, img.as_raw()),
                    egui::TextureOptions::default(),
                );
                self.textures.insert(name, tex);
            }
        }

        egui::SidePanel::left("side_panel").width_range(egui::Rangef::new(200.0, 500.0)).resizable(true).show(ctx, |ui| {
            ui.heading("Whale Chess");
        });
        egui::SidePanel::right("right_panel").width_range(egui::Rangef::new(200.0, 500.0)).resizable(true).show(ctx, |ui| {
            ui.heading("Whale Chess - Right Panel");
        });
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.heading("Whale Chess - Bottom Panel");
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let rect = ui.available_rect_before_wrap();
            let painter = ui.painter().clone();

            let board_size = 8; // 8x8
            let side = rect.width().min(rect.height());
            let top_left = rect.center() - egui::vec2(side / 2.0, side / 2.0);
            let square_size = side / board_size as f32;

            if square_size > 0.0 {
                let color_a = egui::Color32::from_rgb(255, 238, 215);
                let color_b = egui::Color32::from_rgb(58, 34, 0);

                for row in 0..board_size {
                    for col in 0..board_size {
                        let x = top_left.x + col as f32 * square_size;
                        let y = top_left.y + row as f32 * square_size;
                        let rect = egui::Rect::from_min_max(
                            egui::pos2(x, y),
                            egui::pos2(x + square_size, y + square_size),
                        );
                        let color = if (row + col) % 2 == 0 { color_a } else { color_b };
                        painter.rect_filled(rect, 0.0, color);
                        let piece_name = match self.board.cells[row * 8 + col] {
                            0 => None,
                            cell => {
                                let (piece, color) = piece_from_u8(cell);
                                let name = match piece {
                                    Piece::Pawn => "pawn",
                                    Piece::Rook => "rook",
                                    Piece::Knight => "knight",
                                    Piece::Bishop => "bishop",
                                    Piece::Queen => "queen",
                                    Piece::King => "king",
                                };
                                Some(if color.into() {
                                    format!("white_{}", name)
                                } else {
                                    format!("black_{}", name)
                                })
                            }
                        };
                        if let Some(name) = piece_name {
                            if let Some(texture) = self.textures.get(name.as_str()) {
                                painter.image(
                                    texture.id(),
                                    rect,
                                    egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                                    egui::Color32::WHITE,
                                );
                            }
                        }
                    }
                }
            }
        });
    }
}
