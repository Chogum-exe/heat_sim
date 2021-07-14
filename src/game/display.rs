use ggez::{Context, graphics::*};

pub fn rect(x: f32, y: f32, w: f32, h: f32, color: (f32, f32, f32, f32), stroke: bool, thick: f32, ctx: &mut Context) {
    MeshBuilder::new()
        .rectangle(
            if stroke { DrawMode::stroke(thick) } else { DrawMode::fill() },
            Rect::new(x, y, w, h),
            color.into()
        )
        .build(ctx).unwrap()
        .draw(ctx, DrawParam::default()).unwrap();
}