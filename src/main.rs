/*

    This programme aims to:
    1. Have something drawn in real time, continuously refreshing in the terminal.
    2. Accept and process input from the user! :D

*/

// c++'s "include"
mod game_data;
mod render;

// c++'s "using"
use rand::random as rand; // shortens rand::random to rand
use std::sync::{Arc, Mutex}; // shortens std::sync::Arc and Mutex to just Arc and Mutex
use std::{thread, time}; // shortens std::thread and time to just thread and time

use game_data::GameData;
use game_data::ObjectData;
use render::render;
use render::HEIGHT;
use render::WIDTH;

fn main() {
    // We define the data
    let gamedata_arc = Arc::new(Mutex::new(GameData {
        running: true,
        pressed: '\0',
        player: ObjectData {
            x: 20.0,
            y: 5.0,
            character: '#',
        },
        falling_object: ObjectData {
            x: 5.0,
            y: -4.0,
            character: 'V',
        },
    }));

    let gamedata_arc_input = Arc::clone(&gamedata_arc);

    // Clear the screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    // Game rules!
    println!("## Press Enter/Return to exit the game! ##");

    // The input loop
    let input_thread = thread::spawn(move || {
        loop {
            // This loop is frozen up until we click anything,
            // that's why it's on a seperate thread from the rendering!
            let character = winconsole::console::getch(true).unwrap();
            /* mutex lock scope */
            {
                // We get game data's reference
                let mut gamedata_ref = gamedata_arc_input.lock().unwrap();
                // We set the current pressed character to, well, the one that is pressed
                gamedata_ref.pressed = character;
                // Not necessary but I'd like to know what's pressed ^^
                println!("Getch ret: '{}' !!!", gamedata_ref.pressed);
                // If the currently pressed button is Enter, we break out of the loop (on both threads)
                if gamedata_ref.pressed == '\r' {
                    gamedata_ref.running = false;
                    break;
                }
            }
            thread::sleep(time::Duration::from_millis(10));
        }
    });

    // The main loop
    loop {
        /* mutex lock scope */
        {
            // We get the reference to the game data.
            let mut gamedata_ref = gamedata_arc.lock().unwrap();

            // If we broke out of the loop on the other thread - we do so on this one.
            if !gamedata_ref.running {
                break;
            }

            // Player movement
            if gamedata_ref.pressed == 'd' {
                gamedata_ref.player.x += 1.0;
            } else if gamedata_ref.pressed == 'a' {
                gamedata_ref.player.x -= 1.0;
            } else if gamedata_ref.pressed == 's' {
                gamedata_ref.player.y += 1.0;
            } else if gamedata_ref.pressed == 'w' {
                gamedata_ref.player.y -= 1.0;
            }

            // Falling object fall
            gamedata_ref.falling_object.y += 0.1;
            // Falling object crawling back to the top
            if gamedata_ref.falling_object.y as i32 >= HEIGHT {
                gamedata_ref.falling_object.y = -1.0;
                gamedata_ref.falling_object.x = (rand::<i32>() % (WIDTH - 2) + 1) as f32;
            }

            // We render to the area
            render(
                &mut gamedata_ref,
                // This function happens per every character to be rendered
                &|data: &mut GameData, y: i32, x: i32| {
                    // Border around the "screen"
                    if x == 0 || x == WIDTH - 1 || y == 0 || y == HEIGHT - 1 {
                        print!("=");
                    }
                    // Rendering the player
                    else if data.player.x as i32 == x && data.player.y as i32 == y {
                        print!("{}", data.player.character);
                    }
                    // Rendering the falling object
                    else if data.falling_object.x as i32 == x && data.falling_object.y as i32 == y
                    {
                        print!("{}", data.falling_object.character);
                    }
                    // The blank space
                    else {
                        print!(" ");
                    }
                },
            );

            // We reset the character pressed, in order not to preserve it when it's not clicked
            gamedata_ref.pressed = '\0';
        }
        // Makes this thread sleep without a locked mutex, so the other one can catch up and do it's thing (only if it needs to) :D
        thread::sleep(time::Duration::from_millis(10));
    }

    input_thread.join().unwrap();
}
