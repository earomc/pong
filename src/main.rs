#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod ball;
mod middle_line;
mod player;
mod scores_display;

use crate::ball::{Ball, CollisionResult};
use std::env;
use std::path::PathBuf;

use crate::middle_line::MiddleLine;
use crate::player::Player;
use crate::scores_display::ScoresDisplay;
use ggez::graphics::FontData;
use ggez::{
    audio::{SoundSource, Source},
    conf::{WindowMode, WindowSetup},
    event::{self, EventHandler},
    glam::Vec2,
    graphics::{Canvas, Color},
    input::keyboard::KeyInput,
    mint::Point2,
    winit::event::VirtualKeyCode,
    *,
};

const SCREEN_HEIGHT: f32 = 720.0;
const SCREEN_WIDTH: f32 = 1280.0;

const SLIDER_HEIGHT: f32 = 150.0;
const SLIDER_WIDTH: f32 = 20.0;

const X_MARGIN: f32 = 20.0;

const FONT_NAME: &str = "PressStart";

fn main() -> GameResult {
    env::set_var("RUST_BACKTRACE", "1");

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        PathBuf::from("./resources")
    };

    let mut fonts_dir = PathBuf::from(&resource_dir);
    fonts_dir.push("fonts");

    let mut sounds_dir = PathBuf::from(&resource_dir);
    sounds_dir.push("sounds");

    let mut img_dir = PathBuf::from(&resource_dir);
    img_dir.push("img");

    let cb = ContextBuilder::new("pong", "earomc").window_mode(WindowMode {
        height: 720.0,
        width: 1280.0,
        ..Default::default()
    });
    let (mut ctx, event_loop) = cb
        .add_resource_path(resource_dir)
        .add_resource_path(fonts_dir)
        .add_resource_path(sounds_dir)
        .add_resource_path(img_dir)
        .window_setup(
            WindowSetup::default()
                .title("PONG | Made by earomc")
                .icon("/icon.png"),
        )
        .build()?;
    let font_data = FontData::from_path(&ctx, "/PressStart.ttf")?;
    ctx.gfx.add_font(FONT_NAME, font_data);
    let state = PongState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}

struct Sounds {
    click_high: Source,
    click_low: Source,
    score: Source,
}

impl Sounds {
    fn new(ctx: &Context) -> GameResult<Sounds> {
        let sounds = Sounds {
            click_high: Source::new(ctx, "/click_high.wav")?,
            click_low: Source::new(ctx, "/click_low.wav")?,
            score: Source::new(ctx, "/score.wav")?,
        };
        Ok(sounds)
    }
}

struct PongState {
    players: (Player, Player),
    ball: Ball,
    middle_line: MiddleLine,
    button_state: ButtonState,
    scores_display: ScoresDisplay,
    sounds: Sounds,
}
enum Side {
    Left,
    Right,
}

pub fn vec2_to_point(value: Vec2) -> Point2<f32> {
    Point2 {
        x: value.x,
        y: value.y,
    }
}

pub fn point_to_vec2(value: Point2<f32>) -> Vec2 {
    Vec2::new(value.x, value.y)
}

impl PongState {
    fn new(ctx: &mut Context) -> GameResult<PongState> {
        let players = (Player::new(Side::Left, ctx), Player::new(Side::Right, ctx));
        let scores_display = ScoresDisplay::new(&players);
        let state = PongState {
            players,
            button_state: ButtonState::default(),
            scores_display,
            ball: Ball::new(ctx),
            middle_line: MiddleLine::new(ctx),
            sounds: Sounds::new(ctx)?,
        };
        Ok(state)
    }
}

#[derive(Default)]
struct ButtonState {
    w_pressed: bool,
    s_pressed: bool,
    up_pressed: bool,
    down_pressed: bool,
}

impl EventHandler<GameError> for PongState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let pixels_per_second = 1200.0;
        let step = pixels_per_second * ctx.time.average_delta().as_secs_f32();
        if self.button_state.w_pressed {
            self.players.0.move_y(-step)
        } else if self.button_state.s_pressed {
            self.players.0.move_y(step)
        }
        if self.button_state.up_pressed {
            self.players.1.move_y(-step)
        } else if self.button_state.down_pressed {
            self.players.1.move_y(step)
        }
        match self.ball.update(ctx, &mut self.players) {
            CollisionResult::PlayerScores => {
                self.scores_display.update_score(&self.players);
                self.sounds.score.play(ctx)?;
            }
            CollisionResult::CollideSlider(side) => match side {
                Side::Left => self.sounds.click_high.play(ctx)?,
                Side::Right => self.sounds.click_low.play(ctx)?,
            },
            CollisionResult::CollideTopBottom => {}
            CollisionResult::None => {}
        };
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        self.players.0.draw(&mut canvas);
        self.players.1.draw(&mut canvas);

        self.ball.draw(&mut canvas);
        self.middle_line.draw(&mut canvas);

        self.scores_display.draw(&mut canvas);

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: KeyInput,
        repeated: bool,
    ) -> GameResult {
        if repeated {
            return Ok(());
        }
        if let Some(keycode) = input.keycode {
            match keycode {
                VirtualKeyCode::W => {
                    self.button_state.w_pressed = true;
                }
                VirtualKeyCode::S => {
                    self.button_state.s_pressed = true;
                }
                VirtualKeyCode::Up => {
                    self.button_state.up_pressed = true;
                }
                VirtualKeyCode::Down => {
                    self.button_state.down_pressed = true;
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, input: KeyInput) -> GameResult {
        if let Some(keycode) = input.keycode {
            match keycode {
                VirtualKeyCode::W => {
                    self.button_state.w_pressed = false;
                }
                VirtualKeyCode::S => {
                    self.button_state.s_pressed = false;
                }
                VirtualKeyCode::Up => {
                    self.button_state.up_pressed = false;
                }
                VirtualKeyCode::Down => {
                    self.button_state.down_pressed = false;
                }
                _ => (),
            }
        }
        Ok(())
    }
}
