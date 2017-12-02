extern crate tcod;

use self::tcod::console::*;
use self::tcod::colors;

const MAP_X: i32 = 35;
const MAP_Y: i32 = 20;

pub fn make_map() -> Vec<Vec<Option<char>>> {
    vec![
        vec![Some('X'), None, Some('X'), Some('X'), Some('X'), Some('X'), Some('X'), Some('X'), Some('X'), Some('X')],
        vec![Some('X'), None, None, None, None, None, Some('X'), None, None, Some('X')],
        vec![Some('X'), None, Some('X'), Some('X'), Some('X'), None, Some('X'), None, Some('X'), Some('X')],
        vec![Some('X'), None, None, None, Some('X'), None, None, None, None, Some('X')],
        vec![Some('X'), Some('X'), Some('X'), None, Some('X'), Some('X'), Some('X'), Some('X'), None, Some('X')],
        vec![Some('X'), None, Some('X'), None, Some('X'), None, None, None, None, Some('X')],
        vec![Some('X'), None, Some('X'), None, Some('X'), Some('X'), Some('X'), Some('X'), Some('X'), Some('X')],
        vec![Some('X'), None, Some('X'), None, None, None, None, None, Some('X'), Some('X')],
        vec![Some('X'), None, Some('X'), Some('X'), Some('X'), Some('X'), Some('X'), None, Some('X'), Some('X')],
        vec![Some('X'), None, None, None, None, None, Some('X'), None, None, None],
        vec![Some('X'), Some('X'), Some('X'), Some('X'), Some('X'), Some('X'), Some('X'), Some('X'), Some('X'), Some('X')],
    ]
}

pub fn draw(map: &Vec<Vec<Option<char>>>, root: &mut Root) {
    root.set_default_foreground(colors::RED);
    let mut count_y = 0;
    for v_y in map {
        let mut count_x = 0;
        for v_x in v_y {
            match *v_x {
                Some(_) => root.put_char(count_x + MAP_X, count_y + MAP_Y, v_x.unwrap(), BackgroundFlag::None),
                None => {},
            }

            count_x += 1;
            println!("x: {:?}, y: {:?}", count_x, count_y );
        }
        count_y += 1;
    }
}
