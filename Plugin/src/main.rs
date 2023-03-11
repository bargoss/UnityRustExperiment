mod draw_utils;

use bargame_lib::add;



use ggez::event;
use ggez::graphics::{self, Color, DrawMode, Mesh, MeshBuilder, MeshData, Vertex};
use ggez::{Context, GameResult};
use ggez::glam::*;

struct Line {
    start: Vec2,
    end: Vec2,
    color: Color,
}
struct Circle {
    center: Vec2,
    radius: f32,
    color: Color,
}

struct MainState {
    lines: Vec<Line>,
    circles: Vec<Circle>,
    pos_x: f32,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState {
            pos_x: 0.0,
            lines: vec![],
            circles: vec![],
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        let mut circle_count = 0;
        if self.circles.len() < 5 {
            let x = circle_count as f32;
            self.circles.push(Circle {
                center: Vec2::new(x * 50.0, 200.0),
                radius: 100.0,
                color: Color::WHITE,
            });
            circle_count += 1;
        }
        if self.lines.len() < 5 {
            let x = self.lines.len() as f32;
            self.lines.push(Line {
                start: Vec2::new(x * 50.0, 100.0),
                end: Vec2::new(x * 50.0, 300.0),
                color: Color::WHITE,
            });
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from([0.1, 0.2, 0.3, 1.0]),
        );

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            100.0,
            2.0,
            Color::WHITE,
        )?;

        let vertices = vec![
            Vertex{
                position: [0.0,0.0],
                color: [1.0, 1.0, 1.0, 1.0],
                uv: [0.0, 0.0],
            },
            Vertex{
                position: [100.0,100.0],
                color: [1.0, 1.0, 1.0, 1.0],
                uv: [0.0, 0.0],
            },
            Vertex{
                position: [200.0,0.0],
                color: [1.0, 1.0, 1.0, 1.0],
                uv: [0.0, 0.0],
            },
        ];

        let indices = vec![0u32, 1, 2];

        let mesh_data = MeshData{
            indices: &indices,
            vertices: &vertices,
        };

        let mesh = Mesh::from_data(ctx, mesh_data);

        canvas.draw(&circle, Vec2::new(self.pos_x, 380.0));

        canvas.draw(&mesh, Vec2::new(0.0, 0.0));


        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}