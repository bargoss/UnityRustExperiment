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

struct DrawerState {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    colors: Vec<Color>,
}

impl DrawerState {
    fn new() -> GameResult<DrawerState> {
        let s = DrawerState {
            vertices: vec![],
            indices: vec![],
            colors: vec![],
        };
        Ok(s)
    }

    fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
        self.colors.clear();
    }

    fn draw_triangle(&mut self, a: Vec2, b: Vec2, c: Vec2, color: Color) {
        // push the three vertices into the self.vertices, self.indices, self.colors
        let index = self.vertices.len() as u32;
        let vertex_a = Vertex{
            position: [a.x, a.y],
            uv: [0.0, 0.0],
            color: color.into(),
        };
        let vertex_b = Vertex{
            position: [b.x, b.y],
            uv: [0.0, 0.0],
            color: color.into(),
        };
        let vertex_c = Vertex{
            position: [c.x, c.y],
            uv: [0.0, 0.0],
            color: color.into(),
        };

        self.vertices.push(vertex_a);
        self.vertices.push(vertex_b);
        self.vertices.push(vertex_c);

        self.indices.push(index);
        self.indices.push(index + 1);
        self.indices.push(index + 2);

        self.colors.push(color);
        self.colors.push(color);
        self.colors.push(color);
    }

    fn draw_circle(&mut self, center: Vec2, radius: f32, color: Color) {
        let segments = 16;
        let angle = 2.0 * std::f32::consts::PI / segments as f32;
        let mut current_angle = 0.0 as f32;

        for i in 0..segments {
            let start = Vec2::new(center.x + radius * current_angle.cos(), center.y + radius * current_angle.sin());
            current_angle += angle;
            let end = Vec2::new(center.x + radius * current_angle.cos(), center.y + radius * current_angle.sin());
            self.draw_triangle(center, start, end, color);
        }
    }

    fn draw_line(&mut self, start: Vec2, end: Vec2, thickness: f32, color: Color) {
        let forward = (end - start).normalize();
        let right = forward.perp();


        let a = start + right * thickness;
        let b = start - right * thickness;
        let c = end + right * thickness;
        let d = end - right * thickness;

        self.draw_triangle(a, b, c, color);
        self.draw_triangle(c, b, d, color);
    }

    fn get_mesh(&self) -> MeshData {
        //to Vertex array

        let mesh_data = MeshData {
            vertices: &self.vertices,
            indices: &self.indices,
        };
        mesh_data
    }
}

impl event::EventHandler<ggez::GameError> for DrawerState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
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


        canvas.draw(&mesh, Vec2::new(0.0, 0.0));


        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;
    let state = DrawerState::new()?;
    event::run(ctx, event_loop, state)
}