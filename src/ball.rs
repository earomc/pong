use std::time::Duration;

use ggez::Context;
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect};
use crate::{Side, SCREEN_HEIGHT, SCREEN_WIDTH, SLIDER_WIDTH, X_MARGIN};
use crate::player::Player;

const BALL_MAGNITUDE : f32 = 1200.0;
const BALL_SIZE : f32 = 20.0;

pub struct Ball {
    pos: Vec2,
    vel: Vec2,
    mesh: Mesh,
    bounds: Rect,
    paused_until: Duration
}

fn center_pos() -> Vec2 {
    Vec2::new((SCREEN_WIDTH - BALL_SIZE) / 2.0, (SCREEN_HEIGHT - BALL_SIZE) / 2.0)
}

impl Ball {
    pub(crate) fn new(ctx: &mut Context) -> Ball {
        let bounds: Rect = Rect::new(0.0, 0.0, BALL_SIZE, BALL_SIZE);
        let vel = Ball::random_vel();
        Ball {
            pos: center_pos(),
            vel,
            bounds,
            mesh: Mesh::new_rectangle(ctx, DrawMode::fill(), bounds.clone(), Color::WHITE).unwrap(),
            paused_until: ctx.time.time_since_start() + Duration::from_secs(5)
        }
    }

    pub fn update(&mut self, ctx: &mut Context, players: &mut (Player, Player)) -> CollisionResult {
        if !self.is_paused(ctx) {
            self.update_pos(ctx);
            return self.check_collision(ctx, players);  
        }
        CollisionResult::None
    }

    fn is_paused(&self, ctx: &mut Context) -> bool {
        self.paused_until > ctx.time.time_since_start()
    }

    fn random_vel() -> Vec2 {
        let mut vel = Vec2::new(rand::random::<f32>(), rand::random::<f32>());
        vel.y /= 128.0;
        let mut vel = if rand::random::<bool>() { -vel.normalize() } else { vel.normalize() };
        vel = vel.normalize() * BALL_MAGNITUDE;
        vel
    }

    pub(crate) fn update_pos(&mut self, ctx: &mut Context) {
        self.pos.x += self.vel.x * ctx.time.average_delta().as_secs_f32();
        self.pos.y += self.vel.y * ctx.time.average_delta().as_secs_f32();

        self.bounds.x = self.pos.x;
        self.bounds.y = self.pos.y;
    }

    fn set_position(&mut self, vec2: Vec2) {
        self.pos = vec2;
        self.bounds.x = vec2.x;
        self.bounds.y = vec2.y;
    }

    pub fn reset_pos(&mut self) {
        self.set_position(center_pos());    
        self.vel = Ball::random_vel();
    }

    fn check_collision(&mut self, ctx: &mut Context, players: &mut (Player, Player)) -> CollisionResult {
        if self.pos.y > SCREEN_HEIGHT - self.bounds.h || self.pos.y < 0.0  {
            self.vel.y = -self.vel.y;
            return CollisionResult::CollideTopBottom;
        }
        let player_0_bounds = &players.0.bounds;
        let player_1_bounds = &players.1.bounds;
        if player_0_bounds.overlaps(&self.bounds) {
            let x = SLIDER_WIDTH + X_MARGIN;
            self.bounds.x = x;
            self.pos.x = x;      

            self.vel = self.calc_new_vel(&players.0);
            return CollisionResult::CollideSlider(Side::Left);
        }
        if player_1_bounds.overlaps(&self.bounds) {
            let x = SCREEN_WIDTH - SLIDER_WIDTH - BALL_SIZE - X_MARGIN;
            self.bounds.x = x;
            self.pos.x = x;

            self.vel = self.calc_new_vel(&players.1);            
            return CollisionResult::CollideSlider(Side::Right);
        }
        let pause_duration = Duration::from_secs(2);
        if self.pos.x > SCREEN_WIDTH - SLIDER_WIDTH {
            players.0.score += 1;
            self.reset_pos();
            self.pause(ctx, pause_duration);
            return CollisionResult::PlayerScores;
        } else if self.pos.x < 0.0 {
            players.1.score += 1;
            self.reset_pos();
            self.pause(ctx, pause_duration);
            return CollisionResult::PlayerScores;
        }
        CollisionResult::None
    }

    fn pause(&mut self, ctx: &mut Context, duration: Duration) {
        self.paused_until = ctx.time.time_since_start() + duration;
    }

    fn calc_new_vel(&mut self, player: &Player) -> Vec2 {
        let diff = self.pos.y - player.bounds.top();

        let angle = match player.side {
            Side::Left =>  Self::map_range((0.0, player.bounds.h), (-45_f32.to_radians(), 45_f32.to_radians()), diff),
            Side::Right => Self::map_range((0.0, player.bounds.h), (225_f32.to_radians(), 135_f32.to_radians()), diff),
        }; 
        let vel = Vec2::new(angle.cos(), angle.sin());
        vel * BALL_MAGNITUDE
    }

    fn map_range(from_range: (f32, f32), to_range: (f32, f32), s: f32) -> f32 {
        to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
    }

    pub fn draw(&mut self, canvas: &mut Canvas) {
        canvas.draw(&self.mesh, DrawParam::default().dest(self.pos));
    }
}

pub enum CollisionResult {
    PlayerScores,
    CollideSlider(Side),
    CollideTopBottom,
    None
}