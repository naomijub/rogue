extern crate tcod;

mod gameelements;

use tcod::console::*;
use tcod::colors;
use gameelements::GameElement;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 60;
const LIMIT_FPS: i32 = 25;

fn main() {
    tcod::system::set_fps(LIMIT_FPS);

    let mut root = root();
    let mut offscreen = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT / 3);
    let mut player = GameElement::new(10, 14, 'P', colors::WHITE);

    while !root.window_closed() {
        root.set_default_foreground(colors::RED);
        root.horizontal_line(10, 15, 60, BackgroundFlag::None);
        player.draw(&mut root);

        offscreen.set_default_foreground(colors::BLUE);
        offscreen.put_char(10, 2, 'H', BackgroundFlag::None);
        offscreen.put_char(11, 2, 'P', BackgroundFlag::None);
        offscreen.put_char(12, 2, ':', BackgroundFlag::None);

        blit(&mut offscreen, (0, 0), (SCREEN_WIDTH, SCREEN_HEIGHT / 3), &mut root, (0, 50), 1.0, 0.6);
        root.flush();
        let state = handle_move(&mut root, &mut player);
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

fn handle_move(root: &mut Root, player: &mut GameElement) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = root.wait_for_keypress(true);
    match key {
        // movement keys
        Key { code: Up, .. } => { player.move_by(0, -1); root.clear();},
        Key { code: Down, .. } => { player.move_by(0, 1); root.clear();},
        Key { code: Left, .. } => { player.move_by(-1, 0); root.clear();},
        Key { code: Right, .. } => { player.move_by(1, 0); root.clear();},

        Key { code: Escape, .. } => return true,
        _ => {},
    }
    false
}
