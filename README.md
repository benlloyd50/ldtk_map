# ldtk_map
![maintenance-status](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![codecov](https://codecov.io/github/benlloyd50/ldtk_map/branch/master/graph/badge.svg?token=LIAEO385H6)](https://codecov.io/github/benlloyd50/ldtk_map)

A crate for reading the LDtk v1.3.3. maps into a more user-friendly (*read opinionated*) data structure for usage in game.

## What is this crate
LDtk maps contain a lot of data that the __developer__ is not interested in.
This crate implements the structs for the LDtk map but abstracts them behind a DesignMap that contains the minimal amount of data necessary.
The goal of this crate is to provide a simple interface into working with LDtk files.
This crate is not trying to give you access to every field on your LDtk file, there are many existing crates for this.

## Add To Your Project
**`Cargo.toml`**

```toml
ldtk_map = { version = "0.2.0" }
```

## Examples
The public facing of `DesignMap` and child structs aims to be as simple as possible:
```rust
use ldtk_map::prelude::*;

fn main() {
    // Replace with your ldtk file!
    let my_design = DesignMap::load("../tests/testmaps/two_tileatlases.ldtk");

    // Get some info about your tile in Level_0 at (0, 0)
    my_design.levels().get("Level_0").unwrap().level()[0].atlas_index();

    // Use in your game by reading to your own map data struct
    convert_to_games_map(&my_design);
}
```

## Assumptions About Your Game (How to use the library)
- Entity data is stored outside of the LDtk file except for the name of the entity
- Layers all use the same pixel size for the tileset
- Once your LDtk file is loaded it will be not loaded again for the lifetime of the program.
- You will manage the connections between the levels. (thinking of a solution to this)
- You will follow this map format:

### LDtk Map Formatting
The map must be formatted using the guidelines or else it will not be loaded into `DesignMap` properly.
1. Use only 1 world and place free form levels in the world.
2. The "Ground" layer must always be defined in your project as it used for each level to get the following values from: `width`, `height`, `grid_size`, and `tileset_name`
3. Entities must be placed on an "Entities" layer

## Contributing and Issues
Everyone is more than welcome to submit feature requests and bug fixes.
I will comb through these as frequently as possible and try to handle them accordingly.
