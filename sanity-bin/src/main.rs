use std::{fmt::Debug, marker::PhantomData};

use amethyst::{
    assets::{Handle, Processor},
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle, Hidden},
    ecs::{ReadStorage, SystemData, World},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        bundle::{RenderOrder, RenderPlan, Target},
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        Backend, Factory, RenderGroupDesc, RenderPlugin, RenderingBundle, SpriteSheet, Texture,
    },
    shred::DispatcherBuilder,
    tiles::{
        CoordinateEncoder, DrawTiles2DBounds, DrawTiles2DBoundsDefault, DrawTiles2DDesc,
        MortonEncoder2D, Tile, TileMap,
    },
    ui::{RenderUi, UiBundle},
    utils::{application_root_dir, fps_counter::FpsCounterBundle},
    Result,
};

mod component;
mod map;
mod resource;
mod state;
mod system;

use amethyst::{
    assets::{HotReloadBundle, HotReloadStrategy},
    utils::ortho_camera::CameraOrthoSystem,
};

fn main() -> Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let game_data = GameDataBuilder::default()
        .with(CameraOrthoSystem::default(), "ortho_camera_system", &[])
        .with_bundle(HotReloadBundle::new(HotReloadStrategy::every(2)))?
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new()
                .with_bindings_from_file(&app_root.join("config/input.ron"))?,
        )?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(FpsCounterBundle::default())?
        .with(system::fps::FPSSystem::default(), "fps_system", &[])
        .with(
            system::visibility::VisibilitySystem::default(),
            "visibility_system",
            &[],
        )
        .with(
            system::shooting::ShootingSystem::default(),
            "shooting_system",
            &["input_system"],
        )
        .with(
            system::player::PlayerSystem::default(),
            "player_system",
            &["input_system"],
        )
        .with(
            system::movement::MovementSystem::default(),
            "movement_system",
            &["input_system", "player_system", "shooting_system"],
        )
        .with(
            system::collision::CollisionSystem::default(),
            "collision_system",
            &["movement_system"],
        )
        .with(Processor::<sanity_lib::assets::Pairs>::new(), "", &[])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(app_root.join("config/display_config.ron"))?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderTiles2D::<sanity_lib::tile::RoomTile>::default()),
        )?;

    let mut game = Application::build(
        app_root.parent().unwrap().join("assets"),
        state::room::RoomState::new(48, 32),
    )?
    .with_frame_limit(FrameRateLimitStrategy::Yield, 101)
    .build(game_data)?;

    game.run();

    Ok(())
}

/// A `RenderPlugin` for rendering a 2D Tiles entity.
#[derive(Clone, Default)]
pub struct RenderTiles2D<
    T: Tile,
    E: CoordinateEncoder = MortonEncoder2D,
    Z: DrawTiles2DBounds = DrawTiles2DBoundsDefault,
> {
    target: Target,
    _marker: PhantomData<(T, E, Z)>,
}

impl<T: Tile, E: CoordinateEncoder, Z: DrawTiles2DBounds> Debug for RenderTiles2D<T, E, Z> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<T: Tile, E: CoordinateEncoder, Z: DrawTiles2DBounds> RenderTiles2D<T, E, Z> {
    /// Select render target on which Tiles should be rendered.
    #[must_use]
    pub fn with_target(mut self, target: Target) -> Self {
        self.target = target;
        self
    }
}

type SetupData<'a, T, E> = (
    ReadStorage<'a, Handle<SpriteSheet>>,
    ReadStorage<'a, Handle<Texture>>,
    ReadStorage<'a, Hidden>,
    ReadStorage<'a, TileMap<T, E>>,
);
impl<B: Backend, T: Tile, E: CoordinateEncoder, Z: DrawTiles2DBounds> RenderPlugin<B>
    for RenderTiles2D<T, E, Z>
{
    fn on_build<'a, 'b>(
        &mut self,
        world: &mut World,
        _builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<()> {
        SetupData::<T, E>::setup(world);

        Ok(())
    }

    fn on_plan(
        &mut self,
        plan: &mut RenderPlan<B>,
        _factory: &mut Factory<B>,
        _res: &World,
    ) -> Result<()> {
        plan.extend_target(self.target, |ctx| {
            ctx.add(
                RenderOrder::Transparent,
                DrawTiles2DDesc::<T, E, Z>::default().builder(),
            )?;
            Ok(())
        });
        Ok(())
    }
}
