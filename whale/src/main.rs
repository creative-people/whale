mod whale_app;
use whale_app::WhaleApp;

use eframe;
use eframe::{egui, App};

struct Board {
    cells: [u8; 64],
    turn: Color,
    castling_availability: [bool; 4],
    en_passant_target_square: Option<Mailbox64Index>,
    halfmove_clock: u8,
    fullmove_clock: usize,
}

impl Clone for Board {
    fn clone(&self) -> Self {
        Board {
            cells: self.cells,
            turn: self.turn.clone(),
            castling_availability: self.castling_availability,
            en_passant_target_square: self.en_passant_target_square.clone(),
            halfmove_clock: self.halfmove_clock,
            fullmove_clock: self.fullmove_clock,
        }
    }
}

#[repr(u8)]
enum Piece {
    Pawn = 1,
    Bishop,
    Rook,
    Knight,
    Queen,
    King
}

impl TryFrom<u8> for Piece {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use Piece::*;
        match value {
            val if val == Pawn as u8 => Ok(Pawn),
            val if val == Bishop as u8 => Ok(Bishop),
            val if val == Rook as u8 => Ok(Rook),
            val if val == Knight as u8 => Ok(Knight),
            val if val == Queen as u8 => Ok(Queen),
            val if val == King as u8 => Ok(King),
            _ => Err(())
        }
    }
}

#[repr(u8)]
enum Color {
    Black = 0,
    White = 1
}

impl Clone for Color {
    fn clone(&self) -> Self {
        match self {
            Color::Black => Color::Black,
            Color::White => Color::White
        }
    }
}

impl Into<bool> for Color {
    fn into(self) -> bool {
        self as u8 == 1
    }
}

impl From<bool> for Color {
    fn from(value: bool) -> Color {
        if value { Color::White } else { Color::Black }
    }
}

impl TryFrom<u8> for Color {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use Color::*;
        match value {
            val if val == Black as u8 => Ok(Black),
            val if val == White as u8 => Ok(White),
            _ => Err(())
        }
    }
}

fn new_piece(piece: Piece, color: Color) -> u8 {
    ((piece as u8) << 1) | (color as u8)
}

fn piece_from_u8(input: u8) -> (Piece, Color) {
    let piece = (input >> 1).try_into().unwrap();
    let color = (input & 1).try_into().unwrap();
    (piece, color)
}

impl Board {
    /// Build board from FEN notation
    fn new(fen: &str) -> Board {
        let parts: Vec<_> = fen.split_whitespace().collect();
        if parts.len() != 6 {
            panic!("Invalid FEN: expected 6 fields, found {}", parts.len());
        }

        let fen_board = parts[0];
        let turn = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            x => panic!("Invalid turn field: {}", x),
        };

        let castling_availability = parts[2];
        let en_passant_target_square = if parts[3] == "-" {
            None
        } else {
            Some(parts[3].into())
        };

        let halfmove_clock = parts[4].parse::<u8>().expect("Invalid halfmove clock");
        let fullmove_clock = parts[5].parse::<usize>().expect("Invalid fullmove clock");

        let mut board = Board {
            cells: [0; 64],
            turn,
            castling_availability: [false; 4],
            en_passant_target_square,
            halfmove_clock,
            fullmove_clock,
        };

        for x in castling_availability.chars() {
            match x {
                'K' => board.castling_availability[0] = true,
                'Q' => board.castling_availability[1] = true,
                'k' => board.castling_availability[2] = true,
                'q' => board.castling_availability[3] = true,
                '-' => (),
                c => panic!("Invalid castling char '{}'", c),
            }
        }

        for (row_idx, rank) in fen_board.split('/').enumerate() {
            let mut file = 0;
            for c in rank.chars() {
                if c.is_ascii_digit() {
                    file += c.to_digit(10).unwrap() as usize;
                } else {
                    let piece = match c {
                        'P' => new_piece(Piece::Pawn, Color::White),
                        'N' => new_piece(Piece::Knight, Color::White),
                        'B' => new_piece(Piece::Bishop, Color::White),
                        'R' => new_piece(Piece::Rook, Color::White),
                        'Q' => new_piece(Piece::Queen, Color::White),
                        'K' => new_piece(Piece::King, Color::White),
                        'p' => new_piece(Piece::Pawn, Color::Black),
                        'n' => new_piece(Piece::Knight, Color::Black),
                        'b' => new_piece(Piece::Bishop, Color::Black),
                        'r' => new_piece(Piece::Rook, Color::Black),
                        'q' => new_piece(Piece::Queen, Color::Black),
                        'k' => new_piece(Piece::King, Color::Black),
                        x => panic!("Invalid piece char '{}'", x),
                    };
                    board.cells[row_idx * 8 + file] = piece;
                    file += 1;
                }
            }
            if file != 8 {
                panic!("Invalid FEN row '{}': expected 8 columns, got {}", rank, file);
            }
        }

        board
    }

    fn default() -> Board {
        Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }
}

struct Mailbox120Index(pub u8);
struct Mailbox64Index(pub u8);

impl Clone for Mailbox120Index {
    fn clone(&self) -> Self {
        Mailbox120Index(self.0)
    }
}

impl Clone for Mailbox64Index {
    fn clone(&self) -> Self {
        Mailbox64Index(self.0)
    }
}

impl From<&str> for Mailbox64Index {
    fn from(value: &str) -> Self {
        let mut chars = value.chars();
        let file = chars.next().unwrap() as u8 - 'a' as u8;
        let rank = chars.next().unwrap() as u8 - '1' as u8;
        Mailbox64Index(file + rank * 8)
    }
}

impl From<Mailbox64Index> for Mailbox120Index {
    fn from(value: Mailbox64Index) -> Self {
        Mailbox120Index(MAILBOX64[value.0 as usize])
    }
}

impl From<Mailbox120Index> for Mailbox64Index {
    fn from(value: Mailbox120Index) -> Self {
        let idx = MAILBOX120.iter().position(|&x| x == value.0 as i8).unwrap();
        Mailbox64Index(idx as u8)
    }
}

fn offset_index(index: Mailbox64Index, offset: i8) -> Option<Mailbox64Index> {
    let abs_index = MAILBOX64[index.0 as usize] as i8 + offset;
    if abs_index < 0 || abs_index >= 120 {
        panic!("Invalid Mailbox64 index: {}", abs_index);
    }
    let new_index = MAILBOX120[abs_index as usize];
    if new_index == -1 {
        return None;
    }
    Some(Mailbox64Index(new_index as u8))
}

fn offset_index_2d(index: Mailbox64Index, file_offset: i8, rank_offset: i8) -> Option<Mailbox64Index> {
    if file_offset < -2 || file_offset > 2 || rank_offset < -2 || rank_offset > 2 {
        return None;
    }
    offset_index(index, file_offset + rank_offset * 10)
}

fn offset_ray(index: Mailbox64Index, file_offset: i8, rank_offset: i8, length: u8) -> Vec<Mailbox64Index> {
    let mut results = Vec::new();
    let mut current_index = index;
    for _ in 0..length {
        match offset_index_2d(current_index, file_offset, rank_offset) {
            Some(new_index) => {
                results.push(new_index.clone());
                current_index = new_index;
            },
            None => break,
        }
    }
    results
}

const MAILBOX120: [i8; 120] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1,  0,  1,  2,  3,  4,  5,  6,  7, -1,
    -1,  8,  9, 10, 11, 12, 13, 14, 15, -1,
    -1, 16, 17, 18, 19, 20, 21, 22, 23, -1,
    -1, 24, 25, 26, 27, 28, 29, 30, 31, -1,
    -1, 32, 33, 34, 35, 36, 37, 38, 39, -1,
    -1, 40, 41, 42, 43, 44, 45, 46, 47, -1,
    -1, 48, 49, 50, 51, 52, 53, 54, 55, -1,
    -1, 56, 57, 58, 59, 60, 61, 62, 63, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1
];

const MAILBOX64: [u8; 64] = [
    21, 22, 23, 24, 25, 26, 27, 28,
    31, 32, 33, 34, 35, 36, 37, 38,
    41, 42, 43, 44, 45, 46, 47, 48,
    51, 52, 53, 54, 55, 56, 57, 58,
    61, 62, 63, 64, 65, 66, 67, 68,
    71, 72, 73, 74, 75, 76, 77, 78,
    81, 82, 83, 84, 85, 86, 87, 88,
    91, 92, 93, 94, 95, 96, 97, 98
];

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1860.0, 1280.0]).with_resizable(false),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "Assembly VM",
        options,
        Box::new(|_cc| Ok(Box::<WhaleApp>::new(WhaleApp::new()))),
    );
}
