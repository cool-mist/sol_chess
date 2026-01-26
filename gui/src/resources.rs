use macroquad::{
    audio::{self, Sound},
    prelude::*,
};
use sol_lib::board::piece::{Piece, PieceKind};
use std::collections::HashMap;

#[derive(Default)]
pub struct Resources {
    texture_pieces: Option<Texture2D>,
    texture_rects_pieces: HashMap<PieceKind, Rect>,
    texture_rects_inactive_pieces: HashMap<PieceKind, Rect>,

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

    let texture_rects_pieces = generate_texture_rects_pieces(true);
    let texture_rects_inactive_pieces = generate_texture_rects_pieces(false);

    Resources {
        texture_pieces: Some(texture_pieces),
        texture_rects_pieces,
        texture_rects_inactive_pieces,
        sounds,
        font: Some(font),
    }
}

fn generate_texture_rects_pieces(active: bool) -> HashMap<PieceKind, Rect> {
    let mut texture_rects_pieces = HashMap::new();
    texture_rects_pieces.insert(PieceKind::Pawn, piece_texture_rect(PieceKind::Pawn, active));
    texture_rects_pieces.insert(PieceKind::Knight, piece_texture_rect(PieceKind::Knight, active));
    texture_rects_pieces.insert(PieceKind::Bishop, piece_texture_rect(PieceKind::Bishop, active));
    texture_rects_pieces.insert(PieceKind::Rook, piece_texture_rect(PieceKind::Rook, active));
    texture_rects_pieces.insert(PieceKind::Queen, piece_texture_rect(PieceKind::Queen, active));
    texture_rects_pieces.insert(PieceKind::King, piece_texture_rect(PieceKind::King, active));

    texture_rects_pieces
}

fn piece_texture_rect(piece: PieceKind, active: bool) -> Rect {
    let index = match piece {
        PieceKind::Pawn => 0,
        PieceKind::Knight => 1,
        PieceKind::Bishop => 2,
        PieceKind::Rook => 3,
        PieceKind::Queen => 4,
        PieceKind::King => 5,
    };
    let color = match active {
        true => 1,
        false => 0,
    };
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
        let texture_rects_lookup = if piece.active {
            &self.texture_rects_pieces
        } else {
            &self.texture_rects_inactive_pieces
        };

        let texture_rect = texture_rects_lookup.get(&piece.kind).unwrap();
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
