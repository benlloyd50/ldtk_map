use std::collections::HashMap;

use crate::{
    ldtk_helpers::{get_raw_world, gridpx_to_idx, src_to_atlas_index},
    ldtk_structs::{LDtk, Level},
};

/// The friendly, opiniated game map file. Contains the raw data
/// of the map made in ldtk but formatted in a way to be extremely simple
/// when used in game
#[derive(Debug, Default)]
pub struct DesignMap {
    levels: HashMap<String, DesignLevel>,
    tilesets: HashMap<usize, String>,
}

/// Represents a single level designed in LDtk, contains the minimal
/// amount of data necessary to rebuild levels in game.
#[derive(Debug)]
pub struct DesignLevel {
    level: Vec<TileContents>,
    level_name: String,
    width: usize,
    height: usize,
    grid_size_px: usize,
    tileset_name: String,
    world_x: i64,
    world_y: i64,
}

impl DesignLevel {
    fn empty() -> Self {
        Self {
            level: vec![],
            level_name: "Unnamed".to_string(),
            width: 0,
            height: 0,
            grid_size_px: 0,
            tileset_name: "Unset".to_string(),
            world_x: 0,
            world_y: 0,
        }
    }

    /// The x coordinate of the map in the world
    pub fn world_tile_x(&self) -> i64 {
        self.world_x
    }

    /// The y coordinate of the map in the world
    pub fn world_tile_y(&self) -> i64 {
        self.world_y
    }

    /// The x and y coordinates of the map in the world
    pub fn world_xy(&self) -> (i64, i64) {
        (self.world_x, self.world_y)
    }

    /// The contents of the level
    pub fn level(&self) -> &[TileContents] {
        self.level.as_ref()
    }

    /// The name of the level
    pub fn name(&self) -> &str {
        self.level_name.as_ref()
    }

    /// The width of the level based on the Ground layer
    pub fn width(&self) -> usize {
        self.width
    }

    /// The height of the level based on the Ground layer
    pub fn height(&self) -> usize {
        self.height
    }

    /// The size of the sprites used on this level in px
    pub fn grid_size_px(&self) -> usize {
        self.grid_size_px
    }

    /// The tileset identifier being used for this level
    pub fn tileset_name(&self) -> &str {
        self.tileset_name.as_ref()
    }
}

/// Represents a single tile in a LDtk level
#[derive(Clone, Default, Debug)]
pub struct TileContents {
    atlas_index: usize,
    entity_name: Option<String>, // simply the name of the entity as the defs are stored in a raw file
    entity_tag: Option<String>,
    value: usize,
}

impl TileContents {
    /// The index in the tile atlas for this tile's sprite
    pub fn atlas_index(&self) -> usize {
        self.atlas_index
    }

    /// If the tile is blocked by the contents either, entity or terrain features
    pub fn value(&self) -> usize {
        self.value
    }

    /// The name of the entity present in the tile
    pub fn entity_name(&self) -> Option<&str> {
        self.entity_name.as_deref()
    }

    /// The tag associated with an entity.
    /// Currently only supports 1 tag
    pub fn entity_tag(&self) -> Option<&str> {
        self.entity_tag.as_deref()
    }
}

const GROUND: &str = &"Ground";
const ENTITIES: &str = &"Entities";
const VALUES: &str = &"Values";

impl DesignMap {
    fn new() -> Self {
        Self {
            levels: HashMap::new(),
            tilesets: HashMap::new(),
        }
    }

    /// The levels of the world
    pub fn levels(&self) -> &HashMap<String, DesignLevel> {
        &self.levels
    }

    /// Loads the ldtk file located at path and creates a game-friendly DesignMap
    pub fn load(path: impl ToString) -> Self {
        let ldtk_world = get_raw_world(path.to_string());

        let mut design_map = DesignMap::new();
        design_map.tilesets = tilesets(&ldtk_world);

        for level in ldtk_world.levels.iter() {
            design_map.load_level(level);
        }

        design_map
    }

    /// Creates a DesignLevel out of a ldtk level, `level` inserting into the DesignMap
    fn load_level(&mut self, level: &Level) {
        let level_name = &level.identifier;
        let mut new_design_level = DesignLevel::empty();
        new_design_level.level_name = level.identifier.clone();
        new_design_level.world_x = level.world_x / 8;
        new_design_level.world_y = level.world_y / 8;

        if let Some(layer) = level
            .layer_instances
            .iter()
            .find(|layer| layer.identifier.eq(GROUND))
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
                    new_design_level.level[tile_index].atlas_index = src_to_atlas_index(
                        (tile.src_x(), tile.src_y()),
                        new_design_level.grid_size_px(),
                    );
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
            .find(|layer| layer.identifier.eq(ENTITIES))
        {
            // Since we should have matched on the "Entities" layer we have high confidence we will have a Entities vec full of data
            if let Some(entities) = &layer.entity_instances {
                for entity in entities.iter() {
                    let tile_index = gridpx_to_idx((entity.grid_x(), entity.grid_y()), layer.width);
                    let new_name = entity.identifier.replace('_', " ").clone();
                    new_design_level.level[tile_index].entity_name = Some(new_name);
                    if let Some(tag) = entity.tags.first() {
                        new_design_level.level[tile_index].entity_tag = Some(tag.to_string());
                    }
                }
            }
        }

        if let Some(layer) = level
            .layer_instances
            .iter()
            .find(|layer| layer.identifier.eq(VALUES))
        {
            // Since we should have matched on the "Entities" layer we have high confidence we will have a Entities vec full of data
            if let Some(values) = &layer.int_grid_csv {
                for (idx, val) in values.iter().enumerate() {
                    new_design_level.level[idx].value = *val;
                }
            }
        }

        if self
            .levels
            .insert(level_name.to_string(), new_design_level)
            .is_some()
        {
            panic!("{} level already existed, will be overwritten and is undesired behavior. Please consult the ldtk file.", level_name)
        }
    }
}

/// Creates the connection of tileset ids to their names
fn tilesets(data: &LDtk) -> HashMap<usize, String> {
    data.defs
        .tilesets
        .iter()
        .map(|tileset| (tileset.uid, tileset.identifier.clone()))
        .into_iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::DesignMap;

    #[test]
    fn test_load_world_with_different_sized_levels() {
        let world = DesignMap::load("./tests/testmaps/different_level_sizes.ldtk".to_string());
        assert_eq!(world.levels[&"Level_0".to_string()].width, 22);
        assert_eq!(world.levels[&"Level_0".to_string()].height, 19);

        assert_eq!(world.levels[&"Level_1".to_string()].width, 12);
        assert_eq!(world.levels[&"Level_1".to_string()].height, 17);

        assert_eq!(world.levels[&"Level_2".to_string()].width, 18);
        assert_eq!(world.levels[&"Level_2".to_string()].height, 13);
    }

    #[test]
    fn test_load_world_with_entities() {
        let world = DesignMap::load("./tests/testmaps/entities.ldtk".to_string());
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
        let world = DesignMap::load("./tests/testmaps/two_tileatlases.ldtk".to_string());
        assert_eq!(
            world.levels[&"Level_0".to_string()].tileset_name,
            "Forest".to_string()
        );
        assert_eq!(
            world.levels[&"Level_1".to_string()].tileset_name,
            "SecondTileset".to_string()
        );
    }

    #[test]
    #[should_panic]
    /// Tests that the world cannot be opened since it is empty
    fn test_open_empty_world() {
        let _ = DesignMap::load("./tests/testmaps/empty_world.ldtk".to_string());
    }

    #[test]
    #[should_panic]
    /// Tests the world does not load with duplicated level names
    fn test_duplicate_level_name() {
        let _ = DesignMap::load("./tests/testmaps/bad_names.ldtk".to_string());
    }
}
