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
    fn new(string: &str) -> Board {
        let components: Vec<_> = string.split_whitespace()
            .collect();
        if components.len() != 6 {
            panic!("Found invalid length {} of FEN notation", components.len());
        }

        let [fen_board, turn, castling_availability,
            en_passant_target_square, halfmove_clock, fullmove_clock] = components[..] else { todo!() };

        let turn = if turn == "w" { Color::White } else if turn == "b" { Color::Black } else { panic!("Found invalid turn string {turn} while parsing FEN notation") };
        let en_passant_target_square = if en_passant_target_square == "-" { None } else { Some(en_passant_target_square.into()) };

        let mut board = Board {
            cells: [0; 64],
            turn,
            castling_availability: [false; 4],
            en_passant_target_square,
            halfmove_clock: halfmove_clock.parse().unwrap(),
            fullmove_clock: fullmove_clock.parse().unwrap(),
        };

        for x in castling_availability.chars() {
            match x {
                'K' => board.castling_availability[0] = true,
                'Q' => board.castling_availability[1] = true,
                'k' => board.castling_availability[2] = true,
                'q' => board.castling_availability[3] = true,
                '-' => (),
                x => panic!("Found invalid char {x} while parsing FEN notation")
            }
        }

        for (row_index, row_positions) in fen_board.split("/").enumerate() {
            let mut column_index = 0;
            for x in row_positions.chars() {
                if x.is_digit(10) {
                    column_index += x.to_digit(10).unwrap()
                } else {
                    board.cells[row_index * 8 + column_index as usize] = match x {
                        'P' => new_piece(Piece::Pawn, Color::White),
                        'R' => new_piece(Piece::Rook, Color::White),
                        'N' => new_piece(Piece::Knight, Color::White),
                        'B' => new_piece(Piece::Bishop, Color::White),
                        'Q' => new_piece(Piece::Queen, Color::White),
                        'K' => new_piece(Piece::King, Color::White),
                        'p' => new_piece(Piece::Pawn, Color::Black),
                        'r' => new_piece(Piece::Rook, Color::Black),
                        'n' => new_piece(Piece::Knight, Color::Black),
                        'b' => new_piece(Piece::Bishop, Color::Black),
                        'q' => new_piece(Piece::Queen, Color::Black),
                        'k' => new_piece(Piece::King, Color::Black),
                        x => panic!("Found invalid char {x} while parsing FEN notation")
                    }
                }
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

impl From<&str> for Mailbox64Index {
    fn from(value: &str) -> Self {
        let mut chars = value.chars();
        let file = chars.next().unwrap() as u8 - 'a' as u8;
        let rank = chars.next().unwrap() as u8 - '1' as u8;
        Mailbox64Index(file + rank * 8)
    }
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
    let mut board = Board::default();

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

fn print_bord(bord: Board){
    for i in 0..63 {

        if bord.cells[i]<1 {

        } else if bord.cells[i]<7 {

        }
    }
}

struct WhaleApp {

}

impl WhaleApp {
    fn new() -> Self {
        Self {
        }
    }
}

impl App for WhaleApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
    }
}