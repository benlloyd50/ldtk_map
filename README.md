# ldtk_map
A crate for reading the ldtk map into a more user-friendly (*read opinionated*) data structure for usage in game.

LDtk maps take in and carry a lot of data that the __developer__ will probably not be interested in.
The crate implements the structs for the ldtk map but abstracts them behind a DesignMap that contains the minimal amount of data.

## Installation
**`Cargo.toml`**

```toml
ldtk_map = { version = "0.1.0" }
```

## Examples
The public facing of `DesignMap` and child structs aims to be as simple as possible:
```rust
struct DesignMap {
    levels: HashMap<String, DesignLevel>,
}

struct DesignLevel {
    level: Vec<TileContents>,
    width: usize,
    height: usize,
    grid_size: usize,
    tileset_name: String,
}

struct TileContents {
    atlas_index: usize,
    entity_name: Option<String>,
}
```

## Assumptions About Your Game (How to use the library)
These 3 structs provide enough control to utilize in your game a few things. 
- `atlas_index` - an index into your texture atlas that is the same as the one used in LDtk.
- `entity_name` - optional, stringly-typed name that can be used to load from an entity database that is hashable by this name.
- `grid_size` - another slightly tricky variable as every layer must share their pixel size and this may be changed in the future.

The levels HashMap contains all of your levels loaded by their name in game. 
This is something I can see as be limiting but it is the current solution I chose.

### LDtk Map Formatting
The map must be formatted using the guidelines or else it will not be loaded into `DesignMap` properly.
1. Use only 1 world and place free form levels in the world.
2. You are responsible for managing the connections between the levels. (thinking of a solution to this)
3. The "Ground" layer must always be defined in your project as it used for each level to get the following values from: `width`, `height`, `grid_size`, and `tileset_name`
4. Entities must be placed on an "Entities" layer

\* Meaningful in the sense that it will be one of two layers actually being read.







