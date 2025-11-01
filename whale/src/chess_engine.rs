use crate::chess_parts::{Board, Mailbox64Index, piece_from_u8, MOVESETS, offset_index_2d, offset_ray_2d};

fn generate_pseudolegal(board: &Board, index: Mailbox64Index) -> Vec<Mailbox64Index> {
    let mut moves = Vec::new();
    let (piece, color) = piece_from_u8(board.cells[index.0 as usize]);
    let moveset = MOVESETS.get(&(piece, color)).unwrap();
    for (dx, dy) in &moveset.0 {
        if moveset.1 {
            moves.extend(offset_ray_2d(board, index.clone(), *dx, *dy, 7));
        } else {
            if let Some(target_index) = offset_index_2d(index.clone(), *dx, *dy) {
                moves.push(target_index);
            }
        }
    }
    moves
}

pub(crate) fn generate_legal(board: &Board, index: Mailbox64Index) -> Vec<Mailbox64Index> {
    let mut legal_moves = Vec::new();
    let pseudolegal_moves = generate_pseudolegal(board, index.clone());
    let (piece, color) = piece_from_u8(board.cells[index.0 as usize]);
    for target_index in pseudolegal_moves {
        if board.cells[target_index.0 as usize] != 0 {
            let (_, target_color) = piece_from_u8(board.cells[target_index.0 as usize]);
            if target_color == color {
                continue;
            }
        }
        // let mut board_clone = board.clone();
        // board_clone.make_move(index.clone(), target_index.clone());
        // if board_clone.is_in_check(board.cells[index.0 as usize] & 0b0000_0011) {
        //     continue;
        // }
        // TODO: Implement legal move checking
        legal_moves.push(target_index);
    }
    legal_moves
}
