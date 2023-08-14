use std::fs;

use crate::ldtk_structs::LDtk;

/// Deserializes the raw LDtk file into the LDtk struct
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
pub(crate) fn src_to_atlas_index((x, y): (usize, usize), px_size: usize) -> usize {
    x / px_size + y / px_size * ATLAS_WIDTH
}

#[cfg(test)]
mod tests {
    use super::{gridpx_to_idx, src_to_atlas_index, get_raw_world};

    #[test]
    #[should_panic]
    /// Assurance we are not loading a map when there is a bad file path given
    fn test_bad_path_to_file() {
        let _ = get_raw_world("wrong_path".to_string());
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
        let src = (64, 0);
        let src2 = (48, 0);

        let idx = src_to_atlas_index(src, 16);
        let idx2 = src_to_atlas_index(src2, 8);

        assert_eq!(idx, 4, "idx did not match the expected.");
        assert_eq!(idx2, 6, "idx2 did not match the expected.");
    }
}
