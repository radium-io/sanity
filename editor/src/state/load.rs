use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::Transform,
    ecs::prelude::*,
    input::is_close_requested,
    prelude::{Builder, GameData, SimpleState, SimpleTrans, StateData, Trans, WorldExt},
    renderer::camera::Camera,
    ui::{
        get_default_font, Anchor, FontAsset, Interactable, LineMode, UiEvent, UiEventType, UiText,
        UiTransform,
    },
    utils::ortho_camera::{CameraNormalizeMode, CameraOrtho},
    window::ScreenDimensions,
    StateEvent,
};
use nfd::Response;

use super::EditState;

#[derive(Debug, Default)]
pub struct LoadState {
    png_button: Option<Entity>,
    ron_button: Option<Entity>,
    png_path: String,
    ron_path: String,
}

impl SimpleState for LoadState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        // register any components that aren't in a system
        world.register::<Handle<sanity_lib::assets::Pairs>>();

        let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };

        // set up an ortho camera that resizes automatically
        world
            .create_entity()
            .with(Transform::default())
            .with(Camera::standard_2d(width, height))
            .with(CameraOrtho::normalized(CameraNormalizeMode::Contain))
            .build();

        let mut ui_transform = UiTransform::new(
            // ...
            String::from("simple_button"), // id
            Anchor::MiddleLeft,            // anchor
            Anchor::MiddleLeft,            // pivot
            0f32,                          // x
            0f32,                          // y
            0f32,                          // z
            200f32,                        // width
            50f32,                         // height
        );

        let font_handle = {
            let loader = world.read_resource::<Loader>();
            let font_storage = world.read_resource::<AssetStorage<FontAsset>>();
            get_default_font(&loader, &font_storage)
        };

        /* Create the text */
        let mut ui_text = UiText::new(
            // ...
            font_handle,          // font
            String::from("PNG"),  // text
            [1.0, 1.0, 1.0, 0.5], // color
            25f32,                // font_size
            LineMode::Single,
            Anchor::Middle,
        );

        /* Building the entity */
        self.png_button = Some(
            world
                .create_entity()
                .with(ui_transform.clone())
                .with(ui_text.clone())
                .with(Interactable)
                .build(),
        );

        ui_transform.local_y = 100f32;
        ui_text.text = String::from("RON");
        self.ron_button = Some(
            world
                .create_entity()
                .with(ui_transform)
                .with(ui_text)
                .with(Interactable)
                .build(),
        );
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        /*if self.progress_counter.is_complete() {
            Trans::Push(Box::new(crate::state::EditState::default()))
        } else {
            Trans::None
        }*/
        Trans::None
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.png_button {
                    let result = nfd::open_file_dialog(Some("png"), None).unwrap();
                    if let Response::Okay(file_path) = result {
                        self.png_path = file_path
                    }
                }

                if Some(target) == self.ron_button {
                    let result = nfd::open_file_dialog(Some("ron"), None).unwrap();
                    if let Response::Okay(file_path) = result {
                        self.ron_path = file_path
                    }
                }

                if !self.png_path.is_empty() && !self.ron_path.is_empty() {
                    Trans::Push(Box::new(EditState {
                        png: self.png_path.clone(),
                        ron: self.ron_path.clone(),
                        ..Default::default()
                    }))
                } else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
    }
}
