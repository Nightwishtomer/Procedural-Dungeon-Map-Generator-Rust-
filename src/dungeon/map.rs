// dungeon/map.rs

use crate::tile_type::TileType;
use crate::dungeon::room::Room;
pub const HEIGHT : usize = 160;//80
pub const WIDTH  : usize = 160;//80
pub type Map = [[TileType; WIDTH]; HEIGHT];

#[inline]
pub fn set_tile(map: &mut Map, y: usize, x: usize, t: TileType) {
    debug_assert!(x < WIDTH && y < HEIGHT);
    map[y][x] = t;
}

/// Проверяем, влезает ли прямоугольник в карту и не пересекает ли стены/комнаты
pub fn can_place(room: &Room, map: &Map) -> bool {
  if room.x2 >= WIDTH || room.y2 >= HEIGHT { return false; }
  for y in room.y1..=room.y2 {
    for x in room.x1..=room.x2 {
      let cell = map[y][x];
      let perimeter = x == room.x1 || x == room.x2 || y == room.y1 || y == room.y2;
      if perimeter {
        if cell == TileType::FLOOR {
          return false;
        }
      } else {
        if cell != TileType::IMPASS {
          return false;
        }
      }
    }
  }
  true
}

/// Заполняем прямоугольник полом
pub fn carve(room: &Room, map: &mut Map) {
  for y in room.y1..=room.y2 {
      for x in room.x1..=room.x2 {
          let perimeter = x == room.x1 || x == room.x2 || y == room.y1 || y == room.y2;

          if perimeter {
              // Ставим стену, но не затираем уже существующую
              if map[y][x] == TileType::IMPASS || map[y][x] == TileType::DOOR {
                  set_tile(map, y, x, TileType::WALL); // Устанавливаем стену
              }
          } else {
              set_tile(map, y, x, TileType::FLOOR); // Устанавливаем пол
          }
      }
  }
}