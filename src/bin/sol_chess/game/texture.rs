use macroquad::prelude::*;
use sol_chess::board::piece::Piece;

pub struct PieceTexture {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl PieceTexture {
    fn new(x: u32, y: u32) -> Self {
        Self {
            x: x as f32 * 128.0,
            y: y as f32 * 128.0,
            w: 128.0,
            h: 128.0,
        }
    }

    pub fn for_piece(piece: Piece, sprite_size: f32) -> DrawTextureParams {
        let index = match piece {
            Piece::Pawn => 0,
            Piece::Knight => 1,
            Piece::Bishop => 2,
            Piece::Rook => 3,
            Piece::Queen => 4,
            Piece::King => 5,
        };

        let color = 0;
        let texture_rect = PieceTexture::new(index, color);

        DrawTextureParams {
            source: Some(Rect::new(
                texture_rect.x,
                texture_rect.y,
                texture_rect.w,
                texture_rect.h,
            )),
            dest_size: Some(Vec2::new(sprite_size, sprite_size)),
            ..DrawTextureParams::default()
        }
    }
}
