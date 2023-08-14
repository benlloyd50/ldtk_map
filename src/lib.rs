//! A crate for reading the ldtk map into a more user-friendly (*read opinionated*) data structure for usage in game.
//!
//! LDtk maps take in and carry a lot of data that the __developer__ will probably not be interested in.
//! The crate implements the structs for the ldtk map but abstracts them behind a DesignMap that contains the minimal amount of data.

#![warn(missing_docs)]
mod design_map;
mod ldtk_helpers;
mod ldtk_structs; // These are kept internal as they are a rather nasty looking

/// Exports the user facing LDtk structs
/// Example Usage:
/// ```
///
/// ```
pub mod prelude {
    pub use crate::design_map::DesignLevel;
    pub use crate::design_map::DesignMap;
    pub use crate::design_map::TileContents;
}
