extern crate amethyst;

use amethyst::{
    core::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{ColorMask, DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage, ALPHA},
    utils::application_root_dir,
};

struct Example;

impl SimpleState for Example {}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let resources_path = format!("{}/resources", application_root_dir());
    let display_config_path = format!("{}/display.ron", resources_path);
    let key_bindings_path = format!("{}/input.ron", resources_path);

    let display_config = DisplayConfig::load(&display_config_path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.00196, 0.23726, 0.21765, 1.0], 1.0)
            .with_pass(DrawFlat2D::new().with_transparency(ColorMask::all(), ALPHA, None)),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(
            InputBundle::<String, String>::new().with_bindings_from_file(&key_bindings_path)?,
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderBundle::new(pipe, Some(display_config))
                .with_sprite_sheet_processor()
                .with_sprite_visibility_sorting(&[]),
        )?;

    let mut game = Application::new(resources_path, Example, game_data)?;

    game.run();

    Ok(())
}
