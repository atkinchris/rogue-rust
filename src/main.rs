use amethyst::input::{InputBundle, StringBindings};
use amethyst::{
    assets::Processor,
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        sprite_visibility::SpriteVisibilitySortingSystem, types::DefaultBackend, RenderingSystem,
        SpriteSheet,
    },
    utils::{application_root_dir, ortho_camera::CameraOrthoSystem},
    window::WindowBundle,
};

mod components;
mod constants;
mod render;
mod resources;
mod states;
mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources_path = app_root.join("resources");
    let display_config_path = resources_path.join("display.ron");
    let key_bindings_path = resources_path.join("input.ron");

    let render_graph = render::RenderGraph::default();
    let render_system = RenderingSystem::<DefaultBackend, _>::new(render_graph);

    let game_data = GameDataBuilder::default()
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(key_bindings_path)?,
        )?
        .with_bundle(WindowBundle::from_config_path(display_config_path))?
        .with_bundle(TransformBundle::new())?
        .with(
            SpriteVisibilitySortingSystem::new(),
            "sprite_visibility_system",
            &["transform_system"],
        )
        .with(
            Processor::<SpriteSheet>::new(),
            "sprite_sheet_processor",
            &[],
        )
        .with(
            systems::InputSystem,
            "input_parse_system",
            &["input_system"],
        )
        .with(
            systems::EnergySystem,
            "energy_system",
            &["input_parse_system"],
        )
        .with(systems::IntentSystem, "intent_system", &["energy_system"])
        .with(
            systems::MovementSystem,
            "movement_system",
            &["intent_system"],
        )
        .with(CameraOrthoSystem, "camera_ortho_system", &[])
        .with_thread_local(render_system);

    let mut game = Application::new(resources_path, states::Gameplay, game_data)?;
    game.run();

    Ok(())
}
