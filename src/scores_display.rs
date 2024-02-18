use crate::player::Player;
use crate::{FONT_NAME, SCREEN_WIDTH};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Text};

pub struct ScoresDisplay {
    pos0: Vec2,
    pos1: Vec2,
    text0: Text,
    text1: Text,
}

const FONT_SIZE: f32 = 60.;

impl ScoresDisplay {
    pub fn new(players: &(Player, Player)) -> Self {
        let score0 = players.0.score;
        let score1 = players.1.score;
        ScoresDisplay {
            pos0: Self::create_pos0(score0),
            pos1: Vec2::new(SCREEN_WIDTH / 2.0 + 20.0, 10.0),
            text0: Self::create_score_text(score0.to_string()),
            text1: Self::create_score_text(score1.to_string()),
        }
    }

    fn calc_pos0_offset(score: i32) -> f32 {
        if score == 0 {
            return 0.0;
        }
        (((score as f32).log10() + 1.0).floor() - 1.0) * FONT_SIZE
    }

    fn create_pos0(score0: i32) -> Vec2 {
        Vec2::new(
            SCREEN_WIDTH / 2.0 - 20.0 - FONT_SIZE - Self::calc_pos0_offset(score0),
            10.0,
        )
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        canvas.draw(&self.text0, self.pos0);
        canvas.draw(&self.text1, self.pos1);
    }

    fn create_score_text(string: String) -> Text {
        let mut score_text = Text::new(string);
        score_text.set_font(FONT_NAME).set_scale(FONT_SIZE);
        score_text
    }

    pub fn update_score(&mut self, players: &(Player, Player)) {
        self.text0 = Self::create_score_text(players.0.score.to_string());
        self.text1 = Self::create_score_text(players.1.score.to_string());
        self.pos0 = Self::create_pos0(players.0.score);
    }
}
