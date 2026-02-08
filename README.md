# Procedural Dungeon Map Generator (Rust)

A procedural dungeon map generator written in **Rust**, inspired by classic roguelike and Diablo-style level generation.  
The project focuses on **room-based dungeon layouts**, deterministic connectivity, and clean data export for use in external engines or tools.

The generator produces a fully connected dungeon, validates entrances and exits, and can output either a human-readable ASCII map or structured JSON.

---

## Key Features

- Procedural room generation with size variation
- Central room placement followed by recursive expansion
- Wide corridor connections between rooms
- Door and entrance/exit placement
- Guaranteed full connectivity validation
- ASCII rendering for debugging and testing
- JSON export for integration with other engines or tools
- Clean modular architecture

---

## Project Structure

```
src/
├── main.rs
├── tile_type.rs
└── dungeon/
    ├── mod.rs
    ├── generator.rs
    ├── map.rs
    ├── room.rs
    └── json.rs
```

---

## Module Overview

### main.rs

Application entry point.

Responsibilities:
- CLI argument parsing
- Test / debug mode handling
- Dungeon generation loop with validation
- JSON output to stdout

Example usage:
```
cargo run -- --test
cargo run > dungeon.json
```

---

### tile_type.rs

Defines all tile types used in the dungeon grid.

Each tile has:
- Semantic meaning (wall, floor, door, etc.)
- ASCII representation for debugging

---

### dungeon/map.rs

Low-level dungeon map logic.

Responsibilities:
- Fixed-size 2D grid definition
- Safe tile placement helpers
- Room placement validation
- Corridor carving

---

### dungeon/room.rs

Room abstraction.

Stores:
- Bounding coordinates
- Center point calculation
- Overlap detection logic

Rooms are also serializable for export.

---

### dungeon/generator.rs

Core procedural generation logic.

Implements:
- Central starting room
- Recursive room expansion
- Corridor connection logic
- Door placement
- Entrance / exit positioning
- Full connectivity validation

The generator retries until a valid dungeon is produced.

---

### dungeon/json.rs

Defines the JSON export format used for integration.

Includes:
- Map dimensions
- Tile grid
- Room metadata

---

## Output Modes

### ASCII Debug Mode

```
cargo run -- --test
```

Prints a readable ASCII dungeon to the console.

---

### JSON Export (Default)

```
cargo run
```

Outputs a structured JSON representation to stdout.

---

## Design Goals

- Deterministic, debuggable generation
- Clear separation of concerns
- Engine-agnostic output
- Suitable for classic top-down or isometric RPGs

---

## Future Improvements

- Seed-based reproducibility
- Multiple generation strategies
- Room metadata (type, difficulty, loot)
- Graph-based connectivity analysis
- Additional export formats (RON, YAML)

---

## License

MIT
