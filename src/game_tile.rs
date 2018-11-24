use clickable::Clickable;
use drawable::Drawable;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::pixels::Color;

use text;

pub struct GameTile {
  x: i32,
  y: i32,
  width: u32,
  height: u32,
  content: String
}

impl GameTile {
  pub fn new(x: i32, y: i32, width: u32, height: u32, content: &str) -> GameTile {
    GameTile {
      x: x,
      y: y,
      width: width,
      height: height,
      content: content.to_owned()
    }
  }
}

impl Clickable for GameTile {
  fn respond(&self, x: i32, y: i32) {
    if x > self.x && x < self.x + self.width as i32 &&
      y > self.y && y < self.y + self.height as i32 {
        println!("{} was clicked", self.content);
    }
  }
}

impl Drawable for GameTile {
  fn draw(&self, canvas: Canvas<Window>) -> Canvas<Window> {
    let mut canvas = text::line(canvas, &self.content, self.x + (self.width as i32 / 2) - 15, self.y + (self.height as i32 / 2) - 25);
    canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));
    let _ = canvas.draw_rect(Rect::new(self.x, self.y, self.width, self.height));

    canvas
  }
}
