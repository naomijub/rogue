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
        let state = handle_move(&mut root, &mut player_x, &mut player_y);
        if state {
            break;
        }
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

fn handle_move(root: &mut Root, player_x: &mut i32, player_y: &mut i32) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = root.wait_for_keypress(true);
    match key {
        // movement keys
        Key { code: Up, .. } => { *player_y -= 1; root.clear();},
        Key { code: Down, .. } => { *player_y += 1; root.clear();},
        Key { code: Left, .. } => { *player_x -= 1; root.clear();},
        Key { code: Right, .. } => { *player_x += 1; root.clear();},

        Key { code: Escape, .. } => return true,
        _ => {},
    }
    false
}
