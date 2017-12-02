extern crate tcod;

use self::tcod::console::*;
use self::tcod::colors::{Color};

#[derive(Debug,PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use super::tcod::colors;

    #[test]
    fn game_element_has_correct_attr() {
        let actual = GameElement::new(10, 11, 'x', colors::RED);
        let expected = GameElement {
            x: 10,
            y: 11,
            symbol: 'x',
            color: colors::RED,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn element_has_moved_by_step() {
        let step = 5;
        let mut element = GameElement::new(10, 11, 'x', colors::RED);
        element.move_by(step, step);

        assert!(element.x == 15 && element.y == 16);
    }
}
