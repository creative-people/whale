use crate::chess_parts::{Board, Mailbox64Index, piece_from_u8, MOVESETS, offset_index_2d, offset_ray_2d};

pub(crate) fn generate_pseudolegal(board: &Board, index: Mailbox64Index) -> Vec<Mailbox64Index> {
    let mut moves = Vec::new();
    let (piece, color) = piece_from_u8(board.cells[index.0 as usize]);
    let moveset = MOVESETS.get(&(piece, color)).unwrap();
    for (dx, dy) in &moveset.0 {
        if moveset.1 {
            moves.extend(offset_ray_2d(index.clone(), *dx, *dy, 7));
        } else {
            if let Some(target_index) = offset_index_2d(index.clone(), *dx, *dy) {
                moves.push(target_index);
            }
        }
    }

    moves
}