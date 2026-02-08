//dungeon/rooms.rs

use serde::Serialize;

// struct Room
#[derive(Debug, Clone, Serialize)]
pub struct Room { pub x1: usize, pub y1: usize, pub x2: usize, pub y2: usize, }
// implement Room
impl Room {
  pub fn center(&self) -> (usize, usize) { ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2) }
  pub fn center_x(&self) -> usize { (self.x1 + self.x2) / 2 }
  pub fn center_y(&self) -> usize { (self.y1 + self.y2) / 2 }
  
}



    // Пересекаються ли две комнаты (с небольшим буффером в 1 Тайл)
    pub fn rooms_overlap(a: &Room, b: &Room) -> bool {
      !(a.x2 + 1 < b.x1 || 
        a.x1 > b.x2 + 1 || 
        a.y2 + 1 < b.y1 || 
        a.y1 > b.y2 + 1)  
    }

