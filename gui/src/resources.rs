use macroquad::{
    audio::{self, Sound},
    prelude::*,
};
use sol_lib::board::piece::Piece;
use std::collections::HashMap;

#[derive(Default)]
pub struct Resources {
    texture_pieces: Option<Texture2D>,
    texture_rects_pieces: HashMap<Piece, Rect>,

    sounds: HashMap<SoundKind, Sound>,

    font: Option<Font>,
}

pub struct Texture<'a> {
    pub texture: &'a Texture2D,
    pub texture_rect: Rect,
}

macro_rules! load_sound {
    ($file_name:expr) => {
        audio::load_sound_from_bytes(include_bytes!($file_name))
            .await
            .unwrap()
    };
}

pub async fn init() -> Resources {
    // SOUNDS
    let click = load_sound!("../assets/click.wav");
    let win = load_sound!("../assets/win.wav");
    let loss = load_sound!("../assets/loss.wav");
    let button = load_sound!("../assets/button.wav");
    let mode = load_sound!("../assets/mode.wav");
    let mut sounds = HashMap::new();
    sounds.insert(SoundKind::Click, click);
    sounds.insert(SoundKind::Win, win);
    sounds.insert(SoundKind::Loss, loss);
    sounds.insert(SoundKind::Button, button);
    sounds.insert(SoundKind::Mode, mode);

    // FONT
    let font_ttf = include_bytes!("../assets/Junction-regular.otf");
    let font = load_ttf_font_from_bytes(font_ttf).expect("Failed to load font");

    // TEXTURES
    let texture_pieces_bytes = include_bytes!("../assets/pieces.png");
    let texture_pieces = Texture2D::from_file_with_format(&texture_pieces_bytes[..], None);
    texture_pieces.set_filter(FilterMode::Nearest);
    build_textures_atlas();

    let mut texture_rects_pieces = HashMap::new();
    texture_rects_pieces.insert(Piece::Pawn, piece_texture_rect(Piece::Pawn));
    texture_rects_pieces.insert(Piece::Knight, piece_texture_rect(Piece::Knight));
    texture_rects_pieces.insert(Piece::Bishop, piece_texture_rect(Piece::Bishop));
    texture_rects_pieces.insert(Piece::Rook, piece_texture_rect(Piece::Rook));
    texture_rects_pieces.insert(Piece::Queen, piece_texture_rect(Piece::Queen));
    texture_rects_pieces.insert(Piece::King, piece_texture_rect(Piece::King));

    Resources {
        texture_pieces: Some(texture_pieces),
        texture_rects_pieces,
        sounds,
        font: Some(font),
    }
}

fn piece_texture_rect(piece: Piece) -> Rect {
    let index = match piece {
        Piece::Pawn => 0,
        Piece::Knight => 1,
        Piece::Bishop => 2,
        Piece::Rook => 3,
        Piece::Queen => 4,
        Piece::King => 5,
    };
    let color = 0;
    Rect::new(index as f32 * 128.0, color as f32 * 128.0, 128.0, 128.0)
}

#[derive(Eq, PartialEq, Hash)]
pub enum SoundKind {
    Click,
    Win,
    Loss,
    Button,
    Mode,
}

impl Resources {
    pub fn get_piece_texture<'a>(&'a self, piece: &Piece) -> Texture<'a> {
        let texture = self.texture_pieces.as_ref().unwrap();
        let texture_rect = self.texture_rects_pieces.get(piece).unwrap();
        Texture {
            texture: texture,
            texture_rect: *texture_rect,
        }
    }

    pub fn font(&self) -> &Font {
        self.font.as_ref().unwrap()
    }

    pub fn sound(&self, kind: &SoundKind) -> &Sound {
        self.sounds.get(kind).unwrap()
    }
}
