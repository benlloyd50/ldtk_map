use std::fs;

use crate::ldtk_structs::LDtk;

pub(crate) fn get_raw_world(level_path: String) -> LDtk {
    let contents = match fs::read_to_string(&level_path) {
        Ok(contents) => contents,
        Err(e) => panic!("Should have been able to read the file: {} | Error: {} | Please report this error on Github so I can fix the game for you and everyone else :)", &level_path, e) 
    };
    let ldtk_world: LDtk = match serde_json::from_str(&contents) {
        Ok(world) => world,
        Err(e) => panic!("Could not load level at {} | Error: {} | Please report this error on Github so I can fix the game for you and everyone else :)", level_path, e)
    };
    ldtk_world
}

/// Converts the gridpx (x, y) into the index in the 1d vec
pub(crate) fn gridpx_to_idx((x, y): (usize, usize), width: usize) -> usize {
    width * y + x
}

/// Converts the src (x, y) into the index of an atlas sized 16 x 16, 8 pixel square
const ATLAS_WIDTH: usize = 16;
pub(crate) fn src_to_atlas_index((x, y): (usize, usize)) -> usize {
    x / 8 + y / 8 * ATLAS_WIDTH
}

#[cfg(test)]
mod tests {
    use super::{get_raw_world, gridpx_to_idx, src_to_atlas_index};

    #[test]
    #[ignore] // ignored because this map is not pushed to GH
    /// Tests to make sure the level identifier is found in it's correct spot
    fn test_get_raw_world() {
        let world = get_raw_world("./maps/rpg_world_v1.ldtk".to_string());
        let level_1_name: &String = &world.levels[0].identifier;
        assert_eq!(*level_1_name, String::from("Level_0"));
    }

    #[test]
    fn test_gridpx_to_idx() {
        let px = (0 / 8, 0);
        let px2 = (48 / 8, 0);

        let idx = gridpx_to_idx(px, 40);
        let idx2 = gridpx_to_idx(px2, 40);

        assert_eq!(idx, 0, "idx did not match the expected");
        assert_eq!(idx2, 6, "idx2 did not match the expected.");
    }

    #[test]
    fn test_src_to_atlas_idx() {
        let src = (0, 0);
        let src2 = (48, 0);

        let idx = src_to_atlas_index(src);
        let idx2 = src_to_atlas_index(src2);

        assert_eq!(idx, 0, "idx did not match the expected.");
        assert_eq!(idx2, 6, "idx2 did not match the expected.");
    }
}
