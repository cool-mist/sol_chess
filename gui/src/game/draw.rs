use super::{Game, constants};
use crate::widgets::*;
use macroquad::{math, prelude::*};

impl Game {
    fn initialize_drawables(&mut self) {
        let min_dimension = f32::min(self.window_height, self.window_width);
        let square_width = constants::BOARD_SQUARE_WIDTH_MULTIPLIER * min_dimension;
        let num_squares = self.board.num_squares;
        let board_width = square_width * num_squares as f32;
        let board_x = (self.window_width - board_width) / 2.0;
        let board_y = (self.window_height - board_width) / 2.0;
        let board_rect = Rect::new(board_x, board_y, board_width, board_width);
        self.board.initialize_drawables(square_width, board_rect);

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
            board_y - dims.height - constants::TOP_HEADING_OFFSET_MULTIPLIER * square_width,
            dims.width,
            dims.height,
        );

        // Buttons
        let btn_h = constants::BUTTON_HEIGHT_MULTIPLIER * min_dimension;
        let btn_w = board_width * constants::BUTTON_WIDTH_MULTIPLIER;
        let btn_w_sq = btn_h;

        // Bottom row
        let bottom_row_y =
            board_width + board_y + constants::BOTTOM_BUTTON_ROW_OFFSET_MULTIPLIER * square_width;
        self.reset_btn.initialize_drawables(Rect::new(
            board_x + (square_width - btn_w_sq) / 2.,
            bottom_row_y,
            btn_w_sq,
            btn_h,
        ));
        let btn_next_x_offset = (board_width - square_width) + (square_width - btn_w_sq) / 2.;
        self.next_btn.initialize_drawables(Rect::new(
            board_x + btn_next_x_offset,
            bottom_row_y,
            btn_w_sq,
            btn_h,
        ));

        let id_text_rect = Rect::new(
            board_x + 2. * square_width - btn_w,
            bottom_row_y,
            btn_w * 2.,
            btn_h,
        );
        self.id_text = IdTextInput::new(id_text_rect);

        // Left column
        let left_column_x =
            board_x - constants::BOTTOM_RIGHT_ROW_OFFSET_MULTIPLIER * square_width - btn_w;
        self.rules_btn.initialize_drawables(Rect::new(
            left_column_x,
            board_y + board_rect.h - square_width + (square_width - btn_h) / 2.,
            btn_w,
            btn_h,
        ));

        // Right column
        let right_column_x =
            board_x + board_width + constants::BOTTOM_RIGHT_ROW_OFFSET_MULTIPLIER * square_width;
        self.easy_btn.initialize_drawables(Rect::new(
            right_column_x,
            board_y + square_width + (square_width - btn_h) / 2.,
            btn_w,
            btn_h,
        ));

        self.medium_btn.initialize_drawables(Rect::new(
            right_column_x,
            board_y + 2. * square_width + (square_width - btn_h) / 2.,
            btn_w,
            btn_h,
        ));

        self.hard_btn.initialize_drawables(Rect::new(
            right_column_x,
            board_y + 3. * square_width + (square_width - btn_h) / 2.,
            btn_w,
            btn_h,
        ));
    }

    pub fn draw(&mut self) {
        self.update_window_size();
        self.draw_heading();
        self.board.draw(
            self.show_rules,
            self.heading_font_size,
            &self.resources,
            &self.settings,
        );
        self.id_text.draw(&self.resources, &self.puzzle.board.id);
        self.draw_buttons();
        self.draw_debug();
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

    fn draw_buttons(&mut self) {
        self.reset_btn.draw(
            constants::RESET_BUTTON_TEXT,
            &UiColor::Yellow,
            &self.resources,
        );

        self.next_btn.draw(
            constants::NEXT_BUTTON_TEXT,
            &UiColor::Green,
            &self.resources,
        );

        self.easy_btn.draw(
            constants::EASY_BUTTON_TEXT,
            &UiColor::Yellow,
            &self.resources,
        );

        self.medium_btn.draw(
            constants::MEDIUM_BUTTON_TEXT,
            &UiColor::Yellow,
            &self.resources,
        );

        self.hard_btn.draw(
            constants::HARD_BUTTON_TEXT,
            &UiColor::Yellow,
            &self.resources,
        );

        self.rules_btn
            .draw(&self.rules_text, &UiColor::Brown, &self.resources);
    }

    fn draw_debug(&self) {
        if self.settings.debug {
            self.show_fps();
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
}
