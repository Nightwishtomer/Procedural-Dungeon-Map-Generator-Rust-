//dungeon/generator.rs
use super:: room::{Room, rooms_overlap};
use super::json::DungeonJson;
use crate::dungeon::map::{Map, WIDTH, HEIGHT, can_place, carve, set_tile};
use crate::tile_type::TileType;
use rand::{rng, Rng};
use rand::prelude::IndexedRandom;
use std::collections::HashSet;

const MAX_ROOMS: usize = 100;
const SIZES: [usize; 4] = [5, 7, 9, 11]; // Пример размеров комнат

// struct Dungeon
pub struct Dungeon { map: Map, pub rooms: Vec<Room>, } 
// implement Dungeon




impl Dungeon {
  // Конструктор для создания новой карты
  pub fn new() -> Self {
    Self {
      map: [[TileType::IMPASS; WIDTH]; HEIGHT],
      rooms: Vec::new(),
    }
  }




  pub fn as_json(&self) -> DungeonJson {
    let grid = self.map
        .iter()
        .map(|row| row.iter()
                      .map(|t| t.ascii_icon())   // char
                      .collect::<String>())      // ⇒ "##..#"
        .collect();
        

    DungeonJson { 
      width: WIDTH, 
      height: HEIGHT, 
      grid,
      rooms:  self.rooms.clone(),   // ← сериализуем «как есть»
    }
}

  // Generate 1-3 main random rooms
  pub fn place_central_rooms(&mut self){
    let mut rng = rng();
    let room_count = 3;
    //let room_count = rng.random_range(1..=3);
    //eprintln!("{}", room_count);
    while self.rooms.len() < room_count {
      let size = 20;  //Размеры комнат 8, 10, 12
      let half = size / 2;
      let base_x = WIDTH / 2; //базовый центр комнаты X
      let base_y = HEIGHT / 2; //базовый центр комнаты Y
      let dx: i32 = rng.random_range(-25..=25); //Случайный сдвиг +/- 6 Тайлов по X
      let dy: i32 = rng.random_range(-25..=25); //Случайный сдвиг +/- 6 Тайлов по Y
      let mut cx = (base_x as i32 + dx) as usize;
      let mut cy = (base_y as i32 + dy) as usize;
      if (room_count > 1) && (self.rooms.len() == 1) {
        let last_room = self.rooms.last().unwrap();
        //let (last_x, last_y) = last_room.center();
        let last_y = last_room.center_y();
        let last_x = last_room.center_x();
        if rand::random() {
          cx = last_x + 1;
        } else {
          cy = last_y + 1;
        }
      }
      //Установка размеров комнаты и Проверка границ карты
      let room = Room {
        x1: (cx - half).clamp(0, WIDTH - 1) as usize,
        y1: (cy - half).clamp(0, HEIGHT - 1) as usize,
        x2: (cx + half-1).clamp(0, WIDTH - 1) as usize,
        y2: (cy + half-1).clamp(0, HEIGHT - 1) as usize,
      };
      //Проверка пересечения с уже существующими комнатами
      if self.rooms.iter().any(|r| rooms_overlap(r, &room)) {
          continue; //Если пересекаются, пробуем снова
      }
      //Заполнение карты комнатой (рисуем)
      for y in room.y1..=room.y2 {
        for x in room.x1..=room.x2 {
          if y == room.y1 || y == room.y2 || x == room.x1 || x == room.x2 {
            set_tile(&mut self.map, y, x, TileType::WALL); //Устанавливаем стену
          } else {
            set_tile(&mut self.map, y, x, TileType::FLOOR); //Устанавливаем пол
          }
        }
      }
      self.rooms.push(room); //Добавляем комнату в список комнат
    }
  }

  // Печать карты в консоль
  pub fn print_map(&self){    
    for _row in &self.map { 
      for tile in _row {
        print!("{}", tile.ascii_icon());
      }
      println!();
    }
  }

  // Соединяем первые 2 комнаты (6 тайлов) коридором, которые на одной оси
  pub fn connect_rooms_wide_corridor(&mut self){
    if self.rooms.len() < 2 { return; } //Нечего соединять
    let mut room1 = &self.rooms[0];
    let mut room2 = &self.rooms[1];
    let (x1, y1) = room1.center();
    let (x2, y2) = room2.center();
    let corridor_width = 12; //Ширина коридора
    // Проверяем, что комнаты на одной оси  и на какой
    if y1 == y2 {
      if self.rooms[0].center().0 < self.rooms[1].center().0 {
        room1 = &self.rooms[1];
        room2 = &self.rooms[0];
      }
      let (x1, y1) = room1.center();
      let (x2, _) = room2.center();
      // Если комнаты на одной горизонтали, рисуем горизонтальный коридор
      let y_start = y1.saturating_sub(corridor_width / 2) + 1;
      let y_end = (y1 + corridor_width / 2).min(HEIGHT - 1);
     
      for y in y_start..=y_end {
        for x in x1.min(x2)..=x1.max(x2) {
          set_tile(&mut self.map, y, x, TileType::FLOOR); // Устанавливаем пол
          if (y  == y_start || y == y_end) && (!((room1.x1 + 1)..room1.x2).contains(&x) && !((room2.x1 - 1)..room2.x2).contains(&x)) { //  
            set_tile(&mut self.map, y, x, TileType::WALL); // Устанавливаем стену
          }
        }
      }
    } else if x1 == x2{ 
      if self.rooms[0].center().1 < self.rooms[1].center().1 {
        room1 = &self.rooms[1];
        room2 = &self.rooms[0];
      }
      let (x1, y1) = room1.center();
      let (_, y2) = room2.center();

      // Если комнаты на одной вертикали, рисуем вертикальный коридор
      let x_start = x1.saturating_sub(corridor_width / 2) + 1;
      let x_end = (x1 + corridor_width / 2).min(WIDTH - 1);
      
      for x in x_start..=x_end {
        for y in y1.min(y2)..=y1.max(y2) {
          set_tile(&mut self.map, y, x, TileType::FLOOR); // Устанавливаем пол
          if (x  == x_start || x == x_end) && (!((room1.y1 + 1)..room1.y2).contains(&y) && !((room2.y1 - 1)..room2.y2).contains(&y)) { //  
            set_tile(&mut self.map, y, x, TileType::WALL); // Устанавливаем стену
          }
        }
      }
    }
  }















/// ↓ ↓ ГЛАВНЫЙ НОВЫЙ МЕТОД ― «приращивание» дочерних комнат
pub fn grow_children(&mut self) {
  let mut rng = rng();
  let mut frontier = self.rooms.clone();
  while !frontier.is_empty() && self.rooms.len() < MAX_ROOMS {
      let parent = frontier.pop().unwrap();
      // 1–3 дочерних комнат от одного родителя
      for _ in 0..rng.random_range(1..=16) {
          let size_w = *SIZES.choose(&mut rng).unwrap();
          let size_h = *SIZES.choose(&mut rng).unwrap();
          match rng.random_range(0..4) { // направление N,S,W,E
              // ─── вверх ───
              0 => {
                  if parent.y1 <= size_h { continue; }
                  let x1_min = parent.x1.saturating_add(4).saturating_sub(size_w);
                  let x1_max = parent.x2.saturating_sub(3).min(WIDTH - 1);
                  let x1 = rng.random_range(x1_min..=x1_max);
                  let room = Room { 
                      x1,
                      y1: parent.y1 - size_h ,
                      x2: x1 + size_w,
                      y2: parent.y1 ,
                  };
                  if !can_place(&room, &self.map) { continue; }
                  carve(&room, &mut self.map);
                  self.rooms.push(room.clone());
                  frontier.push(room);
              }
              // ─── вниз ───
              1 => {
                  if parent.y2 + size_h >= HEIGHT { continue; }
                  let x1_min = parent.x1.saturating_add(4).saturating_sub(size_w);
                  let x1_max = parent.x2.saturating_sub(3).min(WIDTH - 1);
                  let x1 = rng.random_range(x1_min..=x1_max);
                  let room = Room { 
                      x1,
                      y1: parent.y2 ,
                      x2: x1 +size_w,
                      y2: parent.y2 + size_w,
                  };
                  if !can_place(&room, &self.map) { continue; }
                  carve(&room, &mut self.map);
                  self.rooms.push(room.clone());
                  frontier.push(room);
              }
              // ─── влево ───
              2 => {
                  if parent.x1 <= size_w { continue; }
                  let y1_min = parent.y1.saturating_add(4).saturating_sub(size_w);
                  let y1_max = parent.y2.saturating_sub(3).min(WIDTH - 1);
                  let y1 = rng.random_range(y1_min..=y1_max);
                  let room = Room { 
                      x1: parent.x1 - size_w,
                      y1,
                      x2: parent.x1 ,
                      y2: y1 + size_h,
                  };
                  if !can_place(&room, &self.map) { continue; }
                  carve(&room, &mut self.map);
                  self.rooms.push(room.clone());
                  frontier.push(room);
              }
              // ─── вправо ───
              _ => {
                  if parent.x2 + size_w >= WIDTH { continue; }
                  let y1_min = parent.y1.saturating_add(4).saturating_sub(size_w);
                  let y1_max = parent.y2.saturating_sub(3).min(WIDTH - 1);
                  let y1 = rng.random_range(y1_min..=y1_max);
                  let room = Room { 
                      x1: parent.x2 ,
                      y1,
                      x2: parent.x2 + size_w ,
                      y2: y1 + size_h,
                  };
                  if !can_place(&room, &self.map) { continue; }
                  carve(&room, &mut self.map);
                  self.rooms.push(room.clone());
                  frontier.push(room);
              }
          }
      }
  }

}




pub fn place_doors_from_rooms(&mut self) {
  let mut rng = rng();
  let mut used = HashSet::<(usize, usize)>::new(); // чтобы не дублировать дверь

  for i in 0..self.rooms.len() {
      for j in (i + 1)..self.rooms.len() {
          let a = &self.rooms[i];
          let b = &self.rooms[j];

          // ─── B справа ───
          if a.x2 == b.x1 {
              let y0 = a.y1.max(b.y1)+1;
              let y1 = a.y2.min(b.y2)-1;
              if y0 <= y1 {
                  let y = rng.random_range(y0..=y1);
                  let p = (y, a.x2); // (row, col)
                  if used.insert(p) {
                      set_tile(&mut self.map, y, a.x2, TileType::DOOR); // Устанавливаем дверь
                  }
              }
          }
          // ─── B слева ───
          else if b.x2 == a.x1 {
              let y0 = a.y1.max(b.y1)+1;
              let y1 = a.y2.min(b.y2)-1;
              if y0 <= y1 {
                  let y = rng.random_range(y0..=y1);
                  let p = (y, a.x1);
                  if used.insert(p) {
                      set_tile(&mut self.map, y, a.x1, TileType::DOOR); // Устанавливаем дверь
                  }
              }
          }
          // ─── B снизу ───
          else if a.y2 == b.y1 {
              let x0 = a.x1.max(b.x1)+1;
              let x1 = a.x2.min(b.x2)-1;
              if x0 <= x1 {
                  let x = rng.random_range(x0..=x1);
                  let p = (a.y2, x);
                  if used.insert(p) {
                      set_tile(&mut self.map, a.y2, x, TileType::DOOR); // Устанавливаем дверь
                  }
              }
          }
          // ─── B сверху ───
          else if b.y2 == a.y1 {
              let x0 = a.x1.max(b.x1)+1;
              let x1 = a.x2.min(b.x2)-1;
              if x0 <= x1 {
                  let x = rng.random_range(x0..=x1);
                  let p = (a.y1, x);
                  if used.insert(p) {
                      set_tile(&mut self.map, a.y1, x, TileType::DOOR); // Устанавливаем дверь
                  }
              }
          }
      }
  }
  
}

/// Возвращает true, если все FLOOR‑клетки достижимы
pub fn is_fully_connected(&mut self) -> bool {
  use std::collections::VecDeque;

  // найдём первую клетку FLOOR
  let mut queue = VecDeque::new();
  let mut seen  = vec![vec![false; WIDTH]; HEIGHT];

  'outer: for y in 0..HEIGHT {
      for x in 0..WIDTH {
          if self.map[y][x] == TileType::FLOOR {
              queue.push_back((y, x));
              seen[y][x] = true;
              break 'outer;
          }
      }
  }

  // если пола нет вообще – трактуем как «связно»
  if queue.is_empty() { return true; }

  // BFS 4‑соседей
  while let Some((y, x)) = queue.pop_front() {
      for (ny, nx) in [
          (y.wrapping_sub(1), x),
          (y + 1, x),
          (y, x.wrapping_sub(1)),
          (y, x + 1),
      ] {
          if ny < HEIGHT && nx < WIDTH
              && !seen[ny][nx]
              && (self.map[ny][nx] == TileType::FLOOR || self.map[ny][nx] == TileType::DOOR)
          {
              seen[ny][nx] = true;
              queue.push_back((ny, nx));
          }
      }
  }

  // есть ли хоть один FLOOR не посещённый?
  for y in 0..HEIGHT {
      for x in 0..WIDTH {
          if self.map[y][x] == TileType::FLOOR && !seen[y][x] {
              return false;
          }
      }
  }
  true
}








pub fn place_entrance_exit(&mut self) -> bool {
use std::collections::VecDeque;

// 1. Ищем горизонтальную стену
let (ey, ex) = match self.find_horizontal_wall_entrance() {
    Some(pos) => pos,
    None => {
        //eprintln!("⚠️ Не найдено подходящего места для входа – fallback в центр комнаты");
        //self.rooms[0].center()
        return false;
    }
};

set_tile(&mut self.map, ey, ex, TileType::ENTRANCE); // Устанавливаем вход
//eprintln!("Enterance: x:{}, y:{}", ex, ey);

// 2. BFS от входа до самой дальней клетки пола
let mut dist = vec![vec![usize::MAX; WIDTH]; HEIGHT];
let mut q = VecDeque::new();
dist[ey][ex] = 0;
q.push_back((ey, ex));

let mut far = (ey, ex);

while let Some((y, x)) = q.pop_front() {
    let d = dist[y][x];
    for (ny, nx) in [
        (y.wrapping_sub(1), x),
        (y + 1, x),
        (y, x.wrapping_sub(1)),
        (y, x + 1),
    ] {
        if ny < HEIGHT && nx < WIDTH &&
           dist[ny][nx] == usize::MAX &&
           (self.map[ny][nx] == TileType::FLOOR || self.map[ny][nx] == TileType::DOOR)
        {
            dist[ny][nx] = d + 1;
            if d + 1 > dist[far.0][far.1] {
                far = (ny, nx);
            }
            q.push_back((ny, nx));
        }
    }
}

let (xy, xx) = far;
set_tile(&mut self.map, xy, xx, TileType::EXIT); // Устанавливаем выход
//eprintln!("EXIT: x:{}, y:{}", xx, xy);
true
}




fn find_horizontal_wall_entrance(&self) -> Option<(usize, usize)> {
for y in 1..(HEIGHT - 1) {
    for x in 3..(WIDTH - 3) {
        // Проверяем, что в центре стена, под ней пол
        if self.map[y][x] == TileType::WALL && self.map[y + 1][x] == TileType::FLOOR {
            // Проверяем, что по бокам 3 тайла тоже стены
            let left_free = (1..=3).all(|i| self.map[y][x - i] == TileType::WALL);
            let right_free = (1..=3).all(|i| self.map[y][x + i] == TileType::WALL);

            if left_free && right_free {
                return Some((y + 1, x));
            }
        }
    }
}
None
}






















// Заполняем карту дверями, если есть возможность
} 