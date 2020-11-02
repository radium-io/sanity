use amethyst::{
    assets::{
        Completion, Handle, HotReloadBundle, Prefab, PrefabLoader, PrefabLoaderSystemDesc,
        ProgressCounter, RonFormat,
    },
    core::{
        math::{UnitQuaternion, Vector3},
        timing::Time,
        transform::{Transform, TransformBundle},
    },
    derive::SystemDesc,
    ecs::prelude::{
        Entity, Join, Read, ReadStorage, System, SystemData, WorldExt, Write, WriteStorage,
    },
    input::{
        get_key, is_close_requested, is_key_down, ElementState, InputBundle, StringBindings,
        VirtualKeyCode,
    },
    prelude::*,
    renderer::{
        light::Light,
        palette::{Srgb, Srgba},
        plugins::{RenderShaded3D, RenderToWindow},
        rendy::mesh::{Normal, Position, TexCoord},
        resources::AmbientColor,
        types::DefaultBackend,
        Camera, RenderingBundle,
    },
    ui::{RenderUi, UiBundle, UiCreator, UiFinder, UiText},
    utils::{
        application_root_dir,
        fps_counter::{FpsCounter, FpsCounterBundle},
        scene::BasicScenePrefab,
    },
    Error,
};

#[derive(Default, SystemDesc)]
pub struct ExampleSystem {
    fps_display: Option<Entity>,
}

impl<'a> System<'a> for ExampleSystem {
    type SystemData = (
        Read<'a, Time>,
        WriteStorage<'a, UiText>,
        Read<'a, FpsCounter>,
        UiFinder<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (time, mut ui_text, fps_counter, finder) = data;

        if self.fps_display.is_none() {
            if let Some(fps_entity) = finder.find("fps_text") {
                self.fps_display = Some(fps_entity);
            }
        }
        if let Some(fps_entity) = self.fps_display {
            if let Some(fps_display) = ui_text.get_mut(fps_entity) {
                if time.frame_number() % 20 == 0 {
                    let fps = fps_counter.sampled_fps();
                    fps_display.text = format!("FPS: {:.*}", 2, fps);
                }
            }
        }
    }
}
