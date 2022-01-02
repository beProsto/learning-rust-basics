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
