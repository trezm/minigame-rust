use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait Drawable {
  fn draw(&self, Canvas<Window>) -> Canvas<Window>;
}
