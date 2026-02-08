// dungeon/json.rs

use serde::Serialize;
use super::room::Room;

#[derive(Serialize)]
pub struct DungeonJson {
    pub width:  usize,
    pub height: usize,
    pub grid:   Vec<String>,
    pub rooms:  Vec<Room>,   // ← новое поле
}