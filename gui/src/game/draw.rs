use super::{Game, constants};
use crate::{game::GameMode, widgets::*};
use macroquad::{math, prelude::*};

impl Game {
    fn initialize_drawables(&mut self, new_width: f32, new_height: f32) {
        self.window_height = new_height;
        self.window_width = new_width;

        let min_dimension = f32::min(self.window_height, self.window_width);
        let square_width = constants::BOARD_SQUARE_WIDTH_MULTIPLIER * min_dimension;
        let num_squares = self.board.num_squares;
        let board_width = square_width * num_squares as f32;
        let board_x = (self.window_width * 0.6 - board_width) / 2.0;
        let board_y = (self.window_height - board_width) / 2.0;
        let board_rect = Rect::new(board_x, board_y, board_width, board_width);
        self.board.initialize_drawables(square_width, board_rect);

        let heading_font_size = constants::HEADING_FONT_SIZE_MULTIPLIER * min_dimension;
        let f = heading_font_size.floor() as u16;
        let heading_text_dims = measure_text(
            self.heading_text.as_str(),
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
        self.heading
            .initialize_drawables(heading_rect, heading_font_size, &self.resources);

        // Buttons
        let btn_h = constants::BUTTON_HEIGHT_MULTIPLIER * min_dimension;
        let btn_w = constants::BUTTON_WIDTH_MULTIPLIER * board_width;

        // Bottom row
        let bottom_row_y =
            board_width + board_y + constants::BOTTOM_ROW_OFFSET_MULTIPLIER * square_width;
        self.rules_btn.initialize_drawables(Rect::new(
            board_x + (square_width - btn_w) / 2.,
            bottom_row_y,
            btn_w,
            btn_h,
        ));

        self.reset_btn.initialize_drawables(Rect::new(
            board_x + square_width + (square_width - btn_w) / 2.,
            bottom_row_y,
            btn_w,
            btn_h,
        ));

        let btn_next_x_offset = (board_width - square_width) + (square_width - btn_w) / 2.;
        self.next_btn.initialize_drawables(Rect::new(
            board_x + btn_next_x_offset,
            bottom_row_y,
            btn_w,
            btn_h,
        ));

        let id_text_rect = Rect::new(
            board_x + 2. * square_width - btn_w,
            bottom_row_y,
            btn_w * 2.,
            btn_h,
        );
        self.id_text_btn.initialize_drawables(id_text_rect);

        self.rules_font_size = heading_font_size;

        // Right column
        let right_column_x =
            board_x + board_width + constants::RIGHT_COLUMN_OFFSET_MULTIPLIER * square_width;
        self.get_game_mode_button(&GameMode::Easy)
            .initialize_drawables(Rect::new(
                right_column_x,
                board_y + (square_width - btn_h) / 2.,
                btn_w,
                btn_h,
            ));

        self.get_game_mode_button(&GameMode::Medium)
            .initialize_drawables(Rect::new(
                right_column_x,
                board_y + square_width + (square_width - btn_h) / 2.,
                btn_w,
                btn_h,
            ));

        self.get_game_mode_button(&GameMode::Hard)
            .initialize_drawables(Rect::new(
                right_column_x,
                board_y + 2. * square_width + (square_width - btn_h) / 2.,
                btn_w,
                btn_h,
            ));

        self.get_game_mode_button(&GameMode::Custom)
            .initialize_drawables(Rect::new(
                right_column_x,
                board_y + 3. * square_width + (square_width - btn_h) / 2.,
                btn_w,
                btn_h,
            ));

        // Right column 2
        let right_column_2_x =
            right_column_x + btn_w + constants::RIGHT_COLUMN_OFFSET_MULTIPLIER * square_width;
        self.generate_btn.initialize_drawables(Rect::new(
            right_column_2_x,
            board_y + board_width - square_width + (square_width - btn_h) / 2.,
            2. * btn_w,
            btn_h,
        ));

        self.pieces_label.initialize_drawables(
            Rect::new(
                right_column_2_x,
                board_y + (square_width + btn_h) / 2.,
                btn_w,
                btn_h,
            ),
            0.30 * btn_h,
            &self.resources,
        );
        self.pieces_counter.initialize_drawables(
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
        self.max_age_label.initialize_drawables(
            Rect::new(
                right_column_2_x + btn_w,
                board_y + (square_width + btn_h) / 2.,
                btn_w,
                btn_h,
            ),
            0.30 * btn_h,
            &self.resources,
        );
        self.max_age_counter.initialize_drawables(
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

        self.draw_heading();
        self.draw_board();
        self.draw_id_text_button();
        self.draw_buttons();
        self.draw_setting_controls();
        self.draw_debug();
    }

    fn draw_heading(&self) {
        self.heading.draw(&self.resources);
    }

    fn draw_board(&mut self) {
        let params = BoardDrawParams {
            show_rules: self.show_rules,
            rules_font_size: self.rules_font_size,
        };

        self.board.draw(&params, &self.resources, &self.settings);
    }

    fn get_game_mode_button<'a>(&'a mut self, mode: &GameMode) -> &'a mut ButtonWidget {
        self.game_mode_btns.get_mut(mode).unwrap()
    }

    fn draw_game_mode_button(&mut self, mode: &GameMode, color: &UiColor, text: &str) {
        let btn = self.game_mode_btns.get_mut(mode).unwrap();
        btn.draw(text, color, &self.resources);
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

        self.draw_game_mode_button(
            &GameMode::Easy,
            &UiColor::Yellow,
            constants::EASY_BUTTON_TEXT,
        );

        self.draw_game_mode_button(
            &GameMode::Medium,
            &UiColor::Yellow,
            constants::MEDIUM_BUTTON_TEXT,
        );

        self.draw_game_mode_button(
            &GameMode::Hard,
            &UiColor::Yellow,
            constants::HARD_BUTTON_TEXT,
        );

        self.draw_game_mode_button(
            &GameMode::Custom,
            &UiColor::Blue,
            constants::CUSTOM_BUTTON_TEXT,
        );

        self.generate_btn.draw(
            constants::GENERATE_BUTTON_TEXT,
            &UiColor::Pink,
            &self.resources,
        );

        self.rules_btn
            .draw(&self.rules_btn_text, &UiColor::Brown, &self.resources);
    }

    fn draw_setting_controls(&mut self) {
        self.pieces_label.draw(&self.resources);
        self.pieces_counter.draw(&self.resources);
        self.max_age_label.draw(&self.resources);
        self.max_age_counter.draw(&self.resources);
    }

    fn draw_id_text_button(&self) {
        // self.id_text_btn
        //     .draw(&self.puzzle.board.id, &UiColor::Yellow, &self.resources);
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

        self.initialize_drawables(new_width, new_height);
    }
}
