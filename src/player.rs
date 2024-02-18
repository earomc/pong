use ggez::Context;
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect};
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH, Side, X_MARGIN};

struct Player {
    pos: Vec2,
    score: i32,
    side: Side,
    mesh: Mesh,
    draw_param: DrawParam,
    bounds: Rect
}

impl Player {
    fn new(side: Side, ctx: &Context, bat_width: f32, bat_height: f32) -> Self {
        let pos = match side {
            Side::Left => Vec2::new(X_MARGIN, (SCREEN_HEIGHT - bat_height) / 2.0),
            Side::Right => Vec2::new(SCREEN_WIDTH - X_MARGIN - bat_width, (SCREEN_HEIGHT - bat_height) / 2.0)
        };

        let draw_param: DrawParam = DrawParam::default().dest(pos);
        let bounds = Rect::new(pos.x, pos.y, bat_width, bat_height);

        let mesh_bounds = Rect::new(0.0, 0.0, bat_width, bat_height);

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

    fn move_y(&mut self, y_delta: f32) {
        let mut y = self.pos.y.clone();
        y += y_delta;
        y = y.clamp(0.0, SCREEN_HEIGHT - self.bounds.h);
        self.pos.y = y;
        self.bounds.y = y;
    }

    fn draw(&mut self, canvas: &mut Canvas) {
        self.draw_param = self.draw_param.dest(self.pos);
        canvas.draw(&self.mesh, self.draw_param);
    }
}