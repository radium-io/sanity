use std::time::Duration;

use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationSet, AnimationSetPrefab, EndControl,
    },
    assets::{PrefabLoader, ProgressCounter, RonFormat},
    core::{Time, Transform},
    ecs::{Entities, Entity, WriteStorage},
    prelude::*,
    renderer::rendy::mesh::{Normal, Position, Tangent, TexCoord},
    shred::Read,
    ui::{
        Anchor, LineMode, UiCreator, UiFinder, UiLabel, UiLabelBuilder, UiPrefab, UiText,
        UiTextData,
    },
    utils::scene::BasicScenePrefab,
};
use serde::{Deserialize, Serialize};

use crate::gamedata::CustomGameData;
#[derive(Eq, PartialOrd, PartialEq, Hash, Debug, Copy, Clone, Deserialize, Serialize)]
pub enum AnimationId {
    Scale,
    Rotate,
    Translate,
    Test,
}

pub type StoryPrefab = (
    Option<BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>>,
    Option<AnimationSetPrefab<AnimationId, Transform>>,
);

#[derive(Default)]
pub struct IntroState {
    prog: ProgressCounter,
    moon: Option<Entity>,
    story: Option<Entity>,
    text: Vec<String>,
    zoom_started: bool,
}

impl<'a, 'b> State<crate::gamedata::CustomGameData<'a, 'b>, StateEvent> for IntroState {
    fn on_stop(&mut self, data: StateData<'_, CustomGameData<'a, 'b>>) {
        data.world.exec(|entities: Entities<'_>| {
            entities.delete(self.moon.unwrap());
            entities.delete(self.story.unwrap());
        });
    }

    fn on_start(&mut self, data: StateData<'_, CustomGameData<'a, 'b>>) {
        let StateData { mut world, .. } = data;

        let prefab_handle = world.exec(|loader: PrefabLoader<'_, StoryPrefab>| {
            loader.load("story.ron", RonFormat, &mut self.prog)
        });

        self.moon = Some(world.create_entity().with(prefab_handle).build());

        world.exec(|mut creator: UiCreator<'_>| {
            self.story = Some(creator.create("ui/story_text.ron", ()));
        });
    }

    fn update(
        &mut self,
        data: StateData<'_, CustomGameData<'_, '_>>,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        let StateData { mut world, .. } = data;

        data.data.update(world, false);

        if self.prog.is_complete() {
            if !self.zoom_started {
                // start approach

                let animation = world
                    .read_storage::<AnimationSet<AnimationId, Transform>>()
                    .get(self.moon.unwrap())
                    .and_then(|s| s.get(&AnimationId::Translate))
                    .cloned()
                    .unwrap();
                let mut sets = world.write_storage();
                let control_set =
                    get_animation_set::<AnimationId, Transform>(&mut sets, self.moon.unwrap())
                        .unwrap();
                control_set.add_animation(
                    AnimationId::Translate,
                    &animation,
                    EndControl::Stay,
                    1.0,
                    AnimationCommand::Start,
                );

                self.zoom_started = true;
            }

            // start intro terminal
            world.exec(|data: (Read<'_, Time>, WriteStorage<'_, UiText>)| {
                let (time, mut ui_text) = data;

                if let Some(story_text) = ui_text.get_mut(self.story.unwrap()) {
                    if time.frame_number() % 30 == 0 {
                        story_text.text += "\nBooted";
                    }
                }
            });

            let time = world.read_resource::<Time>();

            if time.absolute_time() > Duration::from_secs(10) {
                return Trans::Switch(Box::new(super::LoadingState::default()));
            }
        }

        Trans::None
    }
}
