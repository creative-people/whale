use eframe::{egui, App};
use crate::{piece_from_u8, Board, Piece};

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
    board: Board
}

impl WhaleApp {
    pub(crate) fn new() -> Self {
        Self {
            board: Board::default()
        }
    }
}

impl App for WhaleApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            print_board(self.board.clone());
        });
    }
}