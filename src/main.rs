mod player;
mod ball;
mod scores_display;
mod middle_line;

use crate::ball::{Ball, CollisionResult};
use std::env;
use std::path::PathBuf;

use ggez::{
    *, conf::WindowMode, event::{self, EventHandler}, glam::Vec2, graphics::{Canvas, Color}, input::keyboard::KeyInput, mint::Point2, winit::event::VirtualKeyCode
};
use ggez::graphics::{FontData};
use crate::middle_line::MiddleLine;
use crate::player::Player;
use crate::scores_display::ScoresDisplay;

const SCREEN_HEIGHT: f32 = 720.0;
const SCREEN_WIDTH: f32 = 1280.0;

const SLIDER_HEIGHT: f32 = 100.0;
const SLIDER_WIDTH: f32 = 20.0;

const X_MARGIN : f32 = 20.0;
const BALL_MAGNITUDE : f32 = 14.0;


const FONT_NAME: &str = "PressStart";

fn main() -> GameResult {
    env::set_var("RUST_BACKTRACE", "1");

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        println!("BOOG {}", &manifest_dir);
        let mut path = PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        PathBuf::from("./resources")
    };


    let cb = ContextBuilder::new("pong_ggez", "earomc").window_mode(WindowMode {
        height: 720.0,
        width: 1280.0,
        ..Default::default()
    });
    let (mut ctx, event_loop) = cb.add_resource_path(resource_dir).build()?;
    let font_data = FontData::from_path(&ctx, "/PressStart.ttf")?;
    ctx.gfx.add_font(FONT_NAME, font_data);
    let state = PongState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}

struct PongState {
    players: (Player, Player),
    ball: Ball,
    middle_line: MiddleLine,
    button_state: ButtonState,
    scores_display: ScoresDisplay
}
enum Side {
    Left,
    Right,
}

fn vec2_to_point(value: Vec2) -> Point2<f32> {
    Point2 {x: value.x, y: value.y}
}


fn point_to_vec2(value: Point2<f32>) -> Vec2 {
    Vec2::new(value.x, value.y)
}

fn center_pos() -> Vec2 {
    Vec2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0)
}

impl PongState {
    fn new(ctx: &mut Context) -> GameResult<PongState> {
        let players = (
            Player::new(Side::Left, ctx, SLIDER_WIDTH, SLIDER_HEIGHT),
            Player::new(Side::Right, ctx, SLIDER_WIDTH, SLIDER_HEIGHT),
        );
        let mut scores_display = ScoresDisplay::new(&players);
        let state = PongState {
            players,
            button_state: ButtonState::default(),
            scores_display,
            ball: Ball::new(ctx),
            middle_line: MiddleLine::new(ctx),
        };
        Ok(state)
    }

    fn print_score_self(&self) {
        Self::print_score(&self.players);
    }

    fn print_score(players: &(Player, Player)) {
        println!("Player 0: {} Player 1: {}", players.0.score, players.1.score);
    }
}


struct ButtonState {
    w_pressed: bool,
    s_pressed: bool,
    up_pressed: bool,
    down_pressed: bool,
}

impl Default for ButtonState {
    fn default() -> Self {
        ButtonState {
            w_pressed: false,
            s_pressed: false,
            up_pressed: false,
            down_pressed: false,
        }
    }
}

impl EventHandler<GameError> for PongState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let step = 20.0;
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
        self.ball.update_pos();
        //self.ball.set_position(point_to_vec2(ctx.mouse.position()));
        match self.ball.check_collision(&mut self.players) {
            CollisionResult::PlayerScores => {
                self.scores_display.update_score(&self.players);
            }
            CollisionResult::CollideSlider { .. } => {}
            CollisionResult::CollideTopBottom => {}
            CollisionResult::None => {}
        };
        /*while ctx.time.check_update_time(5) {
            dbg!(self.ball.bounds);
            dbg!(self.players.0.bounds);
            dbg!(self.players.1.bounds);
        }*/
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // draw players:
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        self.players.0.draw(&mut canvas);
        self.players.1.draw(&mut canvas);

        self.ball.draw(&mut canvas);
        self.middle_line.draw(&mut canvas);

        self.scores_display.draw(&mut canvas);

        //println!("Player1 y: {:?} Player2 y: {:?}", self.players.0.pos.y, self.players.1.pos.y);
        //println!("Player1: {:?} Player2: {:?}", self.players.0.draw_param, self.players.1.draw_param);

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: KeyInput,
        repeated: bool,
    ) -> GameResult {
        if repeated {
            return Ok(());
        }
        if let Some(keycode) = input.keycode {
            match keycode {
                VirtualKeyCode::Space => {
                    self.ball.reset_pos();
                }
                VirtualKeyCode::W => {
                    //println!("W down");
                    self.button_state.w_pressed = true;
                }
                VirtualKeyCode::S => {
                    //println!("S down");
                    self.button_state.s_pressed = true;
                }
                VirtualKeyCode::Up => {
                    //println!("Up down");
                    self.button_state.up_pressed = true;
                }
                VirtualKeyCode::Down => {
                    //println!("Down down");
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
                    //println!("W up");
                    self.button_state.w_pressed = false;
                }
                VirtualKeyCode::S => {
                    //println!("S up");
                    self.button_state.s_pressed = false;
                }
                VirtualKeyCode::Up => {
                    //println!("Up up");
                    self.button_state.up_pressed = false;
                }
                VirtualKeyCode::Down => {
                    //println!("Down up");
                    self.button_state.down_pressed = false;
                }
                _ => (),
            }
        }
        Ok(())
    }
}
