// This will contain the player's data
pub struct ObjectData {
    pub x: f32,
    pub y: f32,
    pub character: char,
}

// This will contain the data that will be used in the loop
pub struct GameData {
    pub running: bool,
    pub pressed: char,
    pub player: ObjectData,
    pub falling_object: ObjectData,
}

// The impl block contains functions associated with the GameData struct
impl GameData {
    // The new function defines what happens when a struct is initiated (GameData::new())
    pub fn new() -> Self {
        Self {
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
        }
    }
}

// Drop specifies what happens when the object is dropped (destructed, out of it's scope)
impl Drop for GameData {
    fn drop(&mut self) {
        println!("GameData Object dropped!");
    }
}
