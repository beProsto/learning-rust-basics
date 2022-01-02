// // c++'s "include"
// mod game_data;

// Apparently mod isn't needed in this case, furthermore - it's not actually possible to use it

// c++'s "using"
use crate::game_data::GameData;

// Width and Height of the square in which we'll draw
pub const WIDTH: i32 = 42;
pub const HEIGHT: i32 = 10;

// render function
pub fn render(data: &mut GameData, rendercharacter: &dyn Fn(&mut GameData, i32, i32)) {
    // first clear the screen
    print!("{esc}[3J{esc}[2;1H", esc = 27 as char);
    // then draw the screen
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            // for every "pixel" invoke a callback
            rendercharacter(data, y, x);
        }
        println!("");
    }
}
