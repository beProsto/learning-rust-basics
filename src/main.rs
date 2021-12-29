/*

	This programme aims to:
	1. Have something drawn in real time, continuously refreshing in the terminal.
	2. Accept and process input from the user! :D

*/

use winconsole;
use std::{thread, time};
use std::sync::{Mutex, Arc};

// Width and Height of the square in which we'll draw
const WIDTH: i32 = 8;
const HEIGHT: i32 = 5;

// This will contain the data that will be used in the loop
struct GameData {
	running: bool,
	character: char
}

// render function
fn render(data: &mut GameData, rendercharacter: &dyn Fn(&mut GameData, i32, i32)) {
	// first clear the screen
	print!("{esc}[2J{esc}[1;1H", esc = 27 as char); 
	// then draw the screen
	for y in 0..HEIGHT {
		for x in 0..WIDTH {
			// for every "pixel" invoke a callback
			rendercharacter(data, y, x);
		}
		println!("");
	}
}

// The loop - it will be done every single frame!
fn looppass(data: &mut GameData, atloopstart: &dyn Fn(&mut GameData), rendercharacter: &dyn Fn(&mut GameData, i32, i32), atloopend: &dyn Fn(&mut GameData)) {
	atloopstart(data);
	render(data, rendercharacter);
	atloopend(data);
}

fn main() {
	// We define the data
	let gamedata = Arc::new(Mutex::new(
		GameData {
			running: true,
			character: '\0'
		}
	));

	let gamedata_copy = Arc::clone(&gamedata);
	
	// The main loop
	let t1 = thread::spawn(move || {
		loop {
			/* mutex lock scope */ {
				let mut dataref = gamedata.lock().unwrap(); 
				
				if !dataref.running {
					break
				}
				
				looppass(&mut dataref,
					&|_data: &mut GameData| {
						
					},
					&|_data: &mut GameData, _y: i32, _x: i32| {
						print!("X");
					},
					&|_data: &mut GameData| {

					}
				);
			}
			thread::sleep(time::Duration::from_millis(10));
		}
	});

	// The input loop
	let t2 = thread::spawn(move || {
		loop {
			let character = winconsole::console::getch(true).unwrap();
			/* mutex lock scope */ {
				let mut dataref = gamedata_copy.lock().unwrap();
				dataref.character = character;
				println!("Getch ret: '{}' !!!", dataref.character);
				if dataref.character == '\r' {
					dataref.running = false;
					break;
				}
			}
		}
	});

	t1.join().unwrap();
	t2.join().unwrap();

}