use crate::prelude::*;

type Point2 = glam::Vec2;

pub struct Canvas {
    pub width: i32,
    pub height: i32,
}

impl Canvas {
    pub fn new(_ctx: &mut Context, width: i32, height: i32) -> Canvas {
        Canvas { width, height }
    }
    pub fn draw(&mut self, ctx: &mut Context, player: &mut Player, map: &mut Map) -> GameResult {
        let ray = crate::Ray::new(player, map, self.width, self.height);

        let p = ggez::graphics::DrawParam::new().dest(Point2::new(0.0, 0.0));
        let image = ggez::graphics::Image::from_rgba8(
            ctx,
            self.width as u16,
            self.height as u16,
            &ray.buffer,
        )?;

        ggez::graphics::draw(ctx, &image, p)?;

        Ok(())
    }
    pub fn draw_fps(&mut self, ctx: &mut Context) -> GameResult {
        let fps = ggez::timer::fps(ctx);
        let fps_display = ggez::graphics::Text::new(format!("FPS: {:.2}", fps));
        let p = cgmath::Point2::new(0.0, 0.0);
        ggez::graphics::draw(ctx, &fps_display, (p,))?;

        Ok(())
    }
}
