use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect};
use ggez::Context;

const LINE_WIDTH: f32 = 5.0;

pub struct MiddleLine {
    mesh: Mesh,
    pos: Vec2,
}

impl MiddleLine {
    pub fn new(ctx: &Context) -> Self {
        let rect = Rect::new(0.0, 0.0, LINE_WIDTH, SCREEN_HEIGHT);
        let mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::WHITE).unwrap();
        MiddleLine {
            mesh,
            pos: Vec2::new((SCREEN_WIDTH - LINE_WIDTH) / 2.0, 0.0),
        }
    }
    pub fn draw(&self, canvas: &mut Canvas) {
        canvas.draw(&self.mesh, DrawParam::default().dest(self.pos))
    }
}