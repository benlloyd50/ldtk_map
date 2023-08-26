use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LDtk {
    #[serde(rename = "__header__")]
    pub(crate) header: Option<Header>,
    pub(crate) iid: Option<String>,
    pub(crate) json_version: Option<String>,
    pub(crate) app_build_id: Option<i64>,
    pub(crate) next_uid: Option<i64>,
    pub(crate) identifier_style: Option<String>,
    pub(crate) toc: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) world_layout: Option<String>,
    pub(crate) world_grid_width: Option<i64>,
    pub(crate) world_grid_height: Option<i64>,
    pub(crate) default_level_width: Option<i64>,
    pub(crate) default_level_height: Option<i64>,
    pub(crate) default_pivot_x: Option<i64>,
    pub(crate) default_pivot_y: Option<i64>,
    pub(crate) default_grid_size: Option<i64>,
    pub(crate) bg_color: Option<String>,
    pub(crate) default_level_bg_color: Option<String>,
    pub(crate) minify_json: Option<bool>,
    pub(crate) external_levels: Option<bool>,
    pub(crate) export_tiled: Option<bool>,
    pub(crate) simplified_export: Option<bool>,
    pub(crate) image_export_mode: Option<String>,
    pub(crate) export_level_bg: Option<bool>,
    pub(crate) png_file_pattern: Option<serde_json::Value>,
    pub(crate) backup_on_save: Option<bool>,
    pub(crate) backup_limit: Option<i64>,
    pub(crate) backup_rel_path: Option<serde_json::Value>,
    pub(crate) level_name_pattern: Option<String>,
    pub(crate) tutorial_desc: Option<serde_json::Value>,
    pub(crate) custom_commands: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) flags: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) defs: Defs,
    pub(crate) levels: Vec<Level>,
    pub(crate) worlds: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) dummy_world_iid: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Defs {
    pub(crate) layers: Option<Vec<Layer>>,
    pub(crate) entities: Option<Vec<Entity>>,
    pub(crate) tilesets: Vec<Tileset>,
    pub(crate) enums: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) external_enums: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) level_fields: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    pub(crate) identifier: Option<String>,
    pub(crate) uid: Option<i64>,
    pub(crate) tags: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) export_to_toc: Option<bool>,
    pub(crate) doc: Option<serde_json::Value>,
    pub(crate) width: Option<i64>,
    pub(crate) height: Option<i64>,
    pub(crate) resizable_x: Option<bool>,
    pub(crate) resizable_y: Option<bool>,
    pub(crate) min_width: Option<serde_json::Value>,
    pub(crate) max_width: Option<serde_json::Value>,
    pub(crate) min_height: Option<serde_json::Value>,
    pub(crate) max_height: Option<serde_json::Value>,
    pub(crate) keep_aspect_ratio: Option<bool>,
    pub(crate) tile_opacity: Option<i64>,
    pub(crate) fill_opacity: Option<f64>,
    pub(crate) line_opacity: Option<i64>,
    pub(crate) hollow: Option<bool>,
    pub(crate) color: Option<String>,
    pub(crate) render_mode: Option<String>,
    pub(crate) show_name: Option<bool>,
    pub(crate) tileset_id: Option<i64>,
    pub(crate) tile_render_mode: Option<String>,
    pub(crate) tile_rect: Option<Tile>,
    pub(crate) nine_slice_borders: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) max_count: Option<i64>,
    pub(crate) limit_scope: Option<String>,
    pub(crate) limit_behavior: Option<String>,
    pub(crate) pivot_x: Option<i64>,
    pub(crate) pivot_y: Option<i64>,
    pub(crate) field_defs: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tile {
    pub(crate) tileset_uid: Option<i64>,
    pub(crate) x: Option<i64>,
    pub(crate) y: Option<i64>,
    pub(crate) w: Option<i64>,
    pub(crate) h: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Layer {
    #[serde(rename = "__type")]
    pub(crate) layer_type: Option<String>,
    pub(crate) identifier: Option<String>,
    #[serde(rename = "type")]
    pub(crate) purple_type: Option<String>,
    pub(crate) uid: Option<i64>,
    pub(crate) doc: Option<String>,
    pub(crate) ui_color: Option<serde_json::Value>,
    pub(crate) grid_size: Option<i64>,
    pub(crate) guide_grid_wid: Option<i64>,
    pub(crate) guide_grid_hei: Option<i64>,
    pub(crate) display_opacity: Option<i64>,
    pub(crate) inactive_opacity: Option<f64>,
    pub(crate) hide_in_list: Option<bool>,
    pub(crate) hide_fields_when_inactive: Option<bool>,
    pub(crate) can_select_when_inactive: Option<bool>,
    pub(crate) render_in_world_view: Option<bool>,
    pub(crate) px_offset_x: Option<i64>,
    pub(crate) px_offset_y: Option<i64>,
    pub(crate) parallax_factor_x: Option<i64>,
    pub(crate) parallax_factor_y: Option<i64>,
    pub(crate) parallax_scaling: Option<bool>,
    pub(crate) required_tags: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) excluded_tags: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) int_grid_values: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) auto_rule_groups: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) auto_source_layer_def_uid: Option<serde_json::Value>,
    pub(crate) tileset_def_uid: Option<i64>,
    pub(crate) tile_pivot_x: Option<i64>,
    pub(crate) tile_pivot_y: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tileset {
    #[serde(rename = "__cWid")]
    pub(crate) c_wid: Option<i64>,
    #[serde(rename = "__cHei")]
    pub(crate) c_hei: Option<i64>,
    pub(crate) identifier: String,
    pub(crate) uid: usize,
    pub(crate) rel_path: Option<String>,
    pub(crate) embed_atlas: Option<String>,
    pub(crate) px_wid: Option<i64>,
    pub(crate) px_hei: Option<i64>,
    pub(crate) tile_grid_size: Option<i64>,
    pub(crate) spacing: Option<i64>,
    pub(crate) padding: Option<i64>,
    pub(crate) tags: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) tags_source_enum_uid: Option<serde_json::Value>,
    pub(crate) enum_tags: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) custom_data: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) saved_selections: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) cached_pixel_data: Option<CachedPixelData>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CachedPixelData {
    pub(crate) opaque_tiles: Option<String>,
    pub(crate) average_colors: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub(crate) file_type: Option<String>,
    pub(crate) app: Option<String>,
    pub(crate) doc: Option<String>,
    pub(crate) schema: Option<String>,
    pub(crate) app_author: Option<String>,
    pub(crate) app_version: Option<String>,
    pub(crate) url: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Level {
    pub(crate) identifier: String,
    pub(crate) iid: Option<String>,
    pub(crate) uid: Option<i64>,
    pub(crate) world_x: i64,
    pub(crate) world_y: i64,
    pub(crate) world_depth: Option<i64>,
    pub(crate) px_wid: Option<i64>,
    pub(crate) px_hei: Option<i64>,
    #[serde(rename = "__bgColor")]
    pub(crate) bg_color: Option<String>,
    #[serde(rename = "bgColor")]
    pub(crate) level_bg_color: Option<serde_json::Value>,
    pub(crate) use_auto_identifier: Option<bool>,
    pub(crate) bg_rel_path: Option<serde_json::Value>,
    #[serde(rename = "bgPos")]
    pub(crate) level_bg_pos: Option<serde_json::Value>,
    pub(crate) bg_pivot_x: Option<f64>,
    pub(crate) bg_pivot_y: Option<f64>,
    #[serde(rename = "__smartColor")]
    pub(crate) smart_color: Option<String>,
    #[serde(rename = "__bgPos")]
    pub(crate) bg_pos: Option<serde_json::Value>,
    pub(crate) external_rel_path: Option<serde_json::Value>,
    pub(crate) field_instances: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) layer_instances: Vec<LayerInstance>,
    #[serde(rename = "__neighbours")]
    pub(crate) neighbours: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerInstance {
    #[serde(rename = "__identifier")]
    pub(crate) identifier: String,
    #[serde(rename = "__type")]
    pub(crate) layer_instance_type: String, // possible values: IntGrid, Entities, Tiles or AutoLayer
    #[serde(rename = "__cWid")]
    pub(crate) width: usize,
    #[serde(rename = "__cHei")]
    pub(crate) height: usize,
    #[serde(rename = "__gridSize")]
    pub(crate) grid_size: usize,
    #[serde(rename = "__opacity")]
    pub(crate) opacity: Option<i64>,
    #[serde(rename = "__pxTotalOffsetX")]
    pub(crate) px_total_offset_x: Option<i64>,
    #[serde(rename = "__pxTotalOffsetY")]
    pub(crate) px_total_offset_y: Option<i64>,
    #[serde(rename = "__tilesetDefUid")]
    pub(crate) tileset_def_uid: Option<usize>,
    #[serde(rename = "__tilesetRelPath")]
    pub(crate) tileset_rel_path: Option<String>,
    pub(crate) iid: Option<String>,
    pub(crate) level_id: Option<i64>,
    pub(crate) layer_def_uid: Option<i64>,
    pub(crate) px_offset_x: Option<i64>,
    pub(crate) px_offset_y: Option<i64>,
    pub(crate) visible: bool,
    pub(crate) optional_rules: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) int_grid_csv: Option<Vec<usize>>,
    pub(crate) auto_layer_tiles: Option<Vec<Option<serde_json::Value>>>,
    pub(crate) seed: Option<i64>,
    pub(crate) override_tileset_uid: Option<serde_json::Value>,
    pub(crate) grid_tiles: Option<Vec<GridTile>>,
    pub(crate) entity_instances: Option<Vec<EntityInstance>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityInstance {
    #[serde(rename = "__identifier")]
    pub(crate) identifier: String,
    #[serde(rename = "__grid")]
    pub(crate) grid: Vec<i64>,
    #[serde(rename = "__pivot")]
    pub(crate) pivot: Option<Vec<i64>>,
    #[serde(rename = "__tags")]
    pub(crate) tags: Vec<String>,
    #[serde(rename = "__tile")]
    pub(crate) tile: Option<Tile>,
    #[serde(rename = "__smartColor")]
    pub(crate) smart_color: Option<String>,
    pub(crate) iid: Option<String>,
    pub(crate) width: Option<i64>,
    pub(crate) height: Option<i64>,
    pub(crate) def_uid: Option<i64>,
    pub(crate) px: Option<Vec<i64>>,
    pub(crate) field_instances: Option<Vec<Option<serde_json::Value>>>,
}

impl EntityInstance {
    pub(crate) fn grid_x(&self) -> usize {
        if self.grid.len() != 2 {
            panic!(
                "{} entity does not have a proper grid position",
                self.identifier
            )
        }
        match self.grid[0].try_into() {
            Ok(val) => val,
            Err(e) => panic!(
                "Could load {} because it's position is negative | Error: {}",
                self.identifier, e
            ),
        }
    }

    pub(crate) fn grid_y(&self) -> usize {
        if self.grid.len() != 2 {
            panic!(
                "{} entity does not have a proper grid position",
                self.identifier
            )
        }
        match self.grid[1].try_into() {
            Ok(val) => val,
            Err(e) => panic!(
                "Could load {} because it's position is negative | Error: {}",
                self.identifier, e
            ),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct GridTile {
    //these two must be positive, i refuse to deal with negatives
    pub(crate) px: (usize, usize), // Pixel coordinates of the tile in the world ([x,y] format)
    pub(crate) src: (usize, usize), // Pixel coordinates of the tile in the atlas ([x,y] format)
    pub(crate) f: i64,
    pub(crate) t: i64,
    pub(crate) d: Vec<i64>,
    pub(crate) a: i64,
}

impl GridTile {
    pub(crate) fn grid_x(&self) -> usize {
        self.px.0
    }

    pub(crate) fn grid_y(&self) -> usize {
        self.px.1
    }

    pub(crate) fn src_x(&self) -> usize {
        self.src.0
    }

    pub(crate) fn src_y(&self) -> usize {
        self.src.1
    }
}
