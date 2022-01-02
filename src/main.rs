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
use std::sync::mpsc; // shortens std::sync::mpsc to mpsc
use std::{thread, time}; // shortens std::thread and time to just thread and time // shortens std::mem to mem

use game_data::GameData;
use render::{render, HEIGHT, WIDTH};

fn main() {
    // We define the data
    let mut gamedata = GameData::new();

    let (input_tx, input_rx) = mpsc::channel();

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
            // We send the gotten character to the main thread
            input_tx.send(character).unwrap();
            // If the currently pressed button is Enter, we break out of the loop
            if character == '\r' {
                break;
            }
        }
    });

    // The main loop
    loop {
        // Recieve the character (if there is any to recieve)
        let recieved = input_rx.try_recv();
        if !recieved.is_err() {
            gamedata.pressed = recieved.unwrap();
        }

        // If we broke out of the loop on the other thread - we do so on this one.
        if gamedata.pressed == '\r' {
            break;
        }

        // Player movement
        if gamedata.pressed == 'd' {
            gamedata.player.x += 1.0;
        } else if gamedata.pressed == 'a' {
            gamedata.player.x -= 1.0;
        } else if gamedata.pressed == 's' {
            gamedata.player.y += 1.0;
        } else if gamedata.pressed == 'w' {
            gamedata.player.y -= 1.0;
        }

        // Falling object fall
        gamedata.falling_object.y += 0.1;
        // Falling object crawling back to the top
        if gamedata.falling_object.y as i32 >= HEIGHT {
            gamedata.falling_object.y = -1.0;
            gamedata.falling_object.x = (rand::<i32>() % (WIDTH - 2) + 1) as f32;
        }

        // We render to the area
        render(
            &mut gamedata,
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
                else if data.falling_object.x as i32 == x && data.falling_object.y as i32 == y {
                    print!("{}", data.falling_object.character);
                }
                // The blank space
                else {
                    print!(" ");
                }
            },
        );

        // Not necessary but I'd like to know what's pressed ^^
        println!("Getch ret: '{}' !!!", gamedata.pressed);

        // We reset the character pressed, in order not to preserve it when it's not clicked
        gamedata.pressed = '\0';

        // Makes this thread sleep without a locked mutex, so the other one can catch up and do it's thing (only if it needs to) :D
        thread::sleep(time::Duration::from_millis(10));
    }

    input_thread.join().unwrap();
}
