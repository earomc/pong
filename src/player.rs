use ggez::Context;
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect};
use crate::{Side, SCREEN_HEIGHT, SCREEN_WIDTH, SLIDER_HEIGHT, SLIDER_WIDTH, X_MARGIN};

pub struct Player {
    pos: Vec2,
    pub bounds: Rect,
    pub score: i32,
    pub side: Side,
    mesh: Mesh,
    draw_param: DrawParam
}

impl Player {
    pub fn new(side: Side, ctx: &Context) -> Self {
        let pos = match side {
            Side::Left => Vec2::new(X_MARGIN, (SCREEN_HEIGHT - SLIDER_HEIGHT) / 2.0),
            Side::Right => Vec2::new(SCREEN_WIDTH - X_MARGIN - SLIDER_WIDTH, (SCREEN_HEIGHT - SLIDER_HEIGHT) / 2.0)
        };

        let draw_param: DrawParam = DrawParam::default().dest(pos);
        let bounds = Rect::new(pos.x, pos.y, SLIDER_WIDTH, SLIDER_HEIGHT);

        let mesh_bounds = Rect::new(0.0, 0.0, SLIDER_WIDTH, SLIDER_HEIGHT);

        Self {
            pos,
            score: 0,
            side,
            bounds,
            mesh: Mesh::new_rectangle(ctx, DrawMode::fill(), mesh_bounds, Color::WHITE)
                .unwrap(),
            draw_param,
        }
    }

    pub fn move_y(&mut self, y_delta: f32) {
        let mut y = self.pos.y.clone();
        y += y_delta;
        y = y.clamp(0.0, SCREEN_HEIGHT - self.bounds.h);
        self.pos.y = y;
        self.bounds.y = y;
    }

    pub fn draw(&mut self, canvas: &mut Canvas) {
        self.draw_param = self.draw_param.dest(self.pos);
        canvas.draw(&self.mesh, self.draw_param);
    }
}