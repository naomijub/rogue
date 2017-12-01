extern crate tcod;

use tcod::console::*;
use tcod::colors;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 60;
const LIMIT_FPS: i32 = 25;

fn main() {
    tcod::system::set_fps(LIMIT_FPS);

    let mut root = root();
    let mut player_x = 10;
    let mut player_y = 14;

    while !root.window_closed() {
        root.set_default_foreground(colors::RED);
        root.horizontal_line(10, 15, 60, BackgroundFlag::None);
        root.set_default_foreground(colors::WHITE);
        root.put_char(player_x, player_y, 'P', BackgroundFlag::None);
        root.flush();
        root.wait_for_keypress(true);
    }
}

fn root() -> Root {
    Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rogue - Naomijubs")
        .init()
}
