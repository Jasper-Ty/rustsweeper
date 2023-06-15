pub mod board;
pub mod draw;
pub mod render;

pub use board::*;
pub use draw::*;
pub use render::*;

pub const SQ_SIZE: usize = 48; 
pub const SQ_I32: i32 = SQ_SIZE as i32;
pub const SQ_U32: u32 = SQ_SIZE as u32;

pub fn reveal((x, y): (usize, usize), overlay: &mut Overlay, board: &Board) {
    overlay[(x, y)] = Cover::Open;
    if let Cell::Num(0) = board[(x, y)] {
        let neighborhood = [
            (-1, -1), (0, -1), (1, -1),
            (-1,  0),          (1,  0),
            (-1,  1), (0,  1), (1,  1),
        ];
        let mut neighborhood = neighborhood.iter()
            .map(|(dx, dy)| (x as i32 + dx, y as i32 + dy))
            .filter(|(x, y)| 
                *x >= 0 && *x < board.width() as i32
                && *y >= 0 && *y < board.height() as i32)
            .map(|(x, y)| (x as usize, y as usize));

        for (x, y) in neighborhood {
            match overlay[(x, y)] {
                Cover::Closed => reveal((x, y), overlay, board),
                _ => {}
            }
        }
    }
}
