/*

	This programme aims to:
	1. Have something drawn in real time, continuously refreshing in the terminal.
	2. Accept and process input from the user! :D

*/

// So-called "Includes" or "Imports"
use winconsole;
use std::{thread, time};
use std::sync::{Mutex, Arc};

// Width and Height of the square in which we'll draw
const WIDTH: i32 = 42;
const HEIGHT: i32 = 10;

// This will contain the player's data
struct PlayerData {
	x: f32,
	y: f32,
	character: char
}

// This will contain the data that will be used in the loop
struct GameData {
	running: bool,
	pressed: char,
	player: PlayerData,
	falling_object: PlayerData
}

// render function
fn render(data: &mut GameData, rendercharacter: &dyn Fn(&mut GameData, i32, i32)) {
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
			pressed: '\0',
			player: PlayerData {
				x: 5.0,
				y: 2.0,
				character: '#'
			},
			falling_object: PlayerData {
				x: 5.0,
				y: -4.0,
				character: 'H'
			}
		}
	));

	let gamedata_copy = Arc::clone(&gamedata);
	
	// Clear the screen
	print!("{esc}[2J{esc}[1;1H", esc = 27 as char); 
	// Game rules!
	println!("## Press Enter/Return to exit the game! ##");

	// The main loop
	let t1 = thread::spawn(move || {
		loop {
			/* mutex lock scope */ {
				// We get the reference to the game data.
				let mut dataref = gamedata.lock().unwrap(); 
				
				// If we broke out of the loop on the other thread - we do so on this one.
				if !dataref.running {
					break;
				}
				
				// we define a loop pass - it will play those functions one by one :D
				looppass(&mut dataref,
					&|data: &mut GameData| {
						// Player movement
						if data.pressed == 'd' {
							data.player.x += 1.0;
						}
						else if data.pressed == 'a' {
							data.player.x -= 1.0;
						}
						else if data.pressed == 's' {
							data.player.y += 1.0;
						}
						else if data.pressed == 'w' {
							data.player.y -= 1.0;
						}

						// Falling object fall
						data.falling_object.y += 0.1;
						// Falling object crawling back to the top
						if data.falling_object.y as i32 >= HEIGHT {
							data.falling_object.y = -1.0;
							data.falling_object.x = (rand::random::<i32>() % (WIDTH-2) + 1) as f32;
						}

					},
					&|data: &mut GameData, y: i32, x: i32| {
						// Border around the "screen"
						if x == 0 || x == WIDTH-1 || y == 0 || y == HEIGHT-1 {
							print!("=");
						}
						// Rendering the player
						else if data.player.x  as i32 == x && data.player.y as i32 == y {
							print!("{}", data.player.character);
						}
						// Rendering the falling object
						else if data.falling_object.x as i32 == x && data.falling_object.y as i32 == y {
							print!("{}", data.falling_object.character);
						}
						// The blank space
						else {
							print!(" ");
						}
					},
					&|data: &mut GameData| {
						
						data.pressed = '\0';
					}
				);
			}
			// Makes this thread sleep without a locked mutex, so the other one can catch up and do it's thing (only if it needs to) :D
			thread::sleep(time::Duration::from_millis(10));
		}
	});

	// The input loop
	let t2 = thread::spawn(move || {
		loop {
			// This loop is frozen up until we click anything, 
			// that's why it's on a seperate thread from the rendering! 
			let character = winconsole::console::getch(true).unwrap();
			/* mutex lock scope */ {
				// We get game data's reference
				let mut dataref = gamedata_copy.lock().unwrap();
				// We set the current pressed character to, well, the one that is pressed
				dataref.pressed = character;
				// Not necessary but I'd like to know what's pressed ^^
				println!("Getch ret: '{}' !!!", dataref.pressed);
				// If the currently pressed button is Enter, we break out of the loop (on both threads)
				if dataref.pressed == '\r' {
					dataref.running = false;
					break;
				}
			}
			thread::sleep(time::Duration::from_millis(10));
		}
	});

	t1.join().unwrap();
	t2.join().unwrap();
}