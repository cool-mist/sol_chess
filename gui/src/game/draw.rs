use super::{Game, GameMode, constants};
use crate::widgets::*;
use macroquad::{math, prelude::*};
use sol_lib::board::piece::Piece;

impl Game {
    pub fn draw(&mut self) {
        self.update_window_size();
        self.draw_heading();
        self.draw_board();
        self.draw_buttons();
        self.draw_id_text();
        self.draw_debug();
    }

    fn update_window_size(&mut self) {
        let new_height = math::clamp(
            screen_height(),
            constants::SCREEN_HEIGHT_MIN,
            constants::SCREEN_HEIGHT_MAX,
        );
        let new_width = math::clamp(
            screen_width(),
            constants::SCREEN_WIDTH_MIN,
            constants::SCREEN_WIDTH_MAX,
        );

        if new_height == self.window_height && new_width == self.window_width {
            return;
        }

        self.window_height = new_height;
        self.window_width = new_width;
        self.initialize_drawables();
    }

    fn initialize_drawables(&mut self) {
        let min_dimension = f32::min(self.window_height, self.window_width);
        self.square_width = constants::BOARD_SQUARE_WIDTH_MULTIPLIER * min_dimension;
        let board_width = self.square_width * self.num_squares as f32;
        let board_x = (self.window_width - board_width) / 2.0;
        let board_y = (self.window_height - board_width) / 2.0;
        self.board_rect = Rect::new(board_x, board_y, board_width, board_width);

        self.heading_font_size = constants::HEADING_FONT_SIZE_MULTIPLIER * min_dimension;
        let f = self.heading_font_size.floor() as u16;
        let dims = measure_text(
            self.heading_text.as_str(),
            Some(&self.resources.font()),
            f,
            1.0,
        );
        self.heading_rect = Rect::new(
            board_x + (board_width - dims.width) / 2.0,
            board_y - dims.height - constants::TOP_HEADING_OFFSET_MULTIPLIER * self.square_width,
            dims.width,
            dims.height,
        );

        let dark = UiColor::Brown.to_bg_color();
        let light = UiColor::Yellow.to_bg_color();
        let mut rects = Vec::new();
        for i in 0..self.num_squares {
            for j in 0..self.num_squares {
                let x_eff = board_x + (i as f32 * self.square_width);
                let y_eff = board_y + (j as f32 * self.square_width);
                let rect = Rect::new(x_eff, y_eff, self.square_width, self.square_width);
                let color = match (i + j) % 2 {
                    1 => dark,
                    _ => light,
                };

                rects.push(super::GameSquare {
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

        // Buttons
        let btn_h = constants::BUTTON_HEIGHT_MULTIPLIER * min_dimension;
        let btn_w = board_width * constants::BUTTON_WIDTH_MULTIPLIER;
        let btn_w_sq = btn_h;

        // Bottom row
        let bottom_row_y = board_width
            + board_y
            + constants::BOTTOM_BUTTON_ROW_OFFSET_MULTIPLIER * self.square_width;
        self.reset_btn = ButtonWidget::new(Rect::new(
            board_x + (self.square_width - btn_w_sq) / 2.,
            bottom_row_y,
            btn_w_sq,
            btn_h,
        ));
        let btn_next_x_offset =
            (self.board_rect.w - self.square_width) + (self.square_width - btn_w_sq) / 2.;
        self.next_btn = ButtonWidget::new(Rect::new(
            board_x + btn_next_x_offset,
            bottom_row_y,
            btn_w_sq,
            btn_h,
        ));
        self.next_btn.is_active = false;

        let id_text_rect = Rect::new(
            board_x + 2. * self.square_width - btn_w,
            bottom_row_y,
            btn_w * 2.,
            btn_h,
        );
        self.id_text = IdTextInput::new(id_text_rect);

        // Left column
        let left_column_x =
            board_x - constants::BOTTOM_RIGHT_ROW_OFFSET_MULTIPLIER * self.square_width - btn_w;
        self.rules_btn = ButtonWidget::new(Rect::new(
            left_column_x,
            board_y + self.board_rect.h - self.square_width + (self.square_width - btn_h) / 2.,
            btn_w,
            btn_h,
        ));

        // Right column
        let right_column_x = board_x
            + board_width
            + constants::BOTTOM_RIGHT_ROW_OFFSET_MULTIPLIER * self.square_width;
        self.easy_btn = ButtonWidget::new(Rect::new(
            right_column_x,
            board_y + self.square_width + (self.square_width - btn_h) / 2.,
            btn_w,
            btn_h,
        ));

        self.medium_btn = ButtonWidget::new(Rect::new(
            right_column_x,
            board_y + 2. * self.square_width + (self.square_width - btn_h) / 2.,
            btn_w,
            btn_h,
        ));

        self.hard_btn = ButtonWidget::new(Rect::new(
            right_column_x,
            board_y + 3. * self.square_width + (self.square_width - btn_h) / 2.,
            btn_w,
            btn_h,
        ));

        match self.game_mode {
            GameMode::Easy => {
                self.easy_btn.is_active = false;
            }
            GameMode::Medium => {
                self.medium_btn.is_active = false;
            }
            GameMode::Hard => {
                self.hard_btn.is_active = false;
            }
        }
    }

    fn draw_heading(&self) {
        let f = self.heading_font_size.floor() as u16;
        let dims = measure_text(
            self.heading_text.as_str(),
            Some(&self.resources.font()),
            f,
            1.0,
        );
        let draw_text_params = TextParams {
            font_size: f,
            font: Some(&self.resources.font()),
            color: BLACK,
            ..Default::default()
        };
        draw_text_ex(
            self.heading_text.as_str(),
            self.heading_rect.x,
            self.heading_rect.y + dims.offset_y,
            draw_text_params,
        );
    }

    fn draw_board(&self) {
        let board_shadow_width = constants::BOARD_SHADOW_MULTIPLIER * self.square_width;
        draw_shadow(&self.board_rect, board_shadow_width);

        if self.show_rules {
            draw_rectangle(
                self.board_rect.x,
                self.board_rect.y,
                self.board_rect.w,
                self.board_rect.h,
                UiColor::Yellow.to_bg_color(),
            );

            let font_size = self.heading_font_size * 0.6;
            let rules = "\
                Every move should be a \n\
                capture. Win when only \n\
                one piece is left.\n";
            let measurement =
                measure_text(rules, Some(&self.resources.font()), font_size as u16, 1.0);
            let draw_text_params = TextParams {
                font_size: font_size as u16,
                font: Some(&self.resources.font()),
                color: UiColor::Brown.to_bg_color(),
                ..Default::default()
            };
            draw_multiline_text_ex(
                rules,
                self.board_rect.x + 0.5 * self.square_width,
                self.board_rect.y + 0.5 * (self.board_rect.h - measurement.height)
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
                let texture_params = self.piece_draw_texture_params(&p, sprite_size);
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
                let texture_params = self.piece_draw_texture_params(&p, sprite_size);
                draw_texture_ex(
                    texture_params.texture,
                    mouse_position().0 - sprite_size / 2.0,
                    mouse_position().1 - sprite_size / 2.0,
                    WHITE,
                    texture_params.draw_text_params,
                );
            }
        }
    }

    fn piece_draw_texture_params<'a>(
        &'a self,
        piece: &Piece,
        sprite_size: f32,
    ) -> PieceDrawTextureParams<'a> {
        let texture = self.resources.get_piece_texture(&piece);
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

    fn draw_buttons(&mut self) {
        self.reset_btn.draw(
            constants::RESET_BUTTON_TEXT,
            &UiColor::Yellow,
            &self.resources.font(),
        );
        self.next_btn.draw(
            constants::NEXT_BUTTON_TEXT,
            &UiColor::Green,
            &self.resources.font(),
        );
        self.easy_btn.draw(
            constants::EASY_BUTTON_TEXT,
            &UiColor::Yellow,
            &self.resources.font(),
        );
        self.medium_btn.draw(
            constants::MEDIUM_BUTTON_TEXT,
            &UiColor::Yellow,
            &self.resources.font(),
        );
        self.hard_btn.draw(
            constants::HARD_BUTTON_TEXT,
            &UiColor::Yellow,
            &self.resources.font(),
        );

        let rules_btn_text = if self.show_rules {
            constants::RULES_BUTTON_ALT_TEXT
        } else {
            constants::RULES_BUTTON_TEXT
        };
        self.rules_btn
            .draw(rules_btn_text, &UiColor::Brown, &self.resources.font());
    }

    fn draw_id_text(&self) {
        self.id_text
            .draw(&self.resources.font(), &self.puzzle.board.id);
    }

    fn draw_debug(&self) {
        if self.debug {
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

            self.show_fps();
        }
    }

    fn add_debug_info(&self, lines: Vec<String>) {
        let mut y = 20.0;
        for line in lines {
            draw_text(&line, 10.0, y, 20.0, BLACK);
            y += 25.0;
        }
    }

    fn show_fps(&self) {
        let fps = get_fps();
        draw_text(
            &format!("FPS: {}", fps),
            10.0,
            screen_height() - 20.0,
            20.0,
            BLACK,
        );
    }
}

struct PieceDrawTextureParams<'a> {
    texture: &'a Texture2D,
    draw_text_params: DrawTextureParams,
}
