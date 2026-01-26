use core::fmt;
use std::fmt::{Display, Formatter};

use crate::{
    game::{GameSettings, constants},
    resources::{Resources, SoundKind},
    widgets::*,
};
use macroquad::prelude::*;
use sol_lib::{
    board::{Board, BoardState, piece::Piece},
    generator::Puzzle,
};

#[derive(Default)]
pub struct BoardWidget {
    current_board: Board,
    square_width: f32,
    pub num_squares: usize, // this is a constant = 4 for now.
    board_rect: Rect,
    squares: Vec<GameSquare>,
    state: GameState,
}

pub struct BoardInteraction<'a> {
    pub interacted: bool,
    pub board_state: Option<&'a BoardState>,
}

impl BoardWidget {
    pub fn initialize_state(num_squares: usize, current_board: Board) -> BoardWidget {
        let mut board = BoardWidget::default();
        board.num_squares = num_squares;
        board.current_board = current_board;
        board
    }

    pub fn initialize_drawables(&mut self, square_width: f32, board_rect: Rect) {
        self.square_width = square_width;
        self.board_rect = board_rect;

        let dark = UiColor::Brown.to_bg_color();
        let light = UiColor::Yellow.to_bg_color();
        let mut rects = Vec::new();
        for i in 0..self.num_squares {
            for j in 0..self.num_squares {
                let x_eff = self.board_rect.x + (i as f32 * self.square_width);
                let y_eff = self.board_rect.y + (j as f32 * self.square_width);
                let rect = Rect::new(x_eff, y_eff, self.square_width, self.square_width);
                let color = match (i + j) % 2 {
                    1 => dark,
                    _ => light,
                };

                rects.push(GameSquare {
                    rect,
                    color,
                    i,
                    j,
                    is_source: false,
                    is_target: false,
                    is_previous_target: false,
                });
            }
        }

        self.squares = rects;
    }

    pub fn draw(&self, params: &BoardDrawParams, resources: &Resources, settings: &GameSettings) {
        let board_shadow_width = constants::BOARD_SHADOW_MULTIPLIER * self.square_width;
        draw_shadow(&self.board_rect, board_shadow_width);

        if params.show_rules {
            draw_rectangle(
                self.board_rect.x,
                self.board_rect.y,
                self.board_rect.w,
                self.board_rect.h,
                UiColor::Yellow.to_bg_color(),
            );

            let font_size = params.rules_font_size * 0.4;
            let rules = "\
                Every move should be a \n\
                capture. Win when only \n\
                one piece is left.\n\
                Age: Each piece can only \n\
                move 'age' times";
            let measurement = measure_text(rules, Some(resources.font()), font_size as u16, 1.0);
            let draw_text_params = TextParams {
                font_size: font_size as u16,
                font: Some(resources.font()),
                color: UiColor::Brown.to_bg_color(),
                ..Default::default()
            };
            draw_multiline_text_ex(
                rules,
                self.board_rect.x + 0.5 * self.square_width,
                self.board_rect.y + 0.4 * (self.board_rect.h - measurement.height)
                    - 2. * measurement.offset_y,
                Some(2.),
                draw_text_params,
            );
            return;
        }

        let sprite_size = constants::BOARD_PIECE_WIDTH_MULTIPLIER * self.square_width;
        let mut selected_square = None;
        self.squares.iter().for_each(|square| {
            let color = match square.is_source {
                true => square.color,
                false => match square.is_target {
                    true => UiColor::Pink.to_shadow_color(),
                    false => square.color,
                },
            };

            draw_rectangle(
                square.rect.x,
                square.rect.y,
                square.rect.w,
                square.rect.h,
                color,
            );

            if let Some(p) = &self.current_board.cells[square.i][square.j] {
                let offset = (square.rect.w - sprite_size) / 2.0;
                let texture_params = piece_draw_texture_params(&p, sprite_size, resources);
                if !square.is_source {
                    draw_texture_ex(
                        texture_params.texture,
                        square.rect.x + offset,
                        square.rect.y + offset,
                        WHITE,
                        texture_params.draw_text_params,
                    );
                } else {
                    selected_square = Some(square);
                }
            }
        });

        if let Some(selected_square) = selected_square {
            if let Some(p) = self.current_board.cells[selected_square.i][selected_square.j] {
                let texture_params = piece_draw_texture_params(&p, sprite_size, resources);
                draw_texture_ex(
                    texture_params.texture,
                    mouse_position().0 - sprite_size / 2.0,
                    mouse_position().1 - sprite_size / 2.0,
                    WHITE,
                    texture_params.draw_text_params,
                );
            }
        }

        if settings.debug {
            self.draw_debug();
        }
    }

    fn get(&mut self, i: usize, j: usize) -> &mut GameSquare {
        &mut self.squares[i * self.num_squares + j]
    }

    fn draw_debug(&self) {
        let mut debug_lines = vec![];
        let (mx, my) = mouse_position();
        let hover_square = self.squares.iter().find(|s| {
            let c = Circle::new(mx, my, 0.0);
            if c.overlaps_rect(&s.rect) {
                return true;
            }
            return false;
        });
        debug_lines.push(format!("Game State: {}", self.state));
        debug_lines.push(format!("Board State: {}", self.current_board.game_state));
        if let Some(hover_square) = hover_square {
            debug_lines.push(format!("Hover: [ {}, {} ]", hover_square.i, hover_square.j));
        }
        self.add_debug_info(debug_lines);
    }

    fn add_debug_info(&self, lines: Vec<String>) {
        let mut y = 20.0;
        for line in lines {
            draw_text(&line, 10.0, y, 20.0, BLACK);
            y += 25.0;
        }
    }

    pub fn handle_input<'a>(
        &'a mut self,
        mouse_pos: Circle,
        resources: &Resources,
        settings: &GameSettings,
    ) -> BoardInteraction<'a> {
        if mouse_pos.overlaps_rect(&self.board_rect) == false {
            return BoardInteraction {
                interacted: false,
                board_state: None,
            };
        }

        if is_mouse_button_released(MouseButton::Left) {
            let current_state = self.state.clone();
            let new_state = match current_state {
                GameState::SelectSource(previous_target) => {
                    self.handle_select_source(mouse_pos, previous_target)
                }
                GameState::SelectTarget(source) => {
                    let next = self.handle_select_target(mouse_pos, source, resources, settings);
                    if let GameState::SelectTarget(_) = next {
                        self.reset_squares();
                        GameState::SelectSource(None)
                    } else {
                        next
                    }
                }

                GameState::GameOver(previous_target) => GameState::GameOver(previous_target),
            };

            self.state = new_state;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let current_state = self.state.clone();
            let new_state = match current_state {
                GameState::SelectSource(previous_target) => {
                    self.handle_select_source(mouse_pos, previous_target)
                }
                GameState::SelectTarget(source) => GameState::SelectTarget(source),
                GameState::GameOver(previous_target) => GameState::GameOver(previous_target),
            };

            self.state = new_state;
        }

        let board_state = &self.current_board.game_state;
        return BoardInteraction {
            interacted: true,
            board_state: Some(board_state),
        };
    }

    pub fn reset(&mut self, puzzle: &Puzzle) {
        self.current_board = puzzle.board.clone();
        self.state = GameState::SelectSource(None);
        self.reset_squares();
    }

    fn handle_select_source(
        &mut self,
        mouse_pos: Circle,
        previous_target: Option<(usize, usize)>,
    ) -> GameState {
        self.reset_squares();
        let mut selected = None;
        for square in &mut self.squares {
            if mouse_pos.overlaps_rect(&square.rect) {
                if let Some(_) = self.current_board.cells[square.i][square.j] {
                    selected = Some((square.i, square.j));
                }
            }
        }

        if let Some((i, j)) = selected {
            self.get(i, j).is_source = true;
            let mut target_squares = vec![];
            for m in self.current_board.legal_moves.iter() {
                if m.from.file == i && m.from.rank == j {
                    target_squares.push((m.to.file, m.to.rank));
                }
            }

            for (i, j) in target_squares {
                self.get(i, j).is_target = true;
            }

            return GameState::SelectTarget(selected.unwrap());
        }

        if let Some((i, j)) = previous_target {
            self.get(i, j).is_previous_target = true;
        }

        return GameState::SelectSource(None);
    }

    fn handle_select_target(
        &mut self,
        mouse_pos: Circle,
        source: (usize, usize),
        resources: &Resources,
        settings: &GameSettings,
    ) -> GameState {
        let mut selected = None;
        for square in &mut self.squares {
            if mouse_pos.overlaps_rect(&square.rect) {
                if let Some(_) = self.current_board.cells[square.i][square.j] {
                    selected = Some((square.i, square.j));
                }
            }
        }

        let (s_x, s_y) = source;
        let Some((x, y)) = selected else {
            self.get(s_x, s_y).is_source = true;
            return GameState::SelectTarget(source);
        };

        if x == s_x && y == s_y {
            self.get(s_x, s_y).is_source = true;
            return GameState::SelectTarget(source);
        }

        let mut is_legal = false;
        if self.get(x, y).is_target {
            is_legal = true;
        }

        if is_legal {
            let m = self.current_board.legal_moves.iter().find(|m| {
                m.from.file == s_x && m.from.rank == s_y && m.to.file == x && m.to.rank == y
            });

            let m = m.expect("legal move should be found");
            self.current_board.make_move(m.clone());

            if self.current_board.game_state == BoardState::Won
                || self.current_board.game_state == BoardState::Lost
            {
                self.reset_squares();
                if self.current_board.game_state == BoardState::Won {
                    play_sound_once(resources.sound(&SoundKind::Win), settings.volume);
                } else {
                    play_sound_once(resources.sound(&SoundKind::Loss), settings.volume);
                }

                return GameState::GameOver((x, y));
            }

            self.reset_squares();
            self.get(x, y).is_target = true;
            play_sound_once(resources.sound(&SoundKind::Click), settings.volume);
            return GameState::SelectSource(Some((x, y)));
        }

        self.reset_squares();
        return GameState::SelectSource(None);
    }

    fn reset_squares(&mut self) {
        for i in 0..self.num_squares {
            for j in 0..self.num_squares {
                self.get(i, j).is_source = false;
                self.get(i, j).is_target = false;
            }
        }
    }
}

struct GameSquare {
    rect: Rect,
    color: Color,
    is_source: bool,
    is_target: bool,
    is_previous_target: bool,
    i: usize,
    j: usize,
}

#[derive(Copy, Clone)]
pub enum GameState {
    SelectSource(Option<(usize, usize)>),
    SelectTarget((usize, usize)),
    GameOver((usize, usize)),
}

impl Default for GameState {
    fn default() -> Self {
        GameState::SelectSource(None)
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GameState::SelectSource(Some(x)) => write!(f, "Select Source [ {}, {} ]", x.0, x.1),
            GameState::SelectSource(None) => write!(f, "Select Source [ ]"),
            GameState::SelectTarget(x) => write!(f, "Select Target [ {}, {} ]", x.0, x.1),
            GameState::GameOver(x) => write!(f, "Game Over [ {}, {} ]", x.0, x.1),
        }
    }
}

fn piece_draw_texture_params<'a>(
    piece: &Piece,
    sprite_size: f32,
    resources: &'a Resources,
) -> PieceDrawTextureParams<'a> {
    let texture = resources.get_piece_texture(&piece);
    let dtp = DrawTextureParams {
        source: Some(Rect::new(
            texture.texture_rect.x,
            texture.texture_rect.y,
            texture.texture_rect.w,
            texture.texture_rect.h,
        )),
        dest_size: Some(Vec2::new(sprite_size, sprite_size)),
        ..DrawTextureParams::default()
    };

    PieceDrawTextureParams {
        texture: texture.texture,
        draw_text_params: dtp,
    }
}

struct PieceDrawTextureParams<'a> {
    texture: &'a Texture2D,
    draw_text_params: DrawTextureParams,
}

pub struct BoardDrawParams {
    pub show_rules: bool,
    pub rules_font_size: f32,
}
