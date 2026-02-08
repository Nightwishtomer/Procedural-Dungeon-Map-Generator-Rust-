// main.rs

mod tile_type;
mod dungeon;

use dungeon::Dungeon;
use serde_json;
use std::env;
ф
fn main() {
    let args: Vec<String> = env::args().collect();
    let is_test_mode = args.len() > 1 && args[1] == "--test";
    generate_dungeon(is_test_mode);
}

fn generate_dungeon(test_mode: bool) {
    loop {
        let mut dungeon = Dungeon::new();
        dungeon.place_central_rooms();
        dungeon.connect_rooms_wide_corridor();
        dungeon.grow_children();
        dungeon.place_doors_from_rooms();
        if !dungeon.is_fully_connected() { continue; }
        if !dungeon.place_entrance_exit() { continue; }
        if test_mode {
            dungeon.print_map();
        } else {
            let json = serde_json::to_string(&dungeon.as_json()).unwrap();
            println!("{json}");
        }
        break;
    }
}