use std::collections::HashMap;

use crate::{
    ldtk_helpers::{get_raw_world, gridpx_to_idx, src_to_atlas_index},
    ldtk_structs::{LDtk, Level},
};

/// The friendly, opiniated game map file. Contains the raw data
/// of the map made in ldtk but formatted in a way to be extremely simple
/// when used in game
pub struct DesignMap {
    pub levels: HashMap<String, DesignLevel>,

    tilesets: HashMap<usize, String>,
}

pub struct DesignLevel {
    pub level: Vec<TileContents>,
    pub width: usize,
    pub height: usize,
    pub grid_size_px: usize,
    pub tileset_name: String,
}

impl DesignLevel {
    fn empty() -> Self {
        Self {
            level: vec![],
            width: 0,
            height: 0,
            grid_size_px: 0,
            tileset_name: "Unset".to_string(),
        }
    }
}

#[derive(Clone, Default)]
pub struct TileContents {
    pub atlas_index: usize,
    pub entity_name: Option<String>, // simply the name of the entity as the defs are stored in a raw file
}

impl DesignMap {
    pub fn empty() -> Self {
        Self {
            levels: HashMap::new(),
            tilesets: HashMap::new(),
        }
    }

    /// Loads the ldtk file located at path and creates a game-friendly DesignMap
    pub fn new(path: String) -> Self {
        let ldtk_world = get_raw_world(path);

        let mut design_map = DesignMap::empty();
        design_map.tilesets = tilesets(&ldtk_world);

        for level in ldtk_world.levels.iter() {
            design_map.load_level(&level);
        }

        design_map
    }

    fn load_level(&mut self, level: &Level) {
        let level_name = &level.identifier;
        let mut new_design_level = DesignLevel::empty();

        if let Some(layer) = level
            .layer_instances
            .iter()
            .filter(|layer| layer.identifier.eq(&"Ground".to_string()))
            .next()
        {
            new_design_level.width = layer.width;
            new_design_level.height = layer.height;
            new_design_level.grid_size_px = layer.grid_size;
            new_design_level.level =
                vec![TileContents::default(); new_design_level.width * new_design_level.height];
            let tileset_id = layer.tileset_def_uid.unwrap();
            new_design_level.tileset_name = match self.tilesets.get(&tileset_id) {
                Some(val) => val.to_string(),
                None => panic!(
                    "Tileset ID: {} was not found in tileset collections.",
                    tileset_id
                ),
            };

            let grid_size = layer.grid_size;

            // Since we should have matched on the "Ground" layer we have high confidence we will have a gridTiles vec full of data
            if let Some(tiles) = &layer.grid_tiles {
                for tile in tiles.iter() {
                    let tile_index = gridpx_to_idx(
                        (tile.grid_x() / grid_size, tile.grid_y() / grid_size),
                        new_design_level.width,
                    );
                    new_design_level.level[tile_index].atlas_index =
                        src_to_atlas_index((tile.src_x(), tile.src_y()));
                }
            }
        } else {
            panic!(
                "Did not add a \"Ground\" layer to the level: {}",
                level_name
            );
        }

        if let Some(layer) = level
            .layer_instances
            .iter()
            .filter(|layer| layer.identifier.eq(&"Entities".to_string()))
            .next()
        {
            // Since we should have matched on the "Entities" layer we have high confidence we will have a Entities vec full of data
            if let Some(entities) = &layer.entity_instances {
                for entity in entities.iter() {
                    let tile_index = gridpx_to_idx((entity.grid_x(), entity.grid_y()), layer.width);
                    new_design_level.level[tile_index].entity_name =
                        Some(entity.identifier.clone());
                }
            }
        }

        if let Some(_) = self.levels.insert(level_name.to_string(), new_design_level) {
            panic!("{} level already existed, will be overwritten and is undesired behavior. Please consult the ldtk file.", level_name)
        }
    }
}

/// Creates the connection of tileset ids to their names
fn tilesets(data: &LDtk) -> HashMap<usize, String> {
    let tileset_names: Vec<(usize, String)> = data
        .defs
        .tilesets
        .iter()
        .map(|tileset| (tileset.uid, tileset.identifier.clone()))
        .collect();
    let tilesets: HashMap<usize, String> = tileset_names.into_iter().collect();
    tilesets
}

#[cfg(test)]
mod tests {

    use super::DesignMap;

    #[test]
    fn test_load_world_with_different_sized_levels() {
        let world = DesignMap::new("./tests/testmaps/different_level_sizes.ldtk".to_string());
        assert_eq!(world.levels[&"Level_0".to_string()].width, 22);
        assert_eq!(world.levels[&"Level_0".to_string()].height, 19);

        assert_eq!(world.levels[&"Level_1".to_string()].width, 12);
        assert_eq!(world.levels[&"Level_1".to_string()].height, 17);

        assert_eq!(world.levels[&"Level_2".to_string()].width, 18);
        assert_eq!(world.levels[&"Level_2".to_string()].height, 13);
    }

    #[test]
    fn test_load_world_with_entities() {
        let world = DesignMap::new("./tests/testmaps/entities.ldtk".to_string());
        assert_eq!(
            world.levels[&"Level_0".to_string()].level[0].entity_name,
            Some("Monster1".to_string())
        );
        assert_eq!(
            world.levels[&"Level_0".to_string()].level[3].entity_name,
            Some("Monster1".to_string())
        );
    }

    #[test]
    fn test_load_levels_with_different_tilesets() {
        let world = DesignMap::new("./tests/testmaps/two_tileatlases.ldtk".to_string());
        assert_eq!(
            world.levels[&"Level_0".to_string()].tileset_name,
            "Forest".to_string()
        );
        assert_eq!(
            world.levels[&"Level_1".to_string()].tileset_name,
            "SecondTileset".to_string()
        );
    }
}
