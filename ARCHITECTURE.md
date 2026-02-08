# Dungeon Generation Architecture (Diablo I Style)

This document describes the internal architecture and procedural generation algorithm
used in the dungeon map generator. The approach is heavily inspired by **Diablo I**
(cathedral / dungeon levels) and classic roguelike techniques.

The focus is on **deterministic structure**, **full connectivity**, and **engine-agnostic output**.

---

## High-Level Overview

Dungeon generation follows a **room-first** approach:

1. Initialize an empty solid map
2. Place a central starting room
3. Expand the dungeon by attaching rooms recursively
4. Connect rooms with wide corridors
5. Place doors at logical transitions
6. Validate connectivity
7. Place entrance and exit
8. Export final representation

The generator is allowed to fail and restart until all constraints are met.

---

## Map Representation

The dungeon map is a fixed-size 2D grid:

- Each cell contains a `TileType`
- Initially filled with `IMPASS`
- Mutated in-place during generation

```text
IMPASS → WALL → FLOOR → DOOR / ARCH → SPECIAL
```

This mirrors Diablo I’s approach of carving walkable space from solid blocks.

---

## Step 1: Central Room Placement

Generation starts by placing a **central room** near the middle of the map.

Why:
- Guarantees a stable anchor point
- Prevents early edge collisions
- Improves visual balance

Algorithm:
- Random room size within allowed bounds
- Position biased toward map center
- Carve floor tiles
- Surround with walls

This room becomes the root of the dungeon graph.

---

## Step 2: Recursive Room Expansion

New rooms are added by attaching them to existing rooms.

For each parent room:
1. Randomly select a direction (N, S, E, W)
2. Generate a candidate room rectangle
3. Check bounds and overlap (with buffer)
4. If valid:
   - Carve room
   - Register room
   - Create a corridor
   - Recurse from the new room

This produces an **organic tree-like layout**, similar to Diablo I cathedral levels.

Constraints:
- Maximum room count
- Maximum recursion depth
- Minimum spacing between rooms

---

## Step 3: Corridor Generation

Rooms are connected using **wide corridors** (not 1-tile tunnels).

Corridor characteristics:
- Usually 2–3 tiles wide
- Aligned orthogonally
- Carved from solid space
- Enclosed with walls

Corridors are carved before door placement.

This preserves Diablo I’s “hallway” feeling rather than roguelike tunnels.

---

## Step 4: Door Placement

Doors are placed at transitions between:
- Room ↔ corridor
- Corridor ↔ room

Rules:
- Only one door per transition
- Doors must connect two walkable areas
- Tile surroundings must support a doorway

Door types may vary:
- `DOOR`
- `DOORGRATE`
- `ARCH`

At this stage, doors are purely structural (no gameplay logic).

---

## Step 5: Connectivity Validation

After generation, the dungeon is validated.

Validation ensures:
- All rooms are reachable
- Entrance can reach exit
- No isolated floor regions exist

Typical approach:
- Flood-fill / BFS from entrance
- Count reachable floor tiles
- Compare against total floor tiles

If validation fails:
- Discard dungeon
- Restart generation

This matches Diablo I’s “generate until valid” strategy.

---

## Step 6: Entrance and Exit Placement

Once connectivity is confirmed:

- Entrance is placed in or near the starting room
- Exit is placed in the room farthest from the entrance
- Distance is typically measured via BFS depth

This ensures:
- Logical progression
- Maximum traversal distance
- Good pacing for gameplay

---

## Step 7: Finalization

Final steps:
- Convert remaining `IMPASS` to walls if needed
- Fix corner tiles
- Replace placeholders with final tile variants
- Prepare data for export

At this stage, the dungeon is immutable.

---

## Dungeon as a Graph

Conceptually, the dungeon is a graph:

- Nodes → rooms
- Edges → corridors
- Doors → edge transitions

Although stored as a grid, this mental model helps when:
- Adding locks and keys
- Designing quest logic
- Implementing AI navigation

---

## Failure and Retry Model

The generator is intentionally allowed to fail.

Reasons:
- Overlapping rooms
- Invalid corridor placement
- Disconnected regions
- Poor entrance/exit distance

Instead of over-constraining generation,
the system retries until a valid dungeon is produced.

This keeps the code simpler and the results more natural.

---

## Differences from Pure Roguelikes

Compared to classic roguelikes:

| Aspect              | Roguelike          | This Generator |
|---------------------|--------------------|----------------|
| Corridors           | 1-tile tunnels     | Wide halls     |
| Structure           | Random graph       | Room tree      |
| Layout feel         | Abstract           | Architectural  |
| Validation          | Often implicit     | Explicit BFS   |

---

## Extension Points

Designed extension areas:

- Seed-based reproducibility
- Room types (boss, shrine, treasure)
- Locked doors and keys
- Multi-floor staircases
- Script hooks per room
- Graph-level metadata

---

## Design Philosophy

- Prefer clarity over cleverness
- Let generation fail instead of overfitting rules
- Separate structure from gameplay
- Keep output engine-agnostic

This architecture is intentionally close to **Diablo I’s original dungeon logic**,
while remaining modern, testable, and extensible.

---

## References

- Jarulf’s Guide to Diablo and Hellfire
- Diablo I Cathedral generation analysis
- Classic roguelike procedural techniques
