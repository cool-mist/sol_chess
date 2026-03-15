use super::{Game, constants};
use crate::{
    game::{self, GameMode, w_board, w_button, w_counter, w_label},
    widgets::*,
};
use macroquad::{math, prelude::*};

impl Game {
    fn initialize_scene(&mut self, new_width: f32, new_height: f32) {
        self.window_height = new_height;
        self.window_width = new_width;

        let min_dimension = f32::min(self.window_height, self.window_width);
        let square_width = constants::BOARD_SQUARE_WIDTH_MULTIPLIER * min_dimension;
        let num_squares = w_board!(self, board).num_squares;
        let board_width = square_width * num_squares as f32;
        let board_x = (self.window_width * 0.6 - board_width) / 2.0;
        let board_y = (self.window_height - board_width) / 2.0;

        let heading_font_size = constants::HEADING_FONT_SIZE_MULTIPLIER * min_dimension;
        let f = heading_font_size.floor() as u16;
        let heading_text_dims = measure_text(
            w_label!(self, label_heading).text.as_str(),
            Some(&self.resources.font()),
            f,
            1.0,
        );
        let heading_rect = Rect::new(
            (self.window_width - heading_text_dims.width) / 2.0,
            board_y
                - heading_text_dims.height
                - constants::TOP_HEADING_OFFSET_MULTIPLIER * square_width,
            heading_text_dims.width,
            heading_text_dims.height,
        );
        w_label!(self, label_heading).initialize_drawables(
            heading_rect,
            heading_font_size,
            &self.resources,
        );

        let board_rect = Rect::new(board_x, board_y, board_width, board_width);
        w_board!(self, board).initialize_drawables(
            square_width,
            board_rect,
            heading_font_size * 0.4,
        );

        // Buttons
        let btn_h = constants::BUTTON_HEIGHT_MULTIPLIER * min_dimension;
        let btn_w = constants::BUTTON_WIDTH_MULTIPLIER * board_width;

        // Bottom row
        let bottom_row_y =
            board_width + board_y + constants::BOTTOM_ROW_OFFSET_MULTIPLIER * square_width;
        w_button!(self, btn_rules).place(Rect::new(
            board_x + (square_width - btn_w) / 2.,
            bottom_row_y,
            btn_w,
            btn_h,
        ));

        w_button!(self, btn_reset).place(Rect::new(
            board_x + square_width + (square_width - btn_w) / 2.,
            bottom_row_y,
            btn_w,
            btn_h,
        ));

        let btn_next_x_offset = (board_width - square_width) + (square_width - btn_w) / 2.;
        w_button!(self, btn_next).place(Rect::new(
            board_x + btn_next_x_offset,
            bottom_row_y,
            btn_w,
            btn_h,
        ));

        // Right column
        let right_column_x =
            board_x + board_width + constants::RIGHT_COLUMN_OFFSET_MULTIPLIER * square_width;
        self.get_game_mode_button(&GameMode::Easy).place(Rect::new(
            right_column_x,
            board_y + (square_width - btn_h) / 2.,
            btn_w,
            btn_h,
        ));

        self.get_game_mode_button(&GameMode::Medium)
            .place(Rect::new(
                right_column_x,
                board_y + square_width + (square_width - btn_h) / 2.,
                btn_w,
                btn_h,
            ));

        self.get_game_mode_button(&GameMode::Hard).place(Rect::new(
            right_column_x,
            board_y + 2. * square_width + (square_width - btn_h) / 2.,
            btn_w,
            btn_h,
        ));

        self.get_game_mode_button(&GameMode::Custom)
            .place(Rect::new(
                right_column_x,
                board_y + 3. * square_width + (square_width - btn_h) / 2.,
                btn_w,
                btn_h,
            ));

        // Right column 2
        let right_column_2_x =
            right_column_x + btn_w + constants::RIGHT_COLUMN_OFFSET_MULTIPLIER * square_width;
        w_button!(self, btn_generate).place(Rect::new(
            right_column_2_x,
            board_y + board_width - square_width + (square_width - btn_h) / 2.,
            2. * btn_w,
            btn_h,
        ));

        w_label!(self, label_pieces).initialize_drawables(
            Rect::new(
                right_column_2_x,
                board_y + (square_width + btn_h) / 2.,
                btn_w,
                btn_h,
            ),
            0.30 * btn_h,
            &self.resources,
        );

        w_counter!(self, counter_pieces).initialize_drawables(
            Rect::new(
                right_column_2_x,
                board_y + square_width + (2. * square_width - 2. * btn_h) / 2.,
                btn_w,
                2. * btn_h,
            ),
            30.,
            &self.resources,
        );

        // Right column 3
        let right_column_3_x = right_column_2_x + btn_w;
        w_label!(self, label_max_age).initialize_drawables(
            Rect::new(
                right_column_2_x + btn_w,
                board_y + (square_width + btn_h) / 2.,
                btn_w,
                btn_h,
            ),
            0.30 * btn_h,
            &self.resources,
        );
        w_counter!(self, counter_moves).initialize_drawables(
            Rect::new(
                right_column_3_x,
                board_y + square_width + (2. * square_width - 2. * btn_h) / 2.,
                btn_w,
                2. * btn_h,
            ),
            30.,
            &self.resources,
        );
    }

    pub fn draw(&mut self) {
        self.update_window_size();

        for (_, widget) in &mut self.widgets {
            widget.draw(&self.resources, &self.settings);
        }

        self.draw_fps();
    }

    fn get_game_mode_button<'a>(&'a mut self, mode: &GameMode) -> &'a mut ButtonWidget {
        let id = self.game_mode_btns.get(mode).unwrap();
        game::button(&mut self.widgets, *id)
    }

    fn draw_fps(&self) {
        if self.settings.debug {
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

        self.initialize_scene(new_width, new_height);
    }
}
