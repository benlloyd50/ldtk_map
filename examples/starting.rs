use ldtk_map::prelude::*;

fn main() {
    let my_design = DesignMap::load("../tests/testmaps/two_tileatlases.ldtk");

    // Get some info about your tile in Level_0 at (0, 0)
    my_design.levels().get("Level_0").unwrap().level()[0].atlas_index();

    convert_to_games_map(&my_design);
}

fn convert_to_games_map(_: &DesignMap) {
    unimplemented!("The contents of this function is irrelavent");
}
