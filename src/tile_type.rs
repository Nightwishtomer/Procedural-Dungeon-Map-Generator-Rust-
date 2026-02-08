//tile_type.rs

#![allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum TileType {
    IMPASS,
    FLOOR,
    WALL,
    GRATE,
    DOOR,
    DOORGRATE,
    ARCH,
    ARCHGRATE,
    CORNERITL,
    CORNERITR,
    CORNERIBL,
    CORNERIBR,
    CORNEROTL,
    CORNEROTR,
    CORNEROBL,
    CORNEROBR,
    PLAYER,
    ENTRANCE,
    EXIT,
}

impl TileType {
    pub fn ascii_icon(&self) -> char{
        match self {
            TileType::IMPASS    => '0',
            TileType::FLOOR     => '.',
            TileType::WALL      => '#',
            TileType::GRATE     => '"',
            TileType::DOOR      => '*',
            TileType::DOORGRATE => '`',
            TileType::ARCH      => '-',
            TileType::ARCHGRATE => '+',
            TileType::CORNERITL => 'A',
            TileType::CORNERITR => 'B',
            TileType::CORNERIBL => 'C',
            TileType::CORNERIBR => 'D',
            TileType::CORNEROTL => 'E',
            TileType::CORNEROTR => 'F',
            TileType::CORNEROBL => 'G',
            TileType::CORNEROBR => 'H',
            TileType::PLAYER    => '@',
            TileType::ENTRANCE  => 'S',
            TileType::EXIT      => 'X',
        }
    }

    pub fn is_passable(&self) -> bool {
        match self {
          TileType::IMPASS => false,
          TileType::FLOOR  => true,
          TileType::WALL   => false,
          TileType::GRATE  => false,
          TileType::DOOR   => true,
          TileType::DOORGRATE => true,
          TileType::ARCH   => true,
          TileType::ARCHGRATE => false,
          TileType::CORNERITL => false,
          TileType::CORNERITR => false,
          TileType::CORNERIBL => false,
          TileType::CORNERIBR => false,
          TileType::CORNEROTL => false,
          TileType::CORNEROTR => false,
          TileType::CORNEROBL => false,
          TileType::CORNEROBR => false,
          TileType::PLAYER => true,
          TileType::ENTRANCE => true,
          TileType::EXIT => true,
        }
    }
}