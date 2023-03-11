// draw simple moving cube


#[cfg(test)]
mod tests {
    use super::*;
    use ggez::event;
    use ggez::graphics;
    //use nalgebra;
    use ggez::{Context, GameResult};
    use ggez::conf::{WindowMode, WindowSetup};
    use ggez::graphics::{DrawMode, DrawParam, Mesh, Rect};
    use ggez::input::keyboard;
    use ggez::timer;

    #[test]
    fn it_works() {
        // draw and display a simple moving cube
        let (mut ctx, mut event_loop) = ggez::ContextBuilder::new("test", "test")
            .window_setup(WindowSetup::default().title("test"))
            .window_mode(WindowMode::default().dimensions(800.0, 600.0))
            .build()
            .expect("aieee, could not create ggez context!");
    }
}