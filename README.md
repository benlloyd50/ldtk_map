# ldtk_map
![maintenance-status](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![codecov](https://codecov.io/github/benlloyd50/ldtk_map/branch/master/graph/badge.svg?token=LIAEO385H6)](https://codecov.io/github/benlloyd50/ldtk_map)

A crate for reading the LDtk v1.3.4 maps into a more user-friendly (*read opinionated*) data structure for usage in game.

## What is this crate for?
LDtk maps contain a lot of data that the __developer__ is probably not going to need to use.
This crate implements the structs for the LDtk map but abstracts them behind a DesignMap.
The DesignMap contains only a small amount of that data but it is what I found necessary when building my game.
In turn the goal of this crate is providing this simple interface when working with this files.
It should function as a good crate for beginners wanting to make games in rust.
However, this crate is not trying to give you access to every field on your LDtk file.
Luckily, some other fine devs made crates that might suit your use case better:

[ldtk](https://crates.io/crates/ldtk) *Note: mind the license*

[ldtk_rust](https://crates.io/crates/ldtk_rust)

[ldtk2](https://crates.io/crates/ldtk2)


## Add To Your Project
**`Cargo.toml`**

```toml
ldtk_map = { version = "0.3.0" }
```

** or in your cmd line **

```bash
cargo add ldtk_map
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
- Entity data is stored outside of the LDtk file except for the name of the entity on the "Entities" layer.
- Layers all use the same pixel size for the tileset
- Once your LDtk file is loaded ingame it will be not reloaded for the lifetime of the program.
- You will manage the connections between the levels. (thinking of a solution to this)
- You will follow this map format:

### LDtk Map Formatting
The map must be formatted using the guidelines or else it will not be loaded into `DesignMap` properly.
1. Use only 1 world and place free form levels in the world.
2. The "Ground" layer must always be defined in your project as it used for each level to get the following values from: `width`, `height`, `grid_size`, and `tileset_name`.
3. Entities must be placed on an "Entities" layer.
4. Values may be placed on the "Values" layer.
5. All layers mentioned must be in each level.

## Contributing and Issues
Everyone is more than welcome to submit feature requests and bug fixes.
I will comb through these as frequently as possible and try to handle them accordingly.
