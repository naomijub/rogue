extern crate tcod;

use self::tcod::console::*;
use self::tcod::colors::{Color};

pub struct GameElement {
    x: i32,
    y: i32,
    symbol: char,
    color: Color,
}

impl GameElement {
    pub fn new(x: i32, y: i32, symbol: char, color: Color) -> GameElement {
        GameElement {
            x: x,
            y: y,
            symbol: symbol,
            color: color,
        }
    }

    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    pub fn draw(&self, root: &mut Root) {
        root.set_default_foreground(self.color);
        root.put_char(self.x, self.y, self.symbol, BackgroundFlag::None);
    }
}
