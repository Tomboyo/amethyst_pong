mod pong;
mod systems;

use amethyst::{
    core::TransformBundle,
    input::{InputBundle, StringBindings},
    renderer::{types::DefaultBackend, RenderFlat2D, RenderToWindow, RenderingBundle},
    utils::application_root_dir,
    Application, GameDataBuilder,
};

use pong::Pong;
use systems::{
    BallSpawnTimeoutSystem, BounceBallsSystem, MoveBallsSystem, PaddleSystem, WinnerSystem,
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let root_dir = application_root_dir()?;
    let config_dir = root_dir.join("config");
    let display_config_path = config_dir.join("display.ron");
    let binding_path = config_dir.join("bindings.ron");
    let assets_dir = root_dir.join("assets");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        // TODO: StirngBindings is discouraged
        .with_bundle(InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?)?
        .with(PaddleSystem, "paddle_system", &["input_system"])
        .with(MoveBallsSystem, "move_balls_system", &[])
        .with(
            BounceBallsSystem,
            "bounce_balls_system",
            &["paddle_system", "move_balls_system"],
        )
        .with(BallSpawnTimeoutSystem, "ball_spawn_timeout_system", &[])
        .with(WinnerSystem, "winner_system", &["move_balls_system"]);

    let mut game = Application::new(assets_dir, Pong::default(), game_data)?;

    game.run();

    Ok(())
}
